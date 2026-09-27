<script lang="ts">
  import { activeStreamStore } from '../store';

  function close() {
    activeStreamStore.set(null);
  }
</script>

{#if $activeStreamStore}
  <div class="modal-backdrop" on:click={close} on:keydown={(e) => e.key === 'Escape' && close()} role="dialog" aria-modal="true" aria-labelledby="stream-title">
    <div class="modal-content" on:click|stopPropagation>
      <div class="modal-header">
        <h3 id="stream-title" class="title">🎬 {$activeStreamStore.title}</h3>
        <button class="btn-close" on:click={close} aria-label="Close media player">✕</button>
      </div>
      <div class="video-wrapper">
        <video controls autoplay src={`/api/stream/${$activeStreamStore.fileId}`}>
          <track kind="captions" />
          Your browser does not support the video tag.
        </video>
      </div>
      <div class="modal-footer">
        <span class="stream-badge">⚡ Instant Range Stream (0 Pre-download)</span>
        <a class="btn-direct-download" href={`/api/stream/${$activeStreamStore.fileId}`} download>Download Copy</a>
      </div>
    </div>
  </div>
{/if}

<style>
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.85);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 500;
    padding: 1.5rem;
  }
  .modal-content {
    background: #111827;
    border: 1px solid #1f2937;
    border-radius: 0.75rem;
    width: 100%;
    max-width: 850px;
    overflow: hidden;
    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.5);
  }
  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem 1.25rem;
    border-bottom: 1px solid #1f2937;
  }
  .title {
    font-size: 1.05rem;
    color: #f8fafc;
    font-weight: 600;
  }
  .btn-close {
    background: none;
    border: none;
    color: #94a3b8;
    font-size: 1.25rem;
    cursor: pointer;
    padding: 0.25rem;
  }
  .btn-close:hover {
    color: #f8fafc;
  }
  .video-wrapper {
    background: #000;
  }
  video {
    width: 100%;
    max-height: 65vh;
    display: block;
  }
  .modal-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.85rem 1.25rem;
    background: #0b0f19;
    border-top: 1px solid #1f2937;
    font-size: 0.85rem;
  }
  .stream-badge {
    color: #38bdf8;
    font-weight: 500;
  }
  .btn-direct-download {
    color: #94a3b8;
    text-decoration: underline;
  }
  .btn-direct-download:hover {
    color: #f8fafc;
  }
</style>
