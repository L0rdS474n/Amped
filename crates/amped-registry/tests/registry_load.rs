// M1 tests — registry loading (amped-registry).
//
// These tests reference types that do NOT exist yet in the skeleton stubs.
// They are intentionally in a "red / compile-fail" state until Gate-4
// (implementation) provides:
//   - amped_registry::local::{load_registry, Registry, RegistryEntry, RegistryError}
//   - RegistryError::MissingChecksum { id: String }
//   - RegistryError::InvalidChecksum { id: String }
//
// DO NOT implement any of those in the skeleton — the red state is correct and expected.

use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    // CARGO_MANIFEST_DIR is <workspace>/crates/amped-registry, so go up two levels.
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap() // crates/
        .parent()
        .unwrap() // workspace root
        .to_path_buf()
}

// ---------------------------------------------------------------------------
// T-REG-load-ok
// Given:  canonical registry/registry.json (single source of truth)
// When:   load_registry() parses it
// Then:   Ok with 1 entry; id=="com.easee.charger"; checksum is 64-hex string
// ---------------------------------------------------------------------------
#[test]
fn t_reg_load_ok() {
    use amped_registry::local::load_registry;

    let path = workspace_root().join("registry/registry.json");
    let src = std::fs::read_to_string(&path).expect("canonical registry must be readable");

    let registry = load_registry(&src).expect("canonical registry must load Ok");

    assert_eq!(registry.entries.len(), 1);

    let entry = &registry.entries[0];
    assert_eq!(entry.id, "com.easee.charger");

    // checksum must be present and exactly 64 lowercase hex characters
    let cs = &entry.checksum;
    assert_eq!(cs.len(), 64, "checksum must be 64 chars, got {}", cs.len());
    assert!(
        cs.chars().all(|c| c.is_ascii_hexdigit()),
        "checksum must be all hex digits, got: {cs}"
    );
}

// ---------------------------------------------------------------------------
// T-REG-missing-checksum
// Given:  registry JSON with checksum field absent
// When:   load_registry() parses it
// Then:   Err(RegistryError::MissingChecksum { id: "com.easee.charger" })
// ---------------------------------------------------------------------------
#[test]
fn t_reg_missing_checksum() {
    use amped_registry::local::{load_registry, RegistryError};

    let fixture = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/registry_missing_checksum.json"),
    )
    .unwrap();

    let err = load_registry(&fixture).unwrap_err();
    assert!(
        matches!(
            err,
            RegistryError::MissingChecksum { ref id }
            if id == "com.easee.charger"
        ),
        "expected MissingChecksum{{id:\"com.easee.charger\"}}, got {:?}",
        err
    );
}

// ---------------------------------------------------------------------------
// T-REG-bad-checksum
// Given:  registry JSON with checksum = "not-a-valid-sha256" (not 64 hex)
// When:   load_registry() parses it
// Then:   Err(RegistryError::InvalidChecksum { id: "com.easee.charger" })
// ---------------------------------------------------------------------------
#[test]
fn t_reg_bad_checksum() {
    use amped_registry::local::{load_registry, RegistryError};

    let fixture = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/registry_bad_checksum.json"),
    )
    .unwrap();

    let err = load_registry(&fixture).unwrap_err();
    assert!(
        matches!(
            err,
            RegistryError::InvalidChecksum { ref id }
            if id == "com.easee.charger"
        ),
        "expected InvalidChecksum{{id:\"com.easee.charger\"}}, got {:?}",
        err
    );
}

// ---------------------------------------------------------------------------
// T-REG-no-signature-ok
// Given:  registry JSON with no "signature" field (plan §6: optional in M1)
// When:   load_registry() parses it
// Then:   Ok — absent signature is acceptable now (additive later)
// ---------------------------------------------------------------------------
#[test]
fn t_reg_no_signature_ok() {
    use amped_registry::local::load_registry;

    // The canonical registry has no signature field — re-use it for this test.
    let path = workspace_root().join("registry/registry.json");
    let src = std::fs::read_to_string(&path).unwrap();

    // Confirm no "signature" key is present in the raw JSON.
    assert!(
        !src.contains("\"signature\""),
        "canonical registry must not contain a signature field (test pre-condition)"
    );

    // Must load Ok even without signature.
    let registry = load_registry(&src).expect("absent signature must be accepted");
    assert_eq!(registry.entries.len(), 1);
}

// ---------------------------------------------------------------------------
// T-REG-unknown-field-ok (forward-compatibility)
// Given:  registry JSON with an unknown extra field in an entry
// When:   load_registry() parses it
// Then:   Ok — registry entries allow unknown fields for forward-compat
// ---------------------------------------------------------------------------
#[test]
fn t_reg_unknown_field_ok() {
    use amped_registry::local::load_registry;

    let src = r#"{
  "entries": [
    {
      "id": "com.easee.charger",
      "name": "Easee Charger",
      "version": "0.1.0",
      "author": "Amped",
      "description": "Test.",
      "manifest_url": "https://example.com/manifest",
      "release_url": "https://example.com/release",
      "checksum": "0000000000000000000000000000000000000000000000000000000000000000",
      "screenshots": [],
      "min_amped_version": "0.1.0",
      "future_unknown_field": "forward-compat-value"
    }
  ]
}"#;

    let registry =
        load_registry(src).expect("unknown registry fields must be accepted (forward-compat)");
    assert_eq!(registry.entries[0].id, "com.easee.charger");
}
