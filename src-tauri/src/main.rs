//! Amped desktop shell (Tauri 2).
//!
//! M3: opens the dashboard window (the Svelte UI in `../ui`) and exposes a
//! read-only command that lists installed plugins from their on-disk manifests.
//! No plugin code is executed here — sandboxed WASM plugin execution is M2
//! (amped-host). Easee data shown in the UI remains MOCK until M4/M5.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;

use amped_plugin_api::manifest::PluginManifest;
use serde::Serialize;

/// Plugin manifests live in `<repo>/plugins`, resolved relative to this crate at
/// compile time so the command is independent of the runtime working directory.
fn plugins_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("plugins")
}

#[derive(Serialize)]
struct Perms {
    network: Vec<String>,
    secrets: Vec<String>,
    filesystem: Vec<String>,
}

/// The subset of a plugin manifest the dashboard renders. Matches `PluginInfo`
/// in `ui/src/lib/contract.ts` field-for-field.
#[derive(Serialize)]
struct PluginInfo {
    id: String,
    name: String,
    version: String,
    author: String,
    description: String,
    permissions: Perms,
}

impl From<PluginManifest> for PluginInfo {
    fn from(m: PluginManifest) -> Self {
        PluginInfo {
            id: m.id,
            name: m.name,
            version: m.version,
            author: m.author,
            description: m.description,
            permissions: Perms {
                network: m.permissions.network,
                secrets: m.permissions.secrets,
                filesystem: m.permissions.filesystem,
            },
        }
    }
}

/// Read every `plugins/*/amped.plugin.toml` and return the installed plugins,
/// sorted by name. Read-only. On failure returns a message string the UI shows
/// as a degraded state rather than crashing.
#[tauri::command]
fn list_plugins() -> Result<Vec<PluginInfo>, String> {
    let dir = plugins_dir();
    let entries = std::fs::read_dir(&dir).map_err(|e| format!("read {}: {e}", dir.display()))?;

    let mut plugins = Vec::new();
    for entry in entries.flatten() {
        let manifest = entry.path().join("amped.plugin.toml");
        if !manifest.exists() {
            continue;
        }
        let parsed = PluginManifest::from_toml_file(&manifest)
            .map_err(|e| format!("{}: {e}", manifest.display()))?;
        plugins.push(PluginInfo::from(parsed));
    }
    plugins.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(plugins)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![list_plugins])
        .run(tauri::generate_context!())
        .expect("error while running the Amped application");
}
