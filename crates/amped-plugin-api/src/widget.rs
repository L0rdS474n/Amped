// amped-plugin-api: widget and refresh-cadence types.
//
// A `Widget` is one dashboard tile a plugin renders. Its `refresh` field declares how
// often the host re-fetches its data, modelled by [`RefreshModel`].
//
// `RefreshModel` is parsed from a TOML table of the form:
//   { model = "interval", secs = 30 }   -> Interval { secs: 30 }
//   { model = "on_demand" }             -> OnDemand
//   { model = "push" }                  -> Push
//
// Negative cases are typed errors, never panics:
//   * `model = "interval"` without `secs`  -> error (interval requires a period)
//   * `secs = 0`                           -> RefreshError::NonPositiveInterval
//   * an unknown `model` value             -> error
//
// Validation lives in a single custom `Deserialize` impl so EVERY path that produces a
// `RefreshModel` — whether the standalone `from_toml_table_str` or a `RefreshModel`
// nested inside a `Widget` inside a `PluginManifest` — enforces the same invariants.

use serde::de::{self, Deserializer};
use serde::{Deserialize, Serialize};

/// Errors produced while parsing a [`RefreshModel`].
#[derive(Debug, thiserror::Error)]
pub enum RefreshError {
    /// `model = "interval"` was given with `secs = 0`.
    ///
    /// A zero-second poll is a footgun (busy-loop fetching), so it is rejected rather
    /// than silently accepted or clamped.
    #[error("interval refresh requires secs >= 1 (got 0)")]
    NonPositiveInterval,

    /// `model = "interval"` was given without a `secs` field.
    #[error("interval refresh requires a `secs` field")]
    MissingSecs,

    /// The `model` value was not one of `interval`, `on_demand`, or `push`.
    #[error("unknown refresh model `{model}` (expected interval, on_demand, or push)")]
    UnknownModel {
        /// The unrecognised model tag.
        model: String,
    },

    /// The input could not be parsed as a TOML table.
    #[error("failed to parse refresh table: {0}")]
    Toml(String),
}

/// How often the host re-fetches a widget's data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "model", rename_all = "snake_case")]
pub enum RefreshModel {
    /// Re-fetch every `secs` seconds. `secs` is always `>= 1`.
    Interval {
        /// Polling period in seconds (validated `>= 1`).
        secs: u64,
    },
    /// Re-fetch only when the user explicitly requests it.
    OnDemand,
    /// The plugin pushes updates; the host does not poll.
    Push,
}

impl RefreshModel {
    /// Parse a [`RefreshModel`] from a TOML table string such as:
    /// `model = "interval"\nsecs = 30`.
    ///
    /// Returns a typed [`RefreshError`] for unknown models, a missing `secs` on an
    /// interval, or `secs == 0`. Never panics.
    pub fn from_toml_table_str(s: &str) -> Result<Self, RefreshError> {
        // Reuse the validating `Deserialize` impl so this entry point and the
        // manifest-embedded path share identical rules.
        toml::from_str(s).map_err(refresh_error_from_toml)
    }
}

/// Translate a `toml` deserialisation error into a [`RefreshError`].
///
/// The custom `Deserialize` impl below raises its domain errors via
/// `serde::de::Error::custom`, so the message already carries the precise cause. We
/// reconstruct the typed variant from that message to keep the public error model rich
/// (so callers can `match` on `NonPositiveInterval` etc.), falling back to
/// [`RefreshError::Toml`] for genuine syntax errors.
fn refresh_error_from_toml(err: toml::de::Error) -> RefreshError {
    let msg = err.to_string();
    if msg.contains(NON_POSITIVE_INTERVAL_MSG) {
        RefreshError::NonPositiveInterval
    } else if msg.contains(MISSING_SECS_MSG) {
        RefreshError::MissingSecs
    } else if let Some(model) = extract_unknown_model(&msg) {
        RefreshError::UnknownModel { model }
    } else {
        RefreshError::Toml(msg)
    }
}

// Sentinel substrings emitted by the custom Deserialize impl. Kept as constants so the
// emit site and the re-parse site cannot drift apart.
const NON_POSITIVE_INTERVAL_MSG: &str = "interval refresh requires secs >= 1";
const MISSING_SECS_MSG: &str = "interval refresh requires a `secs` field";
const UNKNOWN_MODEL_PREFIX: &str = "unknown refresh model `";

/// Recover the offending model tag from an "unknown refresh model" diagnostic.
fn extract_unknown_model(msg: &str) -> Option<String> {
    let start = msg.find(UNKNOWN_MODEL_PREFIX)? + UNKNOWN_MODEL_PREFIX.len();
    let rest = &msg[start..];
    let end = rest.find('`')?;
    Some(rest[..end].to_string())
}

/// Validating `Deserialize` for [`RefreshModel`].
///
/// We deserialise into a permissive raw shape first (so the `secs`/model relationship can
/// be checked together), then enforce the domain invariants. This impl is the single
/// chokepoint for refresh validation across the whole crate.
impl<'de> Deserialize<'de> for RefreshModel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        /// Raw, unvalidated mirror of a refresh table.
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct RawRefresh {
            model: String,
            #[serde(default)]
            secs: Option<u64>,
        }

        let raw = RawRefresh::deserialize(deserializer)?;
        match raw.model.as_str() {
            "interval" => match raw.secs {
                None => Err(de::Error::custom(MISSING_SECS_MSG)),
                Some(0) => Err(de::Error::custom(NON_POSITIVE_INTERVAL_MSG)),
                Some(secs) => Ok(RefreshModel::Interval { secs }),
            },
            "on_demand" => Ok(RefreshModel::OnDemand),
            "push" => Ok(RefreshModel::Push),
            other => Err(de::Error::custom(format!("{UNKNOWN_MODEL_PREFIX}{other}`"))),
        }
    }
}

/// A single dashboard tile provided by a plugin.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Widget {
    /// Stable widget identifier within the plugin (e.g. `"charger_status"`).
    pub id: String,
    /// Human-readable title shown on the tile.
    #[serde(default)]
    pub title: String,
    /// Widget kind / renderer tag (e.g. `"stat_card"`).
    pub kind: String,
    /// Refresh cadence for this widget's data.
    pub refresh: RefreshModel,
    /// Data field names this widget displays.
    #[serde(default)]
    pub fields: Vec<String>,
}
