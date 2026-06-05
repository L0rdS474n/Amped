<div align="center">

# ⚡ Amped

**A plugin-driven green-tech dashboard for kiosk & wall-mounted displays.**

[![CI](https://github.com/L0rdS474n/Amped/actions/workflows/ci.yml/badge.svg)](https://github.com/L0rdS474n/Amped/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![Rust](https://img.shields.io/badge/rust-2021-orange.svg)](https://www.rust-lang.org/)

</div>

> [!WARNING]
> **Early work in progress.** Amped is at milestones **M0–M1** (workspace + plugin
> contract). There is **no live data yet** — the Easee integration ships as a
> read-only *placeholder* and any displayed data is clearly marked **MOCK/DEMO**.
> Do not deploy this expecting live charger data until milestone **M5**.

## What is Amped?

Amped is a modern, dark-mode-first **energy dashboard** designed to run fullscreen on a
wall-mounted screen, tablet, or small energy-dashboard computer. It is **plugin-first**:
the application core hard-codes *no* vendor. Plugins supply the data — the first being a
read-only **Easee** EV-charger integration, with solar production, home batteries,
electricity prices, smart meters, grid usage and weather planned to follow.

The defining design choice: **plugins are untrusted by default.** Every plugin runs as a
sandboxed **WebAssembly component** (Wasmtime, Component Model) with **no ambient
authority** — it cannot touch the network, filesystem, or secrets directly. The host
mediates every capability, owns all credentials, and injects authentication so a plugin
never sees a raw token.

## Architecture at a glance

A Rust [Cargo workspace](Cargo.toml) with a strict one-way dependency direction:

| Crate | Role |
|---|---|
| `amped-plugin-api` | Stable plugin contract — the [WIT](crates/amped-plugin-api/wit/plugin.wit) world + manifest/widget types. The dependency-free leaf. |
| `amped-core` | Domain models, plugin trait, event bus. Knows nothing about Tauri. |
| `amped-registry` | Reads & validates plugin manifests; local now, Git-backed later. Mandatory checksums. |
| `amped-host` | Plugin runtime: loads, isolates (Wasmtime), schedules refresh. |
| `amped-plugin-easee` | First plugin (read-only). Depends *only* on the contract crate. |
| `src-tauri` | Thin Tauri 2 shell: window, kiosk, IPC. |
| `ui/` *(M3)* | Svelte 5 + Vite + TypeScript dashboard. |

```
amped-plugin-api ◄── amped-core ◄── amped-host ◄── src-tauri ──► ui
        ▲                                ▲
        └── amped-registry ─────────────┘
        ▲
        └── amped-plugin-easee  (plugins see the contract only)
```

Key decisions are recorded as ADRs under [`docs/adr/`](docs/adr/).

## Security model

- **Untrusted plugins.** No network / filesystem / secret / system access unless declared
  in the plugin manifest *and* approved. The host enforces a capability allow-list.
- **Host owns credentials.** Secrets live in the OS keychain; the host performs
  authentication and injects `Authorization` — plugins never receive raw long-lived tokens.
- **No secret logging.** Tokens, passwords and PII are redacted host-side.

See [SECURITY.md](SECURITY.md) to report a vulnerability.

## Build & test

Requires a recent stable Rust toolchain (Rust 2021 edition; `rustup` recommended).

```sh
cargo build --workspace          # build everything
cargo test  --workspace          # run the test suite (M1: 33 tests)
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
```

> The WebAssembly component toolchain (`cargo-component` / `wasm-tools`) is **optional**
> for M0–M1 — the WIT contract only needs to exist. WIT-compilation tests self-skip with a
> visible reason when the toolchain is absent (see ADR-0001(c)). It becomes required at M2.

## Roadmap

| Milestone | Scope | Status |
|---|---|---|
| **M0** | Cargo workspace scaffold | ✅ |
| **M1** | WIT plugin contract + manifest/registry parsing (test-first) | ✅ |
| **M2** | Wasmtime host + capability gate + local registry loading | ⏳ |
| **M3** | Tauri shell + Svelte dashboard (fullscreen kiosk) | ⏳ |
| **M4** | Easee plugin (WASM, **mock** data, degraded-state handling) | ⏳ |
| **M5** | Easee live wiring (gated behind API verification + OS keychain) | ⏳ |

## Contributing

Contributions are welcome — please read [CONTRIBUTING.md](CONTRIBUTING.md) first. In short:
work on a feature branch (never push to `main`), keep changes test-first and
single-objective, and open a pull request. CI must be green and at least one review is
required before merge.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option. Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
