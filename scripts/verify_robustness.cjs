const http = require('http');

const endpoints = [
  { name: 'Root UI (HTML)', url: 'http://localhost:47823/' },
  { name: 'Health Endpoint', url: 'http://localhost:47823/api/health' },
  { name: 'Files List', url: 'http://localhost:47823/api/files' },
  { name: 'Connected Peers', url: 'http://localhost:47823/api/peers' },
  { name: 'Mesh Clipboard', url: 'http://localhost:47823/api/clipboard' },
  { name: 'PWA Manifest', url: 'http://localhost:47823/manifest.json' },
  { name: 'Service Worker', url: 'http://localhost:47823/sw.js' },
  { name: 'Photo Thumbnail', url: 'http://localhost:47823/api/thumbnail/32303236303932365f3138323030342e6a7067' },
  { name: 'Video Thumbnail', url: 'http://localhost:47823/api/thumbnail/32303236303931345f3131323232302e6d7034' },
  { name: 'QR Code SVG', url: 'http://localhost:47823/api/qr.svg' }
];

async function runTests() {
  console.log('\n=== AIRMESH SYSTEM ROBUSTNESS VERIFICATION ===\n');
  let passed = 0;
  for (const ep of endpoints) {
    await new Promise((resolve) => {
      const req = http.get(ep.url, (res) => {
        if (res.statusCode === 200) {
          console.log(`✅ [PASS] ${ep.name.padEnd(25)} -> HTTP 200 (Type: ${res.headers['content-type'] || 'none'})`);
          passed++;
        } else {
          console.log(`❌ [FAIL] ${ep.name.padEnd(25)} -> HTTP ${res.statusCode}`);
        }
        res.resume();
        resolve();
      });
      req.on('error', (err) => {
        console.log(`❌ [ERR]  ${ep.name.padEnd(25)} -> ${err.message}`);
        resolve();
      });
      req.setTimeout(4000, () => {
        req.destroy();
        console.log(`❌ [TIMEOUT] ${ep.name.padEnd(25)}`);
        resolve();
      });
    });
  }
  console.log(`\nResults: ${passed} / ${endpoints.length} checks passed.\n`);
}

runTests();
