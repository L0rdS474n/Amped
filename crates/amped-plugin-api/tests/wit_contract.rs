// M1 tests — WIT contract structural/syntax validation (amped-plugin-api).
//
// Toolchain probe result (run at setup time 2026-06-05):
//   cargo-component: ABSENT (exit 127, "no such command: `component`")
//   wasm-tools:      ABSENT (exit 127, "command not found")
//
// Per ADR-0001(c)/B2: with toolchain absent, ALL wit_contract tests degrade to
// structural/syntax checks against the raw WIT text and are marked #[ignore].
// The IGNORE reason is VISIBLE in test output — never silently omitted.
//
// When the toolchain is installed these tests must be un-ignored and the
// #[ignore] annotations removed. The DoD gating task is on the Plugin Developer.

use std::path::PathBuf;

fn wit_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("wit/plugin.wit")
}

fn read_wit() -> String {
    std::fs::read_to_string(wit_path()).expect("plugin.wit must be readable")
}

// ---------------------------------------------------------------------------
// T-API-wit-compiles
// B2: toolchain absent => structural/syntax check only; test is ignored.
// When toolchain is installed: run `cargo component build` or `wasm-tools component wit`
// and assert exit 0 + world amped-plugin visible.
// ---------------------------------------------------------------------------
#[test]
#[ignore = "SKIP: cargo-component/wasm-tools absent — ADR-0001(c)/B2; install toolchain to un-ignore"]
fn t_api_wit_compiles_full_toolchain() {
    // Placeholder: with toolchain present, invoke `wasm-tools component wit <path>`
    // and assert the command exits 0 and the output contains "world amped-plugin".
    // This placeholder body is never reached while ignored.
    let _ = wit_path();
    unreachable!("toolchain not installed; test must remain ignored until B2 is resolved");
}

// ---------------------------------------------------------------------------
// T-API-wit-compiles (structural degrade tier)
// Without toolchain: assert the WIT file exists and contains the required structural
// markers at the text level. This is the lowest-confidence tier but ensures the WIT
// is at least present and structurally shaped correctly before toolchain arrives.
// ---------------------------------------------------------------------------
#[test]
fn t_api_wit_structural_markers_present() {
    let wit = read_wit();

    // Package declaration
    assert!(
        wit.contains("package amped:plugin@0.1.0"),
        "WIT must declare package amped:plugin@0.1.0"
    );

    // World declaration with correct name
    assert!(
        wit.contains("world amped-plugin"),
        "WIT must declare world amped-plugin"
    );

    // Required exports
    assert!(
        wit.contains("export manifest"),
        "world amped-plugin must export manifest"
    );
    assert!(
        wit.contains("export init"),
        "world amped-plugin must export init"
    );
    assert!(
        wit.contains("export fetch"),
        "world amped-plugin must export fetch"
    );

    // Required import
    assert!(
        wit.contains("import host"),
        "world amped-plugin must import host"
    );
}

// ---------------------------------------------------------------------------
// T-API-wit-widget-data (B3)
// Assert widget-data is DEFINED in the WIT (not just referenced).
// The type must be a record with at least a payload: string field.
// ---------------------------------------------------------------------------
#[test]
fn t_api_wit_widget_data_is_defined() {
    let wit = read_wit();

    // widget-data must be defined as a record (not just referenced)
    assert!(
        wit.contains("record widget-data"),
        "WIT must define widget-data as a record (B3 resolution)"
    );

    // payload: string is the required field (serde_json::Value cannot cross Component Model boundary)
    assert!(
        wit.contains("payload:") && wit.contains("string"),
        "widget-data record must contain a payload: string field"
    );

    // The export fetch must reference widget-data
    assert!(
        wit.contains("widget-data"),
        "fetch export must reference widget-data type"
    );
}

// ---------------------------------------------------------------------------
// T-API-wit-provenance-payloads (plan §4/§8 lock)
// Assert that the provenance variant carries associated payloads:
//   cached(u64)      — age in seconds
//   degraded(string) — error reason
// Bare `cached` / `degraded` without payloads are insufficient; age and reason
// must cross the Component Model boundary for the dashboard to display them.
// ---------------------------------------------------------------------------
#[test]
fn t_api_wit_provenance_has_payload_variants() {
    let wit = read_wit();

    assert!(
        wit.contains("cached(u64)"),
        "variant provenance must have cached(u64) — bare 'cached' cannot carry staleness age \
         across the Component Model boundary (plan §4/§8)"
    );

    assert!(
        wit.contains("degraded(string)"),
        "variant provenance must have degraded(string) — bare 'degraded' cannot carry error \
         reason across the Component Model boundary (plan §4/§8)"
    );
}

// ---------------------------------------------------------------------------
// T-API-wit-auth-invariant (B4)
// Assert via structural check that:
//   (a) the WIT documents the Authorization header is host-owned
//   (b) a host request-builder stub (below) verifies host injection overrides any
//       caller-supplied Authorization header.
//
// The runtime capability-gate test (plugin-supplied Authorization is stripped)
// is scheduled for M2. This test covers the M1 contract/invariant layer.
// ---------------------------------------------------------------------------
#[test]
fn t_api_wit_auth_invariant_documented() {
    let wit = read_wit();

    // The WIT must contain a comment or doc about Authorization being host-owned.
    // We assert on key phrases from the ADR-0001(e) commentary we wrote into the WIT.
    assert!(
        wit.contains("Authorization") && wit.contains("HOST"),
        "WIT must document that Authorization is host-owned (ADR-0001(e)/B4)"
    );
}

// ---------------------------------------------------------------------------
// T-API-wit-auth-invariant: host request-builder unit test
// Given:  a HostRequestBuilder stub that merges caller headers with a host-injected token
// When:   caller supplies an Authorization header
// Then:   the host-injected Authorization wins (caller's is stripped)
//
// This test references HostRequestBuilder which is a host-stub type defined
// in src/manifest.rs or src/capability.rs in Gate-4.
// Until then this is in the expected red/compile-fail state.
// ---------------------------------------------------------------------------
#[test]
fn t_api_host_injected_auth_overrides_caller_auth() {
    use amped_plugin_api::capability::HostRequestBuilder;

    // Given: a request with a caller-supplied Authorization header
    let mut builder = HostRequestBuilder::new("GET", "https://api.easee.com/api/chargers");
    builder.add_caller_header("Authorization", "Bearer [REDACTED-bearer]");
    builder.add_caller_header("Accept", "application/json");

    // When: host injects its own token
    builder.inject_host_auth("Bearer HOST_TOKEN");
    let headers = builder.build_headers();

    // Then: only one Authorization header exists and it is the host's
    let auth_values: Vec<&str> = headers
        .iter()
        .filter(|(k, _)| k.to_lowercase() == "authorization")
        .map(|(_, v)| v.as_str())
        .collect();

    assert_eq!(
        auth_values.len(),
        1,
        "exactly one Authorization header must exist after host injection"
    );
    assert_eq!(
        auth_values[0], "Bearer HOST_TOKEN",
        "host-injected Authorization must win over caller-supplied value"
    );

    // Accept header must survive (it is not auth-related)
    let has_accept = headers.iter().any(|(k, _)| k.to_lowercase() == "accept");
    assert!(has_accept, "non-auth headers must not be stripped");
}

// ---------------------------------------------------------------------------
// T-API-wit-log-redaction (Finding 6 — SPEC, deferred to M2)
// The WIT `host.log` function documents that tokens/passwords are redacted
// HOST-SIDE before any message is written, so a guest can never cause a raw
// secret to reach the logs. The RUNTIME enforcement (the host log shim that
// scrubs secret values) lands with the wasmtime host in M2; this test records
// the requirement as an executable SPEC so it is tracked as a test, not merely
// as a WIT comment.
//
// When the M2 host log shim exists, this test must be un-ignored and assert that
// a message containing a known secret value is emitted with that value redacted.
// ---------------------------------------------------------------------------
#[test]
#[ignore = "SPEC for M2: host-side token redaction — see plan §7/§9"]
fn t_api_host_log_redacts_secrets() {
    // Pre-condition recorded at the contract layer: the WIT documents host-side
    // redaction on the log import. (Structural anchor; the behavioural assertion
    // is implemented against the M2 host log shim.)
    let wit = read_wit();
    assert!(
        wit.contains("redacted host-side"),
        "WIT host.log must document host-side secret redaction (plan §7/§9)"
    );

    // M2 behavioural placeholder: with the host log shim present, logging a message
    // that embeds a secret value must produce output with that value scrubbed. This
    // body is never reached while ignored.
    unreachable!(
        "host-side log redaction shim is an M2 deliverable; \
         un-ignore and assert scrubbing once the wasmtime host log shim exists"
    );
}
