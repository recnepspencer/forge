//! Typed events emitted during topology operation lifecycle execution.

use crate::b_rep::data::storage::cache_runtime::TopoCacheEffect;
use crate::identity::{DraftId, OperationCount, OperationId};
use crate::operations::operator::EulerDelta;
use crate::transactions::data::mutation_journal::EntityKindCounts;
use crate::validators::invariant_id::{InvariantId, InvariantRelation};
use forge_core::EntityRef;
use forge_core::LineageDelta;

/// Inter-subscriber data IDs for topo lifecycle runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TopoSubscriberDataId {
    MutationCounts,
    VersionCounters,
    TopologyHash,
    EulerDeltaResult,
    LineageEvents,
    ValidationResult,
    OperationMetrics,
    ReplayEntryFinalization,
}

/// Lifecycle event stream for one topology operation.
///
/// NOTE: Event payload IDs must remain strongly typed.
#[derive(Debug, Clone)]
pub enum TopoOperationEvent {
    OperationStarted {
        op_name: &'static str,
        invocation_id: OperationId,
        draft_id: DraftId,
        schema_version: u32,
        invariant_relation: fn(InvariantId) -> InvariantRelation,
        summary: String,
    },
    OperationCompleted {
        invocation_id: OperationId,
        declared_delta: EulerDelta,
    },
    OperationFailed {
        invocation_id: OperationId,
        error_summary: String,
    },
    DraftRolledBack {
        draft_id: DraftId,
        ops_completed: OperationCount,
    },
    ReplayCacheTraceApplied {
        op_id: OperationId,
        trace: Vec<String>,
    },
    OperationArtifactsBuilt {
        created: EntityKindCounts,
        destroyed: EntityKindCounts,
        lineage_delta: LineageDelta,
    },
    CacheEffect(TopoCacheEffect),
    EntityCreated(EntityRef),
    EntityDestroyed(EntityRef),
    TopologyChanged,
    GeometryChanged,
}
