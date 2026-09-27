const fs = require('fs');
const path = require('path');

const sharedDir = path.join(__dirname, 'shared');

// 1. Create a pleasant 3-second ambient WAV tone
function createSampleWav(filename) {
  const sampleRate = 8000;
  const durationSec = 3;
  const numSamples = sampleRate * durationSec;
  const dataSize = numSamples * 2; // 16-bit mono = 2 bytes per sample
  const buffer = Buffer.alloc(44 + dataSize);

  // RIFF header
  buffer.write('RIFF', 0);
  buffer.writeUInt32LE(36 + dataSize, 4);
  buffer.write('WAVE', 8);
  buffer.write('fmt ', 12);
  buffer.writeUInt32LE(16, 16); // Subchunk1Size
  buffer.writeUInt16LE(1, 20);  // AudioFormat (PCM)
  buffer.writeUInt16LE(1, 22);  // NumChannels (Mono)
  buffer.writeUInt32LE(sampleRate, 24);
  buffer.writeUInt32LE(sampleRate * 2, 28); // ByteRate
  buffer.writeUInt16LE(2, 32);  // BlockAlign
  buffer.writeUInt16LE(16, 34); // BitsPerSample
  buffer.write('data', 36);
  buffer.writeUInt32LE(dataSize, 40);

  // Gentle melody arpeggio: C4, E4, G4, C5
  const notes = [261.63, 329.63, 392.00, 523.25];
  for (let i = 0; i < numSamples; i++) {
    const t = i / sampleRate;
    const noteIdx = Math.floor(t * 1.5) % notes.length;
    const freq = notes[noteIdx];
    // Envelope to smooth clicks
    const env = Math.sin((Math.PI * (i % (sampleRate / 1.5))) / (sampleRate / 1.5));
    const sample = Math.sin(2 * Math.PI * freq * t) * 0.4 * env * 32767;
    buffer.writeInt16LE(Math.floor(sample), 44 + i * 2);
  }

  fs.writeFileSync(path.join(sharedDir, filename), buffer);
  console.log('Created WAV:', filename);
}

// 2. Create sample vector artwork
function createSampleSvg(filename) {
  const svg = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1200 800" width="100%" height="100%">
  <defs>
    <linearGradient id="sky" x1="0%" y1="0%" x2="0%" y2="100%">
      <stop offset="0%" stop-color="#0f172a" />
      <stop offset="40%" stop-color="#3b0764" />
      <stop offset="70%" stop-color="#c026d3" />
      <stop offset="100%" stop-color="#f97316" />
    </linearGradient>
    <linearGradient id="sea" x1="0%" y1="0%" x2="0%" y2="100%">
      <stop offset="0%" stop-color="#0284c7" />
      <stop offset="100%" stop-color="#082f49" />
    </linearGradient>
    <radialGradient id="sun" cx="50%" cy="50%" r="50%">
      <stop offset="0%" stop-color="#fef08a" />
      <stop offset="60%" stop-color="#f59e0b" />
      <stop offset="100%" stop-color="#ea580c" stop-opacity="0" />
    </radialGradient>
  </defs>
  <rect width="1200" height="520" fill="url(#sky)" />
  <circle cx="600" cy="480" r="160" fill="url(#sun)" />
  <rect y="500" width="1200" height="300" fill="url(#sea)" />
  <text x="600" y="240" font-family="system-ui, sans-serif" font-size="44" font-weight="bold" fill="#ffffff" text-anchor="middle" letter-spacing="2">AIRMESH LOCAL P2P STREAM</text>
  <text x="600" y="290" font-family="system-ui, sans-serif" font-size="20" fill="#cbd5e1" text-anchor="middle">Transferred directly over WiFi at 120+ MB/s &bull; Zero Cloud &bull; Offline</text>
</svg>`;
  fs.writeFileSync(path.join(sharedDir, filename), svg);
  console.log('Created SVG:', filename);
}

// 3. Create sample offline documentation
function createSampleDoc(filename) {
  const md = `# AirMesh User & Offline Mesh Guide

Welcome to your 100% offline, local peer-to-peer file mesh.

## How It Works
- **No Internet Required**: AirMesh uses your local WiFi router or phone hotspot. Even if your internet cable is unplugged, all devices on the same WiFi discover each other instantly.
- **Line-Rate WiFi Speeds**: Transfers run at hardware speed (100–300+ MB/s) directly between devices over QUIC/UDP.
- **Zero Cloud & Telemetry**: Your files, filenames, and streaming media never leave your local living room or office network.

## How to Connect Your Phone
1. Connect your phone to the same WiFi as this computer.
2. Tap the **"📱 Connect Phone"** button in AirMesh to show your QR Code.
3. Open your phone camera and tap the link, or type the address into Chrome or Safari.
4. You can now browse all files, stream media, and send photos/videos straight to this PC!

## How to Access Connected Devices Storage
- In the **"Connected Devices"** section, click on any peer (e.g. Pixel 8 Pro or MacBook Pro).
- AirMesh automatically queries that device and displays its shared folders and storage capacity.
- Click **"Stream"** to instantly watch videos or play music, or **"Download"** to save to this PC.
`;
  fs.writeFileSync(path.join(sharedDir, filename), md);
  console.log('Created Doc:', filename);
}

try {
  createSampleWav('ambient_melody_sample.wav');
  createSampleSvg('beach_sunset_wallpaper.svg');
  createSampleDoc('airmesh_user_guide.md');
} catch (e) {
  console.error(e);
}
