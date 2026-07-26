/** True when running inside the Tauri desktop shell (either bundle target). */
export const isTauri = typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window

/** On-site download page (download.html) listing the latest desktop installers,
 * which are published to GitHub Releases by .github/workflows/release-desktop.yml. */
export const DESKTOP_DOWNLOAD_URL = '/download'
