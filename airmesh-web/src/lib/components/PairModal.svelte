<script lang="ts">
  import { pairModalOpenStore } from '../store';
  import { pairPeer } from '../api';

  let peerNodeId = '';
  let pinCode = '';
  let isPairing = false;
  let resultMsg = '';

  function close() {
    pairModalOpenStore.set(false);
    resultMsg = '';
  }

  async function handlePair() {
    if (!peerNodeId || pinCode.length !== 6) {
      resultMsg = 'Please provide peer Node ID and 6-digit PIN';
      return;
    }
    isPairing = true;
    try {
      const salt = '00000000000000000000000000000000';
      await pairPeer(peerNodeId, pinCode, salt);
      resultMsg = '✅ Peer successfully paired and trusted!';
      setTimeout(close, 1500);
    } catch (e: any) {
      resultMsg = `❌ Error: ${e.message}`;
    } finally {
      isPairing = false;
    }
  }
</script>

{#if $pairModalOpenStore}
  <div class="modal-backdrop" on:click={close} on:keydown={(e) => e.key === 'Escape' && close()} role="dialog" aria-modal="true" aria-labelledby="pair-modal-title">
    <div class="modal-content" on:click|stopPropagation>
      <div class="modal-header">
        <h3 id="pair-modal-title">🔐 PAKE Device Pairing (SPAKE2)</h3>
        <button class="btn-close" on:click={close} aria-label="Close modal">✕</button>
      </div>
      <div class="modal-body">
        <p class="desc">
          Pair with another device on this WiFi using a one-time 6-digit code. Zero passwords sent over wire.
        </p>

        <div class="form-group">
          <label for="peerNodeId">Target Peer Node ID</label>
          <input id="peerNodeId" bind:value={peerNodeId} placeholder="e.g. 7f8c92a1..." class="input" />
        </div>

        <div class="form-group">
          <label for="pinCode">6-Digit Pairing PIN</label>
          <input id="pinCode" bind:value={pinCode} maxlength="6" placeholder="849201" class="input pin-input" />
        </div>

        {#if resultMsg}
          <div class="status-msg">{resultMsg}</div>
        {/if}
      </div>
      <div class="modal-footer">
        <button class="btn-cancel" on:click={close}>Cancel</button>
        <button class="btn-submit" on:click={handlePair} disabled={isPairing}>
          {isPairing ? 'Verifying...' : 'Confirm Pairing'}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.85);
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
    max-width: 480px;
    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.5);
  }
  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.25rem;
    border-bottom: 1px solid #1f2937;
    color: #f8fafc;
  }
  .btn-close {
    background: none;
    border: none;
    color: #94a3b8;
    font-size: 1.25rem;
    cursor: pointer;
  }
  .modal-body {
    padding: 1.25rem;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  .desc {
    font-size: 0.85rem;
    color: #94a3b8;
    line-height: 1.4;
  }
  .form-group {
    display: flex;
    flex-direction: column;
    gap: 0.35rem;
  }
  label {
    font-size: 0.8rem;
    font-weight: 600;
    color: #cbd5e1;
  }
  .input {
    background: #0f172a;
    border: 1px solid #1f2937;
    padding: 0.75rem 1rem;
    border-radius: 0.5rem;
    color: #f8fafc;
    font-size: 0.95rem;
  }
  .pin-input {
    font-size: 1.25rem;
    letter-spacing: 0.25rem;
    text-align: center;
    font-family: monospace;
  }
  .input:focus {
    outline: none;
    border-color: #38bdf8;
    box-shadow: 0 0 0 2px rgba(56, 189, 248, 0.3);
  }
  .status-msg {
    font-size: 0.85rem;
    color: #38bdf8;
    padding: 0.5rem;
    background: rgba(56, 189, 248, 0.1);
    border-radius: 0.375rem;
  }
  .modal-footer {
    display: flex;
    justify-content: flex-end;
    gap: 0.75rem;
    padding: 1rem 1.25rem;
    background: #0b0f19;
    border-top: 1px solid #1f2937;
  }
  .btn-cancel {
    background: #1f2937;
    color: #f8fafc;
    border: none;
    padding: 0.6rem 1rem;
    border-radius: 0.375rem;
    font-weight: 600;
    cursor: pointer;
  }
  .btn-submit {
    background: #38bdf8;
    color: #0f172a;
    border: none;
    padding: 0.6rem 1.25rem;
    border-radius: 0.375rem;
    font-weight: 600;
    cursor: pointer;
  }
</style>
