//! Checkpoint barriers for batched entity-tier evaluation.

/// Barrier where deferred Tier-0 refresh is allowed to run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CheckpointBarrier {
    PerMutation,
    PerOperation,
    PerCommit,
    OnDemandRead,
}
