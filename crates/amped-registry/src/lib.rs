//! `amped-registry` — local plugin registry loading and validation.
//!
//! Parses the `registry/registry.json` document into [`local::Registry`], enforcing the
//! M1 security invariant that every entry carries a well-formed `checksum`. Registry DTOs
//! reuse the shared types from `amped-plugin-api` where applicable (one-way dependency:
//! `amped-registry -> amped-plugin-api`).
//!
//! Out of scope for M1 (stubs): remote git-backed loading ([`remote`]) and signature
//! trust-store verification ([`signature`]).

pub mod checksum;
pub mod local;
pub mod remote;
pub mod signature;

// Convenience re-exports of the most-used public types.
pub use checksum::is_valid_checksum_shape;
pub use local::{
    load_registry, load_registry_from_path, load_registry_from_str, Registry, RegistryEntry,
    RegistryError,
};
