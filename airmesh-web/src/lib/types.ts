export interface DiscoveredPeer {
  node_id: string;
  device_name: string;
  address: string;
  method: 'Mdns' | 'UdpBroadcast' | 'QrCode' | 'Ble' | 'Manual';
  last_seen: number;
}

export interface IndexedFile {
  id: string;
  local_path: string;
  virtual_path: string;
  size: number;
  mime_type: string;
  modified_at: number;
  chunk_count: number;
  root_hash: string;
}

export interface SearchResult {
  id: string;
  virtual_path: string;
  size: number;
  mime_type: string;
  root_hash: string;
  score: number;
}

export interface HealthInfo {
  status: string;
  node_id: string;
  device_name: string;
  version: string;
  peer_count: number;
}
