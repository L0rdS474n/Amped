// amped-registry: checksum shape validation.
//
// M1 scope is SHAPE-ONLY validation: a registry checksum must be exactly 64 lowercase
// hexadecimal characters (the textual form of a SHA-256 digest).
//
// DEFERRED to M2 (install-time): rejecting the all-zeros sentinel
// ("0000...0000", 64 zeros) and verifying the checksum against the actual downloaded
// binary hash. In M1 the canonical registry ships the all-zeros placeholder, so this
// module deliberately ACCEPTS it — it is a well-formed 64-hex string. The semantic
// "this digest must match a real artifact" check belongs to the install pipeline, not to
// registry parsing.

/// Number of hex characters in a SHA-256 digest string.
const SHA256_HEX_LEN: usize = 64;

/// Returns `true` if `checksum` is a well-formed SHA-256 hex digest: exactly 64
/// lowercase hexadecimal characters.
///
/// This is a SHAPE check only. It does NOT verify the digest against any artifact, and it
/// intentionally accepts the canonical all-zeros placeholder used in M1 (see module docs).
pub fn is_valid_checksum_shape(checksum: &str) -> bool {
    checksum.len() == SHA256_HEX_LEN
        && checksum
            .chars()
            .all(|c| c.is_ascii_digit() || ('a'..='f').contains(&c))
}
