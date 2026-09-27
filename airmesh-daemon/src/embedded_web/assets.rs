//! Embedded Web UI & PWA single-page application.
//!
//! Fully self-hosted, zero external CDNs, WCAG AA compliant,
//! TV Remote D-Pad spatial navigation, and offline-first PWA.

/// Embedded HTML, CSS and JS for the web interface.
pub const EMBEDDED_INDEX_HTML: &str = r##"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta http-equiv="Content-Security-Policy" content="default-src 'self' data: blob:; connect-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'; img-src 'self' data: blob:; media-src 'self' blob:; object-src 'none'; font-src 'self' data:;">
  <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no, viewport-fit=cover">
  <meta name="theme-color" content="#090a0f">
  <meta name="apple-mobile-web-app-capable" content="yes">
  <meta name="apple-mobile-web-app-status-bar-style" content="black-translucent">
  <meta name="description" content="AirMesh - Minimalist, Systematic 100% Local Peer-to-Peer File & Folder Mesh.">
  <link rel="manifest" href="/manifest.json">
  <title>AirMesh &bull; Local WiFi Mesh</title>
  <style>
    :root {
      --bg: #0a0c10;
      --surface: #11141c;
      --surface-subtle: #161924;
      --surface-hover: #1c2130;
      --surface-active: #232a3d;
      --border: rgba(255, 255, 255, 0.08);
      --border-subtle: rgba(255, 255, 255, 0.04);
      --border-hover: rgba(255, 255, 255, 0.16);
      --border-focus: rgba(56, 189, 248, 0.5);
      --text: #f0f2f5;
      --text-secondary: #c5cad6;
      --text-muted: #8b92a4;
      --text-dim: #545b6d;
      --accent: #38bdf8;
      --accent-hover: #0ea5e9;
      --accent-subtle: rgba(56, 189, 248, 0.08);
      --success: #10b981;
      --success-subtle: rgba(16, 185, 129, 0.08);
      --danger: #f43f5e;
      --danger-subtle: rgba(244, 63, 94, 0.08);
      --radius-sm: 5px;
      --radius: 7px;
      --radius-lg: 12px;
      --radius-full: 9999px;
      --safe-top: env(safe-area-inset-top, 0px);
      --safe-bottom: env(safe-area-inset-bottom, 0px);
    }

    * { box-sizing: border-box; margin: 0; padding: 0; -webkit-tap-highlight-color: transparent; }

    body {
      font-family: -apple-system, BlinkMacSystemFont, "SF Pro Text", "Segoe UI", Roboto, "Helvetica Neue", sans-serif;
      background: var(--bg);
      color: var(--text);
      min-height: 100vh;
      display: flex;
      flex-direction: column;
      line-height: 1.5;
      letter-spacing: -0.01em;
      -webkit-font-smoothing: antialiased;
      -moz-osx-font-smoothing: grayscale;
      padding-bottom: calc(70px + var(--safe-bottom));
    }

    .tnum {
      font-variant-numeric: tabular-nums;
      font-feature-settings: "tnum" 1;
    }

    /* Minimalist Header */
    header {
      background: var(--surface);
      border-bottom: 1px solid var(--border);
      padding: max(0.55rem, var(--safe-top)) 1.25rem 0.55rem 1.25rem;
      display: flex;
      justify-content: space-between;
      align-items: center;
      position: sticky;
      top: 0;
      z-index: 100;
    }

    .brand {
      display: flex;
      align-items: center;
      gap: 0.5rem;
      text-decoration: none;
      color: #fff;
      user-select: none;
    }
    .brand-badge {
      width: 26px;
      height: 26px;
      border-radius: var(--radius-sm);
      background: var(--accent-subtle);
      border: 1px solid rgba(56, 189, 248, 0.25);
      display: flex;
      align-items: center;
      justify-content: center;
      font-size: 0.85rem;
      color: var(--accent);
      flex-shrink: 0;
    }
    .brand-name {
      font-size: 1rem;
      font-weight: 600;
      letter-spacing: -0.02em;
      color: #fff;
    }
    .brand-tag {
      font-size: 0.62rem;
      font-weight: 600;
      font-family: ui-monospace, SFMono-Regular, monospace;
      padding: 1px 5px;
      border-radius: 4px;
      background: var(--surface-subtle);
      border: 1px solid var(--border);
      color: var(--text-dim);
      letter-spacing: 0.04em;
    }

    .header-center {
      display: flex;
      align-items: center;
      gap: 0.5rem;
    }

    /* Unified Status Pill */
    .nav-status-pill {
      display: inline-flex;
      align-items: center;
      gap: 0.5rem;
      padding: 0.28rem 0.75rem;
      border-radius: var(--radius-full);
      background: var(--surface-subtle);
      border: 1px solid var(--border);
      font-size: 0.75rem;
      cursor: pointer;
      user-select: none;
      transition: background 0.12s ease, border-color 0.12s ease;
    }
    .nav-status-pill:hover {
      background: var(--surface-hover);
      border-color: var(--border-hover);
    }
    .status-dot-pulse {
      width: 6px;
      height: 6px;
      border-radius: 50%;
      background: var(--success);
      box-shadow: 0 0 0 2px rgba(16, 185, 129, 0.2);
      flex-shrink: 0;
    }
    .nav-status-mono {
      font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
      color: var(--text-secondary);
      font-variant-numeric: tabular-nums;
    }
    .nav-status-divider {
      width: 1px;
      height: 10px;
      background: var(--border);
    }
    .nav-status-green {
      color: #34d399;
      font-weight: 500;
    }

    .desktop-nav-actions {
      display: flex;
      align-items: center;
      gap: 0.4rem;
    }

    .mobile-header-actions {
      display: none;
      align-items: center;
      gap: 0.4rem;
    }

    /* 100% Offline Strip & Cellular Warning */
    .offline-status-strip {
      background: #0c0f16;
      border-bottom: 1px solid var(--border-subtle);
      padding: 0.35rem 1.25rem;
      display: flex;
      align-items: center;
      justify-content: space-between;
      font-size: 0.74rem;
      color: var(--text-muted);
      cursor: pointer;
      user-select: none;
      touch-action: manipulation;
      transition: background 0.12s ease;
    }
    .offline-status-strip:hover {
      background: #11151f;
    }
    .offline-status-left {
      display: flex;
      align-items: center;
      gap: 0.5rem;
    }
    .offline-dot-pulse {
      width: 6px;
      height: 6px;
      border-radius: 50%;
      background: var(--success);
      flex-shrink: 0;
    }
    .offline-status-tag {
      font-size: 0.7rem;
      color: var(--accent);
      font-weight: 500;
      white-space: nowrap;
      display: flex;
      align-items: center;
      gap: 0.25rem;
    }

    .cellular-warning-bar {
      background: rgba(245, 158, 11, 0.08);
      border-bottom: 1px solid rgba(245, 158, 11, 0.2);
      color: #fbbf24;
      padding: 0.4rem 1.25rem;
      display: none;
      align-items: center;
      justify-content: space-between;
      font-size: 0.76rem;
      gap: 0.5rem;
    }

    /* Minimalist Cards in Offline Modal */
    .offline-card {
      background: var(--surface-subtle);
      border: 1px solid var(--border);
      border-radius: var(--radius);
      padding: 0.8rem;
      display: flex;
      flex-direction: column;
      gap: 0.35rem;
    }
    .offline-card-title {
      font-weight: 600;
      font-size: 0.82rem;
      color: #fff;
      display: flex;
      align-items: center;
      gap: 0.45rem;
    }
    .offline-card-desc {
      font-size: 0.78rem;
      color: var(--text-muted);
      line-height: 1.45;
    }
    .offline-steps {
      display: flex;
      flex-direction: column;
      gap: 0.35rem;
      margin-top: 0.25rem;
    }
    .offline-step-item {
      display: flex;
      align-items: flex-start;
      gap: 0.5rem;
      font-size: 0.76rem;
      color: var(--text-muted);
      line-height: 1.4;
    }
    .offline-step-num {
      width: 17px;
      height: 17px;
      border-radius: 50%;
      background: var(--surface);
      border: 1px solid var(--border);
      color: var(--accent);
      display: flex;
      align-items: center;
      justify-content: center;
      font-size: 0.65rem;
      font-weight: 700;
      flex-shrink: 0;
      margin-top: 1px;
    }

    /* Buttons */
    .btn {
      padding: 0 0.75rem;
      height: 32px;
      border-radius: var(--radius);
      font-weight: 500;
      font-size: 0.8rem;
      cursor: pointer;
      border: 1px solid transparent;
      display: inline-flex;
      align-items: center;
      justify-content: center;
      gap: 0.4rem;
      text-decoration: none;
      white-space: nowrap;
      touch-action: manipulation;
      user-select: none;
      transition: background 0.12s ease, border-color 0.12s ease, color 0.12s ease, opacity 0.12s ease;
    }
    .btn:active { transform: scale(0.98); }
    .btn-primary {
      background: var(--text);
      color: var(--bg);
      border-color: var(--text);
      font-weight: 600;
    }
    .btn-primary:hover {
      background: #ffffff;
      color: #000000;
    }
    .btn-accent {
      background: var(--accent);
      color: #090a0f;
      font-weight: 600;
    }
    .btn-accent:hover { background: var(--accent-hover); }
    .btn-secondary {
      background: var(--surface-subtle);
      color: var(--text);
      border-color: var(--border);
    }
    .btn-secondary:hover {
      background: var(--surface-hover);
      border-color: var(--border-hover);
    }
    .btn-danger {
      background: var(--danger-subtle);
      color: var(--danger);
      border-color: rgba(244, 63, 94, 0.2);
    }
    .btn-danger:hover {
      background: var(--danger);
      color: #fff;
    }
    .btn-sm {
      height: 28px;
      padding: 0 0.55rem;
      font-size: 0.76rem;
    }
    .btn-icon {
      width: 32px;
      height: 32px;
      padding: 0;
      font-size: 0.85rem;
    }

    /* Main Container */
    .main-wrapper {
      max-width: 1140px;
      width: 100%;
      margin: 0 auto;
      padding: 1.15rem;
      flex: 1;
      display: flex;
      flex-direction: column;
      gap: 0.85rem;
    }

    /* Desktop View Switcher Tabs */
    .tabs-bar {
      display: flex;
      justify-content: space-between;
      align-items: center;
      border-bottom: 1px solid var(--border-subtle);
      padding-bottom: 0.45rem;
    }
    .tab-nav {
      display: flex;
      gap: 0.2rem;
    }
    .tab-item {
      padding: 0.35rem 0.75rem;
      border-radius: var(--radius);
      background: transparent;
      color: var(--text-muted);
      border: 1px solid transparent;
      font-weight: 500;
      font-size: 0.82rem;
      cursor: pointer;
      display: flex;
      align-items: center;
      gap: 0.45rem;
      transition: all 0.12s ease;
    }
    .tab-item:hover {
      color: var(--text);
      background: var(--surface-hover);
    }
    .tab-item.active {
      color: #fff;
      background: var(--surface);
      border-color: var(--border);
      font-weight: 600;
    }
    .tab-count {
      font-size: 0.68rem;
      padding: 0.05rem 0.35rem;
      border-radius: 4px;
      background: var(--surface-subtle);
      color: var(--text-muted);
      border: 1px solid var(--border);
      font-variant-numeric: tabular-nums;
    }

    /* View Mode (List vs Grid) */
    .view-toggle-group {
      display: flex;
      background: var(--surface);
      border: 1px solid var(--border);
      border-radius: var(--radius);
      padding: 2px;
      gap: 2px;
    }
    .view-toggle-btn {
      padding: 0.2rem 0.5rem;
      border: none;
      background: transparent;
      color: var(--text-muted);
      cursor: pointer;
      border-radius: 4px;
      font-size: 0.75rem;
      display: flex;
      align-items: center;
      gap: 0.3rem;
      transition: all 0.1s ease;
    }
    .view-toggle-btn:hover { color: var(--text); }
    .view-toggle-btn.active {
      background: var(--surface-active);
      color: #fff;
      font-weight: 600;
    }

    /* Breadcrumbs Toolbar */
    .folder-nav-bar {
      display: flex;
      justify-content: space-between;
      align-items: center;
      flex-wrap: wrap;
      gap: 0.5rem;
      background: var(--surface);
      border: 1px solid var(--border);
      border-radius: var(--radius);
      padding: 0.45rem 0.75rem;
    }
    .breadcrumbs-box {
      display: flex;
      align-items: center;
      gap: 0.25rem;
      font-size: 0.82rem;
      overflow-x: auto;
      white-space: nowrap;
      -webkit-overflow-scrolling: touch;
      scrollbar-width: none;
      flex: 1;
    }
    .breadcrumbs-box::-webkit-scrollbar { display: none; }
    .crumb-link {
      color: var(--accent);
      cursor: pointer;
      text-decoration: none;
      font-weight: 500;
      padding: 0.15rem 0.35rem;
      border-radius: 4px;
      transition: background 0.1s ease;
    }
    .crumb-link:hover {
      background: var(--surface-hover);
    }
    .crumb-sep {
      color: var(--text-dim);
      font-size: 0.75rem;
    }
    .crumb-current {
      color: #fff;
      font-weight: 600;
      padding: 0.15rem 0.35rem;
    }
    .folder-actions-row {
      display: flex;
      align-items: center;
      gap: 0.4rem;
      flex-shrink: 0;
    }

    /* Search & Filter Bar */
    .search-filter-row {
      display: flex;
      flex-wrap: wrap;
      gap: 0.5rem;
      justify-content: space-between;
      align-items: center;
    }
    .search-box {
      flex: 1;
      min-width: 220px;
      position: relative;
      display: flex;
      align-items: center;
    }
    .search-input {
      width: 100%;
      height: 34px;
      padding: 0 4.2rem 0 2.2rem;
      background: var(--surface-subtle);
      border: 1px solid var(--border);
      border-radius: var(--radius);
      color: #fff;
      font-size: 0.82rem;
      transition: border-color 0.12s ease, background 0.12s ease;
    }
    .search-input:focus {
      outline: none;
      border-color: var(--border-focus);
      background: var(--surface);
    }
    .search-icon {
      position: absolute;
      left: 0.75rem;
      color: var(--text-dim);
      pointer-events: none;
    }
    .search-clear-btn {
      position: absolute;
      right: 0.55rem;
      background: none;
      border: none;
      color: var(--text-dim);
      cursor: pointer;
      font-size: 0.75rem;
      padding: 0.2rem 0.35rem;
      border-radius: 4px;
      line-height: 1;
      display: none;
      align-items: center;
      justify-content: center;
    }
    .search-clear-btn:hover {
      color: #fff;
      background: var(--surface-hover);
    }
    .search-kbd {
      position: absolute;
      right: 0.55rem;
      font-family: ui-monospace, SFMono-Regular, monospace;
      font-size: 0.65rem;
      color: var(--text-dim);
      background: var(--surface);
      border: 1px solid var(--border);
      padding: 1px 5px;
      border-radius: 3px;
      pointer-events: none;
    }
    .category-group {
      display: flex;
      gap: 0.25rem;
      overflow-x: auto;
      white-space: nowrap;
      -webkit-overflow-scrolling: touch;
      scrollbar-width: none;
    }
    .category-group::-webkit-scrollbar { display: none; }
    .cat-btn {
      padding: 0 0.65rem;
      height: 32px;
      border-radius: var(--radius);
      font-size: 0.76rem;
      font-weight: 500;
      background: var(--surface-subtle);
      color: var(--text-muted);
      border: 1px solid var(--border);
      cursor: pointer;
      display: inline-flex;
      align-items: center;
      justify-content: center;
      transition: all 0.12s ease;
    }
    .cat-btn:hover {
      color: var(--text);
      background: var(--surface-hover);
      border-color: var(--border-hover);
    }
    .cat-btn.active {
      color: #fff;
      background: var(--surface-active);
      border-color: rgba(56, 189, 248, 0.4);
      font-weight: 600;
    }

    /* Minimalist Drag & Drop Strip */
    .drop-strip {
      border: 1px dashed var(--border);
      border-radius: var(--radius);
      padding: 0.75rem 1rem;
      text-align: center;
      background: var(--surface-subtle);
      cursor: pointer;
      display: flex;
      align-items: center;
      justify-content: center;
      gap: 0.6rem;
      color: var(--text-muted);
      font-size: 0.8rem;
      touch-action: manipulation;
      transition: all 0.15s ease;
    }
    .drop-strip:hover, .drop-strip.dragover {
      border-color: rgba(56, 189, 248, 0.4);
      background: var(--surface-hover);
      color: var(--text);
    }
    .drop-strip-icon {
      color: var(--accent);
      flex-shrink: 0;
    }

    /* Table View Container */
    .table-container {
      background: var(--surface);
      border: 1px solid var(--border);
      border-radius: var(--radius);
      overflow-x: auto;
    }
    table {
      width: 100%;
      border-collapse: collapse;
      text-align: left;
      font-size: 0.82rem;
    }
    th {
      padding: 0.55rem 0.85rem;
      background: var(--surface-subtle);
      color: var(--text-dim);
      font-size: 0.68rem;
      font-weight: 600;
      text-transform: uppercase;
      letter-spacing: 0.05em;
      border-bottom: 1px solid var(--border);
      cursor: pointer;
      user-select: none;
      transition: color 0.1s ease;
    }
    th:hover { color: var(--text); }
    td {
      padding: 0.6rem 0.85rem;
      border-bottom: 1px solid var(--border-subtle);
      vertical-align: middle;
    }
    tr:last-child td { border-bottom: none; }
    tr {
      transition: background 0.1s ease;
    }
    tr:hover td {
      background: var(--surface-hover);
    }

    .file-cell {
      display: flex;
      align-items: center;
      gap: 0.65rem;
      color: #fff;
      font-weight: 500;
      min-height: 36px;
    }
    .table-thumb {
      width: 32px;
      height: 32px;
      border-radius: 4px;
      object-fit: cover;
      background: var(--bg);
      border: 1px solid var(--border);
      flex-shrink: 0;
    }
    .folder-cell {
      display: flex;
      align-items: center;
      gap: 0.65rem;
      color: #fff;
      font-weight: 600;
      cursor: pointer;
      min-height: 36px;
    }
    .folder-cell:hover { color: var(--accent); }
    .folder-icon {
      font-size: 1.15rem;
      color: var(--accent);
      flex-shrink: 0;
    }

    .file-badge {
      font-size: 0.68rem;
      padding: 0.1rem 0.4rem;
      border-radius: 4px;
      background: var(--surface-subtle);
      color: var(--text-muted);
      border: 1px solid var(--border);
      display: inline-flex;
      align-items: center;
      gap: 0.25rem;
      white-space: nowrap;
      font-weight: 500;
    }

    /* Grid View Container */
    .grid-container {
      display: grid;
      grid-template-columns: repeat(auto-fill, minmax(185px, 1fr));
      gap: 0.75rem;
    }
    .grid-card {
      background: var(--surface);
      border: 1px solid var(--border);
      border-radius: var(--radius);
      overflow: hidden;
      display: flex;
      flex-direction: column;
      position: relative;
      transition: border-color 0.12s ease, background 0.12s ease;
    }
    .grid-card:hover {
      border-color: var(--border-hover);
      background: var(--surface-hover);
    }
    .grid-card.selected {
      border-color: var(--accent);
      background: rgba(56, 189, 248, 0.04);
    }
    .grid-card.folder-card {
      cursor: pointer;
      padding: 0.75rem;
      display: flex;
      flex-direction: row;
      align-items: center;
      gap: 0.65rem;
      min-height: 56px;
    }
    .grid-preview-box {
      width: 100%;
      height: 120px;
      background: #07080c;
      position: relative;
      display: flex;
      align-items: center;
      justify-content: center;
      overflow: hidden;
      cursor: pointer;
    }
    .grid-thumb-img {
      width: 100%;
      height: 100%;
      object-fit: cover;
    }
    .grid-file-icon {
      font-size: 2rem;
      color: var(--text-dim);
    }
    .grid-checkbox-wrap {
      position: absolute;
      top: 6px;
      left: 6px;
      z-index: 5;
    }
    .grid-card-body {
      padding: 0.6rem 0.7rem;
      display: flex;
      flex-direction: column;
      gap: 0.3rem;
      flex: 1;
      justify-content: space-between;
    }
    .grid-filename {
      font-size: 0.8rem;
      font-weight: 500;
      color: #fff;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
      line-height: 1.35;
    }
    .grid-meta-row {
      display: flex;
      justify-content: space-between;
      align-items: center;
      font-size: 0.7rem;
      color: var(--text-dim);
      font-variant-numeric: tabular-nums;
    }
    .grid-actions {
      display: flex;
      gap: 0.25rem;
      margin-top: 0.25rem;
      border-top: 1px solid var(--border-subtle);
      padding-top: 0.35rem;
    }

    /* Floating Batch Action Bar */
    .batch-bar {
      position: fixed;
      bottom: calc(65px + var(--safe-bottom));
      left: 50%;
      transform: translateX(-50%) translateY(120px);
      background: var(--surface);
      border: 1px solid var(--border-focus);
      padding: 0.45rem 0.85rem;
      border-radius: var(--radius-full);
      display: flex;
      align-items: center;
      gap: 0.65rem;
      z-index: 500;
      box-shadow: 0 12px 30px rgba(0, 0, 0, 0.7);
      transition: transform 0.2s cubic-bezier(0.16, 1, 0.3, 1);
    }
    .batch-bar.show { transform: translateX(-50%) translateY(0); }

    /* Live Upload Progress Card */
    .upload-hud {
      position: fixed;
      bottom: calc(70px + var(--safe-bottom));
      right: 1.25rem;
      background: var(--surface);
      border: 1px solid var(--border-focus);
      border-radius: var(--radius);
      padding: 0.8rem;
      width: min(340px, calc(100vw - 2.5rem));
      z-index: 1000;
      display: none;
      flex-direction: column;
      gap: 0.45rem;
      box-shadow: 0 10px 25px rgba(0,0,0,0.6);
    }
    .hud-bar {
      height: 4px;
      background: var(--bg);
      border-radius: 2px;
      overflow: hidden;
    }
    .hud-fill {
      height: 100%;
      background: var(--accent);
      width: 0%;
      transition: width 0.1s linear;
    }

    /* Persistent Bottom Audio Player HUD */
    .audio-player-hud {
      position: fixed;
      bottom: 0;
      left: 0;
      right: 0;
      background: var(--surface);
      border-top: 1px solid var(--border);
      padding: 0.5rem 1.25rem;
      display: none;
      align-items: center;
      justify-content: space-between;
      gap: 1rem;
      z-index: 800;
      box-shadow: 0 -4px 20px rgba(0, 0, 0, 0.4);
    }
    .audio-info-col {
      display: flex;
      align-items: center;
      gap: 0.65rem;
      min-width: 160px;
      max-width: 260px;
    }
    .audio-icon-box {
      font-size: 1.1rem;
      color: var(--accent);
    }
    .audio-text-col { overflow: hidden; }
    .audio-hud-title {
      font-size: 0.82rem;
      font-weight: 500;
      color: #fff;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }
    .audio-hud-time {
      font-size: 0.72rem;
      color: var(--text-dim);
      font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
      font-variant-numeric: tabular-nums;
    }
    .audio-controls-col {
      display: flex;
      align-items: center;
      gap: 0.65rem;
      flex: 1;
      max-width: 480px;
    }
    .audio-btn-circle {
      width: 32px;
      height: 32px;
      border-radius: var(--radius-sm);
      background: var(--accent);
      color: #090a0f;
      border: none;
      cursor: pointer;
      display: flex;
      align-items: center;
      justify-content: center;
      font-size: 0.85rem;
      flex-shrink: 0;
    }
    .audio-btn-circle:hover { background: var(--accent-hover); }
    .audio-seek-slider {
      flex: 1;
      accent-color: var(--accent);
      cursor: pointer;
      height: 4px;
    }

    /* Mobile Sticky Bottom Navigation Bar */
    .mobile-bottom-nav {
      display: none;
      position: fixed;
      bottom: 0;
      left: 0;
      right: 0;
      height: calc(56px + var(--safe-bottom));
      padding-bottom: var(--safe-bottom);
      background: var(--surface);
      border-top: 1px solid var(--border);
      z-index: 900;
      justify-content: space-around;
      align-items: center;
    }
    .mobile-nav-btn {
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: center;
      gap: 0.15rem;
      background: none;
      border: none;
      color: var(--text-muted);
      font-size: 0.68rem;
      font-weight: 500;
      cursor: pointer;
      flex: 1;
      height: 100%;
      touch-action: manipulation;
    }
    .mobile-nav-btn.active {
      color: var(--accent);
      font-weight: 600;
    }
    .mobile-nav-icon {
      font-size: 1.15rem;
      line-height: 1;
    }
    .mobile-fab-btn {
      width: 42px;
      height: 42px;
      border-radius: 50%;
      background: var(--accent);
      color: #090a0f;
      border: none;
      display: flex;
      align-items: center;
      justify-content: center;
      font-size: 1.35rem;
      font-weight: 700;
      box-shadow: 0 4px 12px rgba(0, 0, 0, 0.5);
      cursor: pointer;
      touch-action: manipulation;
      margin-top: -12px;
    }

    /* Modals & Native-Style Mobile Bottom Sheets */
    .modal-backdrop {
      position: fixed;
      inset: 0;
      background: rgba(0, 0, 0, 0.75);
      backdrop-filter: blur(4px);
      -webkit-backdrop-filter: blur(4px);
      display: none;
      align-items: center;
      justify-content: center;
      z-index: 1000;
      padding: 1rem;
    }
    .modal-card {
      background: var(--surface);
      border: 1px solid var(--border);
      border-radius: var(--radius-lg);
      width: 100%;
      max-width: 440px;
      overflow: hidden;
      max-height: 90vh;
      box-shadow: 0 16px 40px rgba(0, 0, 0, 0.8);
    }
    .modal-card.media { max-width: 860px; }
    .modal-card.doc { max-width: 760px; }
    .modal-card.lightbox {
      max-width: 95vw;
      max-height: 90vh;
      background: #000;
      border: 1px solid var(--border);
    }
    .modal-head {
      padding: 0.85rem 1.15rem;
      border-bottom: 1px solid var(--border);
      display: flex;
      justify-content: space-between;
      align-items: center;
    }
    .modal-title {
      font-weight: 600;
      font-size: 0.95rem;
      color: #fff;
      letter-spacing: -0.015em;
    }
    .modal-close {
      background: none;
      border: none;
      color: var(--text-dim);
      cursor: pointer;
      font-size: 1.15rem;
      line-height: 1;
      padding: 0.25rem;
      width: 32px;
      height: 32px;
      border-radius: var(--radius-sm);
      display: flex;
      align-items: center;
      justify-content: center;
      transition: all 0.12s ease;
    }
    .modal-close:hover {
      color: #fff;
      background: var(--surface-hover);
    }
    .modal-body {
      padding: 1.15rem;
      display: flex;
      flex-direction: column;
      gap: 0.85rem;
      max-height: 80vh;
      overflow-y: auto;
    }

    /* Form input in modal */
    .form-input {
      width: 100%;
      padding: 0.6rem 0.75rem;
      background: var(--bg);
      border: 1px solid var(--border);
      border-radius: var(--radius);
      color: #fff;
      font-size: 0.88rem;
      min-height: 40px;
    }
    .form-input:focus {
      outline: none;
      border-color: var(--border-focus);
    }

    /* Mobile Action Sheet Rows */
    .sheet-handle {
      display: none;
    }
    .sheet-action-item {
      display: flex;
      align-items: center;
      gap: 0.85rem;
      padding: 0.8rem 0.95rem;
      background: var(--surface-subtle);
      border: 1px solid var(--border);
      border-radius: var(--radius);
      color: #fff;
      font-size: 0.88rem;
      font-weight: 500;
      cursor: pointer;
      text-align: left;
      touch-action: manipulation;
      transition: all 0.12s ease;
    }
    .sheet-action-item:hover, .sheet-action-item:active {
      background: var(--surface-hover);
      border-color: var(--border-hover);
    }
    .sheet-action-icon {
      font-size: 1.25rem;
      line-height: 1;
      color: var(--accent);
    }

    .qr-frame {
      background: #ffffff;
      padding: 0.75rem;
      border-radius: var(--radius);
      width: fit-content;
      margin: 0 auto;
      display: flex;
      align-items: center;
      justify-content: center;
    }
    .qr-frame img {
      display: block;
      width: 200px;
      height: 200px;
    }

    .qr-link-row {
      background: var(--bg);
      border: 1px solid var(--border);
      border-radius: var(--radius);
      padding: 0.5rem 0.75rem;
      display: flex;
      align-items: center;
      justify-content: space-between;
      gap: 0.5rem;
    }
    .qr-link-text {
      font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
      font-size: 0.8rem;
      color: var(--accent);
      word-break: break-all;
    }

    /* Media Player Container */
    .media-stage {
      background: #000;
      border-radius: var(--radius);
      overflow: hidden;
      display: flex;
      align-items: center;
      justify-content: center;
      min-height: 200px;
      max-height: 65vh;
    }
    video {
      width: 100%;
      outline: none;
      max-height: 65vh;
    }
    .lightbox-img {
      max-width: 100%;
      max-height: 75vh;
      object-fit: contain;
    }

    /* Code Viewer */
    .code-preview-area {
      background: var(--bg);
      border: 1px solid var(--border);
      border-radius: var(--radius);
      padding: 0.85rem;
      font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
      font-size: 0.78rem;
      color: #e2e8f0;
      white-space: pre-wrap;
      word-break: break-all;
      max-height: 60vh;
      overflow-y: auto;
    }

    /* Toast */
    .toast-pill {
      position: fixed;
      bottom: calc(75px + var(--safe-bottom));
      right: 1.25rem;
      background: var(--surface);
      border: 1px solid var(--border-hover);
      color: #fff;
      padding: 0.5rem 0.85rem;
      border-radius: var(--radius);
      z-index: 2000;
      display: none;
      align-items: center;
      gap: 0.5rem;
      font-size: 0.8rem;
      font-weight: 500;
      box-shadow: 0 8px 24px rgba(0,0,0,0.6);
    }

    /* Devices List */
    .devices-wrap {
      display: flex;
      flex-direction: column;
      gap: 0.65rem;
    }
    .device-row {
      background: var(--surface);
      border: 1px solid var(--border);
      border-radius: var(--radius);
      padding: 0.8rem 0.95rem;
      display: flex;
      align-items: center;
      justify-content: space-between;
      gap: 0.75rem;
    }
    .device-row.host-device {
      border-color: rgba(56, 189, 248, 0.25);
      background: rgba(56, 189, 248, 0.02);
    }
    .device-title {
      font-weight: 500;
      color: #fff;
      display: flex;
      align-items: center;
      gap: 0.45rem;
      font-size: 0.86rem;
    }
    .device-meta {
      font-size: 0.74rem;
      color: var(--text-dim);
      font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
    }

    .empty-state {
      text-align: center;
      padding: 3rem 1.25rem;
      background: var(--surface);
      border: 1px solid var(--border);
      border-radius: var(--radius);
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: 0.5rem;
    }
    .empty-icon {
      font-size: 2rem;
      color: var(--text-dim);
    }
    .empty-title {
      font-weight: 600;
      color: #fff;
      font-size: 0.92rem;
    }
    .empty-desc {
      font-size: 0.8rem;
      color: var(--text-dim);
      max-width: 400px;
    }

    /* Clipboard */
    .clipboard-box {
      background: var(--surface);
      border: 1px solid var(--border);
      border-radius: var(--radius);
      padding: 1rem;
      display: flex;
      flex-direction: column;
      gap: 0.75rem;
    }
    .clipboard-textarea {
      width: 100%;
      height: 140px;
      background: var(--bg);
      border: 1px solid var(--border);
      border-radius: var(--radius);
      padding: 0.75rem;
      color: #fff;
      font-size: 0.86rem;
      resize: vertical;
      font-family: inherit;
    }
    .clipboard-textarea:focus {
      outline: none;
      border-color: var(--border-focus);
    }

    /* ======================================================== */
    /* MOBILE-SPECIFIC OPTIMIZATIONS (Screen width <= 640px)    */
    /* ======================================================== */
    @media (max-width: 640px) {
      header {
        padding: max(0.5rem, var(--safe-top)) 0.85rem 0.5rem 0.85rem;
      }
      .header-center { display: none; }
      .desktop-nav-actions { display: none; }
      .mobile-header-actions { display: flex; }
      .tabs-bar { display: none; } /* Replaced by bottom nav */
      .mobile-bottom-nav { display: flex; }

      .main-wrapper {
        padding: 0.65rem 0.75rem;
        gap: 0.65rem;
      }

      .folder-nav-bar {
        padding: 0.4rem 0.65rem;
      }

      /* Native-style bottom sheets for modals */
      .modal-backdrop {
        align-items: flex-end;
        padding: 0;
      }
      .modal-card {
        border-radius: 16px 16px 0 0;
        max-width: 100%;
        max-height: 85vh;
        border-bottom: none;
        padding-bottom: var(--safe-bottom);
        animation: sheetSlideUp 0.2s cubic-bezier(0.16, 1, 0.3, 1);
      }
      .modal-card.lightbox {
        border-radius: 0;
        max-height: 100vh;
        height: 100vh;
      }
      .sheet-handle {
        width: 36px;
        height: 4px;
        border-radius: 2px;
        background: rgba(255, 255, 255, 0.2);
        margin: 8px auto 4px auto;
        display: block;
      }

      /* Audio HUD docked above mobile bottom nav */
      .audio-player-hud {
        bottom: calc(56px + var(--safe-bottom));
        padding: 0.5rem 0.85rem;
        flex-direction: column;
        align-items: stretch;
        gap: 0.35rem;
      }
      .audio-info-col { max-width: 100%; }
      .audio-controls-col { max-width: 100%; }

      /* Mobile 2-column grid */
      .grid-container {
        grid-template-columns: repeat(2, 1fr);
        gap: 0.55rem;
      }
      .grid-preview-box { height: 105px; }

      /* Hide extra table columns on mobile */
      th:nth-child(3), td:nth-child(3),
      th:nth-child(4), td:nth-child(4) {
        display: none;
      }
      th:nth-child(2), td:nth-child(2) {
        max-width: 180px;
      }

      /* Touch targets minimum 44px */
      .btn-sm { min-height: 32px; padding: 0 0.55rem; }
      input[type="checkbox"] { width: 18px; height: 18px; }
    }
    @keyframes sheetSlideUp {
      from { transform: translateY(100%); }
      to { transform: translateY(0); }
    }
  </style>
</head>
<body>

  <!-- Minimalist Header -->
  <header>
    <a href="#" class="brand" onclick="navigateToFolder(''); return false;">
      <span class="brand-badge">⚡</span>
      <span class="brand-name">AirMesh</span>
      <span class="brand-tag">P2P</span>
    </a>

    <div class="header-center">
      <div class="nav-status-pill" onclick="openOfflineModal()" title="Node &amp; Network Status &bull; 100% Offline Mesh &bull; Free Storage">
        <span class="status-dot-pulse"></span>
        <span class="nav-status-mono" id="navIpDisplay">10.55.61.235:47823</span>
        <span class="nav-status-divider"></span>
        <span class="nav-status-green">0 KB Data &bull; 100% Offline</span>
        <span class="nav-status-divider"></span>
        <span class="nav-status-mono" id="diskFreeText">54.9 GB free</span>
      </div>
    </div>

    <!-- Desktop Action Buttons -->
    <div class="desktop-nav-actions">
      <button class="btn btn-secondary btn-sm" onclick="openPhoneModal()" title="Show WiFi QR Code for phones">
        <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="5" y="2" width="14" height="20" rx="2" ry="2"></rect><line x1="12" y1="18" x2="12.01" y2="18"></line></svg>
        <span>Connect Phone</span>
      </button>
      <button class="btn btn-secondary btn-sm" onclick="openNewFolderModal()" title="Create folder in current directory">
        <span>+ Folder</span>
      </button>
      <button class="btn btn-secondary btn-sm" onclick="triggerFolderInput()" title="Upload entire folder with subfolders">
        <span>Upload Folder</span>
      </button>
      <button class="btn btn-primary btn-sm" onclick="triggerFileInput()" title="Upload files (Ctrl+U)">
        <span>↑ Upload Files</span>
      </button>
    </div>

    <!-- Mobile Header Quick Actions -->
    <div class="mobile-header-actions">
      <button class="btn btn-secondary btn-icon" onclick="openOfflineModal()" title="100% Offline Proof" style="color: #34d399;">
        <span>🛡️</span>
      </button>
      <button class="btn btn-secondary btn-icon" onclick="openPhoneModal()" title="Phone QR Code">
        <span>📱</span>
      </button>
      <button class="btn btn-primary btn-sm" onclick="openMobileActionSheet()" style="padding: 0 0.65rem;">
        <span>＋ Add</span>
      </button>
    </div>

    <!-- Hidden file/folder pickers -->
    <input type="file" id="filePicker" style="display: none;" onchange="handleFilesPicked(event)" multiple>
    <input type="file" id="folderPicker" style="display: none;" onchange="handleFolderPicked(event)" webkitdirectory directory multiple>
  </header>

  <!-- Offline Guarantee & Verification Strip -->
  <div class="offline-status-strip" onclick="openOfflineModal()" title="Click to verify offline & zero mobile data guarantee">
    <div class="offline-status-left">
      <span class="offline-dot-pulse"></span>
      <span><strong>100% Offline Local Wi-Fi Mesh</strong> &bull; Zero Internet Used &bull; 0 KB Mobile Data</span>
    </div>
    <div class="offline-status-tag">
      <span>Proof &amp; Security</span>
      <span>↗</span>
    </div>
  </div>

  <!-- Cellular Radio Warning Bar (Dynamically shown if phone has cellular on) -->
  <div class="cellular-warning-bar" id="cellularWarningBar">
    <div style="display: flex; align-items: center; gap: 0.5rem; flex: 1;">
      <span>⚠️</span>
      <span><strong>Mobile Data is ON:</strong> AirMesh transfers strictly stay over local Wi-Fi, but you can turn OFF Mobile Data (or turn ON Airplane Mode with Wi-Fi ON) to guarantee zero mobile data.</span>
    </div>
    <button class="btn btn-secondary btn-sm" onclick="openOfflineModal()" style="padding: 0 0.55rem; font-size: 0.72rem; height: 26px;">Quick Guide</button>
  </div>

  <main class="main-wrapper">

    <!-- Desktop View Switcher Tabs -->
    <div class="tabs-bar">
      <div class="tab-nav">
        <button class="tab-item active" id="tabBtnFiles" onclick="switchView('files')">
          <span>Files &amp; Folders</span>
          <span class="tab-count" id="filesTabCount">0</span>
        </button>
        <button class="tab-item" id="tabBtnDevices" onclick="switchView('devices')">
          <span>Devices</span>
          <span class="tab-count" id="devicesTabCount">1</span>
        </button>
        <button class="tab-item" id="tabBtnClipboard" onclick="switchView('clipboard')">
          <span>Clipboard</span>
        </button>
      </div>

      <div class="view-toggle-group" id="viewModeGroup">
        <button class="view-toggle-btn active" id="btnViewTable" onclick="setViewMode('table')" title="List View">
          <span>☷</span> List
        </button>
        <button class="view-toggle-btn" id="btnViewGrid" onclick="setViewMode('grid')" title="Grid View">
          <span>⊞</span> Grid
        </button>
      </div>
    </div>

    <!-- VIEW 1: FILES & FOLDERS -->
    <div id="viewFiles" style="display: flex; flex-direction: column; gap: 0.75rem;">

      <!-- Systematic Breadcrumbs & Current Folder Toolbar -->
      <div class="folder-nav-bar">
        <div class="breadcrumbs-box" id="breadcrumbsWrap">
          <span class="crumb-current">📁 Shared</span>
        </div>

        <div class="folder-actions-row">
          <button class="btn btn-secondary btn-sm" id="btnParentFolder" style="display: none;" onclick="navigateUpFolder()">
            <span>↰ Parent</span>
          </button>
          <button class="btn btn-secondary btn-sm" onclick="downloadCurrentFolderZip()" title="Download current folder as a ZIP archive">
            <span>📦 ZIP Folder</span>
          </button>
          <!-- Mobile view toggle inside bar -->
          <div class="view-toggle-group" id="mobileViewToggle" style="display: none;">
            <button class="view-toggle-btn active" id="btnViewTableMob" onclick="setViewMode('table')">☷</button>
            <button class="view-toggle-btn" id="btnViewGridMob" onclick="setViewMode('grid')">⊞</button>
          </div>
        </div>
      </div>

      <!-- Search & Filters -->
      <div class="search-filter-row">
        <div class="search-box">
          <svg class="search-icon" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="11" cy="11" r="8"></circle><line x1="21" y1="21" x2="16.65" y2="16.65"></line></svg>
          <input type="text" id="searchInput" class="search-input" placeholder="Search files &amp; folders... (Ctrl+K)" oninput="onSearchInput(this.value)">
          <button class="search-clear-btn" id="searchClearBtn" onclick="clearSearch()" title="Clear search">✕</button>
          <kbd class="search-kbd" id="searchKbdHint">Ctrl K</kbd>
        </div>

        <div class="category-group">
          <button class="cat-btn active" onclick="setCategory('all', this)">All</button>
          <button class="cat-btn" onclick="setCategory('video', this)">Videos</button>
          <button class="cat-btn" onclick="setCategory('photo', this)">Photos</button>
          <button class="cat-btn" onclick="setCategory('audio', this)">Audio</button>
          <button class="cat-btn" onclick="setCategory('document', this)">Docs</button>
          <button class="cat-btn" onclick="setCategory('archive', this)">Archives</button>
        </div>
      </div>

      <!-- Drag & Drop Zone for Files AND Folders -->
      <div class="drop-strip" id="dropStrip" onclick="triggerFileInput()">
        <svg class="drop-strip-icon" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path><polyline points="17 8 12 3 7 8"></polyline><line x1="12" y1="3" x2="12" y2="15"></line></svg>
        <span>Drop files or folders here &bull; or <strong style="color: var(--accent); font-weight: 500;">browse to upload</strong></span>
      </div>

      <!-- Table / List View -->
      <div class="table-container" id="tableContainer">
        <table>
          <thead>
            <tr>
              <th style="width: 36px; text-align: center;">
                <input type="checkbox" id="selectAllCheckbox" onchange="toggleSelectAll(this.checked)">
              </th>
              <th onclick="toggleSort('name')" id="thColName">Name <span id="sortArrowName" style="opacity: 0.5; font-size: 0.7rem;">↕</span></th>
              <th onclick="toggleSort('size')" id="thColSize">Size <span id="sortArrowSize" style="opacity: 0.5; font-size: 0.7rem;">↕</span></th>
              <th>Details</th>
              <th style="text-align: right;">Actions</th>
            </tr>
          </thead>
          <tbody id="filesTableBody">
            <tr><td colspan="5" style="text-align: center; color: var(--text-dim); padding: 2rem;">Loading files &amp; folders...</td></tr>
          </tbody>
        </table>
      </div>

      <!-- Grid View -->
      <div class="grid-container" id="gridContainer" style="display: none;">
        <!-- Rendered dynamically -->
      </div>

    </div>

    <!-- VIEW 2: CONNECTED DEVICES -->
    <div id="viewDevices" style="display: none; flex-direction: column; gap: 0.75rem;">
      <div style="display: flex; justify-content: space-between; align-items: center;">
        <span style="font-weight: 500; font-size: 0.85rem; color: #fff;">Devices on WiFi Mesh:</span>
        <button class="btn btn-secondary btn-sm" onclick="loadDevices()">Refresh</button>
      </div>

      <div class="devices-wrap" id="devicesListWrap">
        <!-- Rendered dynamically -->
      </div>
    </div>

    <!-- VIEW 3: CLIPBOARD -->
    <div id="viewClipboard" style="display: none; flex-direction: column; gap: 0.75rem;">
      <div class="clipboard-box">
        <textarea id="clipboardText" class="clipboard-textarea" placeholder="Type or paste text/URLs to sync across your phone and PC..."></textarea>
        <div style="display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 0.5rem;">
          <span style="font-size: 0.75rem; color: var(--text-dim);" id="clipboardStatus">Last updated: Just now</span>
          <div style="display: flex; gap: 0.4rem;">
            <button class="btn btn-secondary btn-sm" onclick="copyClipboard()">Copy</button>
            <button class="btn btn-primary btn-sm" onclick="sendClipboard()">Sync Across Mesh</button>
          </div>
        </div>
      </div>
    </div>

  </main>

  <!-- Mobile Sticky Bottom Navigation Bar (Thumb Zone) -->
  <nav class="mobile-bottom-nav">
    <button class="mobile-nav-btn active" id="mobNavFiles" onclick="switchView('files')">
      <span class="mobile-nav-icon">📁</span>
      <span>Files</span>
    </button>
    <button class="mobile-nav-btn" id="mobNavDevices" onclick="switchView('devices')">
      <span class="mobile-nav-icon">📱</span>
      <span>Devices</span>
    </button>
    <button class="mobile-fab-btn" onclick="openMobileActionSheet()" title="Add / Upload">
      <span>＋</span>
    </button>
    <button class="mobile-nav-btn" id="mobNavClipboard" onclick="switchView('clipboard')">
      <span class="mobile-nav-icon">📋</span>
      <span>Clipboard</span>
    </button>
    <button class="mobile-nav-btn" onclick="openPhoneModal()">
      <span class="mobile-nav-icon">📷</span>
      <span>QR Link</span>
    </button>
  </nav>

  <!-- Floating Batch Selection Action Bar -->
  <div class="batch-bar" id="batchBar">
    <span style="font-size: 0.82rem; font-weight: 500; color: #fff;" id="batchSelectedText">0 selected</span>
    <button class="btn btn-primary btn-sm" onclick="downloadSelectedZip()">
      <span>📦</span> ZIP
    </button>
    <button class="btn btn-danger btn-sm" onclick="deleteSelectedItems()">
      <span>✕</span> Delete
    </button>
    <button class="btn btn-secondary btn-sm" onclick="deselectAll()">Cancel</button>
  </div>

  <!-- Live Resumable Upload HUD -->
  <div class="upload-hud" id="uploadHud">
    <div style="display: flex; justify-content: space-between; align-items: center; gap: 0.5rem;">
      <div style="display: flex; align-items: center; gap: 0.4rem; overflow: hidden; flex: 1;">
        <span id="hudStatusDot" style="width: 7px; height: 7px; border-radius: 50%; background: var(--accent); flex-shrink: 0; display: inline-block;"></span>
        <span style="font-weight: 500; font-size: 0.82rem; color: #fff; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;" id="hudFileName">Uploading...</span>
      </div>
      <div style="display: flex; align-items: center; gap: 0.35rem; flex-shrink: 0;">
        <button id="hudPauseResumeBtn" onclick="toggleUploadPause()" style="background: var(--surface-subtle); border: 1px solid var(--border); color: var(--text); cursor: pointer; font-size: 0.72rem; padding: 0.2rem 0.55rem; border-radius: var(--radius-sm); font-weight: 500; display: inline-flex; align-items: center; gap: 0.25rem;">⏸ Pause</button>
        <button onclick="cancelActiveUpload()" style="background: rgba(244, 63, 94, 0.12); border: 1px solid rgba(244, 63, 94, 0.3); color: var(--danger); cursor: pointer; font-size: 0.72rem; padding: 0.2rem 0.55rem; border-radius: var(--radius-sm); font-weight: 500;">✕ Cancel</button>
      </div>
    </div>
    <div class="hud-bar">
      <div class="hud-fill" id="hudBarFill"></div>
    </div>
    <div style="display: flex; justify-content: space-between; font-size: 0.72rem; color: var(--text-dim);">
      <span id="hudSpeed">0 MB/s</span>
      <span id="hudPercent">0%</span>
    </div>
  </div>

  <!-- Persistent Bottom Audio Player HUD -->
  <div class="audio-player-hud" id="audioPlayerHud">
    <div class="audio-info-col">
      <div class="audio-icon-box">🎵</div>
      <div class="audio-text-col">
        <div class="audio-hud-title" id="audioHudTitle">Audio Track</div>
        <div class="audio-hud-time" id="audioHudTime">0:00 / 0:00</div>
      </div>
    </div>
    <div class="audio-controls-col">
      <button class="audio-btn-circle" id="audioHudPlayBtn" onclick="toggleAudioHudPlay()">▶</button>
      <input type="range" class="audio-seek-slider" id="audioHudSeek" value="0" min="0" max="100" oninput="seekAudioHud(this.value)">
    </div>
    <div style="display: flex; align-items: center; gap: 0.4rem;">
      <a class="btn btn-secondary btn-sm" id="audioHudDownloadBtn" href="#" download>⬇</a>
      <button class="btn btn-secondary btn-sm btn-icon" onclick="closeAudioHud()" title="Close Player">✕</button>
    </div>
    <audio id="globalAudioPlayer" style="display: none;"></audio>
  </div>

  <!-- MODAL: Mobile Slide-Up Action Sheet -->
  <div class="modal-backdrop" id="mobileActionSheet" onclick="closeModal('mobileActionSheet')">
    <div class="modal-card" onclick="event.stopPropagation()">
      <div class="sheet-handle"></div>
      <div class="modal-head">
        <div class="modal-title">AirMesh Actions</div>
        <button class="modal-close" onclick="closeModal('mobileActionSheet')">✕</button>
      </div>
      <div class="modal-body" style="gap: 0.65rem;">
        <div class="sheet-action-item" onclick="closeModal('mobileActionSheet'); triggerFileInput();">
          <span class="sheet-action-icon">📤</span>
          <div>
            <div style="color: #fff; font-weight: 500;">Upload Files</div>
            <div style="font-size: 0.75rem; color: var(--text-dim);">Select photos, videos, or documents</div>
          </div>
        </div>

        <div class="sheet-action-item" onclick="closeModal('mobileActionSheet'); triggerFolderInput();">
          <span class="sheet-action-icon">📥</span>
          <div>
            <div style="color: #fff; font-weight: 500;">Upload Entire Folder</div>
            <div style="font-size: 0.75rem; color: var(--text-dim);">Upload a complete folder with nested files</div>
          </div>
        </div>

        <div class="sheet-action-item" onclick="closeModal('mobileActionSheet'); openNewFolderModal();">
          <span class="sheet-action-icon">📁</span>
          <div>
            <div style="color: #fff; font-weight: 500;">Create New Folder</div>
            <div style="font-size: 0.75rem; color: var(--text-dim);">Create subfolder in current directory</div>
          </div>
        </div>

        <div class="sheet-action-item" onclick="closeModal('mobileActionSheet'); downloadCurrentFolderZip();">
          <span class="sheet-action-icon">📦</span>
          <div>
            <div style="color: #fff; font-weight: 500;">Download Folder as ZIP</div>
            <div style="font-size: 0.75rem; color: var(--text-dim);">Archive current directory as single ZIP</div>
          </div>
        </div>

        <div class="sheet-action-item" onclick="closeModal('mobileActionSheet'); openOfflineModal();">
          <span class="sheet-action-icon">🛡️</span>
          <div>
            <div style="color: #34d399; font-weight: 500;">100% Offline &amp; Zero Data Guarantee</div>
            <div style="font-size: 0.75rem; color: var(--text-dim);">Verify proof: 0 KB internet / cellular used</div>
          </div>
        </div>

        <div class="sheet-action-item" onclick="closeModal('mobileActionSheet'); openPhoneModal();">
          <span class="sheet-action-icon">📱</span>
          <div>
            <div style="color: #fff; font-weight: 500;">Connect Other Device (QR)</div>
            <div style="font-size: 0.75rem; color: var(--text-dim);">Show WiFi QR code for other phones</div>
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- MODAL: 100% Offline & Zero Data Guarantee -->
  <div class="modal-backdrop" id="offlineModal" onclick="closeModal('offlineModal')">
    <div class="modal-card" style="max-width: 540px;" onclick="event.stopPropagation()">
      <div class="sheet-handle"></div>
      <div class="modal-head">
        <div class="modal-title" style="display: flex; align-items: center; gap: 0.45rem;">
          <span>🛡️</span>
          <span>100% Offline &amp; Zero Mobile Data</span>
        </div>
        <button class="modal-close" onclick="closeModal('offlineModal')">✕</button>
      </div>
      <div class="modal-body" style="gap: 0.85rem;">
        
        <!-- Live Verified Badge -->
        <div style="background: rgba(16, 185, 129, 0.08); border: 1px solid rgba(16, 185, 129, 0.25); border-radius: var(--radius); padding: 0.75rem 0.95rem; display: flex; align-items: center; justify-content: space-between;">
          <div style="display: flex; align-items: center; gap: 0.6rem;">
            <span class="offline-dot-pulse"></span>
            <div>
              <div style="color: #34d399; font-weight: 600; font-size: 0.85rem;">Hardware Local Wi-Fi Mesh</div>
              <div style="font-size: 0.74rem; color: var(--text-dim);" id="offlineModalSubnet">Host: 10.55.61.235 &bull; RFC 1918 Private LAN</div>
            </div>
          </div>
          <span style="font-size: 0.72rem; color: #34d399; font-weight: 600; background: rgba(16, 185, 129, 0.15); padding: 0.2rem 0.5rem; border-radius: 4px;">0 KB DATA</span>
        </div>

        <!-- The 5-Second Test: Physical Proof -->
        <div class="offline-card">
          <div class="offline-card-title">
            <span>🧪</span> 5-Second Physical Proof Test (Try It!)
          </div>
          <div class="offline-card-desc">
            You can physically prove that AirMesh uses zero mobile data with this quick test:
          </div>
          <div class="offline-steps">
            <div class="offline-step-item">
              <span class="offline-step-num">1</span>
              <span>Turn <strong>OFF Mobile Data</strong> (Cellular) in your phone's quick settings drawer, OR switch <strong>Airplane Mode ON</strong> and turn Wi-Fi back ON.</span>
            </div>
            <div class="offline-step-item">
              <span class="offline-step-num">2</span>
              <span>Open AirMesh in your mobile browser at <strong id="proofModalUrl" style="color: var(--accent);">http://10.55.61.235:47823</strong>.</span>
            </div>
            <div class="offline-step-item">
              <span class="offline-step-num">3</span>
              <span>Upload, download, or stream any large file or folder.</span>
            </div>
          </div>
          <div style="margin-top: 0.35rem; font-size: 0.75rem; color: #34d399; background: rgba(16, 185, 129, 0.08); padding: 0.45rem 0.6rem; border-radius: 4px;">
            ✓ <strong>Result:</strong> It transfers at maximum Wi-Fi chip speed (100–300+ MB/s). Since cellular is completely disabled, <strong>zero bytes of mobile data can be used!</strong>
          </div>
        </div>

        <!-- Why 4G/5G shows in phone top bar -->
        <div class="offline-card">
          <div class="offline-card-title">
            <span>📱</span> Why does my phone still show "5G" or "LTE"?
          </div>
          <div class="offline-card-desc">
            When your phone connects to a local Wi-Fi router or hotspot that doesn't have an active broadband internet subscription, Android and iOS keep the 4G/5G cellular antenna on standby for phone calls and carrier SMS.
          </div>
          <div class="offline-card-desc" style="margin-top: 0.25rem;">
            However, AirMesh's address (<code style="color: var(--accent);" id="offlineCodeIp">10.55.61.235</code>) is an <strong>RFC 1918 Private LAN IP</strong>. Cellular network towers <em>cannot</em> route private LAN addresses — all packets stay 100% confined inside your local Wi-Fi radio waves.
          </div>
        </div>

        <!-- Essential Phone Settings -->
        <div class="offline-card">
          <div class="offline-card-title">
            <span>⚙️</span> Recommended Phone Settings
          </div>
          <div class="offline-steps">
            <div class="offline-step-item">
              <span style="font-weight: 600; color: #fff; min-width: 60px;">Android:</span>
              <span>If Android shows "Wi-Fi has no internet", tap <strong>Always connect / Stay connected</strong>. In Wi-Fi settings, turn off <em>"Switch to mobile data automatically"</em>.</span>
            </div>
            <div class="offline-step-item">
              <span style="font-weight: 600; color: #fff; min-width: 60px;">iPhone:</span>
              <span>Go to <strong>Settings &gt; Cellular</strong>, scroll to the bottom, and turn OFF <strong>Wi-Fi Assist</strong> (prevents iOS from switching to mobile data when Wi-Fi has no internet).</span>
            </div>
            <div class="offline-step-item">
              <span style="font-weight: 600; color: #fff; min-width: 60px;">Tip:</span>
              <span>You can leave Mobile Data switched OFF whenever transferring files with AirMesh.</span>
            </div>
          </div>
        </div>

        <!-- Technical Sandbox -->
        <div class="offline-card">
          <div class="offline-card-title">
            <span>🔒</span> Technical Security Sandbox
          </div>
          <div style="font-size: 0.76rem; color: var(--text-dim); display: flex; flex-direction: column; gap: 0.25rem;">
            <div>&bull; <strong>Content-Security-Policy:</strong> Browser strictly prohibits requests outside LAN origin.</div>
            <div>&bull; <strong>Cloud Dependencies:</strong> 0 external CDNs, 0 telemetry trackers, 0 STUN/TURN servers.</div>
            <div>&bull; <strong>Protocol:</strong> Direct HTTP/1.1 over 802.11 Wi-Fi radio frequencies.</div>
          </div>
        </div>

        <button class="btn btn-primary" onclick="closeModal('offlineModal')" style="width: 100%; margin-top: 0.25rem;">Got it, close</button>
      </div>
    </div>
  </div>

  <!-- MODAL: New Folder -->
  <div class="modal-backdrop" id="newFolderModal" onclick="closeModal('newFolderModal')">
    <div class="modal-card" onclick="event.stopPropagation()">
      <div class="sheet-handle"></div>
      <div class="modal-head">
        <div class="modal-title">Create Folder</div>
        <button class="modal-close" onclick="closeModal('newFolderModal')">✕</button>
      </div>
      <div class="modal-body">
        <label style="font-size: 0.78rem; color: var(--text-dim);">Folder Name</label>
        <input type="text" id="newFolderNameInput" class="form-input" placeholder="e.g. Vacation, Documents, Music" onkeydown="if(event.key==='Enter') submitCreateFolder()">
        <div style="display: flex; justify-content: flex-end; gap: 0.4rem; margin-top: 0.5rem;">
          <button class="btn btn-secondary btn-sm" onclick="closeModal('newFolderModal')">Cancel</button>
          <button class="btn btn-primary btn-sm" onclick="submitCreateFolder()">Create</button>
        </div>
      </div>
    </div>
  </div>

  <!-- MODAL: Connect Phone via QR Code -->
  <div class="modal-backdrop" id="phoneModal" onclick="closeModal('phoneModal')">
    <div class="modal-card" onclick="event.stopPropagation()">
      <div class="sheet-handle"></div>
      <div class="modal-head">
        <div class="modal-title">WiFi Mesh QR Link</div>
        <button class="modal-close" onclick="closeModal('phoneModal')">✕</button>
      </div>
      <div class="modal-body" style="text-align: center;">
        <p style="font-size: 0.82rem; color: var(--text-muted);">
          Scan with your iPhone or Android camera:
        </p>

        <div class="qr-frame">
          <img id="qrImg" src="/api/qr.svg" alt="Scan QR Code">
        </div>

        <div class="qr-link-row">
          <span class="qr-link-text" id="phoneModalUrl">http://...</span>
          <button class="btn btn-secondary btn-sm" onclick="copyPhoneUrl()">Copy</button>
        </div>

        <div style="text-align: left; font-size: 0.78rem; color: var(--text-dim); line-height: 1.45; padding: 0.25rem;">
          &bull; Connected to same WiFi network.<br>
          &bull; Scan QR code or enter link in Safari / Chrome.<br>
          &bull; Browse folders, stream videos, or upload photos to PC!
        </div>
      </div>
    </div>
  </div>

  <!-- MODAL: Video Player -->
  <div class="modal-backdrop" id="mediaModal" onclick="closeMediaModal()">
    <div class="modal-card media" onclick="event.stopPropagation()">
      <div class="modal-head">
        <div class="modal-title" id="mediaTitle">Video Stream</div>
        <button class="modal-close" onclick="closeMediaModal()">✕</button>
      </div>
      <div class="modal-body">
        <div class="media-stage" id="mediaStage"></div>
        <div style="display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 0.5rem;">
          <div style="display: flex; align-items: center; gap: 0.4rem; flex-wrap: wrap;">
            <span style="font-size: 0.75rem; color: var(--text-dim);" id="mediaInfo">Direct Stream</span>
            <span id="audioModeBadge" class="file-badge" style="color: var(--accent); display: none;">Universal AAC</span>
            <span id="subBadge" class="file-badge" style="color: #c084fc; display: none;">CC</span>
            <span id="resBadge" class="file-badge" style="color: #facc15; display: none;">4K</span>
          </div>
          <div style="display: flex; gap: 0.4rem; align-items: center;">
            <button class="btn btn-secondary btn-sm" id="btnAudioToggle" style="display: none;" onclick="toggleAudioMode()">
              <span>🔊</span> Audio Fix
            </button>
            <a class="btn btn-secondary btn-sm" id="mediaOpenBtn" href="#" target="_blank">Tab</a>
            <a class="btn btn-primary btn-sm" id="mediaDownloadBtn" href="#" download>Download</a>
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- MODAL: Document Viewer -->
  <div class="modal-backdrop" id="docModal" onclick="closeModal('docModal')">
    <div class="modal-card doc" onclick="event.stopPropagation()">
      <div class="modal-head">
        <div class="modal-title" id="docTitle">Document Viewer</div>
        <button class="modal-close" onclick="closeModal('docModal')">✕</button>
      </div>
      <div class="modal-body" id="docModalBody">
        <div class="code-preview-area" id="codePreviewArea">Loading...</div>
        <div style="display: flex; justify-content: space-between; align-items: center;">
          <span style="font-size: 0.75rem; color: var(--text-dim);" id="docMetaInfo">Text File</span>
          <div style="display: flex; gap: 0.4rem;">
            <button class="btn btn-secondary btn-sm" onclick="copyDocContent()">Copy</button>
            <a class="btn btn-primary btn-sm" id="docDownloadBtn" href="#" download>Download</a>
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- MODAL: Photo Lightbox -->
  <div class="modal-backdrop" id="lightboxModal" onclick="closeModal('lightboxModal')">
    <div class="modal-card lightbox" onclick="event.stopPropagation()">
      <div class="modal-head">
        <div class="modal-title" id="lightboxTitle">Photo</div>
        <button class="modal-close" onclick="closeModal('lightboxModal')">✕</button>
      </div>
      <div class="modal-body" id="lightboxBody" style="padding: 0; align-items: center;">
        <img id="lightboxImg" class="lightbox-img" src="" alt="Photo preview">
      </div>
      <div style="display: flex; justify-content: space-between; align-items: center; padding: 0.65rem 1rem; border-top: 1px solid var(--border);">
        <button class="btn btn-secondary btn-sm" onclick="navPhoto(-1)">← Prev</button>
        <span style="font-size: 0.78rem; color: var(--text-dim);" id="lightboxCounter">1 of 4</span>
        <button class="btn btn-secondary btn-sm" onclick="navPhoto(1)">Next →</button>
      </div>
    </div>
  </div>

  <!-- Toast -->
  <div class="toast-pill" id="toastPill">
    <span id="toastMsg">Action completed</span>
  </div>

  <script>
    // State
    let currentPath = '';
    let currentBreadcrumbs = [{ name: 'Shared', path: '' }];
    let currentFolders = [];
    let currentFiles = [];
    let currentCategory = 'all';
    let searchQuery = '';
    let sortColumn = 'name';
    let sortAsc = true;
    let viewMode = localStorage.getItem('airmesh_view_mode') || 'table';
    let selectedItems = new Set();
    let photoList = [];
    let currentPhotoIdx = 0;
    let activeUploadXhr = null;
    let uploadQueue = [];
    let isUploading = false;
    let isUploadPaused = false;
    let currentUploadItem = null;
    let currentChunkXhr = null;
    let wakeLockSentinel = null;
    let autoRetryTimer = null;
    let retryAttempt = 0;
    let currentDocText = '';
    let localIp = '10.55.61.235';
    let localPort = 47823;
    let localUrl = `http://${localIp}:${localPort}`;

    if ('serviceWorker' in navigator) {
      navigator.serviceWorker.register('/sw.js').catch(() => {});
    }

    function getClientId() {
      let id = localStorage.getItem('airmesh_client_id');
      if (!id) {
        id = 'client_' + Math.random().toString(36).substring(2, 10);
        localStorage.setItem('airmesh_client_id', id);
      }
      return id;
    }

    function showToast(msg) {
      const t = document.getElementById('toastPill');
      document.getElementById('toastMsg').textContent = msg;
      t.style.display = 'flex';
      setTimeout(() => { t.style.display = 'none'; }, 2600);
    }

    function openModal(id) { document.getElementById(id).style.display = 'flex'; }
    function closeModal(id) { document.getElementById(id).style.display = 'none'; }
    function openMobileActionSheet() { openModal('mobileActionSheet'); }

    function switchView(view) {
      ['files', 'devices', 'clipboard'].forEach(v => {
        const btn = document.getElementById(`tabBtn${capitalize(v)}`);
        const mobBtn = document.getElementById(`mobNav${capitalize(v)}`);
        const content = document.getElementById(`view${capitalize(v)}`);
        if (v === view) {
          if (btn) btn.classList.add('active');
          if (mobBtn) mobBtn.classList.add('active');
          content.style.display = 'flex';
        } else {
          if (btn) btn.classList.remove('active');
          if (mobBtn) mobBtn.classList.remove('active');
          content.style.display = 'none';
        }
      });
      document.getElementById('viewModeGroup').style.display = (view === 'files') ? 'flex' : 'none';
      if (view === 'files') loadCurrentFolder();
      if (view === 'devices') loadDevices();
      if (view === 'clipboard') loadClipboard();
    }
    function capitalize(s) { return s.charAt(0).toUpperCase() + s.slice(1); }

    function setViewMode(mode) {
      viewMode = mode;
      localStorage.setItem('airmesh_view_mode', mode);
      document.getElementById('btnViewTable').classList.toggle('active', mode === 'table');
      document.getElementById('btnViewGrid').classList.toggle('active', mode === 'grid');
      const mobT = document.getElementById('btnViewTableMob');
      const mobG = document.getElementById('btnViewGridMob');
      if (mobT) mobT.classList.toggle('active', mode === 'table');
      if (mobG) mobG.classList.toggle('active', mode === 'grid');

      document.getElementById('tableContainer').style.display = (mode === 'table') ? 'block' : 'none';
      document.getElementById('gridContainer').style.display = (mode === 'grid') ? 'grid' : 'none';
      renderContent();
    }

    // Network & Disk Info
    async function initNetwork() {
      try {
        const res = await fetch('/api/network-info');
        if (res.ok) {
          const d = await res.json();
          localIp = d.local_ip;
          localPort = d.port;
          localUrl = d.url;
          document.getElementById('navIpDisplay').textContent = `${localIp}:${localPort}`;
          document.getElementById('phoneModalUrl').textContent = localUrl;
          document.getElementById('qrImg').src = `/api/qr.svg?url=${encodeURIComponent(localUrl)}&t=${Date.now()}`;
          if (d.disk) {
            document.getElementById('diskFreeText').textContent = `${d.disk.free_gb} GB free`;
          }
          const proofEl = document.getElementById('proofModalUrl');
          const subnetEl = document.getElementById('offlineModalSubnet');
          const codeIpEl = document.getElementById('offlineCodeIp');
          if (proofEl) proofEl.textContent = localUrl;
          if (subnetEl) subnetEl.textContent = `Host: ${localIp}:${localPort} • RFC 1918 Private LAN`;
          if (codeIpEl) codeIpEl.textContent = localIp;
        }
      } catch (e) {}
    }

    function openOfflineModal() {
      const proofEl = document.getElementById('proofModalUrl');
      const subnetEl = document.getElementById('offlineModalSubnet');
      const codeIpEl = document.getElementById('offlineCodeIp');
      if (proofEl) proofEl.textContent = localUrl;
      if (subnetEl) subnetEl.textContent = `Host: ${localIp}:${localPort} • RFC 1918 Private LAN`;
      if (codeIpEl) codeIpEl.textContent = localIp;
      openModal('offlineModal');
    }

    function checkCellularRadio() {
      try {
        const conn = navigator.connection || navigator.mozConnection || navigator.webkitConnection;
        const banner = document.getElementById('cellularWarningBar');
        if (conn && banner) {
          if (conn.type === 'cellular') {
            banner.style.display = 'flex';
          } else {
            banner.style.display = 'none';
          }
          conn.addEventListener('change', checkCellularRadio);
        }
      } catch(e) {}
    }

    function openPhoneModal() {
      document.getElementById('qrImg').src = `/api/qr.svg?url=${encodeURIComponent(localUrl)}&t=${Date.now()}`;
      openModal('phoneModal');
    }

    function copyPhoneUrl() {
      navigator.clipboard.writeText(localUrl);
      showToast('Link copied: ' + localUrl);
    }

    // Server-Sent Events (SSE)
    function initSse() {
      try {
        const es = new EventSource('/api/events');
        es.addEventListener('file_change', () => {
          loadCurrentFolder();
          initNetwork();
        });
        es.addEventListener('file_receiving_start', (e) => {
          loadCurrentFolder();
          showToast('⚡ Live file incoming! You can stream it now.');
        });
        es.addEventListener('file_receiving_progress', (e) => {
          try {
            const d = JSON.parse(e.data);
            handleReceivingProgress(d);
          } catch(err) {}
        });
        es.addEventListener('file_receiving_complete', (e) => {
          loadCurrentFolder();
        });
        es.addEventListener('file_receiving_abort', (e) => {
          loadCurrentFolder();
        });
        es.addEventListener('clipboard_change', (e) => {
          try {
            const d = JSON.parse(e.data);
            const tb = document.getElementById('clipboardText');
            if (document.activeElement !== tb) {
              tb.value = d.text || '';
              document.getElementById('clipboardStatus').textContent = `Last updated: ${new Date(d.updated_at).toLocaleTimeString()} (${d.author})`;
            }
          } catch(err) {}
        });
        es.addEventListener('peers_change', () => {
          loadDevices();
        });
      } catch(e) {}
    }

    function handleReceivingProgress(d) {
      const f = currentFiles.find(item => item.path === d.path);
      if (f) {
        f.written_bytes = d.written_bytes;
        f.receiving_percent = d.percent;
        f.is_receiving = true;
        const pctEl = document.getElementById(`receiving_pct_${f.id}`);
        const bytesEl = document.getElementById(`receiving_bytes_${f.id}`);
        const barEl = document.getElementById(`receiving_bar_${f.id}`);
        const gridPctEl = document.getElementById(`grid_receiving_pct_${f.id}`);
        if (pctEl) pctEl.textContent = `⚡ ${d.percent}%`;
        if (bytesEl) bytesEl.textContent = `${formatBytes(d.written_bytes)} / ${formatBytes(d.total_bytes)}`;
        if (barEl) barEl.style.width = `${d.percent}%`;
        if (gridPctEl) gridPctEl.textContent = `⚡ Live (${d.percent}%)`;
      }
      const videoLiveBadge = document.getElementById('videoLiveReceivingBadge');
      if (videoLiveBadge && currentPlayingFile && currentPlayingFile.path === d.path) {
        videoLiveBadge.textContent = `⚡ Live Stream (${d.percent}%)`;
      }
      const audioLiveBadge = document.getElementById('audioLiveReceivingBadge');
      if (audioLiveBadge && currentPlayingAudio && currentPlayingAudio.path === d.path) {
        audioLiveBadge.textContent = `(⚡ Streaming Live ${d.percent}%)`;
      }
    }

    // Load Hierarchical Folder Content
    async function loadCurrentFolder() {
      try {
        const res = await fetch(`/api/files?path=${encodeURIComponent(currentPath)}`);
        if (res.ok) {
          const data = await res.json();
          currentPath = data.current_path || '';
          currentBreadcrumbs = data.breadcrumbs || [{ name: 'Shared', path: '' }];
          currentFolders = data.folders || [];
          currentFiles = data.files || [];
          photoList = currentFiles.filter(f => f.category === 'photo');

          document.getElementById('filesTabCount').textContent = (currentFolders.length + currentFiles.length);
          renderBreadcrumbs();
          renderContent();
        }
      } catch(e) {}
    }

    function navigateToFolder(relPath) {
      currentPath = relPath || '';
      selectedItems.clear();
      updateBatchBar();
      loadCurrentFolder();
    }

    function navigateUpFolder() {
      if (!currentPath) return;
      const parts = currentPath.split('/');
      parts.pop();
      navigateToFolder(parts.join('/'));
    }

    function renderBreadcrumbs() {
      const wrap = document.getElementById('breadcrumbsWrap');
      const parentBtn = document.getElementById('btnParentFolder');

      if (currentBreadcrumbs.length <= 1) {
        parentBtn.style.display = 'none';
        wrap.innerHTML = '<span class="crumb-current" style="display:flex;align-items:center;gap:0.35rem;"><span style="color:var(--accent);">📁</span> Shared</span>';
        return;
      }

      parentBtn.style.display = 'inline-flex';
      let html = '';
      currentBreadcrumbs.forEach((b, idx) => {
        const isLast = (idx === currentBreadcrumbs.length - 1);
        const name = escapeHtml(b.name);
        if (isLast) {
          html += `<span class="crumb-current">${idx === 0 ? '<span style="color:var(--accent);">📁</span> ' : ''}${name}</span>`;
        } else {
          html += `
            <a class="crumb-link" onclick="navigateToFolder('${escapeHtml(b.path)}')">${idx === 0 ? '<span style="color:var(--accent);">📁</span> ' : ''}${name}</a>
            <span class="crumb-sep">›</span>
          `;
        }
      });
      wrap.innerHTML = html;
      wrap.scrollLeft = wrap.scrollWidth;
    }

    function onSearchInput(val) {
      const clearBtn = document.getElementById('searchClearBtn');
      const kbd = document.getElementById('searchKbdHint');
      if (val && val.length > 0) {
        if (clearBtn) clearBtn.style.display = 'flex';
        if (kbd) kbd.style.display = 'none';
      } else {
        if (clearBtn) clearBtn.style.display = 'none';
        if (kbd) kbd.style.display = 'inline-block';
      }
      filterCurrentDirectory(val);
    }

    function clearSearch() {
      const input = document.getElementById('searchInput');
      if (input) {
        input.value = '';
        onSearchInput('');
        input.focus();
      }
    }

    function filterCurrentDirectory(val) {
      searchQuery = (val || '').toLowerCase().trim();
      renderContent();
    }

    function setCategory(cat, btn) {
      currentCategory = cat;
      document.querySelectorAll('.cat-btn').forEach(b => b.classList.remove('active'));
      btn.classList.add('active');
      renderContent();
    }

    function toggleSort(col) {
      if (sortColumn === col) {
        sortAsc = !sortAsc;
      } else {
        sortColumn = col;
        sortAsc = true;
      }
      updateSortHeaders();
      renderContent();
    }

    function updateSortHeaders() {
      const arrName = document.getElementById('sortArrowName');
      const arrSize = document.getElementById('sortArrowSize');
      if (arrName) {
        arrName.textContent = (sortColumn === 'name') ? (sortAsc ? '▲' : '▼') : '↕';
        arrName.style.opacity = (sortColumn === 'name') ? '1' : '0.4';
      }
      if (arrSize) {
        arrSize.textContent = (sortColumn === 'size') ? (sortAsc ? '▲' : '▼') : '↕';
        arrSize.style.opacity = (sortColumn === 'size') ? '1' : '0.4';
      }
    }

    function renderContent() {
      let filteredFolders = [...currentFolders];
      let filteredFiles = [...currentFiles];

      if (currentCategory !== 'all') {
        filteredFolders = [];
        filteredFiles = filteredFiles.filter(f => f.category === currentCategory);
      }

      if (searchQuery) {
        filteredFolders = filteredFolders.filter(f => f.name.toLowerCase().includes(searchQuery));
        filteredFiles = filteredFiles.filter(f => f.filename.toLowerCase().includes(searchQuery));
      }

      filteredFolders.sort((a, b) => a.name.localeCompare(b.name));
      filteredFiles.sort((a, b) => {
        let valA = a[sortColumn === 'name' ? 'filename' : sortColumn];
        let valB = b[sortColumn === 'name' ? 'filename' : sortColumn];
        if (typeof valA === 'string') return sortAsc ? valA.localeCompare(valB) : valB.localeCompare(valA);
        return sortAsc ? valA - valB : valB - valA;
      });

      if (viewMode === 'table') {
        renderTableView(filteredFolders, filteredFiles);
      } else {
        renderGridView(filteredFolders, filteredFiles);
      }

      updateBatchBar();
    }

    function renderTableView(folders, files) {
      const tbody = document.getElementById('filesTableBody');
      if (folders.length === 0 && files.length === 0) {
        tbody.innerHTML = '<tr><td colspan="5" style="text-align: center; color: var(--text-dim); padding: 3rem;">This folder is empty. Upload files or folders above.</td></tr>';
        return;
      }

      let html = '';

      // Render Folders
      folders.forEach(folder => {
        const isChecked = selectedItems.has(folder.path) ? 'checked' : '';
        html += `
          <tr>
            <td style="text-align: center;">
              <input type="checkbox" ${isChecked} onchange="toggleSelectItem('${escapeHtml(folder.path)}', this.checked)">
            </td>
            <td>
              <div class="folder-cell" onclick="navigateToFolder('${escapeHtml(folder.path)}')">
                <span class="folder-icon">📁</span>
                <span title="${escapeHtml(folder.name)}">${escapeHtml(folder.name)}</span>
              </div>
            </td>
            <td style="color: var(--text-dim);" class="tnum">${folder.item_count} item${folder.item_count === 1 ? '' : 's'}</td>
            <td><span class="file-badge">Folder</span></td>
            <td style="text-align: right; white-space: nowrap;">
              <button class="btn btn-secondary btn-sm" onclick="navigateToFolder('${escapeHtml(folder.path)}')">Open</button>
              <button class="btn btn-secondary btn-sm" onclick="downloadFolderZip('${escapeHtml(folder.path)}')">⬇ ZIP</button>
              <button class="btn btn-secondary btn-sm btn-icon" onclick="deleteItem('${escapeHtml(folder.path)}', true)" style="color: var(--danger);" title="Delete Folder">✕</button>
            </td>
          </tr>
        `;
      });

      // Render Files
      files.forEach(f => {
        const isChecked = selectedItems.has(f.path) ? 'checked' : '';
        const thumbUrl = f.has_thumbnail ? `/api/thumbnail/${f.id}` : null;
        const icon = getFileIcon(f.category);

        let sizeHtml = `<span style="color: var(--text-dim);" class="tnum">${formatBytes(f.size)}</span>`;
        let badgesHtml = '';
        let actionButtonsHtml = '';

        if (f.is_receiving) {
          sizeHtml = `
            <div style="display: flex; flex-direction: column; gap: 3px; min-width: 130px;">
              <div style="display: flex; justify-content: space-between; font-size: 0.72rem;">
                <span id="receiving_pct_${f.id}" style="color: var(--accent); font-weight: 600;" class="tnum">⚡ ${f.receiving_percent}%</span>
                <span id="receiving_bytes_${f.id}" style="color: var(--text-dim);" class="tnum">${formatBytes(f.written_bytes)} / ${formatBytes(f.size)}</span>
              </div>
              <div style="width: 100%; height: 3px; background: var(--bg); border-radius: 2px; overflow: hidden;">
                <div id="receiving_bar_${f.id}" style="height: 100%; width: ${f.receiving_percent}%; background: var(--accent); transition: width 0.1s linear;"></div>
              </div>
            </div>
          `;
          badgesHtml = `
            <span class="file-badge" style="background: var(--accent-subtle); color: var(--accent); border-color: rgba(56, 189, 248, 0.3);">⚡ Live Stream</span>
            <span class="file-badge">${f.category}</span>
          `;
          if (f.category === 'video') {
            actionButtonsHtml = `<button class="btn btn-accent btn-sm" onclick="playVideo('${f.id}')">⚡ Stream Live</button>`;
          } else if (f.category === 'audio') {
            actionButtonsHtml = `<button class="btn btn-accent btn-sm" onclick="playAudio('${f.id}')">⚡ Play Live</button>`;
          } else {
            actionButtonsHtml = `<button class="btn btn-secondary btn-sm" disabled style="opacity: 0.6;">Receiving...</button>`;
          }
          actionButtonsHtml += `
            <a class="btn btn-secondary btn-sm btn-icon" href="/api/stream/${f.id}" download="${escapeHtml(f.filename)}" title="Progressive Download">⬇</a>
            <button class="btn btn-secondary btn-sm btn-icon" onclick="deleteItem('${escapeHtml(f.path)}', false)" title="Cancel / Delete" style="color: var(--danger);">✕</button>
          `;
        } else {
          badgesHtml = `
            <span class="file-badge">${f.category}</span>
            ${f.resolution ? `<span class="file-badge" style="color: #facc15;">${f.resolution}</span>` : ''}
            ${f.duration_label ? `<span class="file-badge tnum">${f.duration_label}</span>` : ''}
            ${f.category === 'video' && !f.has_audio ? `<span class="file-badge" style="color: var(--text-dim);">🔇 Muted</span>` : ''}
            ${f.needs_audio_transcode ? `<span class="file-badge" style="color: var(--accent);">Dolby ${escapeHtml((f.audio_codec||'').toUpperCase())}</span>` : ''}
            ${f.has_subtitles ? `<span class="file-badge" style="color: #c084fc;">CC</span>` : ''}
          `;
          if (f.category === 'video') actionButtonsHtml += `<button class="btn btn-secondary btn-sm" onclick="playVideo('${f.id}')">▶ Stream</button>`;
          if (f.category === 'audio') actionButtonsHtml += `<button class="btn btn-secondary btn-sm" onclick="playAudio('${f.id}')">▶ Play</button>`;
          if (f.category === 'photo') actionButtonsHtml += `<button class="btn btn-secondary btn-sm" onclick="openLightbox('${escapeHtml(f.filename)}')">👁️ View</button>`;
          if (isPreviewableDoc(f.filename)) actionButtonsHtml += `<button class="btn btn-secondary btn-sm" onclick="openDocPreview('${f.id}')">📄 View</button>`;
          actionButtonsHtml += `
            <a class="btn btn-secondary btn-sm btn-icon" href="/api/stream/${f.id}" download="${escapeHtml(f.filename)}" title="Download">⬇</a>
            <button class="btn btn-secondary btn-sm btn-icon" onclick="deleteItem('${escapeHtml(f.path)}', false)" title="Delete" style="color: var(--danger);">✕</button>
          `;
        }

        html += `
          <tr ${f.is_receiving ? 'style="background: rgba(56, 189, 248, 0.03); border-left: 2px solid var(--accent);"' : ''}>
            <td style="text-align: center;">
              <input type="checkbox" ${isChecked} onchange="toggleSelectItem('${escapeHtml(f.path)}', this.checked)">
            </td>
            <td>
              <div class="file-cell" onclick="handleFileClick('${f.id}')">
                ${thumbUrl ? `
                  <img src="${thumbUrl}" class="table-thumb" alt="" loading="lazy" onerror="this.style.display='none'">
                ` : `
                  <span style="font-size: 1.15rem; color: ${f.is_receiving ? 'var(--accent)' : 'var(--text-dim)'};">${f.is_receiving ? '⚡' : icon}</span>
                `}
                <span style="cursor: pointer;" title="${escapeHtml(f.filename)}">
                  ${escapeHtml(f.filename)}
                  ${f.is_receiving ? `<span style="font-size: 0.72rem; color: var(--accent); margin-left: 0.4rem; font-weight: 500;">(Live)</span>` : ''}
                </span>
              </div>
            </td>
            <td>${sizeHtml}</td>
            <td>${badgesHtml}</td>
            <td style="text-align: right; white-space: nowrap;">
              ${actionButtonsHtml}
            </td>
          </tr>
        `;
      });

      tbody.innerHTML = html;
    }

    function renderGridView(folders, files) {
      const container = document.getElementById('gridContainer');
      if (folders.length === 0 && files.length === 0) {
        container.innerHTML = '<div style="grid-column: 1/-1; text-align: center; color: var(--text-dim); padding: 3rem;">This folder is empty. Upload files or folders above.</div>';
        return;
      }

      let html = '';

      // Folders
      folders.forEach(folder => {
        const isChecked = selectedItems.has(folder.path);
        html += `
          <div class="grid-card folder-card ${isChecked ? 'selected' : ''}" onclick="navigateToFolder('${escapeHtml(folder.path)}')">
            <span class="folder-icon" style="font-size: 1.6rem;">📁</span>
            <div style="overflow: hidden; flex: 1;">
              <div style="font-weight: 600; font-size: 0.85rem; color: #fff; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;">${escapeHtml(folder.name)}</div>
              <div style="font-size: 0.7rem; color: var(--text-dim);" class="tnum">${folder.item_count} items</div>
            </div>
            <button class="btn btn-secondary btn-sm btn-icon" onclick="event.stopPropagation(); downloadFolderZip('${escapeHtml(folder.path)}')">⬇</button>
          </div>
        `;
      });

      // Files
      files.forEach(f => {
        const isChecked = selectedItems.has(f.path);
        const thumbUrl = f.has_thumbnail ? `/api/thumbnail/${f.id}` : null;
        const icon = getFileIcon(f.category);

        let liveBadge = '';
        let gridActions = '';

        if (f.is_receiving) {
          liveBadge = `<span id="grid_receiving_pct_${f.id}" class="file-badge tnum" style="position: absolute; top: 6px; left: 6px; background: rgba(2, 132, 199, 0.95); color: #fff; font-weight: 600; z-index: 2;">⚡ Live (${f.receiving_percent}%)</span>`;
          if (f.category === 'video') {
            gridActions = `<button class="btn btn-accent btn-sm" style="flex:1;" onclick="playVideo('${f.id}')">⚡ Stream Live</button>`;
          } else if (f.category === 'audio') {
            gridActions = `<button class="btn btn-accent btn-sm" style="flex:1;" onclick="playAudio('${f.id}')">⚡ Play Live</button>`;
          } else {
            gridActions = `<span style="font-size:0.75rem; color:var(--accent); flex:1; text-align:center;">Receiving...</span>`;
          }
          gridActions += `
            <a class="btn btn-secondary btn-sm btn-icon" href="/api/stream/${f.id}" download="${escapeHtml(f.filename)}" title="Progressive Download">⬇</a>
            <button class="btn btn-secondary btn-sm btn-icon" onclick="deleteItem('${escapeHtml(f.path)}', false)" style="color: var(--danger);">✕</button>
          `;
        } else {
          if (f.category === 'video') gridActions += `<button class="btn btn-secondary btn-sm" style="flex:1;" onclick="playVideo('${f.id}')">▶ Stream</button>`;
          if (f.category === 'audio') gridActions += `<button class="btn btn-secondary btn-sm" style="flex:1;" onclick="playAudio('${f.id}')">▶ Play</button>`;
          if (f.category === 'photo') gridActions += `<button class="btn btn-secondary btn-sm" style="flex:1;" onclick="openLightbox('${escapeHtml(f.filename)}')">👁️ View</button>`;
          if (isPreviewableDoc(f.filename)) gridActions += `<button class="btn btn-secondary btn-sm" style="flex:1;" onclick="openDocPreview('${f.id}')">📄 View</button>`;
          gridActions += `
            <a class="btn btn-secondary btn-sm btn-icon" href="/api/stream/${f.id}" download="${escapeHtml(f.filename)}" title="Download">⬇</a>
            <button class="btn btn-secondary btn-sm btn-icon" onclick="deleteItem('${escapeHtml(f.path)}', false)" title="Delete" style="color: var(--danger);">✕</button>
          `;
        }

        html += `
          <div class="grid-card ${isChecked ? 'selected' : ''}" style="${f.is_receiving ? 'border-color: rgba(56, 189, 248, 0.4);' : ''}">
            ${liveBadge}
            <div class="grid-checkbox-wrap">
              <input type="checkbox" ${isChecked ? 'checked' : ''} onchange="toggleSelectItem('${escapeHtml(f.path)}', this.checked)">
            </div>
            <div class="grid-preview-box" onclick="handleFileClick('${f.id}')" style="position: relative;">
              ${thumbUrl ? `
                <img src="${thumbUrl}" class="grid-thumb-img" alt="" loading="lazy">
              ` : `
                <span class="grid-file-icon" style="color: ${f.is_receiving ? 'var(--accent)' : 'inherit'};">${f.is_receiving ? '⚡' : icon}</span>
              `}
              ${f.category === 'video' ? `
                <div style="position: absolute; inset: 0; display: flex; align-items: center; justify-content: center; background: rgba(0,0,0,0.25);">
                  <div style="width: 32px; height: 32px; border-radius: 50%; background: rgba(0,0,0,0.65); border: 1px solid rgba(255,255,255,0.2); display: flex; align-items: center; justify-content: center; color: #fff; font-size: 0.75rem;">▶</div>
                </div>
              ` : ''}
            </div>
            <div class="grid-card-body">
              <div class="grid-filename" title="${escapeHtml(f.filename)}">${escapeHtml(f.filename)}</div>
              <div class="grid-meta-row">
                <span class="tnum">${f.is_receiving ? formatBytes(f.written_bytes) + ' / ' + formatBytes(f.size) : formatBytes(f.size)}</span>
                ${f.is_receiving ? '<span class="file-badge" style="color:var(--accent);">Live</span>' : (f.resolution ? `<span class="file-badge">${f.resolution}</span>` : '')}
              </div>
              <div class="grid-actions">
                ${gridActions}
              </div>
            </div>
          </div>
        `;
      });

      container.innerHTML = html;
    }

    function getFileIcon(cat) {
      switch (cat) {
        case 'video': return '🎬';
        case 'photo': return '🖼️';
        case 'audio': return '🎵';
        case 'document': return '📄';
        case 'archive': return '📦';
        default: return '📄';
      }
    }

    function isPreviewableDoc(filename) {
      const lower = (filename || '').toLowerCase();
      return lower.endsWith('.txt') || lower.endsWith('.md') || lower.endsWith('.json') || 
             lower.endsWith('.log') || lower.endsWith('.rs') || lower.endsWith('.js') || 
             lower.endsWith('.py') || lower.endsWith('.bat') || lower.endsWith('.csv') ||
             lower.endsWith('.pdf');
    }

    function handleFileClick(id) {
      const file = currentFiles.find(f => f.id === id);
      if (!file) return;
      if (file.category === 'video') playVideo(id);
      else if (file.category === 'audio') playAudio(id);
      else if (file.category === 'photo') openLightbox(file.filename);
      else if (isPreviewableDoc(file.filename)) openDocPreview(id);
      else window.location.href = `/api/stream/${file.id}`;
    }

    // New Folder
    function openNewFolderModal() {
      document.getElementById('newFolderNameInput').value = '';
      openModal('newFolderModal');
      setTimeout(() => document.getElementById('newFolderNameInput').focus(), 150);
    }

    async function submitCreateFolder() {
      const name = document.getElementById('newFolderNameInput').value.trim();
      if (!name) return;

      try {
        const res = await fetch('/api/mkdir', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ path: currentPath, name })
        });
        if (res.ok) {
          closeModal('newFolderModal');
          showToast(`Created folder "${name}"`);
          loadCurrentFolder();
        } else {
          const err = await res.json();
          alert(err.error || 'Failed to create folder');
        }
      } catch (e) {
        alert('Error: ' + e.message);
      }
    }

    // Download Folder as ZIP
    function downloadFolderZip(folderRelPath) {
      window.location.href = `/api/download-folder?path=${encodeURIComponent(folderRelPath)}`;
      showToast('Archiving folder as ZIP...');
    }

    function downloadCurrentFolderZip() {
      window.location.href = `/api/download-folder?path=${encodeURIComponent(currentPath)}`;
      showToast('Archiving folder as ZIP...');
    }

    // Batch Selection
    function toggleSelectItem(itemPath, checked) {
      if (checked) selectedItems.add(itemPath);
      else selectedItems.delete(itemPath);
      updateBatchBar();
      renderContent();
    }

    function toggleSelectAll(checked) {
      if (checked) {
        currentFolders.forEach(f => selectedItems.add(f.path));
        currentFiles.forEach(f => selectedItems.add(f.path));
      } else {
        selectedItems.clear();
      }
      renderContent();
    }

    function deselectAll() {
      selectedItems.clear();
      document.getElementById('selectAllCheckbox').checked = false;
      renderContent();
    }

    function updateBatchBar() {
      const bar = document.getElementById('batchBar');
      const count = selectedItems.size;
      document.getElementById('batchSelectedText').textContent = `${count} item${count === 1 ? '' : 's'} selected`;
      if (count > 0) bar.classList.add('show');
      else bar.classList.remove('show');
    }

    function downloadSelectedZip() {
      if (selectedItems.size === 0) return;
      const itemsArr = Array.from(selectedItems).map(encodeURIComponent).join(',');
      window.location.href = `/api/archive?files=${itemsArr}`;
      showToast('Preparing ZIP download...');
    }

    async function deleteSelectedItems() {
      if (selectedItems.size === 0) return;
      if (!confirm(`Delete ${selectedItems.size} selected item(s)?`)) return;

      for (const itemPath of Array.from(selectedItems)) {
        try {
          await fetch(`/api/files/${encodeURIComponent(itemPath)}`, { method: 'DELETE' });
        } catch(e) {}
      }
      selectedItems.clear();
      showToast('Deleted selected items');
      loadCurrentFolder();
    }

    async function deleteItem(itemPath, isFolder) {
      const msg = isFolder ? `Delete folder "${itemPath}" and all its contents?` : `Delete "${itemPath}"?`;
      if (!confirm(msg)) return;

      try {
        const res = await fetch(`/api/files/${encodeURIComponent(itemPath)}`, { method: 'DELETE' });
        if (res.ok) {
          showToast(`Deleted ${itemPath}`);
          loadCurrentFolder();
        } else {
          showToast('Failed to delete');
        }
      } catch(e) {
        showToast('Error: ' + e.message);
      }
    }

    // Screen Wake Lock & Disconnect Prevention
    async function requestWakeLock() {
      try {
        if ('wakeLock' in navigator && !wakeLockSentinel) {
          wakeLockSentinel = await navigator.wakeLock.request('screen');
          wakeLockSentinel.addEventListener('release', () => { wakeLockSentinel = null; });
        }
      } catch (e) {}
    }

    function releaseWakeLock() {
      if (wakeLockSentinel) {
        try { wakeLockSentinel.release(); } catch(e) {}
        wakeLockSentinel = null;
      }
    }

    document.addEventListener('visibilitychange', () => {
      if (document.visibilityState === 'visible' && isUploading && !isUploadPaused) {
        requestWakeLock();
      }
    });

    window.addEventListener('beforeunload', (e) => {
      if (isUploading && !isUploadPaused) {
        e.preventDefault();
        e.returnValue = 'File transfer in progress. Leaving will pause the upload.';
        return e.returnValue;
      }
    });

    window.addEventListener('online', () => {
      if (isUploading && !isUploadPaused && autoRetryTimer) {
        clearTimeout(autoRetryTimer);
        autoRetryTimer = null;
        updateHudState('uploading', 'Network restored • Resuming...');
        processUploadQueue();
      }
    });

    // Upload HUD State Updates
    function updateHudState(state, customMsg) {
      const dot = document.getElementById('hudStatusDot');
      const btn = document.getElementById('hudPauseResumeBtn');
      const speed = document.getElementById('hudSpeed');

      if (!dot || !btn || !speed) return;

      if (state === 'uploading') {
        dot.style.background = 'var(--accent)';
        dot.style.boxShadow = '0 0 6px var(--accent)';
        btn.textContent = '⏸ Pause';
        btn.style.display = 'inline-flex';
        if (customMsg) speed.textContent = customMsg;
      } else if (state === 'paused') {
        dot.style.background = '#8b92a4';
        dot.style.boxShadow = 'none';
        btn.textContent = '▶ Resume';
        btn.style.display = 'inline-flex';
        speed.textContent = customMsg || 'Paused';
      } else if (state === 'retrying') {
        dot.style.background = '#f59e0b';
        dot.style.boxShadow = '0 0 6px #f59e0b';
        btn.textContent = '⏸ Pause';
        btn.style.display = 'inline-flex';
        if (customMsg) speed.textContent = customMsg;
      } else if (state === 'completed') {
        dot.style.background = 'var(--success)';
        dot.style.boxShadow = '0 0 6px var(--success)';
        btn.style.display = 'none';
        if (customMsg) speed.textContent = customMsg;
      }
    }

    function toggleUploadPause() {
      if (isUploadPaused) {
        isUploadPaused = false;
        retryAttempt = 0;
        updateHudState('uploading', 'Resuming transfer...');
        showToast('Resuming upload...');
        processUploadQueue();
      } else {
        isUploadPaused = true;
        if (currentChunkXhr) {
          currentChunkXhr.abort();
          currentChunkXhr = null;
        }
        if (autoRetryTimer) {
          clearTimeout(autoRetryTimer);
          autoRetryTimer = null;
        }
        releaseWakeLock();
        updateHudState('paused', 'Paused • Progress saved');
        showToast('Upload paused. All progress is safely saved.');
      }
    }

    function cancelActiveUpload() {
      const activeItem = currentUploadItem;
      isUploading = false;
      isUploadPaused = false;

      if (currentChunkXhr) {
        currentChunkXhr.abort();
        currentChunkXhr = null;
      }
      if (autoRetryTimer) {
        clearTimeout(autoRetryTimer);
        autoRetryTimer = null;
      }
      releaseWakeLock();

      if (activeItem) {
        fetch('/api/upload-cancel', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ currentDir: currentPath, path: activeItem.relativePath })
        }).catch(() => {});
      }

      uploadQueue = [];
      currentUploadItem = null;
      document.getElementById('uploadHud').style.display = 'none';
      showToast('Upload cancelled');
    }

    // Upload Files & Entire Folders Trigger
    function triggerFileInput() { document.getElementById('filePicker').click(); }
    function triggerFolderInput() { document.getElementById('folderPicker').click(); }

    async function handleFilesPicked(e) {
      const files = Array.from(e.target.files);
      if (!files || files.length === 0) return;
      await uploadFileList(files.map(f => ({ file: f, relativePath: f.name })));
      e.target.value = '';
    }

    async function handleFolderPicked(e) {
      const files = Array.from(e.target.files);
      if (!files || files.length === 0) return;
      await uploadFileList(files.map(f => ({ file: f, relativePath: f.webkitRelativePath || f.name })));
      e.target.value = '';
    }

    const CHUNK_SIZE = 2 * 1024 * 1024; // 2 MB chunks (safe on mobile WiFi, rapid chunking)

    async function uploadFileList(items) {
      if (!items || items.length === 0) return;

      const newItems = items.map((it, idx) => ({
        file: it.file,
        relativePath: it.relativePath,
        index: idx,
        totalItems: items.length,
        offset: undefined
      }));

      uploadQueue = newItems;
      isUploading = true;
      isUploadPaused = false;
      retryAttempt = 0;

      updateHudState('uploading', 'Starting transfer...');
      processUploadQueue();
    }

    async function processUploadQueue() {
      if (isUploadPaused || !isUploading) return;
      if (uploadQueue.length === 0) {
        isUploading = false;
        releaseWakeLock();
        const hud = document.getElementById('uploadHud');
        const hudFill = document.getElementById('hudBarFill');
        const hudPercent = document.getElementById('hudPercent');
        const hudSpeed = document.getElementById('hudSpeed');

        hudFill.style.width = '100%';
        hudPercent.textContent = '100%';
        hudSpeed.textContent = 'Done!';
        updateHudState('completed', 'Finished!');

        setTimeout(() => {
          hud.style.display = 'none';
        }, 1200);

        showToast('All items uploaded successfully!');
        loadCurrentFolder();
        initNetwork();
        return;
      }

      await requestWakeLock();
      const item = uploadQueue[0];
      currentUploadItem = item;

      const hud = document.getElementById('uploadHud');
      const hudName = document.getElementById('hudFileName');
      const hudFill = document.getElementById('hudBarFill');
      const hudPercent = document.getElementById('hudPercent');
      const hudSpeed = document.getElementById('hudSpeed');
      hud.style.display = 'flex';

      const file = item.file;
      const totalSize = file.size;
      const queuePos = `(${item.index + 1}/${item.totalItems}) ${item.relativePath}`;
      hudName.textContent = queuePos;

      // 1. Query server for resume status if not already determined
      if (item.offset === undefined) {
        try {
          const statusRes = await fetch(`/api/upload-status?currentDir=${encodeURIComponent(currentPath)}&relativePath=${encodeURIComponent(item.relativePath)}&fileSize=${totalSize}`);
          if (statusRes.ok) {
            const data = await statusRes.json();
            if (data.status === 'completed') {
              // File is already fully uploaded on server
              uploadQueue.shift();
              processUploadQueue();
              return;
            }
            item.offset = data.bytesReceived || 0;
          } else {
            item.offset = 0;
          }
        } catch (e) {
          scheduleRetry('Checking connection...');
          return;
        }
      }

      // 2. Upload chunks until this file is complete
      let lastTime = Date.now();
      let lastLoaded = item.offset;

      while (item.offset < totalSize) {
        if (isUploadPaused || !isUploading) return;

        const chunkEnd = Math.min(totalSize, item.offset + CHUNK_SIZE);
        const chunkBlob = file.slice(item.offset, chunkEnd);
        const chunkLength = chunkBlob.size;
        const currentOffset = item.offset;

        updateHudState('uploading');

        const success = await new Promise((resolve) => {
          const xhr = new XMLHttpRequest();
          currentChunkXhr = xhr;

          xhr.upload.onprogress = (evt) => {
            if (evt.lengthComputable) {
              const currentTotalLoaded = currentOffset + evt.loaded;
              const percent = totalSize > 0 ? Math.min(99, Math.round((currentTotalLoaded / totalSize) * 100)) : 0;
              hudFill.style.width = percent + '%';
              hudPercent.textContent = percent + '%';

              const now = Date.now();
              const dt = (now - lastTime) / 1000;
              if (dt > 0.4) {
                const speedBps = (currentTotalLoaded - lastLoaded) / dt;
                hudSpeed.textContent = (speedBps / (1024 * 1024)).toFixed(1) + ' MB/s';
                lastTime = now;
                lastLoaded = currentTotalLoaded;
              }
            }
          };

          xhr.onload = () => {
            currentChunkXhr = null;
            if (xhr.status >= 200 && xhr.status < 300) {
              retryAttempt = 0;
              resolve(true);
            } else if (xhr.status === 409) {
              // Offset mismatch: sync to server's expected offset
              try {
                const d = JSON.parse(xhr.responseText);
                if (typeof d.expectedOffset === 'number') {
                  item.offset = d.expectedOffset;
                }
              } catch (e) {}
              resolve(false);
            } else {
              resolve(false);
            }
          };

          xhr.onerror = () => {
            currentChunkXhr = null;
            resolve(false);
          };

          xhr.ontimeout = () => {
            currentChunkXhr = null;
            resolve(false);
          };

          xhr.timeout = 25000; // 25s timeout for 2MB chunk

          xhr.open('POST', '/api/upload');
          xhr.setRequestHeader('x-current-dir', encodeURIComponent(currentPath));
          xhr.setRequestHeader('x-relative-path', encodeURIComponent(item.relativePath));
          xhr.setRequestHeader('x-file-size', totalSize);
          xhr.setRequestHeader('x-upload-offset', currentOffset);
          xhr.setRequestHeader('x-chunk-size', chunkLength);
          xhr.setRequestHeader('x-client-id', getClientId());
          xhr.send(chunkBlob);
        });

        if (success) {
          item.offset = chunkEnd;
          const percent = totalSize > 0 ? Math.min(100, Math.round((item.offset / totalSize) * 100)) : 100;
          hudFill.style.width = percent + '%';
          hudPercent.textContent = percent + '%';
        } else {
          if (isUploadPaused || !isUploading) return;
          scheduleRetry('Connection drop');
          return;
        }
      }

      // Item completed! Move to next item
      uploadQueue.shift();
      processUploadQueue();
    }

    function scheduleRetry(reason) {
      if (isUploadPaused || !isUploading) return;
      retryAttempt++;
      const delay = Math.min(8000, 1500 * Math.pow(1.4, retryAttempt - 1));
      const secs = Math.ceil(delay / 1000);

      updateHudState('retrying', `Reconnecting in ${secs}s... (Auto-resume)`);

      autoRetryTimer = setTimeout(() => {
        autoRetryTimer = null;
        if (isUploadPaused || !isUploading) return;

        fetch('/api/health')
          .then(res => {
            if (res.ok) {
              updateHudState('uploading', 'Reconnected! Resuming...');
              if (uploadQueue[0]) uploadQueue[0].offset = undefined;
              processUploadQueue();
            } else {
              scheduleRetry('Server not ready');
            }
          })
          .catch(() => {
            scheduleRetry('Network offline');
          });
      }, delay);
    }

    // Drag & Drop for Files AND Folders
    const dropStrip = document.getElementById('dropStrip');
    ['dragenter', 'dragover'].forEach(n => {
      dropStrip.addEventListener(n, (e) => { e.preventDefault(); dropStrip.classList.add('dragover'); }, false);
    });
    ['dragleave', 'drop'].forEach(n => {
      dropStrip.addEventListener(n, (e) => { e.preventDefault(); dropStrip.classList.remove('dragover'); }, false);
    });
    dropStrip.addEventListener('drop', async (e) => {
      const dt = e.dataTransfer;
      if (!dt) return;

      const itemsToUpload = [];

      if (dt.items && dt.items.length > 0 && dt.items[0].webkitGetAsEntry) {
        const queue = [];
        for (let i = 0; i < dt.items.length; i++) {
          const entry = dt.items[i].webkitGetAsEntry();
          if (entry) queue.push(traverseFileTree(entry, ''));
        }
        const results = await Promise.all(queue);
        results.forEach(arr => itemsToUpload.push(...arr));
      } else if (dt.files && dt.files.length > 0) {
        for (let i = 0; i < dt.files.length; i++) {
          itemsToUpload.push({ file: dt.files[i], relativePath: dt.files[i].name });
        }
      }

      if (itemsToUpload.length > 0) {
        uploadFileList(itemsToUpload);
      }
    });

    function traverseFileTree(item, currentPathPrefix) {
      return new Promise((resolve) => {
        if (item.isFile) {
          item.file((file) => {
            resolve([{ file, relativePath: currentPathPrefix ? `${currentPathPrefix}/${file.name}` : file.name }]);
          });
        } else if (item.isDirectory) {
          const dirReader = item.createReader();
          const dirPath = currentPathPrefix ? `${currentPathPrefix}/${item.name}` : item.name;
          const readAllEntries = () => {
            dirReader.readEntries(async (entries) => {
              if (entries.length === 0) {
                resolve([]);
              } else {
                const subTasks = entries.map(e => traverseFileTree(e, dirPath));
                const subResults = await Promise.all(subTasks);
                const flat = [];
                subResults.forEach(r => flat.push(...r));
                resolve(flat);
              }
            });
          };
          readAllEntries();
        } else {
          resolve([]);
        }
      });
    }

    // Video Player & Audio Mode
    let currentPlayingFile = null;
    let currentAudioMode = 'direct';

    function playVideo(id) {
      const file = currentFiles.find(f => f.id === id);
      if (!file) return;
      currentPlayingFile = file;

      const stage = document.getElementById('mediaStage');
      const title = document.getElementById('mediaTitle');
      const openBtn = document.getElementById('mediaOpenBtn');
      const dlBtn = document.getElementById('mediaDownloadBtn');
      const audioBadge = document.getElementById('audioModeBadge');
      const audioToggle = document.getElementById('btnAudioToggle');
      const subBadge = document.getElementById('subBadge');
      const resBadge = document.getElementById('resBadge');
      const mediaInfo = document.getElementById('mediaInfo');

      title.textContent = file.filename;
      dlBtn.href = `/api/stream/${id}`;
      dlBtn.download = file.filename;

      stage.innerHTML = '';

      if (file.resolution) {
        resBadge.style.display = 'inline-flex';
        resBadge.textContent = file.resolution;
      } else {
        resBadge.style.display = 'none';
      }

      if (file.is_receiving) {
        audioToggle.style.display = 'none';
        audioBadge.style.display = 'inline-flex';
        audioBadge.textContent = `⚡ Live Stream (${file.receiving_percent || 0}%)`;
        audioBadge.style.color = '#38bdf8';
        audioBadge.style.borderColor = 'rgba(56, 189, 248, 0.4)';
        audioBadge.id = 'videoLiveReceivingBadge';
        mediaInfo.textContent = 'Streaming Live while Receiving in Background';
        currentAudioMode = 'direct';
      } else if (file.has_audio === false) {
        audioToggle.style.display = 'none';
        audioBadge.style.display = 'inline-flex';
        audioBadge.textContent = '🔇 Muted Video';
        mediaInfo.textContent = 'Direct Stream (Muted)';
        currentAudioMode = 'direct';
      } else if (file.needs_audio_transcode) {
        audioToggle.style.display = 'inline-flex';
        currentAudioMode = 'transcode';
        audioBadge.style.display = 'inline-flex';
        audioBadge.textContent = `🔊 Dolby ${escapeHtml((file.audio_codec||'').toUpperCase())} → AAC`;
        audioToggle.textContent = 'Switch to Direct Stream';
        mediaInfo.textContent = 'Universal Audio Active';
      } else {
        audioToggle.style.display = 'inline-flex';
        currentAudioMode = 'direct';
        audioBadge.style.display = 'none';
        audioToggle.textContent = 'Fix Audio (AAC)';
        mediaInfo.textContent = 'Direct Range Stream';
      }

      const video = document.createElement('video');
      video.id = 'activeVideoPlayer';
      video.controls = true;
      video.autoplay = true;
      video.playsInline = true;
      video.src = (currentAudioMode === 'transcode') ? `/api/transcode/${id}` : `/api/stream/${id}`;

      if (file.has_subtitles) {
        subBadge.style.display = 'inline-flex';
        const track = document.createElement('track');
        track.kind = 'subtitles';
        track.label = 'English';
        track.srclang = 'en';
        track.src = `/api/subtitles/${id}`;
        track.default = true;
        video.appendChild(track);
      } else {
        subBadge.style.display = 'none';
      }

      openBtn.href = video.src;
      stage.appendChild(video);
      openModal('mediaModal');
    }

    function toggleAudioMode() {
      if (!currentPlayingFile) return;
      const video = document.getElementById('activeVideoPlayer');
      if (!video) return;

      const audioBadge = document.getElementById('audioModeBadge');
      const audioToggle = document.getElementById('btnAudioToggle');
      const mediaInfo = document.getElementById('mediaInfo');
      const openBtn = document.getElementById('mediaOpenBtn');

      if (currentAudioMode === 'transcode') {
        currentAudioMode = 'direct';
        video.src = `/api/stream/${currentPlayingFile.id}`;
        audioBadge.style.display = 'none';
        audioToggle.textContent = 'Fix Audio (AAC)';
        mediaInfo.textContent = 'Direct Stream (Original Audio)';
      } else {
        currentAudioMode = 'transcode';
        video.src = `/api/transcode/${currentPlayingFile.id}`;
        audioBadge.style.display = 'inline-flex';
        audioBadge.textContent = '🔊 Dolby → AAC Stereo';
        audioToggle.textContent = 'Switch to Direct Stream';
        mediaInfo.textContent = 'Universal Audio Active';
      }
      openBtn.href = video.src;
      video.play().catch(() => {});
    }

    function closeMediaModal() {
      const stage = document.getElementById('mediaStage');
      const v = stage.querySelector('video');
      if (v) v.pause();
      stage.innerHTML = '';
      closeModal('mediaModal');
    }

    // Audio HUD
    let currentPlayingAudio = null;

    function playAudio(id) {
      const file = currentFiles.find(f => f.id === id);
      if (!file) return;
      currentPlayingAudio = file;

      const player = document.getElementById('globalAudioPlayer');
      const hud = document.getElementById('audioPlayerHud');
      const title = document.getElementById('audioHudTitle');
      const dlBtn = document.getElementById('audioHudDownloadBtn');

      if (file.is_receiving) {
        title.innerHTML = `<span>${escapeHtml(file.filename)}</span> <span id="audioLiveReceivingBadge" style="font-size:0.72rem; color:var(--accent); font-weight:600;">(⚡ Streaming Live ${file.receiving_percent||0}%)</span>`;
      } else {
        title.textContent = file.filename;
      }
      dlBtn.href = `/api/stream/${id}`;
      dlBtn.download = file.filename;

      player.src = `/api/stream/${id}`;
      player.play().catch(() => {});
      hud.style.display = 'flex';
      updateAudioHudPlayBtn(true);
    }

    const globalAudio = document.getElementById('globalAudioPlayer');
    globalAudio.ontimeupdate = () => {
      const timeDisplay = document.getElementById('audioHudTime');
      const slider = document.getElementById('audioHudSeek');
      if (globalAudio.duration) {
        timeDisplay.textContent = `${formatDuration(globalAudio.currentTime)} / ${formatDuration(globalAudio.duration)}`;
        slider.value = (globalAudio.currentTime / globalAudio.duration) * 100;
      }
    };
    globalAudio.onended = () => { updateAudioHudPlayBtn(false); };

    function toggleAudioHudPlay() {
      if (globalAudio.paused) {
        globalAudio.play();
        updateAudioHudPlayBtn(true);
      } else {
        globalAudio.pause();
        updateAudioHudPlayBtn(false);
      }
    }

    function updateAudioHudPlayBtn(isPlaying) {
      document.getElementById('audioHudPlayBtn').textContent = isPlaying ? '⏸' : '▶';
    }

    function seekAudioHud(val) {
      if (globalAudio.duration) {
        globalAudio.currentTime = (val / 100) * globalAudio.duration;
      }
    }

    function closeAudioHud() {
      globalAudio.pause();
      document.getElementById('audioPlayerHud').style.display = 'none';
    }

    // Document Viewer
    async function openDocPreview(id) {
      const file = currentFiles.find(f => f.id === id);
      if (!file) return;

      document.getElementById('docTitle').textContent = file.filename;
      document.getElementById('docMetaInfo').textContent = `${formatBytes(file.size)} &bull; ${file.mime_type}`;
      document.getElementById('docDownloadBtn').href = `/api/stream/${id}`;
      document.getElementById('docDownloadBtn').download = file.filename;

      const body = document.getElementById('docModalBody');
      const isPdf = file.filename.toLowerCase().endsWith('.pdf');

      if (isPdf) {
        body.innerHTML = `
          <iframe src="/api/stream/${id}" style="width: 100%; height: 70vh; border: 1px solid var(--border); border-radius: 4px;"></iframe>
          <div style="display: flex; justify-content: flex-end;">
            <a class="btn btn-primary btn-sm" href="/api/stream/${id}" download="${escapeHtml(file.filename)}">Download PDF</a>
          </div>
        `;
      } else {
        body.innerHTML = `
          <div class="code-preview-area" id="codePreviewArea">Loading content...</div>
          <div style="display: flex; justify-content: space-between; align-items: center;">
            <span style="font-size: 0.75rem; color: var(--text-dim);">${formatBytes(file.size)}</span>
            <div style="display: flex; gap: 0.4rem;">
              <button class="btn btn-secondary btn-sm" onclick="copyDocContent()">Copy</button>
              <a class="btn btn-primary btn-sm" href="/api/stream/${id}" download="${escapeHtml(file.filename)}">Download</a>
            </div>
          </div>
        `;
        try {
          const res = await fetch(`/api/stream/${id}`);
          const text = await res.text();
          currentDocText = text;
          document.getElementById('codePreviewArea').textContent = text;
        } catch (e) {
          document.getElementById('codePreviewArea').textContent = 'Failed to load content: ' + e.message;
        }
      }

      openModal('docModal');
    }

    function copyDocContent() {
      if (currentDocText) {
        navigator.clipboard.writeText(currentDocText);
        showToast('Copied to clipboard');
      }
    }

    // Photo Lightbox with Touch Swiping
    function openLightbox(filename) {
      photoList = currentFiles.filter(f => f.category === 'photo');
      currentPhotoIdx = photoList.findIndex(f => f.filename === filename);
      if (currentPhotoIdx < 0) currentPhotoIdx = 0;
      updateLightboxView();
      openModal('lightboxModal');
    }

    function updateLightboxView() {
      if (photoList.length === 0) return;
      const photo = photoList[currentPhotoIdx];
      document.getElementById('lightboxTitle').textContent = photo.filename;
      document.getElementById('lightboxImg').src = `/api/stream/${photo.id}`;
      document.getElementById('lightboxCounter').textContent = `${currentPhotoIdx + 1} of ${photoList.length}`;
    }

    function navPhoto(delta) {
      if (photoList.length <= 1) return;
      currentPhotoIdx = (currentPhotoIdx + delta + photoList.length) % photoList.length;
      updateLightboxView();
    }

    // Touch Swipe handling on Lightbox
    let touchStartX = 0;
    const lightboxBody = document.getElementById('lightboxBody');
    lightboxBody.addEventListener('touchstart', (e) => {
      touchStartX = e.changedTouches[0].screenX;
    }, { passive: true });
    lightboxBody.addEventListener('touchend', (e) => {
      const touchEndX = e.changedTouches[0].screenX;
      const diff = touchEndX - touchStartX;
      if (Math.abs(diff) > 40) {
        if (diff > 0) navPhoto(-1); // Swipe right -> prev
        else navPhoto(1);          // Swipe left -> next
      }
    }, { passive: true });

    // Devices List
    async function loadDevices() {
      try {
        const res = await fetch('/api/peers', { headers: { 'x-client-id': getClientId() } });
        if (res.ok) {
          const data = await res.json();
          renderDevices(data);
        }
      } catch (e) {}
    }

    function renderDevices(data) {
      const wrap = document.getElementById('devicesListWrap');
      const host = data.host;
      const clients = (data.devices || []).filter(c => !c.is_host);

      document.getElementById('devicesTabCount').textContent = (1 + clients.length);

      let html = `
        <div class="device-row host-device">
          <div>
            <div class="device-title">
              <span>🖥️</span> ${escapeHtml(host.name)} (Host PC)
              <span class="file-badge" style="color: var(--accent);">Host</span>
            </div>
            <div class="device-meta">${host.address} &bull; Shared Storage Online</div>
          </div>
          <span style="font-size: 0.78rem; color: var(--success); font-weight: 600;">● Online</span>
        </div>
      `;

      if (clients.length === 0) {
        html += `
          <div class="empty-state">
            <div class="empty-icon">📱</div>
            <div class="empty-title">No other devices connected yet</div>
            <div class="empty-desc">
              Scan the QR code with your phone or open <code>${localUrl}</code> in any browser on this WiFi to connect.
            </div>
            <button class="btn btn-primary btn-sm" onclick="openPhoneModal()" style="margin-top: 0.5rem;">
              <span>📷</span> Show Phone QR Code
            </button>
          </div>
        `;
      } else {
        html += clients.map(c => `
          <div class="device-row">
            <div>
              <div class="device-title">
                <span>${c.device_type === 'phone' ? '📱' : c.device_type === 'tablet' ? '📱' : '💻'}</span>
                ${escapeHtml(c.device_name)}
              </div>
              <div class="device-meta">${c.ip} &bull; Connected to Mesh</div>
            </div>
            <span style="font-size: 0.78rem; color: var(--success); font-weight: 600;">● Active</span>
          </div>
        `).join('');
      }

      wrap.innerHTML = html;
    }

    // Clipboard
    async function loadClipboard() {
      try {
        const res = await fetch('/api/clipboard');
        if (res.ok) {
          const d = await res.json();
          document.getElementById('clipboardText').value = d.text || '';
          document.getElementById('clipboardStatus').textContent = `Last updated: ${new Date(d.updated_at).toLocaleTimeString()} (${d.author})`;
        }
      } catch (e) {}
    }

    async function sendClipboard() {
      const text = document.getElementById('clipboardText').value;
      try {
        const res = await fetch('/api/clipboard', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ text })
        });
        if (res.ok) {
          showToast('Clipboard synchronized!');
          loadClipboard();
        }
      } catch (e) {
        showToast('Sync error');
      }
    }

    function copyClipboard() {
      const text = document.getElementById('clipboardText').value;
      navigator.clipboard.writeText(text);
      showToast('Copied to clipboard');
    }

    // Formatting utilities
    function formatBytes(bytes) {
      if (!bytes || bytes === 0) return '0 B';
      const k = 1024;
      const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
      const i = Math.floor(Math.log(bytes) / Math.log(k));
      return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
    }

    function formatDuration(sec) {
      if (!sec || isNaN(sec)) return '0:00';
      const s = Math.floor(sec);
      const m = Math.floor(s / 60);
      const remS = s % 60;
      return `${m}:${remS.toString().padStart(2, '0')}`;
    }

    function escapeHtml(str) {
      return (str || '').replace(/[&<>"']/g, m => ({
        '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#39;'
      })[m]);
    }

    // Keyboard navigation
    window.addEventListener('keydown', (e) => {
      if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'k') {
        e.preventDefault();
        switchView('files');
        document.getElementById('searchInput').focus();
      }
      if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'u') {
        e.preventDefault();
        triggerFileInput();
      }
      if (e.key === 'Escape') {
        if (document.activeElement === document.getElementById('searchInput')) {
          clearSearch();
          document.getElementById('searchInput').blur();
          return;
        }
        closeModal('offlineModal');
        closeModal('phoneModal');
        closeModal('newFolderModal');
        closeModal('mobileActionSheet');
        closeModal('lightboxModal');
        closeModal('docModal');
        closeMediaModal();
      }
      if (e.key === ' ' && !['INPUT', 'TEXTAREA'].includes(document.activeElement?.tagName)) {
        const videoModal = document.getElementById('mediaModal');
        if (videoModal && videoModal.style.display === 'flex') {
          const v = document.getElementById('activeVideoPlayer');
          if (v) {
            e.preventDefault();
            if (v.paused) v.play(); else v.pause();
          }
        } else if (document.getElementById('audioPlayerHud')?.style.display === 'flex') {
          e.preventDefault();
          toggleAudioHudPlay();
        }
      }
      if (document.getElementById('lightboxModal').style.display === 'flex') {
        if (e.key === 'ArrowLeft') navPhoto(-1);
        if (e.key === 'ArrowRight') navPhoto(1);
      }
    });

    // Initial Start
    setViewMode(viewMode);
    updateSortHeaders();
    initNetwork();
    checkCellularRadio();
    initSse();
    loadCurrentFolder();
    loadDevices();
    setInterval(loadDevices, 6000);
  </script>
</body>
</html>"##;
