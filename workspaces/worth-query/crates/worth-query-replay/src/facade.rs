//! Replay-audience surface: exact re-exports from the Query engine.

/// Narrow cert-only scoped replay basis.
///
/// ```
/// use worth_query_replay::facade::ScopedReplayBasis;
/// # fn _inspect(capability: &ScopedReplayBasis) {
/// #     let _ = capability;
/// # }
/// ```
pub use worth_query::facade::foundation::ScopedReplayBasis;
