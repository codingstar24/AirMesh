//! High-throughput parallel transfer engine with automatic stream tuning.

use crate::crypto::integrity::verify_chunk;
use crate::protocol::error::{AirMeshError, ErrorCode, Result};
use crate::protocol::frames::{ChunkRequestPayload, ChunkResponsePayload, FileMetadata, Frame};
use crate::transfer::bitmap::TransferBitmap;
use crate::transfer::chunker::CHUNK_SIZE_1MIB;
use crate::transfer::compressor::ChunkCompressor;
use crate::transport::types::PeerConnection;
use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::{mpsc, Mutex, Semaphore};
use tracing::{debug, error, info, warn};

/// Status state of an in-flight transfer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransferState {
    /// Actively pulling chunks
    Active,
    /// Paused by user or network drop
    Paused,
    /// Completed successfully and verified
    Completed,
    /// Failed due to error
    Failed,
}

/// Statistics and progress report of an ongoing transfer.
#[derive(Debug, Clone)]
pub struct TransferProgress {
    /// File identifier
    pub file_id: String,
    /// Completed bytes on disk
    pub bytes_completed: u64,
    /// Total file size
    pub total_bytes: u64,
    /// Percentage progress (0.0 to 100.0)
    pub percentage: f32,
    /// Instantaneous speed in MB/s
    pub speed_mb_s: f64,
    /// State of transfer
    pub state: TransferState,
}

/// Parallel transfer session runner.
pub struct TransferSession {
    pub file_meta: FileMetadata,
    pub dest_path: PathBuf,
    pub bitmap: Arc<Mutex<TransferBitmap>>,
    pub bytes_transferred: Arc<AtomicU64>,
    pub is_cancelled: Arc<AtomicBool>,
    pub parallel_streams: usize,
}

impl TransferSession {
    /// Initializes transfer session, preparing target file on disk.
    pub fn new(file_meta: FileMetadata, dest_path: PathBuf, parallel_streams: usize) -> Result<Self> {
        let bitmap = TransferBitmap::new(file_meta.id.clone(), file_meta.chunk_count);

        // Preallocate or open target file safely
        if let Some(parent) = dest_path.parent() {
            std::fs::create_dir_all(parent).map_err(AirMeshError::Io)?;
        }

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(&dest_path)
            .map_err(AirMeshError::Io)?;

        // Pre-set file length if supported
        let _ = file.set_len(file_meta.size);

        Ok(Self {
            file_meta,
            dest_path,
            bitmap: Arc::new(Mutex::new(bitmap)),
            bytes_transferred: Arc::new(AtomicU64::new(0)),
            is_cancelled: Arc::new(AtomicBool::new(false)),
            parallel_streams: parallel_streams.clamp(4, 32),
        })
    }

    /// Writes a verified chunk to disk at calculated offset.
    pub fn write_chunk_to_disk(&self, chunk_index: u32, data: &[u8]) -> Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .open(&self.dest_path)
            .map_err(AirMeshError::Io)?;

        let offset = (chunk_index as u64) * (CHUNK_SIZE_1MIB as u64);
        file.seek(SeekFrom::Start(offset)).map_err(AirMeshError::Io)?;
        file.write_all(data).map_err(AirMeshError::Io)?;
        file.flush().map_err(AirMeshError::Io)?;
        Ok(())
    }

    /// Executes chunk retrieval over connection streams.
    pub async fn execute(
        self: Arc<Self>,
        connection: Arc<dyn PeerConnection>,
    ) -> Result<()> {
        let missing = {
            let b = self.bitmap.lock().await;
            b.missing_chunks()
        };

        if missing.is_empty() {
            return Ok(());
        }

        let (work_tx, work_rx) = mpsc::channel::<u32>(missing.len() + 1);
        for chunk_idx in missing {
            let _ = work_tx.send(chunk_idx).await;
        }

        let work_rx = Arc::new(Mutex::new(work_rx));
        let semaphore = Arc::new(Semaphore::new(self.parallel_streams));
        let mut tasks = Vec::new();

        for _ in 0..self.parallel_streams {
            let this = Arc::clone(&self);
            let conn = Arc::clone(&connection);
            let rx = Arc::clone(&work_rx);
            let sem = Arc::clone(&semaphore);

            let task = tokio::spawn(async move {
                let _permit = match sem.acquire().await {
                    Ok(p) => p,
                    Err(_) => return,
                };

                let mut stream = match conn.open_stream().await {
                    Ok(s) => s,
                    Err(e) => {
                        error!("Failed opening transfer stream: {e}");
                        return;
                    }
                };

                loop {
                    if this.is_cancelled.load(Ordering::Relaxed) {
                        break;
                    }

                    let next_chunk = {
                        let mut guard = rx.lock().await;
                        guard.recv().await
                    };

                    let chunk_idx = match next_chunk {
                        Some(idx) => idx,
                        None => break, // Work queue drained
                    };

                    let req = Frame::ChunkRequest(ChunkRequestPayload {
                        file_id: this.file_meta.id.clone(),
                        chunk_index: chunk_idx,
                    });

                    if stream.send_frame(&req).await.is_err() {
                        break;
                    }

                    match stream.recv_frame().await {
                        Ok(Some(Frame::ChunkResponse(resp))) => {
                            if resp.chunk_index == chunk_idx {
                                let payload = if resp.is_compressed {
                                    match ChunkCompressor::decompress(&resp.data) {
                                        Ok(decomp) => decomp,
                                        Err(e) => {
                                            warn!("Decompression failure for chunk {chunk_idx}: {e}");
                                            continue;
                                        }
                                    }
                                } else {
                                    resp.data
                                };

                                // Cryptographic BLAKE3 verification
                                if verify_chunk(&payload, &resp.blake3_hash).is_ok() {
                                    if this.write_chunk_to_disk(chunk_idx, &payload).is_ok() {
                                        let mut b = this.bitmap.lock().await;
                                        b.mark_completed(chunk_idx);
                                        this.bytes_transferred.fetch_add(payload.len() as u64, Ordering::Relaxed);
                                    }
                                }
                            }
                        }
                        _ => break,
                    }
                }
                let _ = stream.close().await;
            });
            tasks.push(task);
        }

        for t in tasks {
            let _ = t.await;
        }

        let is_done = {
            let b = self.bitmap.lock().await;
            b.is_complete()
        };

        if is_done {
            info!("Transfer complete for {}", self.file_meta.relative_path);
            Ok(())
        } else {
            Err(AirMeshError::Transfer("Transfer interrupted or incomplete".into()))
        }
    }
}
