// M1 tests — manifest parsing (amped-plugin-api).
//
// These tests reference types that do NOT exist yet in the skeleton stubs.
// They are intentionally in a "red / compile-fail" state until Gate-4
// (implementation) provides:
//   - amped_plugin_api::manifest::{PluginManifest, ManifestError}
//   - amped_plugin_api::widget::RefreshModel
//   - PluginManifest::from_toml_str(s: &str) -> Result<PluginManifest, ManifestError>
//
// DO NOT implement any of those in the skeleton — the red state is correct and expected.

use std::path::PathBuf;

/// Returns the absolute path to the workspace root, derived from this crate's
/// CARGO_MANIFEST_DIR (so no /home/<user>/ literals in code).
fn workspace_root() -> PathBuf {
    // CARGO_MANIFEST_DIR is set by Cargo to the crate root.
    // This crate lives at <workspace>/crates/amped-plugin-api, so we go up two levels.
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent() // crates/
        .unwrap()
        .parent() // workspace root
        .unwrap()
        .to_path_buf()
}

// ---------------------------------------------------------------------------
// T-API-manifest-ok
// Given:  the canonical plugins/easee/amped.plugin.toml (plan §5, single source of truth)
// When:   PluginManifest::from_toml_str() is called
// Then:   Ok with all expected field values
// ---------------------------------------------------------------------------
#[test]
fn t_api_manifest_ok() {
    use amped_plugin_api::manifest::PluginManifest;
    use amped_plugin_api::widget::RefreshModel;

    let toml_path = workspace_root()
        .join("plugins")
        .join("easee")
        .join("amped.plugin.toml");
    let src =
        std::fs::read_to_string(&toml_path).expect("canonical manifest fixture must be readable");

    let manifest: PluginManifest =
        PluginManifest::from_toml_str(&src).expect("canonical manifest must parse Ok");

    assert_eq!(manifest.schema_version, 1);
    assert_eq!(manifest.id, "com.easee.charger");
    assert_eq!(manifest.version, "0.1.0");

    // permissions
    assert_eq!(manifest.permissions.network, vec!["api.easee.com"]);
    assert_eq!(
        manifest.permissions.secrets,
        vec!["easee.username", "easee.password", "easee.refresh_token"]
    );
    assert!(manifest.permissions.filesystem.is_empty());

    // widgets
    assert_eq!(manifest.widgets.len(), 1);
    let w = &manifest.widgets[0];
    assert_eq!(w.id, "charger_status");
    assert_eq!(w.kind, "stat_card");
    assert!(
        matches!(w.refresh, RefreshModel::Interval { secs: 30 }),
        "expected Interval{{secs:30}}, got {:?}",
        w.refresh
    );
}

// ---------------------------------------------------------------------------
// T-API-manifest-roundtrip
// Given:  canonical manifest parsed to PluginManifest
// When:   serialised back to TOML then parsed again
// Then:   both parsed values are equal (value equality, not byte equality)
// ---------------------------------------------------------------------------
#[test]
fn t_api_manifest_roundtrip() {
    use amped_plugin_api::manifest::PluginManifest;

    let toml_path = workspace_root()
        .join("plugins")
        .join("easee")
        .join("amped.plugin.toml");
    let src = std::fs::read_to_string(&toml_path).unwrap();

    let first: PluginManifest = PluginManifest::from_toml_str(&src).unwrap();
    let serialised = first.to_toml_str().expect("serialise must succeed");
    let second: PluginManifest = PluginManifest::from_toml_str(&serialised).unwrap();

    assert_eq!(first, second, "round-trip must preserve value equality");
}

// ---------------------------------------------------------------------------
// T-API-manifest-missing — three cases (one per required field)
// Given:  TOML with schema_version / id / version removed
// When:   parsed
// Then:   Err(ManifestError::MissingField { .. }), never panic
// ---------------------------------------------------------------------------
#[test]
fn t_api_manifest_missing_schema_version() {
    use amped_plugin_api::manifest::{ManifestError, PluginManifest};

    let src = r#"
id      = "com.example.plugin"
name    = "Test"
version = "0.1.0"
author  = "Test"
description = "no schema_version"

[permissions]
network = []
secrets = []
filesystem = []

[[widgets]]
id = "w"
title = "W"
kind = "stat_card"
refresh = { model = "on_demand" }
fields = []
"#;
    let err = PluginManifest::from_toml_str(src).unwrap_err();
    assert!(
        matches!(err, ManifestError::MissingField { .. }),
        "expected MissingField, got {:?}",
        err
    );
}

#[test]
fn t_api_manifest_missing_id() {
    use amped_plugin_api::manifest::{ManifestError, PluginManifest};

    let fixture = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/manifest_missing_id.toml"),
    )
    .unwrap();
    let err = PluginManifest::from_toml_str(&fixture).unwrap_err();
    assert!(
        matches!(err, ManifestError::MissingField { .. }),
        "expected MissingField, got {:?}",
        err
    );
}

#[test]
fn t_api_manifest_missing_version() {
    use amped_plugin_api::manifest::{ManifestError, PluginManifest};

    let src = r#"
schema_version = 1
id      = "com.example.plugin"
name    = "Test"
author  = "Test"
description = "no version"

[permissions]
network = []
secrets = []
filesystem = []

[[widgets]]
id = "w"
title = "W"
kind = "stat_card"
refresh = { model = "on_demand" }
fields = []
"#;
    let err = PluginManifest::from_toml_str(src).unwrap_err();
    assert!(
        matches!(err, ManifestError::MissingField { .. }),
        "expected MissingField, got {:?}",
        err
    );
}

// ---------------------------------------------------------------------------
// T-API-manifest-missing-permissions => deny-all (AC-M1-2 edge)
// Given:  manifest with no [permissions] table
// When:   parsed
// Then:   Ok with permissions.network==[], secrets==[], filesystem==[]
// Decision (user-approved): missing [permissions] => deny-all default.
// ---------------------------------------------------------------------------
#[test]
fn t_api_manifest_missing_permissions_is_deny_all() {
    use amped_plugin_api::manifest::PluginManifest;

    let src = r#"
schema_version = 1
id      = "com.example.plugin"
name    = "Test"
version = "0.1.0"
author  = "Test"
description = "no permissions table"

[[widgets]]
id = "w"
title = "W"
kind = "stat_card"
refresh = { model = "on_demand" }
fields = []
"#;
    let manifest =
        PluginManifest::from_toml_str(src).expect("missing permissions must be deny-all Ok");
    assert!(
        manifest.permissions.network.is_empty(),
        "deny-all: network must be empty"
    );
    assert!(
        manifest.permissions.secrets.is_empty(),
        "deny-all: secrets must be empty"
    );
    assert!(
        manifest.permissions.filesystem.is_empty(),
        "deny-all: filesystem must be empty"
    );
}

// ---------------------------------------------------------------------------
// T-API-schema-version
// Given:  manifest with schema_version = 2
// When:   parsed
// Then:   Err(ManifestError::UnsupportedSchemaVersion { found: 2, supported: 1 })
// ---------------------------------------------------------------------------
#[test]
fn t_api_schema_version_unsupported() {
    use amped_plugin_api::manifest::{ManifestError, PluginManifest};

    let fixture = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/manifest_schema_v2.toml"),
    )
    .unwrap();
    let err = PluginManifest::from_toml_str(&fixture).unwrap_err();
    assert!(
        matches!(
            err,
            ManifestError::UnsupportedSchemaVersion {
                found: 2,
                supported: 1,
            }
        ),
        "expected UnsupportedSchemaVersion{{found:2,supported:1}}, got {:?}",
        err
    );
}

// ---------------------------------------------------------------------------
// T-API-unknown-field
// Given:  manifest with an unknown top-level key ("extra_field")
// When:   parsed (deny_unknown_fields is in effect)
// Then:   Err (serde rejects unknown field)
// ---------------------------------------------------------------------------
#[test]
fn t_api_unknown_field_rejected() {
    use amped_plugin_api::manifest::PluginManifest;

    let src = r#"
schema_version = 1
id      = "com.example.plugin"
name    = "Test"
version = "0.1.0"
author  = "Test"
description = "has unknown field"
extra_field = "this must cause an error"

[permissions]
network = []
secrets = []
filesystem = []

[[widgets]]
id = "w"
title = "W"
kind = "stat_card"
refresh = { model = "on_demand" }
fields = []
"#;
    assert!(
        PluginManifest::from_toml_str(src).is_err(),
        "unknown field must be rejected (deny_unknown_fields)"
    );
}

// ---------------------------------------------------------------------------
// T-API-no-panic
// Given:  empty string, truncated TOML, garbage bytes
// When:   parsed
// Then:   Err(...), never panics
// ---------------------------------------------------------------------------
#[test]
fn t_api_no_panic_empty_string() {
    use amped_plugin_api::manifest::PluginManifest;
    assert!(PluginManifest::from_toml_str("").is_err());
}

#[test]
fn t_api_no_panic_truncated_toml() {
    use amped_plugin_api::manifest::PluginManifest;
    let truncated = "schema_version = 1\nid = \"com.ea";
    assert!(PluginManifest::from_toml_str(truncated).is_err());
}

#[test]
fn t_api_no_panic_garbage() {
    use amped_plugin_api::manifest::PluginManifest;
    // non-TOML garbage — must return Err, not panic.
    // \xFF is not valid in a Rust &str literal; use a byte-escaped sequence that is
    // valid UTF-8 but guaranteed invalid TOML structure.
    assert!(PluginManifest::from_toml_str("$$$NOT TOML\x00\x7f<><>{}]]").is_err());
}

// ---------------------------------------------------------------------------
// T-API-network-allowlist-guard (Finding 4 — security adjudication)
// Given:  a manifest whose permissions.network contains a wildcard "*" OR an
//         empty / whitespace-only host
// When:   parsed
// Then:   Err(ManifestError::InvalidNetworkEntry { entry }), never a silent Ok
//
// Rationale: a wildcard host would defeat the network allow-list entirely, and an
// empty host is meaningless. This is a MINIMAL guard (only "*" and blank entries);
// full RFC-1123 hostname validation is intentionally deferred beyond M1.
// ---------------------------------------------------------------------------
#[test]
fn t_api_network_wildcard_rejected() {
    use amped_plugin_api::manifest::{ManifestError, PluginManifest};

    let src = r#"
schema_version = 1
id      = "com.example.plugin"
name    = "Test"
version = "0.1.0"
author  = "Test"
description = "wildcard network entry must be rejected"

[permissions]
network = ["*"]
secrets = []
filesystem = []

[[widgets]]
id = "w"
title = "W"
kind = "stat_card"
refresh = { model = "on_demand" }
fields = []
"#;
    let err = PluginManifest::from_toml_str(src).unwrap_err();
    assert!(
        matches!(err, ManifestError::InvalidNetworkEntry { ref entry } if entry == "*"),
        "expected InvalidNetworkEntry{{entry:\"*\"}}, got {:?}",
        err
    );
}

#[test]
fn t_api_network_blank_host_rejected() {
    use amped_plugin_api::manifest::{ManifestError, PluginManifest};

    let src = r#"
schema_version = 1
id      = "com.example.plugin"
name    = "Test"
version = "0.1.0"
author  = "Test"
description = "blank network entry must be rejected"

[permissions]
network = ["   "]
secrets = []
filesystem = []

[[widgets]]
id = "w"
title = "W"
kind = "stat_card"
refresh = { model = "on_demand" }
fields = []
"#;
    let err = PluginManifest::from_toml_str(src).unwrap_err();
    assert!(
        matches!(err, ManifestError::InvalidNetworkEntry { .. }),
        "expected InvalidNetworkEntry for whitespace-only host, got {:?}",
        err
    );
}
