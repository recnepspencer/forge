//! Replay-audience surface: exact re-exports from the Query engine.

/// Narrow cert-only replay basis capability marker.
///
/// ```
/// use worth_query_replay::facade::ReplayBasisCapability;
/// # fn _inspect(capability: &ReplayBasisCapability) {
/// #     let _ = capability;
/// # }
/// ```
pub use worth_query::facade::ReplayBasisCapability;
