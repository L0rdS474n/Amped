// amped-plugin-api: plugin manifest types and parsing.
//
// The `PluginManifest` is the single source of truth shared by the TOML plugin
// manifest (`amped.plugin.toml`) and the JSON registry post — both deserialise into
// the SAME struct (AC-M1-11 type unification). Parsing is total: every public entry
// point returns `Result<_, ManifestError>` and never panics on malformed input.
//
// Security posture (STRICT, user-approved):
//   * Unknown fields are rejected (`#[serde(deny_unknown_fields)]`) — typos and
//     forward-incompatible keys fail closed.
//   * A missing `[permissions]` table => deny-all default (empty allow-lists). A
//     plugin can never gain ambient authority by omitting the table.
//   * `permissions.secrets` is `Vec<String>` (identifier names only). The type cannot
//     hold a secret VALUE; a manifest that inlines `{ name, value }` is a shape error
//     (AC-M1-6).
//   * Network allow-list entries are validated: wildcard `"*"` and empty/whitespace
//     hosts are rejected (Finding 4). This is a minimal guard, NOT full RFC-1123
//     hostname validation (deferred).

use serde::{Deserialize, Serialize};

use crate::widget::Widget;

/// The only manifest schema version this build understands.
const SUPPORTED_SCHEMA_VERSION: u32 = 1;

/// Errors produced while parsing or validating a [`PluginManifest`].
///
/// All variants are typed so tests and callers can `match` on the cause rather than
/// parsing error strings.
#[derive(Debug, thiserror::Error)]
pub enum ManifestError {
    /// A required top-level field (`schema_version`, `id`, or `version`) was absent.
    #[error("manifest is missing required field `{field}`")]
    MissingField {
        /// Name of the missing field.
        field: String,
    },

    /// `schema_version` was present but is not a version this build supports.
    #[error("unsupported manifest schema_version {found} (this build supports {supported})")]
    UnsupportedSchemaVersion {
        /// The version found in the manifest.
        found: u32,
        /// The version this build supports.
        supported: u32,
    },

    /// A `permissions.network` entry was a wildcard or empty/whitespace host.
    ///
    /// Minimal guard only (Finding 4): rejects `"*"` and empty/whitespace entries.
    /// Full hostname validation is intentionally out of scope for M1.
    #[error("invalid network allow-list entry `{entry}` (wildcard and empty hosts are forbidden)")]
    InvalidNetworkEntry {
        /// The offending entry.
        entry: String,
    },

    /// The input could not be parsed as TOML (truncated, garbage, wrong shape, or an
    /// inlined secret value, etc.). Carries the underlying parser message for context.
    #[error("failed to parse manifest TOML: {0}")]
    Toml(String),

    /// The input could not be parsed as JSON.
    #[error("failed to parse manifest JSON: {0}")]
    Json(String),

    /// The manifest could not be serialised back to TOML (round-trip support).
    #[error("failed to serialise manifest to TOML: {0}")]
    Serialize(String),

    /// The manifest file could not be read from disk.
    #[error("failed to read manifest file: {0}")]
    Io(String),
}

/// Capability allow-lists declared by a plugin.
///
/// Every list is deny-by-default: an empty list grants nothing. A missing
/// `[permissions]` table deserialises to this all-empty value (see
/// [`PluginManifest::from_toml_str`]).
///
/// `secrets` holds only identifier NAMES (e.g. `"easee.username"`). It is deliberately
/// `Vec<String>` so the manifest type can never carry a secret VALUE.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct Permissions {
    /// Hostnames the plugin may reach over the network (deny-all when empty).
    #[serde(default)]
    pub network: Vec<String>,
    /// Secret identifier names the plugin may request from the host keyring.
    ///
    /// Names only — never values. The host owns and injects the actual secrets.
    #[serde(default)]
    pub secrets: Vec<String>,
    /// Filesystem paths the plugin may access (deny-all when empty).
    #[serde(default)]
    pub filesystem: Vec<String>,
}

/// A single configuration field a plugin exposes to the user.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConfigField {
    /// The configuration key (e.g. `"username"`).
    pub key: String,
    /// The value type as a string tag (e.g. `"string"`).
    ///
    /// `type` is a Rust keyword, so the Rust field is `r#type` and serialises as `type`.
    #[serde(rename = "type")]
    pub r#type: String,
    /// Whether this field holds a secret (stored in the keyring, never logged).
    #[serde(default)]
    pub secret: bool,
    /// Whether the user must supply this field.
    #[serde(default)]
    pub required: bool,
    /// Optional human-readable help text shown in the configuration UI.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub help: Option<String>,
}

/// The configuration schema a plugin declares.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ConfigSchema {
    /// The configuration fields, in declaration order.
    #[serde(default)]
    pub fields: Vec<ConfigField>,
}

/// A parsed plugin manifest.
///
/// Shared by the TOML manifest and the JSON registry post (AC-M1-11): both formats
/// deserialise into this exact type, so a TOML-parsed and JSON-parsed manifest compare
/// equal field-for-field.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PluginManifest {
    /// Manifest schema version. Only `1` is accepted by this build.
    pub schema_version: u32,
    /// Reverse-DNS plugin identifier (e.g. `"com.easee.charger"`).
    pub id: String,
    /// Human-readable plugin name.
    #[serde(default)]
    pub name: String,
    /// Plugin semantic version string (e.g. `"0.1.0"`).
    pub version: String,
    /// Plugin author.
    #[serde(default)]
    pub author: String,
    /// Short description shown in the dashboard / registry.
    #[serde(default)]
    pub description: String,
    /// Capability allow-lists. A missing table => deny-all (all lists empty).
    #[serde(default)]
    pub permissions: Permissions,
    /// Optional configuration schema.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config_schema: Option<ConfigSchema>,
    /// Widgets this plugin provides.
    #[serde(default)]
    pub widgets: Vec<Widget>,
}

impl PluginManifest {
    /// Parse a manifest from a TOML string.
    ///
    /// Returns:
    /// * [`ManifestError::MissingField`] if `schema_version`, `id`, or `version` is absent.
    /// * [`ManifestError::UnsupportedSchemaVersion`] if `schema_version != 1`.
    /// * [`ManifestError::InvalidNetworkEntry`] if a network entry is `"*"` or blank.
    /// * [`ManifestError::Toml`] for any other shape/syntax error (including an inlined
    ///   secret value, an unknown field, or truncated/garbage input).
    ///
    /// Never panics.
    pub fn from_toml_str(s: &str) -> Result<Self, ManifestError> {
        let manifest: PluginManifest = toml::from_str(s).map_err(map_toml_de_error)?;
        manifest.validate()?;
        Ok(manifest)
    }

    /// Parse a manifest from a JSON string. Same validation rules as
    /// [`PluginManifest::from_toml_str`]. Never panics.
    pub fn from_json_str(s: &str) -> Result<Self, ManifestError> {
        let manifest: PluginManifest = serde_json::from_str(s).map_err(map_json_de_error)?;
        manifest.validate()?;
        Ok(manifest)
    }

    /// Read and parse a manifest from a TOML file on disk.
    pub fn from_toml_file(path: impl AsRef<std::path::Path>) -> Result<Self, ManifestError> {
        let src = std::fs::read_to_string(path).map_err(|e| ManifestError::Io(e.to_string()))?;
        Self::from_toml_str(&src)
    }

    /// Serialise this manifest back to a TOML string (round-trip support).
    ///
    /// Re-parsing the output yields a value equal to `self` (AC-M1-1 round-trip).
    pub fn to_toml_str(&self) -> Result<String, ManifestError> {
        toml::to_string(self).map_err(|e| ManifestError::Serialize(e.to_string()))
    }

    /// Apply post-deserialisation semantic validation.
    ///
    /// serde guarantees the struct shape; this enforces the value-level invariants that
    /// serde alone cannot (supported schema version, network allow-list hygiene).
    fn validate(&self) -> Result<(), ManifestError> {
        if self.schema_version != SUPPORTED_SCHEMA_VERSION {
            return Err(ManifestError::UnsupportedSchemaVersion {
                found: self.schema_version,
                supported: SUPPORTED_SCHEMA_VERSION,
            });
        }

        for entry in &self.permissions.network {
            // Minimal guard (Finding 4): reject the wildcard and empty/whitespace hosts.
            // This is intentionally NOT full RFC-1123 hostname validation.
            if entry == "*" || entry.trim().is_empty() {
                return Err(ManifestError::InvalidNetworkEntry {
                    entry: entry.clone(),
                });
            }
        }

        Ok(())
    }
}

/// Map a TOML deserialisation error to a typed [`ManifestError`].
///
/// serde reports a missing required field with a "missing field `x`" message. We detect
/// that case and surface it as [`ManifestError::MissingField`] so callers get a typed,
/// field-named error instead of an opaque parse string.
fn map_toml_de_error(err: toml::de::Error) -> ManifestError {
    let msg = err.to_string();
    if let Some(field) = extract_missing_field(&msg) {
        return ManifestError::MissingField { field };
    }
    ManifestError::Toml(msg)
}

/// Map a JSON deserialisation error to a typed [`ManifestError`].
fn map_json_de_error(err: serde_json::Error) -> ManifestError {
    let msg = err.to_string();
    if let Some(field) = extract_missing_field(&msg) {
        return ManifestError::MissingField { field };
    }
    ManifestError::Json(msg)
}

/// Extract the field name from a serde "missing field" diagnostic.
///
/// Both `toml` and `serde_json` render this as: `missing field \`<name>\``. We parse the
/// backtick-delimited name. Returns `None` if the message is not a missing-field error.
fn extract_missing_field(msg: &str) -> Option<String> {
    const MARKER: &str = "missing field `";
    let start = msg.find(MARKER)? + MARKER.len();
    let rest = &msg[start..];
    let end = rest.find('`')?;
    Some(rest[..end].to_string())
}
