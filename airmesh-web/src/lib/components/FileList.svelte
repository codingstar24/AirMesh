<script lang="ts">
  import type { IndexedFile } from '../types';
  import { activeStreamStore } from '../store';

  export let files: IndexedFile[] = [];

  function formatBytes(bytes: number): string {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  }

  function handleStream(file: IndexedFile) {
    activeStreamStore.set({ fileId: file.id, title: file.virtual_path });
  }
</script>

<div class="table-card">
  <table role="table" aria-label="Shared mesh files">
    <thead>
      <tr>
        <th scope="col">File Name</th>
        <th scope="col">Size</th>
        <th scope="col">MIME</th>
        <th scope="col">BLAKE3 Hash</th>
        <th scope="col">Actions</th>
      </tr>
    </thead>
    <tbody>
      {#if files.length === 0}
        <tr>
          <td colspan="5" class="empty-state">No files indexed yet. Drop files into your shared folder.</td>
        </tr>
      {:else}
        {#each files as file}
          <tr tabindex="0">
            <td class="filename">
              <span class="file-icon">📄</span>
              {file.virtual_path}
            </td>
            <td>{formatBytes(file.size)}</td>
            <td><code class="mime-badge">{file.mime_type}</code></td>
            <td class="hash-col">{file.root_hash.substring(0, 12)}...</td>
            <td>
              <div class="actions">
                {#if file.mime_type.startsWith('video/') || file.mime_type.startsWith('audio/')}
                  <button class="btn-action stream" on:click={() => handleStream(file)} aria-label="Stream media without downloading">
                    ▶ Stream
                  </button>
                {/if}
                <a class="btn-action download" href={`/api/stream/${file.id}`} download={file.virtual_path} aria-label="Download full file">
                  ↓ Download
                </a>
              </div>
            </td>
          </tr>
        {/each}
      {/if}
    </tbody>
  </table>
</div>

<style>
  .table-card {
    background: #111827;
    border: 1px solid #1f2937;
    border-radius: 0.75rem;
    overflow-x: auto;
  }
  table {
    width: 100%;
    border-collapse: collapse;
    text-align: left;
  }
  th, td {
    padding: 0.85rem 1.25rem;
    border-bottom: 1px solid #1f2937;
    font-size: 0.9rem;
  }
  th {
    color: #94a3b8;
    font-size: 0.75rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }
  tr:hover, tr:focus {
    background: rgba(255, 255, 255, 0.02);
    outline: none;
  }
  .filename {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-weight: 500;
    color: #f8fafc;
  }
  .file-icon {
    font-size: 1.1rem;
  }
  .mime-badge {
    background: #1e293b;
    padding: 0.2rem 0.4rem;
    border-radius: 4px;
    font-size: 0.75rem;
    color: #cbd5e1;
  }
  .hash-col {
    font-family: monospace;
    color: #64748b;
    font-size: 0.8rem;
  }
  .actions {
    display: flex;
    gap: 0.5rem;
  }
  .btn-action {
    padding: 0.35rem 0.75rem;
    border-radius: 0.375rem;
    font-size: 0.8rem;
    font-weight: 600;
    text-decoration: none;
    cursor: pointer;
    border: none;
    transition: background 0.15s;
    display: inline-flex;
    align-items: center;
  }
  .btn-action.stream {
    background: rgba(56, 189, 248, 0.15);
    color: #38bdf8;
  }
  .btn-action.stream:hover {
    background: rgba(56, 189, 248, 0.3);
  }
  .btn-action.download {
    background: #1f2937;
    color: #f8fafc;
  }
  .btn-action.download:hover {
    background: #374151;
  }
  .empty-state {
    text-align: center;
    padding: 2.5rem;
    color: #64748b;
  }
</style>
