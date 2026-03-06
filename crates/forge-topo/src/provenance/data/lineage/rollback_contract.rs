//! Rollback strategy contract for lineage/replay semantics.

use serde::{Deserialize, Serialize};

/// Contract schema version for rollback semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RollbackContractVersion {
    V1,
}

/// Rollback execution strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RollbackStrategy {
    /// Restore from an immutable checkpoint and emit `EntityReverted` lineage events.
    SnapshotRestore,
    /// Apply inverse operations and emit compensating lineage events.
    InverseReplay,
}

/// Lineage rollback event semantics for this contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RollbackLineageMode {
    /// Rollback events are represented as `LineageEvent::EntityReverted`.
    Reverted,
    /// Rollback events are represented as compensating events.
    Compensated,
}

/// Active rollback contract parameters for this crate version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackContract {
    pub version: RollbackContractVersion,
    pub strategy: RollbackStrategy,
    pub lineage_mode: RollbackLineageMode,
}

impl RollbackContract {
    /// Active rollback contract used by `forge-topo`.
    pub const CURRENT: Self = Self {
        version: RollbackContractVersion::V1,
        strategy: RollbackStrategy::SnapshotRestore,
        lineage_mode: RollbackLineageMode::Reverted,
    };
}
