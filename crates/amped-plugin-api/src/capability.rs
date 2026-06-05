// amped-plugin-api: capability types
//
// M1 content: HostRequestBuilder — host-side HTTP request assembler.
//   Enforces the security invariant declared in plugin.wit (ADR-0001(e)/AC-M1-10):
//   Authorization is HOST-OWNED. Plugin-supplied Authorization headers are stripped;
//   the host-injected value is the sole Authorization header in the final request.
//
// Gate-4 implementation adds: CapabilityGrants, capability gating logic, network
// allowlist enforcement, and the actual wasmtime capability-import linkage (M2).

/// Host-side HTTP request builder that enforces the auth-injection security invariant.
///
/// The plugin submits an `http-request` (WIT) with an optional `headers` list containing
/// only allowlisted non-auth headers. The host:
///   1. Accepts caller headers (Content-Type, Accept, etc.) via `add_caller_header`.
///   2. Strips any Authorization or Proxy-Authorization the caller may have included.
///   3. Injects its own token via `inject_host_auth` — this value wins unconditionally.
///   4. Returns the assembled header list via `build_headers`.
///
/// Enforced by test `t_api_host_injected_auth_overrides_caller_auth` in wit_contract.rs.
pub struct HostRequestBuilder {
    method: String,
    url: String,
    /// Caller-supplied headers; Authorization and Proxy-Authorization are stripped on build.
    caller_headers: Vec<(String, String)>,
    /// Host-injected Authorization value; set via inject_host_auth.
    host_auth: Option<String>,
}

impl HostRequestBuilder {
    /// Create a new builder for a request with the given HTTP method and URL.
    pub fn new(method: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            method: method.into(),
            url: url.into(),
            caller_headers: Vec::new(),
            host_auth: None,
        }
    }

    /// Accept a caller-supplied header. Authorization and Proxy-Authorization are stored
    /// but will be silently stripped in `build_headers`; they never reach the wire.
    pub fn add_caller_header(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.caller_headers.push((key.into(), value.into()));
    }

    /// Set the host-owned Authorization token. Must be called before `build_headers`.
    /// The supplied value replaces any caller-supplied Authorization header.
    pub fn inject_host_auth(&mut self, token: impl Into<String>) {
        self.host_auth = Some(token.into());
    }

    /// Assemble the final header list.
    ///
    /// Invariant: exactly one Authorization header exists if `inject_host_auth` was called,
    /// and it is the host's value. All caller-supplied Authorization / Proxy-Authorization
    /// headers are stripped regardless of casing.
    ///
    /// Non-auth caller headers (Content-Type, Accept, etc.) are preserved in order.
    pub fn build_headers(&self) -> Vec<(String, String)> {
        // Forbidden header names (case-insensitive).
        let is_auth_header = |k: &str| {
            let lower = k.to_lowercase();
            lower == "authorization" || lower == "proxy-authorization"
        };

        // Pass through caller headers, stripping any auth entries.
        let mut headers: Vec<(String, String)> = self
            .caller_headers
            .iter()
            .filter(|(k, _)| !is_auth_header(k))
            .cloned()
            .collect();

        // Inject the host-owned Authorization header last.
        if let Some(auth) = &self.host_auth {
            headers.push(("Authorization".to_string(), auth.clone()));
        }

        headers
    }

    /// The HTTP method for this request.
    pub fn method(&self) -> &str {
        &self.method
    }

    /// The target URL for this request.
    pub fn url(&self) -> &str {
        &self.url
    }
}
