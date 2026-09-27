//! AirMesh CLI main entry point.

mod commands;
mod output;

use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "airmesh", author, version, about = "AirMesh 100% Local P2P File Mesh CLI", long_about = None)]
struct Cli {
    /// URL of local daemon HTTP API
    #[arg(short = 'u', long, default_value = "http://127.0.0.1:47823")]
    api_url: String,

    /// Output results in JSON format
    #[arg(long, global = true)]
    json: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Discover peers on local network
    Discover,

    /// List indexed files or active transfers
    List,

    /// Search files across mesh with FTS5 index
    Search {
        /// Search query term
        query: String,

        /// Optional MIME filter (e.g. video/, image/)
        #[arg(short, long)]
        mime: Option<String>,
    },

    /// Send or queue a file to a remote peer
    Send {
        /// Remote peer node ID
        #[arg(short, long)]
        peer: String,

        /// Path to file on local disk
        file: PathBuf,
    },

    /// Download a file by content ID
    Receive {
        /// Target file ID (BLAKE3 root hash)
        file_id: String,

        /// Output path on disk
        #[arg(short, long)]
        output: PathBuf,
    },

    /// Authenticate and pair with a peer using PAKE PIN code
    Pair {
        /// Target peer node ID
        #[arg(short, long)]
        peer: String,

        /// 6-digit numeric pairing code
        #[arg(long)]
        pin: String,
    },

    /// Inspect local daemon status
    Daemon,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Discover => {
            commands::execute_discover(&cli.api_url, cli.json).await?;
        }
        Commands::List => {
            commands::execute_list(&cli.api_url, cli.json).await?;
        }
        Commands::Search { query, mime } => {
            commands::execute_search(&cli.api_url, &query, mime, cli.json).await?;
        }
        Commands::Send { peer, file } => {
            commands::execute_send(&cli.api_url, &peer, file, cli.json).await?;
        }
        Commands::Receive { file_id, output } => {
            commands::execute_receive(&cli.api_url, &file_id, output, cli.json).await?;
        }
        Commands::Pair { peer, pin } => {
            commands::execute_pair(&cli.api_url, &peer, &pin, cli.json).await?;
        }
        Commands::Daemon => {
            commands::execute_daemon_status(&cli.api_url, cli.json).await?;
        }
    }

    Ok(())
}
