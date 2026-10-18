//! Instance TTL management for `DripStream`.
//!
//! The threshold/extend-to values are protocol-wide policy and live in
//! [`drip_common::ttl`], so every contract renews instance storage on the same
//! schedule. This module only keeps the crate-local `ttl::bump` call sites
//! unchanged — it must not restate the constants (issue #649).

/// Maximum safe duration a stream may remain paused before the instance
/// storage TTL window is no longer sufficient to resume it safely.
///
/// A single `extend_ttl` bump only renews the instance record to
/// `TTL_EXTEND_TO` (200_000 ledgers). Any pause that exceeds the window can
/// leave the stream archived before a normal `resume()` or `force_cancel()`
/// call can run.
pub const MAX_PAUSE_SECS: u64 = 2_592_000; // 30 days

/// Extends the stream's instance storage TTL.
///
/// Re-exported from [`drip_common::ttl::bump_instance`] — see the module docs.
pub use drip_common::ttl::bump_instance as bump;

/// Low-cost keeper maintenance function for stream instances.
///
/// Extends the TTL of an active stream's instance storage when the remaining
/// TTL falls below `TTL_THRESHOLD`. This allows keepers or automated processes
/// to maintain stream instances with minimal gas cost, avoiding archival of
/// paused streams during long-duration pauses.
///
/// # Arguments
/// - `env` — Soroban environment
/// - `stream_id` — The unique identifier of the stream to bump
///
/// # Errors
/// - `StreamNotFound` if the stream instance does not exist
pub fn bump_stream_instance(env: &soroban_sdk::Env, stream_id: &str) -> Result<(), crate::errors::Error> {
    use crate::storage::DataKey;

    let key = DataKey::StreamInfo(stream_id.into());

    if !env.storage().instance().has(&key) {
        return Err(crate::errors::Error::StreamNotFound);
    }

    bump(env);
    Ok(())
}
