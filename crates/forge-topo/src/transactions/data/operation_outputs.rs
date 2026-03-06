//! Typed operation outputs staged by transaction lifecycle subscribers.

use crate::identity::OperationId;
use crate::operations::operator::EulerDelta;
use crate::transactions::data::mutation_journal::EntityKindCounts;
use forge_core::LineageDelta;

/// Output of the journal subscriber.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MutationCounts {
    pub created: EntityKindCounts,
    pub destroyed: EntityKindCounts,
}

/// Output of the version subscriber.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct VersionCounters {
    pub topology_bumps: u64,
    pub geometry_bumps: u64,
}

/// Output of the replay subscriber.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ReplayStats {
    pub op_starts: u64,
    pub entry_records: u64,
    pub entry_finalizations: u64,
    pub cache_trace_updates: u64,
    pub last_recorded_op: Option<OperationId>,
    pub last_finalized_op: Option<OperationId>,
}

/// Output of Euler-delta verification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EulerDeltaCheck {
    pub declared: EulerDelta,
    pub actual: EulerDelta,
    pub matched: bool,
}

/// Output of invariant execution summary.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ValidationSummary {
    pub checks_run: u32,
    pub checks_failed: u32,
}

/// Output of lineage deletion stamping.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LineageSummary {
    pub deletions_seen: u32,
    pub deletions_stamped: u32,
}

/// Output of operation artifact construction.
#[derive(Debug, Clone)]
pub struct OperationArtifacts {
    pub entities_created: u32,
    pub entities_deleted: u32,
    pub lineage_delta: LineageDelta,
}
