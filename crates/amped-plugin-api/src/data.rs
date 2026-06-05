// amped-plugin-api: host-side mirrors of the WIT widget-data types.
//
// These Rust types mirror the `provenance` variant and `widget-data` record defined in
// `wit/plugin.wit`. They are the host-side representation used BEFORE encoding to / AFTER
// decoding from the Component Model boundary.
//
// Boundary note: across the Component Model the WIT `widget-data.payload` is a JSON
// `string` (a `serde_json::Value` cannot cross the boundary). Host-side we keep the
// decoded `serde_json::Value` so the dashboard can validate it against the widget schema.
// The locked payload semantics (M1 contract — see plugin.wit) are reproduced on
// [`Provenance`] below and must not change without an ADR + version bump.

use serde::{Deserialize, Serialize};

/// Source classification for a piece of widget data.
///
/// Mirrors the WIT `variant provenance`. Payload semantics are locked (plugin.wit §M1):
/// * `Live` — data arrived from the live API this fetch cycle; no payload.
/// * `Mock` — data is synthesized (demo/dev mode); no payload.
/// * `Cached(u64)` — live fetch failed; returning a cached value; payload is the age in
///   SECONDS since that value was originally fetched.
/// * `Degraded(String)` — plugin is in an error state; payload is a human-readable reason
///   shown in the widget error card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Provenance {
    /// Fresh data from the live source this cycle.
    Live,
    /// Synthesized demo/dev data.
    Mock,
    /// Stale cached data; the `u64` is the age in seconds since it was fetched.
    Cached(u64),
    /// No usable data; the `String` is a human-readable error reason.
    Degraded(String),
}

/// Data produced by a plugin for one widget.
///
/// Host-side mirror of the WIT `record widget-data`. The `payload` is held as a decoded
/// [`serde_json::Value`]; it is encoded to / decoded from a JSON string when crossing the
/// Component Model boundary.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WidgetData {
    /// The widget this data belongs to.
    pub widget_id: String,
    /// Where this data came from (and its staleness/error context).
    pub provenance: Provenance,
    /// Fetch timestamp in Unix epoch milliseconds (deterministic; no guest wall-clock).
    pub fetched_at_ms: u64,
    /// The widget payload as structured JSON (validated against the widget schema host-side).
    pub payload: serde_json::Value,
}
