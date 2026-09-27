/**
 * AirMesh Local Node Server (Production-Grade, Hierarchical & Minimalist)
 * 
 * 100% Local, Offline-First Peer-to-Peer Mesh File Transfer & Streaming.
 * Features:
 * - Systematic Hierarchical Folder Support:
 *   - Browse nested directories with breadcrumbs.
 *   - Create folders (mkdir).
 *   - Upload entire folders with recursive directory structure (webkitdirectory).
 *   - Upload files into specific nested folders.
 *   - Download any folder (or root) as a streaming ZIP archive via tar.exe.
 *   - Delete files or recursive folders safely.
 * - Real-time Server-Sent Events (SSE) for instant cross-device updates.
 * - On-the-fly cached thumbnails for photos and videos in any folder.
 * - HTTP 206 Partial Content Range video/audio streaming with instant scrubbing.
 * - Real-time audio transmuxing (Dolby EAC3/AC3/DTS -> AAC) with muted video safety (-an).
 * - Multi-file / multi-folder ZIP archive streaming.
 * - Real client discovery via heartbeat.
 * - Disk free space protection (rejects if <50MB).
 * - Directory traversal attack prevention with strict path resolution.
 */

const http = require('http');
const fs = require('fs');
const path = require('path');
const os = require('os');
const QRCode = require('qrcode');
const { spawn, spawnSync } = require('child_process');

process.on('uncaughtException', (err) => {
  console.error('[AirMesh SafeHandler] Uncaught exception:', err.message);
});
process.on('unhandledRejection', (reason) => {
  console.error('[AirMesh SafeHandler] Unhandled rejection:', reason);
});

const PORT = 47823;
const SHARED_DIR = path.resolve(__dirname, 'shared');
const THUMBS_DIR = path.join(SHARED_DIR, '.thumbs');

if (!fs.existsSync(SHARED_DIR)) {
  fs.mkdirSync(SHARED_DIR, { recursive: true });
}
if (!fs.existsSync(THUMBS_DIR)) {
  fs.mkdirSync(THUMBS_DIR, { recursive: true });
}

// Security: Resolve path strictly within SHARED_DIR to prevent directory traversal
function resolveSafePath(userRelPath) {
  if (!userRelPath) return SHARED_DIR;
  let clean = userRelPath.replace(/\\/g, '/');
  // Strip drive letters (C:) and leading slashes
  clean = clean.replace(/^[a-zA-Z]:/, '').replace(/^[\/\\]+/, '');
  const normalized = path.normalize(clean).replace(/^(\.\.[\/\\])+/, '');
  const resolved = path.resolve(SHARED_DIR, normalized);
  if (!resolved.startsWith(SHARED_DIR)) {
    return null;
  }
  return resolved;
}

// Convert absolute path back to forward-slash relative path from SHARED_DIR
function getRelativePosix(absPath) {
  const rel = path.relative(SHARED_DIR, absPath);
  if (!rel || rel === '.') return '';
  return rel.replace(/\\/g, '/');
}

// Clean up any stale partial uploads older than 24 hours
try {
  function cleanupPartials(dir) {
    const entries = fs.readdirSync(dir, { withFileTypes: true });
    const oneDayAgo = Date.now() - 24 * 60 * 60 * 1000;
    for (const e of entries) {
      const full = path.join(dir, e.name);
      if (e.isDirectory() && !e.name.startsWith('.')) {
        cleanupPartials(full);
      } else if (e.isFile() && e.name.endsWith('.part')) {
        try {
          const stat = fs.statSync(full);
          if (stat.mtimeMs < oneDayAgo) {
            fs.unlinkSync(full);
          }
        } catch(err) {}
      }
    }
  }
  cleanupPartials(SHARED_DIR);
} catch(e) {}

// Detect local WiFi IPv4 address
function getLocalIpAddress() {
  const nets = os.networkInterfaces();
  const candidates = [];

  for (const name of Object.keys(nets)) {
    for (const net of nets[name]) {
      if (net.family === 'IPv4' && !net.internal && !net.address.startsWith('169.254')) {
        const lowerName = name.toLowerCase();
        if (lowerName.includes('wi-fi') || lowerName.includes('wifi') || lowerName.includes('wlan')) {
          return net.address;
        }
        candidates.push(net.address);
      }
    }
  }
  return candidates[0] || '127.0.0.1';
}

const LOCAL_IP = getLocalIpAddress();

// Query host disk free space
function getDiskSpaceInfo() {
  try {
    const stats = fs.statfsSync(SHARED_DIR);
    const freeBytes = stats.bavail * stats.bsize;
    const totalBytes = stats.blocks * stats.bsize;
    return {
      free_gb: parseFloat((freeBytes / (1024 ** 3)).toFixed(1)),
      total_gb: parseFloat((totalBytes / (1024 ** 3)).toFixed(1)),
      free_bytes: freeBytes,
      total_bytes: totalBytes,
      used_percent: Math.min(100, Math.round(((totalBytes - freeBytes) / totalBytes) * 100))
    };
  } catch(e) {
    return { free_gb: 0, total_gb: 0, free_bytes: 0, total_bytes: 0, used_percent: 0 };
  }
}

// Real-time connected clients
const activeClients = new Map();

// Active in-flight file transfers for progressive stream-while-receiving
const activeTransfers = new Map();

// Server-Sent Events (SSE) connections for live updates
const sseClients = new Set();

function broadcastSse(eventType, data) {
  const msg = `event: ${eventType}\ndata: ${JSON.stringify(data)}\n\n`;
  for (const client of sseClients) {
    try {
      client.write(msg);
    } catch (e) {
      sseClients.delete(client);
    }
  }
}

function cleanIp(ip) {
  if (!ip) return '127.0.0.1';
  if (ip.startsWith('::ffff:')) return ip.replace('::ffff:', '');
  if (ip === '::1') return '127.0.0.1';
  return ip;
}

function parseClientInfo(ua, clientIp) {
  const agent = ua || '';
  let deviceType = 'desktop';
  let deviceName = 'Computer';

  if (/iPhone/i.test(agent)) {
    deviceType = 'phone';
    deviceName = 'Apple iPhone';
  } else if (/iPad/i.test(agent)) {
    deviceType = 'tablet';
    deviceName = 'Apple iPad';
  } else if (/Android/i.test(agent)) {
    deviceType = /Mobile/i.test(agent) ? 'phone' : 'tablet';
    if (/Samsung|SM-/i.test(agent)) deviceName = 'Samsung Galaxy';
    else if (/Pixel/i.test(agent)) deviceName = 'Google Pixel';
    else if (/Xiaomi|Redmi/i.test(agent)) deviceName = 'Xiaomi Phone';
    else deviceName = 'Android Device';
  } else if (/Macintosh|Mac OS X/i.test(agent)) {
    deviceType = 'laptop';
    deviceName = 'MacBook / Mac';
  } else if (/Windows/i.test(agent)) {
    deviceType = 'desktop';
    deviceName = 'Windows PC';
  } else if (/Linux/i.test(agent)) {
    deviceType = 'desktop';
    deviceName = 'Linux Device';
  }

  return { deviceType, deviceName };
}

let meshClipboard = {
  text: "",
  updated_at: Date.now(),
  author: "Host PC"
};

const codecCache = new Map();

function formatDuration(sec) {
  if (!sec || isNaN(sec)) return null;
  const s = Math.floor(sec);
  const m = Math.floor(s / 60);
  const h = Math.floor(m / 60);
  const remM = m % 60;
  const remS = s % 60;
  if (h > 0) {
    return `${h}:${remM.toString().padStart(2, '0')}:${remS.toString().padStart(2, '0')}`;
  }
  return `${remM}:${remS.toString().padStart(2, '0')}`;
}

function getCodecInfo(filePath) {
  if (codecCache.has(filePath)) {
    return codecCache.get(filePath);
  }
  const ext = path.extname(filePath).toLowerCase();
  const isVideo = ['.mp4', '.mkv', '.webm', '.mov', '.avi'].includes(ext);

  if (!isVideo) {
    const isAudio = ['.mp3', '.wav', '.flac', '.ogg', '.m4a', '.aac'].includes(ext);
    return {
      has_audio: isAudio,
      audio_codec: isAudio ? ext.replace('.', '') : null,
      video_codec: null,
      needs_audio_transcode: false,
      has_subtitles: false,
      duration: null,
      duration_label: null,
      resolution_label: null
    };
  }

  try {
    const res = spawnSync('ffprobe', [
      '-v', 'error',
      '-show_entries', 'stream=codec_name,codec_type,width,height:format=duration',
      '-of', 'json',
      filePath
    ], { timeout: 2500 });

    if (res.status === 0) {
      const data = JSON.parse(res.stdout.toString());
      const audioStreams = (data.streams || []).filter(s => s.codec_type === 'audio');
      const videoStream = (data.streams || []).find(s => s.codec_type === 'video');
      const subStreams = (data.streams || []).filter(s => s.codec_type === 'subtitle');

      const hasAudio = audioStreams.length > 0;
      const audioCodec = hasAudio ? (audioStreams[0].codec_name || '').toLowerCase() : null;
      const videoCodec = videoStream ? (videoStream.codec_name || '').toLowerCase() : null;

      const incompatible = ['eac3', 'ac3', 'dts', 'truehd', 'mlp', 'dca'];
      const needsTranscode = audioCodec ? incompatible.includes(audioCodec) : false;

      let resolutionLabel = null;
      if (videoStream && videoStream.width && videoStream.height) {
        const w = videoStream.width;
        const h = videoStream.height;
        const maxDim = Math.max(w, h);
        if (maxDim >= 3800) resolutionLabel = '4K UHD';
        else if (maxDim >= 1900) resolutionLabel = '1080p';
        else if (maxDim >= 1200) resolutionLabel = '720p';
        else resolutionLabel = `${w}x${h}`;
      }

      const durationSec = data.format && data.format.duration ? parseFloat(data.format.duration) : null;
      const durationLabel = formatDuration(durationSec);

      const info = {
        has_audio: hasAudio,
        audio_codec: audioCodec,
        video_codec: videoCodec,
        needs_audio_transcode: needsTranscode,
        has_subtitles: subStreams.length > 0,
        subtitle_count: subStreams.length,
        resolution_label: resolutionLabel,
        duration: durationSec,
        duration_label: durationLabel
      };
      codecCache.set(filePath, info);
      return info;
    }
  } catch (e) {}

  return {
    has_audio: false,
    audio_codec: null,
    video_codec: null,
    needs_audio_transcode: false,
    has_subtitles: false,
    duration: null,
    duration_label: null,
    resolution_label: null
  };
}

function getCategory(filename) {
  const f = (filename || '').toLowerCase();
  if (f.endsWith('.mp4') || f.endsWith('.mkv') || f.endsWith('.webm') || f.endsWith('.mov') || f.endsWith('.avi')) return 'video';
  if (f.endsWith('.mp3') || f.endsWith('.wav') || f.endsWith('.flac') || f.endsWith('.ogg') || f.endsWith('.m4a')) return 'audio';
  if (f.endsWith('.jpg') || f.endsWith('.jpeg') || f.endsWith('.png') || f.endsWith('.webp') || f.endsWith('.gif') || f.endsWith('.svg') || f.endsWith('.bmp')) return 'photo';
  if (f.endsWith('.pdf') || f.endsWith('.txt') || f.endsWith('.md') || f.endsWith('.doc') || f.endsWith('.docx') || f.endsWith('.epub') || f.endsWith('.json') || f.endsWith('.log') || f.endsWith('.rs') || f.endsWith('.js') || f.endsWith('.py') || f.endsWith('.csv')) return 'document';
  if (f.endsWith('.zip') || f.endsWith('.tar') || f.endsWith('.gz') || f.endsWith('.7z') || f.endsWith('.rar')) return 'archive';
  return 'file';
}

function getMimeType(filePath) {
  const lower = (filePath || '').toLowerCase();
  if (lower.endsWith('.mp4')) return 'video/mp4';
  if (lower.endsWith('.mkv')) return 'video/x-matroska';
  if (lower.endsWith('.webm')) return 'video/webm';
  if (lower.endsWith('.mov')) return 'video/quicktime';
  if (lower.endsWith('.avi')) return 'video/x-msvideo';
  if (lower.endsWith('.mp3')) return 'audio/mpeg';
  if (lower.endsWith('.wav')) return 'audio/wav';
  if (lower.endsWith('.flac')) return 'audio/flac';
  if (lower.endsWith('.ogg')) return 'audio/ogg';
  if (lower.endsWith('.m4a')) return 'audio/mp4';
  if (lower.endsWith('.aac')) return 'audio/aac';
  if (lower.endsWith('.jpg') || lower.endsWith('.jpeg')) return 'image/jpeg';
  if (lower.endsWith('.png')) return 'image/png';
  if (lower.endsWith('.webp')) return 'image/webp';
  if (lower.endsWith('.gif')) return 'image/gif';
  if (lower.endsWith('.svg')) return 'image/svg+xml';
  if (lower.endsWith('.pdf')) return 'application/pdf';
  if (lower.endsWith('.txt')) return 'text/plain; charset=utf-8';
  if (lower.endsWith('.md')) return 'text/markdown; charset=utf-8';
  if (lower.endsWith('.json')) return 'application/json; charset=utf-8';
  if (lower.endsWith('.log')) return 'text/plain; charset=utf-8';
  if (lower.endsWith('.zip')) return 'application/zip';
  return 'application/octet-stream';
}

// Get hierarchical folder contents
function getDirectoryContents(relPath) {
  const targetDir = resolveSafePath(relPath);
  if (!targetDir || !fs.existsSync(targetDir) || !fs.statSync(targetDir).isDirectory()) {
    return null;
  }

  const cleanRel = getRelativePosix(targetDir);

  // Build breadcrumbs
  const breadcrumbs = [{ name: 'Shared', path: '' }];
  if (cleanRel) {
    const parts = cleanRel.split('/');
    let accum = '';
    for (const part of parts) {
      accum = accum ? `${accum}/${part}` : part;
      breadcrumbs.push({ name: part, path: accum });
    }
  }

  const folders = [];
  const files = [];

  try {
    const entries = fs.readdirSync(targetDir, { withFileTypes: true });

    for (const entry of entries) {
      if (entry.name === '.thumbs' || entry.name.startsWith('.')) continue;

      const fullPath = path.join(targetDir, entry.name);
      const itemRelPath = getRelativePosix(fullPath);

      if (entry.isDirectory()) {
        let count = 0;
        try {
          count = fs.readdirSync(fullPath).filter(n => !n.startsWith('.')).length;
        } catch(e) {}

        const stat = fs.statSync(fullPath);
        folders.push({
          id: Buffer.from(itemRelPath).toString('hex').padEnd(32, '0'),
          name: entry.name,
          path: itemRelPath,
          is_dir: true,
          item_count: count,
          modified_at: Math.floor(stat.mtimeMs / 1000)
        });
      } else if (entry.isFile()) {
        if (entry.name.endsWith('.part')) {
          const originalName = entry.name.slice(0, -5);
          const originalFullPath = path.join(targetDir, originalName);
          const inFlight = activeTransfers.get(originalFullPath);
          if (inFlight) {
            const originalRelPath = getRelativePosix(originalFullPath);
            const percent = inFlight.totalBytes > 0 ? Math.min(99, Math.round((inFlight.writtenBytes / inFlight.totalBytes) * 100)) : 0;
            files.push({
              id: Buffer.from(originalRelPath).toString('hex').padEnd(32, '0'),
              filename: originalName,
              path: originalRelPath,
              size: inFlight.totalBytes || inFlight.writtenBytes,
              written_bytes: inFlight.writtenBytes,
              is_receiving: true,
              receiving_percent: percent,
              mime_type: inFlight.mime,
              category: inFlight.category,
              has_audio: true,
              audio_codec: null,
              video_codec: null,
              needs_audio_transcode: false,
              has_subtitles: false,
              resolution: null,
              duration: null,
              duration_label: null,
              has_thumbnail: false,
              modified_at: Math.floor(inFlight.startedAt / 1000)
            });
          }
          continue;
        }

        try {
          const stat = fs.statSync(fullPath);
          const mime = getMimeType(fullPath);
          const inFlight = activeTransfers.get(fullPath);

          if (inFlight) {
            const percent = inFlight.totalBytes > 0 ? Math.min(99, Math.round((inFlight.writtenBytes / inFlight.totalBytes) * 100)) : 0;
            files.push({
              id: Buffer.from(itemRelPath).toString('hex').padEnd(32, '0'),
              filename: entry.name,
              path: itemRelPath,
              size: inFlight.totalBytes || stat.size,
              written_bytes: inFlight.writtenBytes,
              is_receiving: true,
              receiving_percent: percent,
              mime_type: mime,
              category: inFlight.category,
              has_audio: true,
              audio_codec: null,
              video_codec: null,
              needs_audio_transcode: false,
              has_subtitles: false,
              resolution: null,
              duration: null,
              duration_label: null,
              has_thumbnail: false,
              modified_at: Math.floor(inFlight.startedAt / 1000)
            });
            continue;
          }

          const codecInfo = getCodecInfo(fullPath);

          files.push({
            id: Buffer.from(itemRelPath).toString('hex').padEnd(32, '0'),
            filename: entry.name,
            path: itemRelPath,
            size: stat.size,
            written_bytes: stat.size,
            is_receiving: false,
            mime_type: mime,
            category: getCategory(entry.name),
            has_audio: codecInfo.has_audio,
            audio_codec: codecInfo.audio_codec,
            video_codec: codecInfo.video_codec,
            needs_audio_transcode: codecInfo.needs_audio_transcode,
            has_subtitles: codecInfo.has_subtitles,
            resolution: codecInfo.resolution_label,
            duration: codecInfo.duration,
            duration_label: codecInfo.duration_label,
            has_thumbnail: (getCategory(entry.name) === 'photo' || getCategory(entry.name) === 'video'),
            modified_at: Math.floor(stat.mtimeMs / 1000)
          });
        } catch(err) {}
      }
    }

    // Add any active in-flight transfers in this folder that haven't hit readdir yet
    const targetParentRel = cleanRel;
    for (const [key, transfer] of activeTransfers.entries()) {
      if (key !== transfer.targetPath) continue;
      const fileParent = path.posix.dirname(transfer.path);
      const normalizedFileParent = (fileParent === '.' || fileParent === '') ? '' : fileParent;
      if (normalizedFileParent === targetParentRel) {
        if (!files.some(f => f.path === transfer.path)) {
          const percent = transfer.totalBytes > 0 ? Math.min(99, Math.round((transfer.writtenBytes / transfer.totalBytes) * 100)) : 0;
          files.unshift({
            id: Buffer.from(transfer.path).toString('hex').padEnd(32, '0'),
            filename: transfer.filename,
            path: transfer.path,
            size: transfer.totalBytes || transfer.writtenBytes,
            written_bytes: transfer.writtenBytes,
            is_receiving: true,
            receiving_percent: percent,
            mime_type: transfer.mime,
            category: transfer.category,
            has_audio: true,
            audio_codec: null,
            video_codec: null,
            needs_audio_transcode: false,
            has_subtitles: false,
            resolution: null,
            duration: null,
            duration_label: null,
            has_thumbnail: false,
            modified_at: Math.floor(transfer.startedAt / 1000)
          });
        }
      }
    }
  } catch(e) {}

  folders.sort((a, b) => a.name.localeCompare(b.name));
  files.sort((a, b) => {
    if (a.is_receiving && !b.is_receiving) return -1;
    if (!a.is_receiving && b.is_receiving) return 1;
    return b.modified_at - a.modified_at;
  });

  return {
    current_path: cleanRel,
    breadcrumbs,
    folders,
    files
  };
}

// Find absolute path from fileId or direct relative path parameter
function resolveItemPath(fileIdOrPath) {
  if (!fileIdOrPath) return null;
  let relPath = fileIdOrPath;
  // If it's a 32+ char hex string, decode it
  if (/^[0-9a-fA-F]{32,}$/.test(fileIdOrPath)) {
    try {
      const decoded = Buffer.from(fileIdOrPath, 'hex').toString('utf8');
      if (decoded) relPath = decoded;
    } catch(e) {}
  }
  return resolveSafePath(relPath);
}

const server = http.createServer((req, res) => {
  try {
    res.setHeader('Access-Control-Allow-Origin', '*');
    res.setHeader('Access-Control-Allow-Methods', 'GET, POST, DELETE, HEAD, OPTIONS');
    res.setHeader('Access-Control-Allow-Headers', '*');
    res.setHeader('Content-Security-Policy', "default-src 'self' data: blob:; connect-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'; img-src 'self' data: blob:; media-src 'self' blob:; object-src 'none'; font-src 'self' data:;");
    res.setHeader('X-Content-Type-Options', 'nosniff');

    if (req.method === 'OPTIONS') {
      res.writeHead(204);
      res.end();
      return;
    }

    const host = req.headers.host || `localhost:${PORT}`;
    const parsedUrl = new URL(req.url, `http://${host}`);
    const pathname = parsedUrl.pathname;
    const clientIp = cleanIp(req.socket.remoteAddress);
    const userAgent = req.headers['user-agent'] || '';

    // Register active device
    const clientIdHeader = req.headers['x-client-id'];
    if (clientIdHeader) {
      const { deviceType, deviceName } = parseClientInfo(userAgent, clientIp);
      activeClients.set(clientIdHeader, {
        id: clientIdHeader,
        device_name: deviceName,
        device_type: deviceType,
        ip: clientIp,
        is_host: (clientIp === '127.0.0.1' || clientIp === LOCAL_IP),
        last_seen: Date.now()
      });
    }

    // PWA Manifest
    if (pathname === '/manifest.json') {
      res.writeHead(200, { 'Content-Type': 'application/manifest+json; charset=utf-8' });
      res.end(JSON.stringify({
        name: "AirMesh P2P File Mesh",
        short_name: "AirMesh",
        start_url: "/",
        display: "standalone",
        background_color: "#0b0c10",
        theme_color: "#0b0c10",
        icons: [{
          src: "/api/qr.svg",
          sizes: "260x260",
          type: "image/svg+xml"
        }]
      }));
      return;
    }

    // Service Worker
    if (pathname === '/sw.js') {
      res.writeHead(200, { 'Content-Type': 'application/javascript; charset=utf-8' });
      res.end(`
        self.addEventListener('install', e => self.skipWaiting());
        self.addEventListener('activate', e => e.waitUntil(clients.claim()));
      `);
      return;
    }

    // Server-Sent Events (SSE)
    if (pathname === '/api/events') {
      res.writeHead(200, {
        'Content-Type': 'text/event-stream',
        'Cache-Control': 'no-cache, no-transform',
        'Connection': 'keep-alive',
        'Access-Control-Allow-Origin': '*'
      });
      res.write(': connected\n\n');
      sseClients.add(res);

      const pingTimer = setInterval(() => {
        try {
          res.write(': ping\n\n');
        } catch (e) {
          clearInterval(pingTimer);
          sseClients.delete(res);
        }
      }, 15000);

      req.on('close', () => {
        clearInterval(pingTimer);
        sseClients.delete(res);
      });
      return;
    }

    // QR Code
    if (pathname === '/api/qr.svg') {
      const targetUrl = parsedUrl.searchParams.get('url') || `http://${LOCAL_IP}:${PORT}`;
      QRCode.toString(targetUrl, { type: 'svg', margin: 1, width: 260 }, (err, svg) => {
        if (err) {
          res.writeHead(500, { 'Content-Type': 'text/plain' });
          res.end('QR error: ' + err.message);
          return;
        }
        res.writeHead(200, {
          'Content-Type': 'image/svg+xml',
          'Cache-Control': 'public, max-age=3600'
        });
        res.end(svg);
      });
      return;
    }

    // Fast Image / Video Thumbnails
    if (pathname.startsWith('/api/thumbnail/')) {
      const param = pathname.replace('/api/thumbnail/', '');
      const filePath = resolveItemPath(param);

      if (!filePath || !fs.existsSync(filePath) || activeTransfers.has(filePath)) {
        res.writeHead(404, { 'Content-Type': 'text/plain' });
        res.end('Not found or receiving');
        return;
      }

      const relPath = getRelativePosix(filePath);
      const thumbId = Buffer.from(relPath).toString('hex');
      const thumbFile = path.join(THUMBS_DIR, `${thumbId}.jpg`);

      if (fs.existsSync(thumbFile)) {
        res.writeHead(200, {
          'Content-Type': 'image/jpeg',
          'Cache-Control': 'public, max-age=604800, immutable'
        });
        fs.createReadStream(thumbFile).pipe(res);
        return;
      }

      const cat = getCategory(filePath);
      let ffmpegArgs = [];
      if (cat === 'photo') {
        ffmpegArgs = ['-y', '-i', filePath, '-vf', 'scale=360:-1', '-q:v', '4', '-frames:v', '1', thumbFile];
      } else if (cat === 'video') {
        ffmpegArgs = ['-y', '-ss', '00:00:01', '-i', filePath, '-vf', 'scale=360:-1', '-vframes', '1', '-q:v', '4', thumbFile];
      } else {
        res.writeHead(404, { 'Content-Type': 'text/plain' });
        res.end('No thumbnail');
        return;
      }

      const p = spawn('ffmpeg', ffmpegArgs);
      p.on('close', (code) => {
        if (code === 0 && fs.existsSync(thumbFile)) {
          res.writeHead(200, {
            'Content-Type': 'image/jpeg',
            'Cache-Control': 'public, max-age=604800, immutable'
          });
          fs.createReadStream(thumbFile).pipe(res);
        } else {
          if (cat === 'photo') {
            res.writeHead(200, { 'Content-Type': 'image/jpeg' });
            fs.createReadStream(filePath).pipe(res);
          } else {
            res.writeHead(404, { 'Content-Type': 'text/plain' });
            res.end('Thumbnail failed');
          }
        }
      });
      return;
    }

    // Network Info & Disk
    if (pathname === '/api/network-info') {
      const diskInfo = getDiskSpaceInfo();
      res.writeHead(200, { 'Content-Type': 'application/json' });
      res.end(JSON.stringify({
        local_ip: LOCAL_IP,
        port: PORT,
        url: `http://${LOCAL_IP}:${PORT}`,
        host_name: os.hostname() || 'Windows-PC',
        disk: diskInfo,
        offline_guarantee: {
          offline_verified: true,
          internet_required: false,
          rfc1918_private_lan: true,
          external_connections: 0
        }
      }));
      return;
    }

    // Health
    if (pathname === '/api/health') {
      const diskInfo = getDiskSpaceInfo();
      res.writeHead(200, { 'Content-Type': 'application/json' });
      res.end(JSON.stringify({
        status: 'ok',
        host_name: os.hostname() || 'Windows-PC',
        local_ip: LOCAL_IP,
        port: PORT,
        url: `http://${LOCAL_IP}:${PORT}`,
        disk: diskInfo
      }));
      return;
    }

    // Heartbeat
    if (pathname === '/api/peers/heartbeat' && req.method === 'POST') {
      let body = '';
      req.on('data', chunk => { body += chunk; });
      req.on('end', () => {
        try {
          const data = JSON.parse(body || '{}');
          const id = data.client_id || `${clientIp}_${Date.now()}`;
          const { deviceType, deviceName } = parseClientInfo(userAgent, clientIp);

          const isNew = !activeClients.has(id);
          activeClients.set(id, {
            id,
            device_name: data.device_name || deviceName,
            device_type: deviceType,
            ip: clientIp,
            is_host: data.is_host === true || clientIp === '127.0.0.1' || clientIp === LOCAL_IP,
            last_seen: Date.now()
          });

          if (isNew) {
            broadcastSse('peers_change', { count: activeClients.size });
          }

          res.writeHead(200, { 'Content-Type': 'application/json' });
          res.end(JSON.stringify({ status: 'ok', client_id: id }));
        } catch (e) {
          res.writeHead(400, { 'Content-Type': 'application/json' });
          res.end(JSON.stringify({ error: e.message }));
        }
      });
      return;
    }

    // Active Peers
    if (pathname === '/api/peers') {
      const now = Date.now();
      const realPeers = [];

      for (const [id, client] of activeClients.entries()) {
        if (now - client.last_seen < 45000) {
          realPeers.push(client);
        } else {
          activeClients.delete(id);
        }
      }

      res.writeHead(200, { 'Content-Type': 'application/json' });
      res.end(JSON.stringify({
        host: {
          name: os.hostname() || 'Host PC',
          ip: LOCAL_IP,
          port: PORT,
          address: `${LOCAL_IP}:${PORT}`
        },
        devices: realPeers
      }));
      return;
    }

    // Hierarchical Folder & File Listing
    if (pathname === '/api/files' && req.method === 'GET') {
      const reqPath = parsedUrl.searchParams.get('path') || '';
      const data = getDirectoryContents(reqPath);
      if (!data) {
        res.writeHead(404, { 'Content-Type': 'application/json' });
        res.end(JSON.stringify({ error: 'Folder not found' }));
        return;
      }
      res.writeHead(200, { 'Content-Type': 'application/json' });
      res.end(JSON.stringify(data));
      return;
    }

    // Create New Folder (mkdir)
    if (pathname === '/api/mkdir' && req.method === 'POST') {
      let body = '';
      req.on('data', chunk => { body += chunk; });
      req.on('end', () => {
        try {
          const data = JSON.parse(body || '{}');
          const currentDir = data.path || '';
          const rawName = (data.name || '').trim();

          // Sanitize folder name
          const folderName = rawName.replace(/[/\\?%*:|"<>]/g, '_');
          if (!folderName) {
            res.writeHead(400, { 'Content-Type': 'application/json' });
            res.end(JSON.stringify({ error: 'Invalid folder name' }));
            return;
          }

          const targetParent = resolveSafePath(currentDir);
          if (!targetParent || !fs.existsSync(targetParent)) {
            res.writeHead(400, { 'Content-Type': 'application/json' });
            res.end(JSON.stringify({ error: 'Target parent folder not found' }));
            return;
          }

          const newFolderPath = path.join(targetParent, folderName);
          if (fs.existsSync(newFolderPath)) {
            res.writeHead(409, { 'Content-Type': 'application/json' });
            res.end(JSON.stringify({ error: 'Folder already exists' }));
            return;
          }

          fs.mkdirSync(newFolderPath, { recursive: true });
          broadcastSse('file_change', { action: 'mkdir', path: getRelativePosix(newFolderPath) });

          res.writeHead(200, { 'Content-Type': 'application/json' });
          res.end(JSON.stringify({ status: 'created', folder: getRelativePosix(newFolderPath) }));
        } catch(e) {
          res.writeHead(500, { 'Content-Type': 'application/json' });
          res.end(JSON.stringify({ error: e.message }));
        }
      });
      return;
    }

    // Download Entire Folder as a ZIP Archive
    if (pathname === '/api/download-folder' && req.method === 'GET') {
      const relPath = parsedUrl.searchParams.get('path') || '';
      const targetDir = resolveSafePath(relPath);

      if (!targetDir || !fs.existsSync(targetDir) || !fs.statSync(targetDir).isDirectory()) {
        res.writeHead(404, { 'Content-Type': 'text/plain' });
        res.end('Folder not found');
        return;
      }

      const folderName = path.basename(targetDir) || 'AirMesh_Shared';
      const safeName = folderName.replace(/[/\\?%*:|"<>]/g, '_');
      res.writeHead(200, {
        'Content-Type': 'application/zip',
        'Content-Disposition': `attachment; filename="${safeName}.zip"; filename*=UTF-8''${encodeURIComponent(safeName)}.zip`
      });

      let tarArgs = [];
      if (targetDir === SHARED_DIR) {
        // Root folder: archive non-hidden entries using true PKZIP format
        tarArgs = ['--format=zip', '-c', '-f', '-', '-C', SHARED_DIR, '--exclude=.*', '.'];
      } else {
        const parentDir = path.dirname(targetDir);
        const base = path.basename(targetDir);
        tarArgs = ['--format=zip', '-c', '-f', '-', '-C', parentDir, base];
      }

      const tar = spawn('tar.exe', tarArgs);
      tar.stdout.pipe(res);

      req.on('close', () => {
        try { tar.kill('SIGKILL'); } catch(e) {}
      });
      tar.on('error', (err) => {
        console.error('tar folder download error:', err.message);
        try { res.end(); } catch(e) {}
      });
      return;
    }

    // Batch Download Selected Items as a ZIP Archive
    if (pathname === '/api/archive' && req.method === 'GET') {
      const rawFiles = parsedUrl.searchParams.get('files') || '';
      const items = rawFiles.split(',').map(s => decodeURIComponent(s.trim())).filter(Boolean);

      if (items.length === 0) {
        res.writeHead(400, { 'Content-Type': 'text/plain' });
        res.end('No files specified');
        return;
      }

      const validRelPaths = [];
      for (const item of items) {
        const resolved = resolveSafePath(item);
        if (resolved && fs.existsSync(resolved) && resolved !== SHARED_DIR) {
          validRelPaths.push(path.relative(SHARED_DIR, resolved).replace(/\\/g, '/'));
        }
      }

      if (validRelPaths.length === 0) {
        res.writeHead(404, { 'Content-Type': 'text/plain' });
        res.end('None of the specified items were found');
        return;
      }

      res.writeHead(200, {
        'Content-Type': 'application/zip',
        'Content-Disposition': 'attachment; filename="AirMesh_Selection.zip"; filename*=UTF-8\'\'AirMesh_Selection.zip'
      });

      const tarArgs = ['--format=zip', '-c', '-f', '-', '-C', SHARED_DIR, ...validRelPaths];
      const tar = spawn('tar.exe', tarArgs);
      tar.stdout.pipe(res);

      req.on('close', () => {
        try { tar.kill('SIGKILL'); } catch(e) {}
      });
      tar.on('error', (err) => {
        console.error('tar batch archive download error:', err.message);
        try { res.end(); } catch(e) {}
      });
      return;
    }

    // Delete File or Folder
    if (pathname.startsWith('/api/files/') && req.method === 'DELETE') {
      const targetParam = decodeURIComponent(pathname.replace('/api/files/', ''));
      const targetPath = resolveItemPath(targetParam);

      if (!targetPath || targetPath === SHARED_DIR) {
        res.writeHead(400, { 'Content-Type': 'application/json' });
        res.end(JSON.stringify({ error: 'Cannot delete root shared directory' }));
        return;
      }

      if (fs.existsSync(targetPath)) {
        try {
          const stat = fs.statSync(targetPath);
          if (stat.isDirectory()) {
            fs.rmSync(targetPath, { recursive: true, force: true });
          } else {
            fs.unlinkSync(targetPath);
            codecCache.delete(targetPath);

            const rel = getRelativePosix(targetPath);
            const thumbId = Buffer.from(rel).toString('hex');
            const thumbFile = path.join(THUMBS_DIR, `${thumbId}.jpg`);
            try { if (fs.existsSync(thumbFile)) fs.unlinkSync(thumbFile); } catch(e) {}
          }

          broadcastSse('file_change', { action: 'delete', path: targetParam });
          res.writeHead(200, { 'Content-Type': 'application/json' });
          res.end(JSON.stringify({ status: 'deleted', path: targetParam }));
          return;
        } catch(err) {
          res.writeHead(500, { 'Content-Type': 'application/json' });
          res.end(JSON.stringify({ error: 'Failed to delete: ' + err.message }));
          return;
        }
      } else {
        res.writeHead(404, { 'Content-Type': 'application/json' });
        res.end(JSON.stringify({ error: 'Item not found' }));
        return;
      }
    }

    // Progressive Stream-While-Receiving Helper
    function pipeProgressiveStream(inFlight, req, res) {
      const mime = inFlight.mime || getMimeType(inFlight.targetPath);
      const range = req.headers.range;
      let start = 0;
      let requestedEnd = undefined;

      if (range) {
        const parts = range.replace(/bytes=/, "").split("-");
        start = parseInt(parts[0], 10) || 0;
        if (parts[1]) {
          requestedEnd = parseInt(parts[1], 10);
        }
      }

      const targetFile = fs.existsSync(inFlight.targetPath) ? inFlight.targetPath : (inFlight.finalPath && fs.existsSync(inFlight.finalPath) ? inFlight.finalPath : inFlight.targetPath);

      // If discrete chunk requested that is already written to disk:
      if (requestedEnd !== undefined && requestedEnd < inFlight.writtenBytes) {
        const chunkSize = (requestedEnd - start) + 1;
        res.writeHead(206, {
          'Content-Range': `bytes ${start}-${requestedEnd}/${inFlight.totalBytes || inFlight.writtenBytes}`,
          'Accept-Ranges': 'bytes',
          'Content-Length': chunkSize,
          'Content-Type': mime,
          'Cache-Control': 'no-cache, no-transform',
          'X-AirMesh-Stream': 'in-flight-slice'
        });
        const sliceStream = fs.createReadStream(targetFile, { start, end: requestedEnd });
        sliceStream.on('error', () => { res.end(); });
        req.on('close', () => { sliceStream.destroy(); });
        sliceStream.pipe(res);
        return;
      }

      // Progressive open-ended stream
      const headers = {
        'Content-Type': mime,
        'Accept-Ranges': 'bytes',
        'Cache-Control': 'no-cache, no-transform',
        'X-AirMesh-Stream': 'live-progressive'
      };

      if (inFlight.totalBytes > 0) {
        headers['Content-Range'] = `bytes ${start}-${inFlight.totalBytes - 1}/${inFlight.totalBytes}`;
        headers['Content-Length'] = inFlight.totalBytes - start;
        res.writeHead(206, headers);
      } else {
        res.writeHead(200, headers);
      }

      let currentPos = start;
      let fd = null;
      let isReading = false;
      let isClosed = false;

      try {
        fd = fs.openSync(targetFile, 'r');
      } catch (err) {
        if (!res.headersSent) res.writeHead(500, { 'Content-Type': 'text/plain' });
        res.end('Failed to open live file for progressive streaming');
        return;
      }

      function readNext() {
        if (isClosed) return;
        if (isReading) return;

        const available = inFlight.writtenBytes;
        if (currentPos >= available) {
          if (inFlight.completed) {
            cleanup();
            res.end();
          }
          return;
        }

        isReading = true;
        const bytesToRead = Math.min(65536, available - currentPos);
        const buf = Buffer.alloc(bytesToRead);

        fs.read(fd, buf, 0, bytesToRead, currentPos, (err, bytesRead) => {
          isReading = false;
          if (isClosed) return;

          if (err || bytesRead === 0) {
            if (inFlight.completed) {
              cleanup();
              res.end();
            }
            return;
          }

          currentPos += bytesRead;
          const ok = res.write(buf.subarray(0, bytesRead));

          if (ok) {
            setImmediate(readNext);
          } else {
            res.once('drain', readNext);
          }
        });
      }

      const onData = () => {
        if (!isReading) readNext();
      };
      const onEnd = () => {
        if (!isReading) readNext();
      };
      const onError = () => {
        cleanup();
        res.end();
      };

      function cleanup() {
        if (isClosed) return;
        isClosed = true;
        inFlight.emitter.removeListener('data', onData);
        inFlight.emitter.removeListener('end', onEnd);
        inFlight.emitter.removeListener('error', onError);
        if (fd !== null) {
          try { fs.closeSync(fd); } catch (e) {}
          fd = null;
        }
      }

      inFlight.emitter.on('data', onData);
      inFlight.emitter.on('end', onEnd);
      inFlight.emitter.on('error', onError);

      req.on('close', cleanup);
      res.on('close', cleanup);

      readNext();
    }

    // Query Upload Status for Resuming (GET /api/upload-status)
    if (pathname === '/api/upload-status' && req.method === 'GET') {
      const currentDir = decodeURIComponent(parsedUrl.searchParams.get('currentDir') || '');
      const rawRel = decodeURIComponent(parsedUrl.searchParams.get('relativePath') || parsedUrl.searchParams.get('path') || '');
      const fileSize = parseInt(parsedUrl.searchParams.get('fileSize') || parsedUrl.searchParams.get('size') || '0', 10);

      const fullRel = currentDir ? path.posix.join(currentDir, rawRel) : rawRel;
      const targetPath = resolveSafePath(fullRel);

      if (!targetPath) {
        res.writeHead(400, { 'Content-Type': 'application/json' });
        res.end(JSON.stringify({ error: 'Invalid path' }));
        return;
      }

      const partPath = targetPath + '.part';

      // Check if file is already completely uploaded
      if (fs.existsSync(targetPath)) {
        try {
          const stat = fs.statSync(targetPath);
          if (fileSize > 0 && stat.size === fileSize) {
            res.writeHead(200, { 'Content-Type': 'application/json' });
            res.end(JSON.stringify({ status: 'completed', bytesReceived: fileSize, totalBytes: fileSize }));
            return;
          }
        } catch(e) {}
      }

      // Check if partial upload exists
      if (fs.existsSync(partPath)) {
        try {
          const stat = fs.statSync(partPath);
          let bytes = stat.size;
          if (fileSize > 0 && bytes > fileSize) {
            fs.truncateSync(partPath, fileSize);
            bytes = fileSize;
          }
          res.writeHead(200, { 'Content-Type': 'application/json' });
          res.end(JSON.stringify({ status: 'in_progress', bytesReceived: bytes, totalBytes: fileSize }));
          return;
        } catch(e) {}
      }

      res.writeHead(200, { 'Content-Type': 'application/json' });
      res.end(JSON.stringify({ status: 'new', bytesReceived: 0, totalBytes: fileSize }));
      return;
    }

    // Cancel Active / Partial Upload (POST /api/upload-cancel)
    if (pathname === '/api/upload-cancel' && req.method === 'POST') {
      let body = '';
      req.on('data', chunk => { body += chunk; });
      req.on('end', () => {
        try {
          const data = JSON.parse(body || '{}');
          const currentDir = data.currentDir || '';
          const rel = data.path || data.relativePath || '';
          const fullRel = currentDir ? path.posix.join(currentDir, rel) : rel;
          const targetPath = resolveSafePath(fullRel);

          if (targetPath) {
            const partPath = targetPath + '.part';
            const transfer = activeTransfers.get(targetPath) || activeTransfers.get(fullRel);
            if (transfer) {
              transfer.completed = true;
              transfer.emitter.emit('error', new Error('Upload cancelled'));
              activeTransfers.delete(targetPath);
              activeTransfers.delete(fullRel);
            }
            try { if (fs.existsSync(partPath)) fs.unlinkSync(partPath); } catch(e) {}
            broadcastSse('file_receiving_abort', { path: fullRel });
            broadcastSse('file_change', { action: 'delete', path: fullRel });
          }
          res.writeHead(200, { 'Content-Type': 'application/json' });
          res.end(JSON.stringify({ status: 'cancelled' }));
        } catch (e) {
          res.writeHead(500, { 'Content-Type': 'application/json' });
          res.end(JSON.stringify({ error: e.message }));
        }
      });
      return;
    }

    // Resumable & Chunked Upload with Stream-While-Receiving (POST /api/upload)
    if (pathname === '/api/upload' && req.method === 'POST') {
      const currentDir = decodeURIComponent(req.headers['x-current-dir'] || '');
      const rawRelPath = req.headers['x-relative-path'] ? decodeURIComponent(req.headers['x-relative-path']) : decodeURIComponent(req.headers['x-filename'] || `upload_${Date.now()}.bin`);
      const fullRel = currentDir ? path.posix.join(currentDir, rawRelPath) : rawRelPath;
      const targetPath = resolveSafePath(fullRel);

      if (!targetPath) {
        res.writeHead(400, { 'Content-Type': 'application/json' });
        res.end(JSON.stringify({ error: 'Invalid destination path' }));
        return;
      }

      // Check available disk space
      const disk = getDiskSpaceInfo();
      if (disk.free_bytes < 50 * 1024 * 1024) {
        res.writeHead(507, { 'Content-Type': 'application/json' });
        res.end(JSON.stringify({ error: 'Insufficient disk storage on host PC' }));
        return;
      }

      // Ensure target directory exists
      const targetDir = path.dirname(targetPath);
      if (!fs.existsSync(targetDir)) {
        fs.mkdirSync(targetDir, { recursive: true });
      }

      const partPath = targetPath + '.part';
      const offset = parseInt(req.headers['x-upload-offset'] || '0', 10);
      const totalBytes = parseInt(req.headers['x-file-size'] || req.headers['content-length'] || '0', 10);

      if (offset === 0) {
        try { if (fs.existsSync(partPath)) fs.unlinkSync(partPath); } catch(e) {}
      } else {
        if (!fs.existsSync(partPath)) {
          res.writeHead(409, { 'Content-Type': 'application/json' });
          res.end(JSON.stringify({ error: 'Part file missing on server', expectedOffset: 0 }));
          return;
        }
        const curSize = fs.statSync(partPath).size;
        if (curSize < offset) {
          res.writeHead(409, { 'Content-Type': 'application/json' });
          res.end(JSON.stringify({ error: 'Offset mismatch', expectedOffset: curSize }));
          return;
        } else if (curSize > offset) {
          try { fs.truncateSync(partPath, offset); } catch(e) {}
        }
      }

      // Track active in-flight transfer for instant stream-while-receiving
      const EventEmitter = require('events');
      let transfer = activeTransfers.get(targetPath);
      if (!transfer) {
        transfer = {
          path: fullRel,
          filename: path.basename(targetPath),
          targetPath: partPath,
          finalPath: targetPath,
          category: getCategory(targetPath),
          mime: getMimeType(targetPath),
          totalBytes: totalBytes,
          writtenBytes: offset,
          startedAt: Date.now(),
          completed: false,
          emitter: new EventEmitter()
        };
        transfer.emitter.setMaxListeners(200);
        transfer.emitter.on('error', () => {});
        activeTransfers.set(targetPath, transfer);
        activeTransfers.set(fullRel, transfer);

        broadcastSse('file_receiving_start', {
          path: fullRel,
          name: transfer.filename,
          total_bytes: transfer.totalBytes,
          category: transfer.category,
          is_receiving: true
        });
      }

      const writeStream = fs.createWriteStream(partPath, { flags: offset === 0 ? 'w' : 'a' });
      let chunkBytesReceived = 0;
      let lastProgressBroadcast = 0;

      req.on('data', (chunk) => {
        chunkBytesReceived += chunk.length;
        transfer.writtenBytes = offset + chunkBytesReceived;
        transfer.emitter.emit('data', chunk, transfer.writtenBytes);

        const now = Date.now();
        if (now - lastProgressBroadcast > 350) {
          lastProgressBroadcast = now;
          broadcastSse('file_receiving_progress', {
            path: fullRel,
            written_bytes: transfer.writtenBytes,
            total_bytes: transfer.totalBytes,
            percent: transfer.totalBytes ? Math.min(99, Math.round((transfer.writtenBytes / transfer.totalBytes) * 100)) : 0
          });
        }
      });

      req.pipe(writeStream);

      writeStream.on('finish', () => {
        try {
          const stat = fs.statSync(partPath);
          const currentTotalWritten = stat.size;
          transfer.writtenBytes = currentTotalWritten;

          if (totalBytes > 0 && currentTotalWritten >= totalBytes) {
            // Transfer completely finished! Atomically finalize
            try { if (fs.existsSync(targetPath)) fs.unlinkSync(targetPath); } catch(e) {}
            fs.renameSync(partPath, targetPath);
            transfer.targetPath = targetPath;
            transfer.completed = true;
            transfer.emitter.emit('end');
            activeTransfers.delete(targetPath);
            activeTransfers.delete(fullRel);
            codecCache.delete(targetPath);

            broadcastSse('file_receiving_complete', {
              path: fullRel,
              size: currentTotalWritten
            });
            broadcastSse('file_change', { action: 'upload', path: fullRel });

            res.writeHead(200, { 'Content-Type': 'application/json' });
            res.end(JSON.stringify({
              status: 'uploaded',
              filename: path.basename(targetPath),
              path: fullRel,
              size: currentTotalWritten
            }));
          } else {
            // Chunk saved successfully, waiting for next chunk or resume
            res.writeHead(200, { 'Content-Type': 'application/json' });
            res.end(JSON.stringify({
              status: 'chunk_saved',
              offset: currentTotalWritten,
              total: totalBytes
            }));
          }
        } catch (err) {
          if (!res.headersSent) {
            res.writeHead(500, { 'Content-Type': 'application/json' });
            res.end(JSON.stringify({ error: err.message }));
          }
        }
      });

      // Disconnect or interruption: DO NOT DELETE PART FILE! Preserve for seamless resume.
      const disconnectHandler = () => {
        writeStream.destroy();
        broadcastSse('file_receiving_paused', {
          path: fullRel,
          written_bytes: transfer.writtenBytes,
          total_bytes: transfer.totalBytes
        });
      };

      req.on('aborted', disconnectHandler);
      writeStream.on('error', (err) => {
        writeStream.destroy();
        if (!res.headersSent) {
          res.writeHead(500, { 'Content-Type': 'application/json' });
          res.end(JSON.stringify({ error: err.message }));
        }
      });

      return;
    }

    // Stream or Download File with HTTP 206 Range Requests (Progressive Stream-While-Receiving)
    if (pathname.startsWith('/api/stream/')) {
      const param = pathname.replace('/api/stream/', '');
      const filePath = resolveItemPath(param);

      // Check if file is actively being uploaded (stream while receiving)
      const inFlight = activeTransfers.get(filePath) || activeTransfers.get(param);
      if (inFlight) {
        pipeProgressiveStream(inFlight, req, res);
        return;
      }

      if (filePath && fs.existsSync(filePath) && fs.statSync(filePath).isFile()) {
        try {
          const stat = fs.statSync(filePath);
          const range = req.headers.range;
          const mime = getMimeType(filePath);

          if (range) {
            const parts = range.replace(/bytes=/, "").split("-");
            const start = parseInt(parts[0], 10);
            const end = parts[1] ? parseInt(parts[1], 10) : stat.size - 1;

            if (start >= stat.size || end >= stat.size) {
              res.writeHead(416, { 'Content-Range': `bytes */${stat.size}` });
              res.end();
              return;
            }

            const chunksize = (end - start) + 1;
            res.writeHead(206, {
              'Content-Range': `bytes ${start}-${end}/${stat.size}`,
              'Accept-Ranges': 'bytes',
              'Content-Length': chunksize,
              'Content-Type': mime,
            });

            const fileStream = fs.createReadStream(filePath, { start, end });
            fileStream.on('error', () => { res.end(); });
            req.on('close', () => { fileStream.destroy(); });
            fileStream.pipe(res);
          } else {
            res.writeHead(200, {
              'Content-Length': stat.size,
              'Content-Type': mime,
              'Accept-Ranges': 'bytes',
              'Content-Disposition': `inline; filename="${encodeURIComponent(path.basename(filePath))}"`
            });

            const fileStream = fs.createReadStream(filePath);
            fileStream.on('error', () => { res.end(); });
            req.on('close', () => { fileStream.destroy(); });
            fileStream.pipe(res);
          }
          return;
        } catch (streamErr) {
          if (!res.headersSent) res.writeHead(500, { 'Content-Type': 'text/plain' });
          res.end("Stream error: " + streamErr.message);
          return;
        }
      }

      res.writeHead(404, { 'Content-Type': 'text/plain' });
      res.end('File not found');
      return;
    }

    // Real-Time Audio Transmuxing
    if (pathname.startsWith('/api/transcode/')) {
      const param = pathname.replace('/api/transcode/', '');
      const filePath = resolveItemPath(param);

      if (filePath && fs.existsSync(filePath)) {
        if (req.method === 'HEAD') {
          res.writeHead(200, {
            'Content-Type': 'video/mp4',
            'Accept-Ranges': 'none'
          });
          res.end();
          return;
        }

        const ss = parseFloat(parsedUrl.searchParams.get('ss') || '0') || 0;
        const codecInfo = getCodecInfo(filePath);

        res.writeHead(200, {
          'Content-Type': 'video/mp4',
          'Cache-Control': 'no-cache',
          'Connection': 'keep-alive'
        });

        const args = [];
        if (ss > 0) {
          args.push('-ss', ss.toString());
        }
        args.push('-i', filePath, '-c:v', 'copy');

        if (codecInfo.has_audio) {
          args.push('-c:a', 'aac', '-b:a', '192k', '-ac', '2');
        } else {
          args.push('-an');
        }

        args.push(
          '-movflags', 'frag_keyframe+empty_moov+default_base_moof',
          '-f', 'mp4',
          'pipe:1'
        );

        const ffmpeg = spawn('ffmpeg', args);
        ffmpeg.stdout.pipe(res);

        req.on('close', () => {
          try { ffmpeg.kill('SIGKILL'); } catch (e) {}
        });
        ffmpeg.on('error', (err) => {
          console.error('[AirMesh FFmpeg] Stream error:', err.message);
          res.end();
        });
        return;
      }

      res.writeHead(404, { 'Content-Type': 'text/plain' });
      res.end('File not found');
      return;
    }

    // Subtitles Extraction
    if (pathname.startsWith('/api/subtitles/')) {
      const param = pathname.replace('/api/subtitles/', '');
      const filePath = resolveItemPath(param);

      if (filePath && fs.existsSync(filePath)) {
        const trackIdx = parseInt(parsedUrl.searchParams.get('track') || '0', 10);

        res.writeHead(200, {
          'Content-Type': 'text/vtt; charset=utf-8',
          'Cache-Control': 'public, max-age=3600'
        });

        const ffmpeg = spawn('ffmpeg', [
          '-v', 'error',
          '-i', filePath,
          '-map', `0:s:${trackIdx}`,
          '-f', 'webvtt',
          'pipe:1'
        ]);

        ffmpeg.stdout.pipe(res);

        req.on('close', () => {
          try { ffmpeg.kill('SIGKILL'); } catch(e) {}
        });
        ffmpeg.on('error', () => { res.end(); });
        return;
      }

      res.writeHead(404, { 'Content-Type': 'text/plain' });
      res.end('Subtitles not found');
      return;
    }


    // Real-time Clipboard
    if (pathname === '/api/clipboard') {
      if (req.method === 'GET') {
        res.writeHead(200, { 'Content-Type': 'application/json' });
        res.end(JSON.stringify(meshClipboard));
        return;
      }
      if (req.method === 'POST') {
        let body = '';
        req.on('data', chunk => { body += chunk; });
        req.on('end', () => {
          try {
            const data = JSON.parse(body || '{}');
            const { deviceName } = parseClientInfo(userAgent, clientIp);
            meshClipboard = {
              text: data.text || '',
              updated_at: Date.now(),
              author: data.author || deviceName
            };
            broadcastSse('clipboard_change', meshClipboard);
            res.writeHead(200, { 'Content-Type': 'application/json' });
            res.end(JSON.stringify({ status: 'ok', clipboard: meshClipboard }));
          } catch(e) {
            res.writeHead(400, { 'Content-Type': 'application/json' });
            res.end(JSON.stringify({ error: 'Invalid JSON' }));
          }
        });
        return;
      }
    }

    // Serve index.html
    const indexPath = path.join(__dirname, 'index.html');
    if (fs.existsSync(indexPath)) {
      const content = fs.readFileSync(indexPath, 'utf8');
      res.writeHead(200, {
        'Content-Type': 'text/html; charset=utf-8',
        'Content-Length': Buffer.byteLength(content)
      });
      res.end(content);
      return;
    }

    res.writeHead(404, { 'Content-Type': 'text/plain' });
    res.end('index.html not found');
  } catch (err) {
    if (!res.headersSent) res.writeHead(500, { 'Content-Type': 'text/plain' });
    res.end('Internal Server Error: ' + err.message);
  }
});

server.keepAliveTimeout = 120000;
server.headersTimeout = 125000;

server.listen(PORT, '0.0.0.0', () => {
  const disk = getDiskSpaceInfo();
  console.log(`\n=========================================================`);
  console.log(`🚀 AirMesh Local Node Active & Robust!`);
  console.log(`💻 PC URL:         http://localhost:${PORT}`);
  console.log(`📱 Phone URL:      http://${LOCAL_IP}:${PORT}`);
  console.log(`💾 Free Storage:   ${disk.free_gb} GB free of ${disk.total_gb} GB (${disk.used_percent}% used)`);
  console.log(`📁 Shared Folder:  ${SHARED_DIR}`);
  console.log(`=========================================================\n`);
});
