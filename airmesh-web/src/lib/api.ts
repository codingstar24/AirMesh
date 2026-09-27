import type { DiscoveredPeer, HealthInfo, IndexedFile, SearchResult } from './types';

export const API_BASE = '';

export async function fetchHealth(): Promise<HealthInfo> {
  const res = await fetch(`${API_BASE}/api/health`);
  if (!res.ok) throw new Error('Failed fetching health');
  return res.json();
}

export async function fetchPeers(): Promise<DiscoveredPeer[]> {
  const res = await fetch(`${API_BASE}/api/peers`);
  if (!res.ok) throw new Error('Failed fetching peers');
  return res.json();
}

export async function fetchFiles(): Promise<IndexedFile[]> {
  const res = await fetch(`${API_BASE}/api/files`);
  if (!res.ok) throw new Error('Failed fetching files');
  const data = await res.json();
  return data.files || [];
}

export async function searchFiles(query: string, mime?: string): Promise<SearchResult[]> {
  const params = new URLSearchParams({ q: query });
  if (mime) params.append('mime', mime);
  const res = await fetch(`${API_BASE}/api/search?${params.toString()}`);
  if (!res.ok) throw new Error('Search failed');
  const data = await res.json();
  return data.results || [];
}

export async function pairPeer(peerNodeId: string, pin: string, saltHex: string): Promise<any> {
  const res = await fetch(`${API_BASE}/api/peers/pair`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      peer_node_id: peerNodeId,
      pin_code: pin,
      salt_hex: saltHex
    })
  });
  if (!res.ok) throw new Error('Pairing failed');
  return res.json();
}
