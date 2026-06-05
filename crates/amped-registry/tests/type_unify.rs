// M1 tests — manifest/registry type unification (amped-registry).
//
// AC-M1-11: the same logical manifest deserialised from TOML (via amped-plugin-api)
// and from JSON must produce equal PluginManifest values.
// Shared types live in amped-plugin-api; amped-registry reuses them.

use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap() // crates/
        .parent()
        .unwrap() // workspace root
        .to_path_buf()
}

// ---------------------------------------------------------------------------
// T-UNIFY-manifest-json-toml
// Given:  the canonical amped.plugin.toml AND an equivalent JSON representation
// When:   both are parsed into PluginManifest
// Then:   the two values are equal (same types, shared from amped-plugin-api)
// ---------------------------------------------------------------------------
#[test]
fn t_unify_manifest_json_toml_equal() {
    use amped_plugin_api::manifest::PluginManifest;

    // Parse from canonical TOML.
    let toml_path = workspace_root().join("plugins/easee/amped.plugin.toml");
    let toml_src = std::fs::read_to_string(&toml_path).expect("canonical TOML must be readable");
    let from_toml = PluginManifest::from_toml_str(&toml_src).expect("canonical TOML must parse Ok");

    // JSON equivalent — manually constructed to match the canonical TOML.
    // Using serde_json inline to keep the test self-contained and deterministic.
    let json_src = r#"{
  "schema_version": 1,
  "id": "com.easee.charger",
  "name": "Easee Charger",
  "version": "0.1.0",
  "author": "Amped",
  "description": "Read-only status for Easee chargers (power, session energy, mode).",
  "permissions": {
    "network": ["api.easee.com"],
    "secrets": ["easee.username", "easee.password", "easee.refresh_token"],
    "filesystem": []
  },
  "config_schema": {
    "fields": [
      { "key": "username",   "type": "string", "secret": true,  "required": true },
      { "key": "password",   "type": "string", "secret": true,  "required": true },
      { "key": "charger_id", "type": "string", "required": false, "help": "Empty = show all." }
    ]
  },
  "widgets": [
    {
      "id": "charger_status",
      "title": "Charger Status",
      "kind": "stat_card",
      "refresh": { "model": "interval", "secs": 30 },
      "fields": ["op_mode", "power_kw", "session_energy_kwh", "online"]
    }
  ]
}"#;

    let from_json = PluginManifest::from_json_str(json_src).expect("JSON manifest must parse Ok");

    assert_eq!(
        from_toml, from_json,
        "TOML and JSON parsed PluginManifest must be equal (type unification)"
    );
}
