const fs = require('fs');
const path = require('path');

const html = fs.readFileSync(path.join(__dirname, 'index.html'), 'utf8');
const rs = `//! Embedded Web UI & PWA single-page application.
//!
//! Fully self-hosted, zero external CDNs, WCAG AA compliant,
//! TV Remote D-Pad spatial navigation, and offline-first PWA.

/// Embedded HTML, CSS and JS for the web interface.
pub const EMBEDDED_INDEX_HTML: &str = r##"${html}"##;
`;

fs.writeFileSync(path.join(__dirname, 'airmesh-daemon', 'src', 'embedded_web', 'assets.rs'), rs, 'utf8');
console.log('Successfully updated assets.rs');
