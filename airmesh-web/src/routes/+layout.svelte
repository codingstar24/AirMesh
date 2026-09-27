<script lang="ts">
  import { onMount } from 'svelte';
  import { healthStore, commandPaletteOpenStore, pairModalOpenStore } from '$lib/store';
  import { fetchHealth } from '$lib/api';
  import CommandPalette from '$lib/components/CommandPalette.svelte';
  import PairModal from '$lib/components/PairModal.svelte';
  import StreamPlayer from '$lib/components/StreamPlayer.svelte';

  onMount(() => {
    fetchHealth().then(h => healthStore.set(h)).catch(() => {});
    const interval = setInterval(() => {
      fetchHealth().then(h => healthStore.set(h)).catch(() => {});
    }, 5000);

    const handleKeydown = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
        e.preventDefault();
        commandPaletteOpenStore.update(v => !v);
      }
    };
    window.addEventListener('keydown', handleKeydown);

    return () => {
      clearInterval(interval);
      window.removeEventListener('keydown', handleKeydown);
    };
  });
</script>

<div class="app-shell">
  <header>
    <div class="brand">
      <span class="logo">🌐⚡ AirMesh</span>
      <span class="badge"><span class="pulse"></span> 100% Offline Mesh</span>
    </div>
    <div class="header-right">
      {#if $healthStore}
        <span class="node-badge">{$healthStore.device_name} ({$healthStore.peer_count} online)</span>
      {/if}
      <button class="btn-pair" on:click={() => pairModalOpenStore.set(true)}>Pair (PAKE)</button>
    </div>
  </header>

  <main>
    <slot />
  </main>

  <footer>
    AirMesh v1.0.0 &bull; Offline-First Peer-to-Peer &bull; Press <kbd>Ctrl+K</kbd> for Command Palette
  </footer>

  <CommandPalette />
  <PairModal />
  <StreamPlayer />
</div>

<style>
  :global(:root) {
    --bg: #090d16;
    --surface: #111827;
    --border: #1f2937;
    --text: #f8fafc;
    --text-muted: #94a3b8;
    --accent: #38bdf8;
  }
  :global(*) { box-sizing: border-box; margin: 0; padding: 0; }
  :global(body) {
    background: var(--bg);
    color: var(--text);
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
    min-height: 100vh;
  }
  .app-shell {
    display: flex;
    flex-direction: column;
    min-height: 100vh;
  }
  header {
    background: var(--surface);
    border-bottom: 1px solid var(--border);
    padding: 1rem 1.5rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
    position: sticky;
    top: 0;
    z-index: 100;
  }
  .brand {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }
  .logo {
    font-weight: 700;
    font-size: 1.25rem;
    color: var(--accent);
  }
  .badge {
    background: rgba(16, 185, 129, 0.15);
    color: #10b981;
    border: 1px solid rgba(16, 185, 129, 0.3);
    padding: 0.2rem 0.5rem;
    border-radius: 9999px;
    font-size: 0.75rem;
    display: inline-flex;
    align-items: center;
    gap: 0.35rem;
  }
  .pulse {
    width: 6px;
    height: 6px;
    background: #10b981;
    border-radius: 50%;
  }
  .header-right {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }
  .node-badge {
    font-size: 0.85rem;
    color: var(--text-muted);
  }
  .btn-pair {
    background: var(--border);
    color: var(--text);
    border: none;
    padding: 0.5rem 0.85rem;
    border-radius: 0.375rem;
    font-weight: 600;
    cursor: pointer;
    font-size: 0.85rem;
  }
  .btn-pair:hover {
    background: #374151;
  }
  main {
    flex: 1;
    max-width: 1200px;
    width: 100%;
    margin: 0 auto;
    padding: 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }
  footer {
    text-align: center;
    padding: 1.25rem;
    font-size: 0.8rem;
    color: var(--text-muted);
    border-top: 1px solid var(--border);
  }
  kbd {
    background: #1e293b;
    padding: 0.15rem 0.4rem;
    border-radius: 4px;
    border: 1px solid #334155;
  }
</style>
