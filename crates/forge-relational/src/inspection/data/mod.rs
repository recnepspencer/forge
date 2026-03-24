use serde::{Deserialize, Serialize};

use crate::history::data::{
    AspectHistoryQueryResult, BranchId, CommitReference,
};
use crate::identity::data::{
    EntityId, KindId, LineageId, PartitionId, RelationId, StructuralFingerprint, VersionId,
};
use crate::indexes::data::DerivedIndexGeneration;
use crate::lineage::data::{LineageArtifactCounters, LineageDigestBasis};
use crate::lineage::data::{HistoricalLineageResolution, LineageEventRecord};
use crate::publication::patch::data::CanonicalAspectSet;
use crate::snapshots::data::{SnapshotHandle, SnapshotInspectionSummary};
use crate::storage::data::{
    EntityReadRecord, RecordLifecycleState, RelationReadRecord, RelationalReadView,
    RetentionPassOutcome,
};
use crate::symbols::data::Symbol;
use crate::transactions::data::{RecordRef, SavepointId, TransactionId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum InspectionOrigin {
    CurrentTruth,
    VisibilitySnapshot,
    CanonicalCommitStorage,
    LineageGraph,
    RetentionState,
    TransactionStaging,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum InspectionAccessPath {
    DirectLookup,
    SnapshotRead,
    VersionRead,
    HistoricalRetainedRead,
    HistoricalReconstructedRead,
    CommitIndexRead,
    GraphTraversal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum InspectionResolutionContext {
    NoContext,
    BranchAncestry,
    LineageTraversal,
    RelationNeighborhood,
    ConnectivityTraversal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum InspectionAvailability {
    Direct,
    Reconstructed,
    UnavailableByBudget,
    UnavailableByRetention,
    UnavailableByPolicy,
    UnavailableByMissingCanonicalArtifacts,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum InspectionDegradation {
    MissingStructuralFingerprint,
    MissingLineageIdentity,
    SummaryOnly,
    WorkBudgetExceeded,
    EntityBudgetExceeded,
    RelationBudgetExceeded,
    FrontierBudgetExceeded,
    ComponentBudgetExceeded,
    EntitySlotBudgetExceeded,
    RelationSlotBudgetExceeded,
    ReconstructionOmittedByMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum InspectionRecordClass {
    Entity,
    Relation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InspectionScope {
    Current,
    Version(VersionId),
    Snapshot(SnapshotHandle),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[must_use]
pub struct StructuralIdentityEvidence {
    pub target: RecordRef,
    pub record_class: InspectionRecordClass,
    pub kind_id: KindId,
    pub storage_identity: RecordRef,
    pub lineage_id: Option<LineageId>,
    pub structural_fingerprint: Option<StructuralFingerprint>,
    pub observed_version: VersionId,
    pub lifecycle: RecordLifecycleState,
    pub origin: InspectionOrigin,
    pub access_path: InspectionAccessPath,
    pub availability: InspectionAvailability,
    pub degradations: Vec<InspectionDegradation>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum StructuralIdentityComparisonVerdict {
    EqualByFingerprint,
    NotEqualByFingerprint,
    IncomparableMissingFingerprint,
    IncomparableFingerprintFamilyMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[must_use]
pub struct StructuralIdentityComparison {
    pub left: Option<StructuralIdentityEvidence>,
    pub right: Option<StructuralIdentityEvidence>,
    pub verdict: StructuralIdentityComparisonVerdict,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructuralIdentityQueryRequest {
    pub scope: InspectionScope,
    pub partition_scope: Option<Vec<PartitionId>>,
    pub fingerprint_family: Symbol,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphInspectionRequest {
    pub scope: InspectionScope,
    pub partition_scope: Option<Vec<PartitionId>>,
    pub relation_kind_scope: Option<Vec<KindId>>,
    pub summary_only: bool,
    pub budget: GraphInspectionBudget,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphInspectionBudget {
    pub max_entities: u64,
    pub max_relations: u64,
    pub max_work_units: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KindInspectionRequest {
    pub scope: InspectionScope,
    pub partition_scope: Option<Vec<PartitionId>>,
    pub kind_id: KindId,
    pub record_class: InspectionRecordClass,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectivityInspectionRequest {
    pub scope: InspectionScope,
    pub partition_scope: Option<Vec<PartitionId>>,
    pub relation_kind_scope: Option<Vec<KindId>>,
    pub include_members: bool,
    pub budget: ConnectivityInspectionBudget,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectivityInspectionBudget {
    pub max_entities: u64,
    pub max_relations: u64,
    pub max_frontier: u64,
    pub max_components: u64,
    pub max_work_units: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[must_use]
pub struct GraphInspectionSummary {
    pub scope: InspectionScope,
    pub version_id: VersionId,
    pub partition_count: u64,
    pub entity_count: u64,
    pub relation_count: u64,
    pub entity_kinds: Vec<(KindId, u64)>,
    pub relation_kinds: Vec<(KindId, u64)>,
    pub origin: InspectionOrigin,
    pub access_path: InspectionAccessPath,
    pub availability: InspectionAvailability,
    pub degradations: Vec<InspectionDegradation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[must_use]
pub struct KindInspectionSummary {
    pub scope: InspectionScope,
    pub version_id: VersionId,
    pub kind_id: KindId,
    pub record_class: InspectionRecordClass,
    pub count: u64,
    pub touched_partitions: Vec<PartitionId>,
    pub origin: InspectionOrigin,
    pub access_path: InspectionAccessPath,
    pub availability: InspectionAvailability,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[must_use]
pub struct ConnectivityComponentSummary {
    pub member_count: u64,
    pub members: Option<Vec<EntityId>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[must_use]
pub struct ConnectivityInspectionSummary {
    pub scope: InspectionScope,
    pub version_id: VersionId,
    pub component_count: u64,
    pub largest_component_size: u64,
    pub enumerated_entity_count: u64,
    pub components: Vec<ConnectivityComponentSummary>,
    pub origin: InspectionOrigin,
    pub access_path: InspectionAccessPath,
    pub resolution_context: InspectionResolutionContext,
    pub availability: InspectionAvailability,
    pub degradations: Vec<InspectionDegradation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[must_use]
pub struct NeighborInspectionResult {
    pub entity_id: EntityId,
    pub version_id: VersionId,
    pub outgoing_relation_ids: Vec<RelationId>,
    pub incoming_relation_ids: Vec<RelationId>,
    pub origin: InspectionOrigin,
    pub access_path: InspectionAccessPath,
    pub resolution_context: InspectionResolutionContext,
    pub availability: InspectionAvailability,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum HistoricalInspectionMode {
    RetainedOnly,
    AllowCanonicalReconstruction,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[must_use]
pub struct HistoricalSnapshotView {
    pub snapshot: SnapshotHandle,
    pub read_view: RelationalReadView,
    pub origin: InspectionOrigin,
    pub access_path: InspectionAccessPath,
    pub availability: InspectionAvailability,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[must_use]
pub struct HistoricalOpenResult {
    pub view: Option<HistoricalSnapshotView>,
    pub origin: InspectionOrigin,
    pub access_path: InspectionAccessPath,
    pub availability: InspectionAvailability,
    pub degradations: Vec<InspectionDegradation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HistoricalRecordValue {
    Entity(EntityReadRecord),
    Relation(RelationReadRecord),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[must_use]
pub struct HistoricalRecordObservation {
    pub target: RecordRef,
    pub version_id: VersionId,
    pub value: Option<HistoricalRecordValue>,
    pub origin: InspectionOrigin,
    pub access_path: InspectionAccessPath,
    pub availability: InspectionAvailability,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[must_use]
pub struct HistoricalAspectObservation {
    pub query_result: AspectHistoryQueryResult,
    pub origin: InspectionOrigin,
    pub access_path: InspectionAccessPath,
    pub availability: InspectionAvailability,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[must_use]
pub struct HistoricalAvailabilityObservation {
    pub version_id: VersionId,
    pub availability: InspectionAvailability,
    pub retained_directly: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[must_use]
pub struct HistoricalRecordInspection {
    pub branch_id: BranchId,
    pub record_observation: HistoricalRecordObservation,
    pub lineage_resolution_context: Option<HistoricalLineageResolution>,
    pub aspect_history_observation: Option<HistoricalAspectObservation>,
    pub structural_identity_evidence: Option<StructuralIdentityEvidence>,
    pub retention_availability_observation: Option<HistoricalAvailabilityObservation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetentionStateObservation {
    pub target: RecordRef,
    pub lifecycle: RecordLifecycleState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PinStateObservation {
    pub target: RecordRef,
    pub snapshot_pins: u32,
    pub branch_pins: u32,
    pub replay_pins: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ReclaimEligibility {
    EligibleNow,
    BlockedBySnapshotPins,
    BlockedByBranchPins,
    BlockedByReplayPins,
    BlockedByRetentionFence,
    BlockedByPolicy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[must_use]
pub struct RecordRetentionInspection {
    pub state: RetentionStateObservation,
    pub pins: PinStateObservation,
    pub reclaim_eligibility: ReclaimEligibility,
    pub historical_availability: HistoricalAvailabilityObservation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[must_use]
pub struct RetentionInspectionSummary {
    pub current_version_id: VersionId,
    pub active_snapshot_count: u64,
    pub branch_pinned_entities: u64,
    pub replay_pinned_entities: u64,
    pub snapshot_pinned_entities: u64,
    pub branch_pinned_relations: u64,
    pub replay_pinned_relations: u64,
    pub snapshot_pinned_relations: u64,
    pub reclaimable_entities: u64,
    pub reclaimable_relations: u64,
    pub origin: InspectionOrigin,
    pub access_path: InspectionAccessPath,
    pub availability: InspectionAvailability,
    pub degradations: Vec<InspectionDegradation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetentionInspectionRequest {
    pub max_entity_slots_scanned: u64,
    pub max_relation_slots_scanned: u64,
    pub max_work_units: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[must_use]
pub struct SnapshotPinInspection {
    pub snapshot: SnapshotInspectionSummary,
    pub origin: InspectionOrigin,
    pub access_path: InspectionAccessPath,
    pub availability: InspectionAvailability,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitInspection {
    pub commit: CommitReference,
    pub changed_records: Vec<RecordRef>,
    pub lineage_event_ids: Vec<u64>,
    pub lineage_events: Vec<LineageEventRecord>,
    pub lineage_digest_basis: LineageDigestBasis,
    pub lineage_artifact_counters: LineageArtifactCounters,
    pub index_generation_ids: Vec<u64>,
    pub index_generations: Vec<DerivedIndexGeneration>,
    pub changed_aspects: CanonicalAspectSet,
    pub origin: InspectionOrigin,
    pub access_path: InspectionAccessPath,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecentCommitInspectionRequest {
    pub branch_id: Option<BranchId>,
    pub limit: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use]
pub struct RecentCommitInspectionWindow {
    pub branch_head: Option<CommitReference>,
    pub commits: Vec<CommitInspection>,
    pub origin: InspectionOrigin,
    pub access_path: InspectionAccessPath,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TransactionIntentCounts {
    pub create_count: u64,
    pub entity_mutation_count: u64,
    pub relation_mutation_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavepointInspectionSurface {
    pub savepoint_id: SavepointId,
    pub retained_batch_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[must_use]
pub struct TransactionInspectionSurface {
    pub transaction_id: TransactionId,
    pub target_branch: Option<BranchId>,
    pub batch_count: u64,
    pub savepoints: Vec<SavepointInspectionSurface>,
    pub touched_records: Vec<RecordRef>,
    pub intent_counts: TransactionIntentCounts,
    pub reserved_bulk_entity_slots: u64,
    pub reserved_bulk_relation_slots: u64,
    pub contains_lineage_affecting_intents: bool,
    pub origin: InspectionOrigin,
    pub access_path: InspectionAccessPath,
    pub availability: InspectionAvailability,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[must_use]
pub struct RetentionExecutionInspection {
    pub outcome: RetentionPassOutcome,
    pub origin: InspectionOrigin,
    pub access_path: InspectionAccessPath,
}
