//! `amped-plugin-api` — the Amped plugin contract surface.
//!
//! This crate is the leaf of the workspace dependency graph (it has no intra-workspace
//! dependencies). It defines the types that cross the host/plugin boundary:
//!
//! * [`manifest`] — the `PluginManifest` (shared by the TOML manifest and the JSON
//!   registry post), capability allow-lists, and the configuration schema.
//! * [`widget`] — widgets and their refresh cadence (`RefreshModel`).
//! * [`data`] — host-side mirrors of the WIT `widget-data` / `provenance` types.
//! * [`capability`] — the host-side HTTP request builder enforcing host-owned auth.
//!
//! The canonical wire contract lives in `wit/plugin.wit` (`amped:plugin@0.1.0`).

pub mod capability;
pub mod data;
pub mod manifest;
pub mod widget;

// Convenience re-exports of the most-used public types.
pub use capability::HostRequestBuilder;
pub use data::{Provenance, WidgetData};
pub use manifest::{ConfigField, ConfigSchema, ManifestError, Permissions, PluginManifest};
pub use widget::{RefreshError, RefreshModel, Widget};
