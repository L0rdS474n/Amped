# Amped — UI

The Svelte 5 + TypeScript dashboard frontend for [Amped](../README.md).

- Browser preview (mock data): `npm run dev`
- Type-check: `npm run check`
- Production build: `npm run build`

Inside the Tauri desktop shell the plugin panel loads **real** plugin manifests
via the `list_plugins` IPC command; in a plain browser it falls back to mock
data. Run the full desktop app with `cargo tauri dev` from the repo root.
