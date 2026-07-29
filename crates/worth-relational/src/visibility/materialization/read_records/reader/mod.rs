mod aspect_catalog;
mod aspect_versions;
mod context;
mod diagnostics;
mod query_execution;
mod query_fragment_scratch;
mod query_fragment_work;
mod query_packet_scope;
mod query_packetization;
mod query_plan_execution;
mod query_planning;
mod query_traversal;
mod snapshot_reads;
mod truth_access;
mod truth_adjacency;
mod truth_record_access;

use crate::authority::commit::preparation::planning::strategy::{
    coarse_preparation_packet_count, PreparationStrategySelection,
    TARGET_PREPARATION_ITEMS_PER_PACKET,
};
use crate::capabilities::{
    SchemaVersionSource, SnapshotSource, VersionSource, VisibilityPolicySource,
};
use crate::diagnostics::data::{
    DeterminismExpectation, DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope,
    RelationalDiagnosticArtifact, RelationalDiagnosticsEntry,
};
use crate::logic::runtime::{RelationalRuntime, VisibilityResidency};
use crate::query::data::{
    PlannedQueryPacket, QueryExecutionOutcome, QueryOrderingContract, QueryParallelLegality,
    QueryParallelProfitability, QueryPlanContextId, QueryPlanEvidenceBasis, QueryScope,
    QuerySerialReason, SnapshotPinnedQueryPlan,
};
use crate::schema::data::runtime_descriptor_semantics_policy;
use crate::snapshots::data::{SnapshotHandle, SnapshotInspectionSummary, SnapshotReadPolicy};
use crate::storage::data::{EntityReadRecord, RelationReadRecord, RelationalReadView};
use crate::storage::logic::state::{DenseSlotBitSet, PartitionAccess};
use crate::visibility::cache_state::{
    cached_state_for_version, reconstruct_state, residency_for_version,
};
use crate::visibility::snapshot_states::{
    build_visibility_state, read_view_from_snapshot_state, resolve_snapshot_handle,
    resolve_snapshot_inspection, resolve_snapshot_state,
};
use rayon::prelude::*;
use std::collections::{BTreeMap, BTreeSet};
use worth_foundational::facade::AspectKey;

use super::materialization::{
    materialize_authoritative_entity_record_at_version,
    materialize_authoritative_relation_record_at_version,
    materialize_current_authoritative_entity_record,
    materialize_current_authoritative_relation_record,
};
use super::visibility::{
    entity_slot_matches_kind_at_version, relation_slot_matches_kind_at_version,
    slot_kind_matches_current, visible_slots_in_partition_from_state,
};
use super::ProjectionAspectFilter;

const TARGET_TRAVERSAL_SEEDS_PER_PACKET: usize = 4;
pub use context::VisibilityReadContext;
use query_fragment_scratch::QueryFragmentScratch;
pub use truth_adjacency::{AdjacencyTruthReadLimitExceeded, BoundedAdjacencyTruthRead};
