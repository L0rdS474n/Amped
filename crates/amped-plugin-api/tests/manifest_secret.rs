// M1 tests — secret name-only invariant (amped-plugin-api).
//
// Security invariant (plan §7/§9, AC-M1-6):
//   permissions.secrets contains only identifier strings (key names).
//   The PluginManifest type MUST NOT contain a field that could hold a secret VALUE.
//   A manifest that tries to inline a raw secret value is rejected at the type/shape level.

use std::path::PathBuf;

// ---------------------------------------------------------------------------
// T-API-secret-name-only (part 1)
// Given:  canonical manifest
// When:   parsed
// Then:   permissions.secrets is Vec<String> (identifiers only); no value field exists
// ---------------------------------------------------------------------------
#[test]
fn t_api_secret_identifiers_only_in_canonical() {
    use amped_plugin_api::manifest::PluginManifest;

    // Read canonical manifest relative to workspace root via CARGO_MANIFEST_DIR.
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let canonical = manifest_dir
        .parent()
        .unwrap() // crates/
        .parent()
        .unwrap() // workspace root
        .join("plugins/easee/amped.plugin.toml");

    let src = std::fs::read_to_string(&canonical).unwrap();
    let manifest = PluginManifest::from_toml_str(&src).unwrap();

    // Each element in permissions.secrets is a plain identifier String.
    //
    // Finding 7 refinement: assert a strict identifier FORMAT rather than fragile
    // negative heuristics. A secret name must be non-empty and match
    // ^[a-z][a-z0-9._-]*$ — i.e. start with a lowercase letter and contain only
    // lowercase alphanumerics plus `.`, `_`, `-`. An inlined object/value (e.g. the
    // serialisation of `{ name = "x", value = "y" }`) cannot satisfy this format, so the
    // format check subsumes the old "no `{`" / "no 'value'" heuristics.
    for s in &manifest.permissions.secrets {
        assert!(
            is_secret_identifier(s),
            "secret name is not a valid identifier: {s:?}"
        );
    }
}

/// Returns true iff `name` matches `^[a-z][a-z0-9._-]*$` (non-empty).
///
/// Implemented without a regex dependency to keep the test crate dependency-free.
fn is_secret_identifier(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        // First char must be a lowercase ASCII letter.
        Some(c) if c.is_ascii_lowercase() => {}
        _ => return false,
    }
    // Remaining chars: lowercase alphanumeric or one of `.`, `_`, `-`.
    chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || matches!(c, '.' | '_' | '-'))
}

// ---------------------------------------------------------------------------
// T-API-secret-name-only (part 2)
// Given:  fixture with secrets = [{ name = "api_key", value = "hunter2" }]
// When:   parsed
// Then:   Err (shape mismatch — the type is Vec<String>, not Vec<{name,value}>)
// ---------------------------------------------------------------------------
#[test]
fn t_api_secret_inline_value_rejected() {
    use amped_plugin_api::manifest::PluginManifest;

    let fixture = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/manifest_secret_value.toml"),
    )
    .unwrap();

    assert!(
        PluginManifest::from_toml_str(&fixture).is_err(),
        "manifest that inlines a secret value must be rejected (shape error)"
    );
}

// ---------------------------------------------------------------------------
// T-API-secret-name-only (part 3, compile-time)
// Assert at the type level that PluginManifest / Permissions has no field
// that could carry a secret value. This test verifies the structural invariant
// by confirming the public API does not expose any "value" accessor on secrets.
//
// If the Gate-4 implementation accidentally adds a `value` field to a secret type,
// this test will fail to compile (because there is no such field) OR will fail
// at runtime if we check via serde round-trip. We use the runtime variant here.
// ---------------------------------------------------------------------------
#[test]
fn t_api_permissions_type_has_no_secret_value_field() {
    use amped_plugin_api::manifest::Permissions;

    // Construct Permissions directly with secrets as Vec<String>.
    // If Permissions ever introduces a different secrets type (e.g. Vec<SecretEntry>
    // with a value field), this construction site will fail to compile — which is
    // exactly the desired gate.
    let perms = Permissions {
        network: vec![],
        secrets: vec!["my.secret.name".to_string()],
        filesystem: vec![],
    };

    // The type system enforces that secrets is Vec<String>; a value field cannot
    // exist on the string element itself.
    assert_eq!(perms.secrets[0], "my.secret.name");
}
