//! Checkpoint barriers for batched entity-tier evaluation.

/// Barrier where deferred Tier-0 refresh is allowed to run.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum CheckpointBarrier {
    PerMutation,
    PerOperation,
    PerCommit,
    OnDemandRead,
}
