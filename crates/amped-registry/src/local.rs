// amped-registry: local registry loading and validation.
//
// The registry is a JSON document (`registry/registry.json`) listing installable plugins.
// `load_registry` parses it and enforces the M1 security invariant: every entry MUST
// carry a well-formed `checksum`.
//
// Forward-compatibility posture (user-approved): registry entries ALLOW unknown fields,
// so a newer registry that adds fields still loads on an older client. (This is the
// opposite of the manifest, which denies unknown fields — the manifest is an authored,
// validated artifact; the registry must tolerate additive growth.)
//
// Checksum posture (Finding 5): validation is SHAPE-ONLY in M1 — a 64-lowercase-hex
// string, INCLUDING the all-zeros placeholder. Binary-hash verification and all-zeros
// rejection are deferred to the M2 install pipeline (see `checksum.rs`).

use serde::{Deserialize, Serialize};

use crate::checksum::is_valid_checksum_shape;

/// Errors produced while loading or validating a [`Registry`].
#[derive(Debug, thiserror::Error)]
pub enum RegistryError {
    /// An entry was missing the mandatory `checksum` field.
    #[error("registry entry `{id}` is missing the mandatory checksum field")]
    MissingChecksum {
        /// The `id` of the offending entry.
        id: String,
    },

    /// An entry's `checksum` was present but not a well-formed 64-char hex digest.
    #[error("registry entry `{id}` has an invalid checksum (expected 64 lowercase hex chars)")]
    InvalidChecksum {
        /// The `id` of the offending entry.
        id: String,
    },

    /// The registry document could not be parsed as JSON.
    #[error("failed to parse registry JSON: {0}")]
    Json(String),

    /// The registry file could not be read from disk.
    #[error("failed to read registry file: {0}")]
    Io(String),
}

/// One installable plugin entry in the registry.
///
/// Unknown fields are tolerated (forward-compat): a future field added by a newer
/// registry is silently ignored by an older client rather than rejected.
///
/// `checksum` is mandatory and guaranteed well-formed once an entry reaches this public
/// type (validation happens during [`load_registry`]).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistryEntry {
    /// Reverse-DNS plugin identifier (e.g. `"com.easee.charger"`).
    pub id: String,
    /// Human-readable plugin name.
    #[serde(default)]
    pub name: String,
    /// Plugin version string.
    #[serde(default)]
    pub version: String,
    /// Plugin author.
    #[serde(default)]
    pub author: String,
    /// Short description.
    #[serde(default)]
    pub description: String,
    /// URL of the plugin manifest.
    #[serde(default)]
    pub manifest_url: String,
    /// URL of the release artifact (the `.wasm` component).
    #[serde(default)]
    pub release_url: String,
    /// SHA-256 checksum of the release artifact, as 64 lowercase hex chars.
    ///
    /// Mandatory and shape-validated during loading.
    pub checksum: String,
    /// Screenshot URLs shown in the dashboard.
    #[serde(default)]
    pub screenshots: Vec<String>,
    /// Minimum Amped version required to run this plugin.
    #[serde(default)]
    pub min_amped_version: String,
    /// Optional detached signature over the entry/artifact.
    ///
    /// Absent in M1 (signature trust is deferred); accepted when present for forward-compat.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

/// A parsed plugin registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Registry {
    /// The installable plugin entries.
    pub entries: Vec<RegistryEntry>,
}

/// Raw, pre-validation mirror of a registry document.
///
/// `checksum` is captured as `Option<String>` so an absent checksum surfaces as the typed
/// [`RegistryError::MissingChecksum`] (carrying the entry `id`) rather than an opaque
/// serde "missing field" error that would lose the id. Unknown fields are tolerated
/// because `#[serde(deny_unknown_fields)]` is deliberately NOT applied here.
#[derive(Deserialize)]
struct RawRegistry {
    #[serde(default)]
    entries: Vec<RawEntry>,
}

/// Raw, pre-validation mirror of a single entry.
#[derive(Deserialize)]
struct RawEntry {
    id: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    version: String,
    #[serde(default)]
    author: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    manifest_url: String,
    #[serde(default)]
    release_url: String,
    /// Optional at the parser level so a MISSING checksum is a typed domain error, not a
    /// serde shape error. Validated to be present-and-well-formed in `load_registry`.
    #[serde(default)]
    checksum: Option<String>,
    #[serde(default)]
    screenshots: Vec<String>,
    #[serde(default)]
    min_amped_version: String,
    #[serde(default)]
    signature: Option<String>,
}

/// Parse a [`Registry`] from a JSON string and validate every entry's checksum.
///
/// Despite the name this takes the registry CONTENTS (the JSON string), matching the M1
/// test surface. Use [`load_registry_from_path`] to read from disk first.
///
/// Returns:
/// * [`RegistryError::MissingChecksum`] if any entry omits `checksum`.
/// * [`RegistryError::InvalidChecksum`] if any entry's `checksum` is not 64 lowercase hex.
/// * [`RegistryError::Json`] if the document is not valid JSON.
///
/// Never panics.
pub fn load_registry(src: &str) -> Result<Registry, RegistryError> {
    let raw: RawRegistry =
        serde_json::from_str(src).map_err(|e| RegistryError::Json(e.to_string()))?;

    let mut entries = Vec::with_capacity(raw.entries.len());
    for raw_entry in raw.entries {
        entries.push(validate_entry(raw_entry)?);
    }

    Ok(Registry { entries })
}

/// Convenience alias for [`load_registry`] with an explicit "from string" name.
///
/// Provided for call-site clarity where the input is plainly an in-memory string.
pub fn load_registry_from_str(src: &str) -> Result<Registry, RegistryError> {
    load_registry(src)
}

/// Read a registry file from disk and parse it via [`load_registry`].
pub fn load_registry_from_path(
    path: impl AsRef<std::path::Path>,
) -> Result<Registry, RegistryError> {
    let src = std::fs::read_to_string(path).map_err(|e| RegistryError::Io(e.to_string()))?;
    load_registry(&src)
}

/// Validate a single raw entry and promote it to a public [`RegistryEntry`].
///
/// Enforces the mandatory, well-formed checksum invariant. The entry `id` is always
/// available here, so both failure modes name the offending entry.
fn validate_entry(raw: RawEntry) -> Result<RegistryEntry, RegistryError> {
    let checksum = match raw.checksum {
        None => {
            return Err(RegistryError::MissingChecksum { id: raw.id });
        }
        Some(cs) => cs,
    };

    if !is_valid_checksum_shape(&checksum) {
        return Err(RegistryError::InvalidChecksum { id: raw.id });
    }

    Ok(RegistryEntry {
        id: raw.id,
        name: raw.name,
        version: raw.version,
        author: raw.author,
        description: raw.description,
        manifest_url: raw.manifest_url,
        release_url: raw.release_url,
        checksum,
        screenshots: raw.screenshots,
        min_amped_version: raw.min_amped_version,
        signature: raw.signature,
    })
}
