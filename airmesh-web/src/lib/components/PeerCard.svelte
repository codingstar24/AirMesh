<script lang="ts">
  import type { DiscoveredPeer } from '../types';
  export let peer: DiscoveredPeer;

  function methodIcon(m: string): string {
    switch (m) {
      case 'Mdns': return '📡 mDNS';
      case 'UdpBroadcast': return '📻 UDP Bcast';
      case 'QrCode': return '📷 QR Optical';
      case 'Ble': return '📶 Bluetooth';
      default: return '🔗 Direct';
    }
  }
</script>

<div class="peer-card" tabindex="0" role="region" aria-label={`Peer ${peer.device_name}`}>
  <div class="peer-header">
    <span class="device-icon">💻</span>
    <div>
      <div class="device-name">{peer.device_name}</div>
      <div class="node-id">{peer.node_id.substring(0, 16)}...</div>
    </div>
  </div>
  <div class="peer-details">
    <div class="detail-row">
      <span>Address:</span>
      <code>{peer.address}</code>
    </div>
    <div class="detail-row">
      <span>Discovery:</span>
      <span class="method-tag">{methodIcon(peer.method)}</span>
    </div>
  </div>
  <div class="status-indicator">
    <span class="online-dot"></span> Online on Local Subnet
  </div>
</div>

<style>
  .peer-card {
    background: #111827;
    border: 1px solid #1f2937;
    border-radius: 0.75rem;
    padding: 1.25rem;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    transition: all 0.2s ease;
  }
  .peer-card:hover, .peer-card:focus {
    border-color: #38bdf8;
    transform: translateY(-2px);
    outline: none;
    box-shadow: 0 0 0 2px rgba(56, 189, 248, 0.4);
  }
  .peer-header {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }
  .device-icon {
    font-size: 1.5rem;
  }
  .device-name {
    font-weight: 600;
    font-size: 1.05rem;
    color: #f8fafc;
  }
  .node-id {
    font-size: 0.75rem;
    color: #94a3b8;
    font-family: monospace;
  }
  .peer-details {
    display: flex;
    flex-direction: column;
    gap: 0.35rem;
    font-size: 0.85rem;
    color: #94a3b8;
  }
  .detail-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  .method-tag {
    background: rgba(56, 189, 248, 0.1);
    color: #38bdf8;
    padding: 0.15rem 0.45rem;
    border-radius: 4px;
    font-size: 0.75rem;
  }
  .status-indicator {
    display: flex;
    align-items: center;
    gap: 0.4rem;
    font-size: 0.8rem;
    color: #10b981;
    font-weight: 500;
    margin-top: 0.25rem;
  }
  .online-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: #10b981;
    box-shadow: 0 0 8px #10b981;
  }
</style>
