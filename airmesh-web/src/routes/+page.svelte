<script lang="ts">
  import { onMount } from 'svelte';
  import { peersStore, filesStore, searchQueryStore } from '$lib/store';
  import { fetchPeers, fetchFiles, searchFiles } from '$lib/api';
  import PeerCard from '$lib/components/PeerCard.svelte';
  import FileList from '$lib/components/FileList.svelte';

  let debounceTimer: any;

  onMount(() => {
    refreshAll();
    const interval = setInterval(refreshPeers, 3000);
    return () => clearInterval(interval);
  });

  async function refreshAll() {
    refreshPeers();
    refreshFiles();
  }

  async function refreshPeers() {
    try {
      const p = await fetchPeers();
      peersStore.set(p);
    } catch {}
  }

  async function refreshFiles() {
    try {
      const f = await fetchFiles();
      filesStore.set(f);
    } catch {}
  }

  function handleSearchInput(e: Event) {
    const val = (e.target as HTMLInputElement).value;
    searchQueryStore.set(val);

    clearTimeout(debounceTimer);
    debounceTimer = setTimeout(async () => {
      if (!val.trim()) {
        refreshFiles();
      } else {
        try {
          const hits = await searchFiles(val);
          // Map search hits to file format
          filesStore.set(hits.map(h => ({
            id: h.id,
            local_path: '',
            virtual_path: h.virtual_path,
            size: h.size,
            mime_type: h.mime_type,
            modified_at: 0,
            chunk_count: Math.ceil(h.size / 1048576) || 1,
            root_hash: h.root_hash
          })));
        } catch {}
      }
    }, 150);
  }
</script>

<div class="search-section">
  <div class="search-bar">
    <span class="search-icon">🔍</span>
    <input
      type="text"
      placeholder="Instant search across 10M+ local & mesh files (<50ms)..."
      on:input={handleSearchInput}
      aria-label="Search files"
    />
  </div>
</div>

<section aria-labelledby="peers-heading">
  <div class="section-header">
    <h2 id="peers-heading">Online Mesh Peers ({$peersStore.length})</h2>
    <span class="subtext">Auto-discovered via mDNS + UDP Broadcast</span>
  </div>

  {#if $peersStore.length === 0}
    <div class="no-peers">
      <p>Looking for peers on local WiFi... Open AirMesh on any other phone, laptop, or browser.</p>
    </div>
  {:else}
    <div class="peers-grid">
      {#each $peersStore as peer}
        <PeerCard {peer} />
      {/each}
    </div>
  {/if}
</section>

<section aria-labelledby="files-heading">
  <div class="section-header">
    <h2 id="files-heading">Available Files ({$filesStore.length})</h2>
    <button class="btn-refresh" on:click={refreshFiles} aria-label="Refresh catalog">↻ Scan Directory</button>
  </div>

  <FileList files={$filesStore} />
</section>

<div class="drag-drop-zone">
  <p class="drop-title">📁 Drag and drop files or folders here</p>
  <p class="drop-desc">Automatically chunked (1 MiB), indexed with BLAKE3, and streamed on demand</p>
</div>

<style>
  .search-section {
    margin-bottom: 0.5rem;
  }
  .search-bar {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    background: #111827;
    border: 1px solid #1f2937;
    border-radius: 0.5rem;
    padding: 0.85rem 1.25rem;
  }
  .search-icon {
    font-size: 1.1rem;
    color: #94a3b8;
  }
  .search-bar input {
    flex: 1;
    background: none;
    border: none;
    color: #f8fafc;
    font-size: 1rem;
    outline: none;
  }
  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.85rem;
  }
  h2 {
    font-size: 1.15rem;
    font-weight: 600;
    color: #f8fafc;
  }
  .subtext {
    font-size: 0.8rem;
    color: #94a3b8;
  }
  .btn-refresh {
    background: #1f2937;
    border: none;
    color: #cbd5e1;
    padding: 0.4rem 0.75rem;
    border-radius: 0.375rem;
    font-size: 0.8rem;
    cursor: pointer;
  }
  .btn-refresh:hover {
    background: #374151;
  }
  .peers-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
    gap: 1rem;
  }
  .no-peers {
    background: #111827;
    border: 1px solid #1f2937;
    border-radius: 0.5rem;
    padding: 1.5rem;
    text-align: center;
    color: #94a3b8;
    font-size: 0.9rem;
  }
  .drag-drop-zone {
    border: 2px dashed #1f2937;
    border-radius: 0.75rem;
    padding: 2.5rem;
    text-align: center;
    background: rgba(17, 24, 39, 0.4);
    cursor: pointer;
    transition: border-color 0.2s;
  }
  .drag-drop-zone:hover {
    border-color: #38bdf8;
  }
  .drop-title {
    font-size: 1.05rem;
    font-weight: 600;
    color: #f8fafc;
    margin-bottom: 0.35rem;
  }
  .drop-desc {
    font-size: 0.85rem;
    color: #94a3b8;
  }
</style>
