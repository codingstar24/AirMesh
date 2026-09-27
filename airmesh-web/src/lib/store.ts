import { writable } from 'svelte/store';
import type { DiscoveredPeer, HealthInfo, IndexedFile } from './types';

export const healthStore = writable<HealthInfo | null>(null);
export const peersStore = writable<DiscoveredPeer[]>([]);
export const filesStore = writable<IndexedFile[]>([]);
export const searchQueryStore = writable<string>('');
export const activeStreamStore = writable<{ fileId: string; title: string } | null>(null);
export const pairModalOpenStore = writable<boolean>(false);
export const commandPaletteOpenStore = writable<boolean>(false);
