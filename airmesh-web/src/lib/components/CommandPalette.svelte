<script lang="ts">
  import { commandPaletteOpenStore, pairModalOpenStore } from '../store';

  let filter = '';

  const actions = [
    { id: 'pair', title: 'Pair New Device (PAKE PIN)', icon: '🔐', run: () => pairModalOpenStore.set(true) },
    { id: 'refresh', title: 'Scan WiFi Mesh Now', icon: '📡', run: () => window.location.reload() },
    { id: 'docs', title: 'Open Protocol Documentation', icon: '📖', run: () => window.open('/docs/PROTOCOL.md', '_blank') },
  ];

  function close() {
    commandPaletteOpenStore.set(false);
  }

  function execute(action: typeof actions[0]) {
    close();
    action.run();
  }
</script>

{#if $commandPaletteOpenStore}
  <div class="palette-backdrop" on:click={close} on:keydown={(e) => e.key === 'Escape' && close()} role="dialog" aria-modal="true" aria-labelledby="cmd-palette-title">
    <div class="palette-box" on:click|stopPropagation>
      <div class="search-row">
        <span class="icon">🔍</span>
        <input
          id="cmd-palette-title"
          bind:value={filter}
          placeholder="Type a command or search action (ESC to exit)..."
          class="palette-input"
          autofocus
        />
      </div>
      <div class="actions-list" role="menu">
        {#each actions as action}
          <button class="action-item" role="menuitem" on:click={() => execute(action)}>
            <span class="action-icon">{action.icon}</span>
            <span class="action-title">{action.title}</span>
          </button>
        {/each}
      </div>
    </div>
  </div>
{/if}

<style>
  .palette-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.8);
    display: flex;
    align-items: flex-start;
    justify-content: center;
    padding-top: 15vh;
    z-index: 600;
  }
  .palette-box {
    background: #111827;
    border: 1px solid #1f2937;
    border-radius: 0.75rem;
    width: 100%;
    max-width: 550px;
    overflow: hidden;
    box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.7);
  }
  .search-row {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 1rem 1.25rem;
    border-bottom: 1px solid #1f2937;
  }
  .icon {
    font-size: 1.1rem;
    color: #94a3b8;
  }
  .palette-input {
    background: none;
    border: none;
    flex: 1;
    color: #f8fafc;
    font-size: 1rem;
    outline: none;
  }
  .actions-list {
    display: flex;
    flex-direction: column;
    padding: 0.5rem;
  }
  .action-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem 1rem;
    background: none;
    border: none;
    border-radius: 0.375rem;
    color: #f8fafc;
    font-size: 0.95rem;
    cursor: pointer;
    text-align: left;
    transition: background 0.15s;
  }
  .action-item:hover, .action-item:focus {
    background: #1f2937;
    outline: none;
    color: #38bdf8;
  }
</style>
