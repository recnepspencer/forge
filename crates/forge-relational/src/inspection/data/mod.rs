use serde::{Deserialize, Serialize};

use crate::history::data::{
    AspectHistoryQueryResult, BranchId, CommitReference,
};
use crate::identity::data::{
    EntityId, KindId, LineageId, PartitionId, RelationId, StructuralFingerprint, VersionId,
};
use crate::lineage::data::HistoricalLineageResolution;
use crate::publication::patch::data::CanonicalAspectSet;
use crate::snapshots::data::{SnapshotHandle, SnapshotInspectionSummary};
use crate::storage::data::{
    EntityReadRecord, RecordLifecycleState, RelationReadRecord, RelationalReadView,
    RetentionPassOutcome,
};
use crate::symbols::data::Symbol;
use crate::transactions::data::{RecordRef, SavepointId, TransactionId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InspectionOrigin {
    CurrentTruth,
    VisibilitySnapshot,
    CanonicalCommitStorage,
    LineageGraph,
    RetentionState,
    TransactionStaging,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
pub enum InspectionResolutionContext {
    None,
    BranchAncestry,
    LineageTraversal,
    RelationNeighborhood,
    ConnectivityTraversal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InspectionAvailability {
    Direct,
    Reconstructed,
    UnavailableByRetention,
    UnavailableByPolicy,
    UnavailableByMissingCanonicalArtifacts,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InspectionDegradation {
    MissingStructuralFingerprint,
    MissingLineageIdentity,
    SummaryOnly,
    ReconstructionOmittedByMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
pub enum StructuralIdentityComparisonVerdict {
    EqualByFingerprint,
    NotEqualByFingerprint,
    IncomparableMissingFingerprint,
    IncomparableFingerprintFamilyMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphInspectionSummary {
    pub scope: InspectionScope,
    pub version_id: VersionId,
    pub partition_count: usize,
    pub entity_count: usize,
    pub relation_count: usize,
    pub entity_kinds: Vec<(KindId, usize)>,
    pub relation_kinds: Vec<(KindId, usize)>,
    pub origin: InspectionOrigin,
    pub access_path: InspectionAccessPath,
    pub availability: InspectionAvailability,
    pub degradations: Vec<InspectionDegradation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KindInspectionSummary {
    pub scope: InspectionScope,
    pub version_id: VersionId,
    pub kind_id: KindId,
    pub record_class: InspectionRecordClass,
    pub count: usize,
    pub touched_partitions: Vec<PartitionId>,
    pub origin: InspectionOrigin,
    pub access_path: InspectionAccessPath,
    pub availability: InspectionAvailability,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectivityComponentSummary {
    pub member_count: usize,
    pub members: Option<Vec<EntityId>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectivityInspectionSummary {
    pub scope: InspectionScope,
    pub version_id: VersionId,
    pub component_count: usize,
    pub largest_component_size: usize,
    pub enumerated_entity_count: usize,
    pub components: Vec<ConnectivityComponentSummary>,
    pub origin: InspectionOrigin,
    pub access_path: InspectionAccessPath,
    pub resolution_context: InspectionResolutionContext,
    pub availability: InspectionAvailability,
    pub degradations: Vec<InspectionDegradation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
pub enum HistoricalInspectionMode {
    RetainedOnly,
    AllowCanonicalReconstruction,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoricalSnapshotView {
    pub snapshot: SnapshotHandle,
    pub read_view: RelationalReadView,
    pub origin: InspectionOrigin,
    pub access_path: InspectionAccessPath,
    pub availability: InspectionAvailability,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
pub struct HistoricalRecordObservation {
    pub target: RecordRef,
    pub version_id: VersionId,
    pub value: Option<HistoricalRecordValue>,
    pub origin: InspectionOrigin,
    pub access_path: InspectionAccessPath,
    pub availability: InspectionAvailability,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoricalAspectObservation {
    pub query_result: AspectHistoryQueryResult,
    pub origin: InspectionOrigin,
    pub access_path: InspectionAccessPath,
    pub availability: InspectionAvailability,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoricalAvailabilityObservation {
    pub version_id: VersionId,
    pub availability: InspectionAvailability,
    pub retained_directly: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
pub enum ReclaimEligibility {
    EligibleNow,
    BlockedBySnapshotPins,
    BlockedByBranchPins,
    BlockedByReplayPins,
    BlockedByRetentionFence,
    BlockedByPolicy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordRetentionInspection {
    pub state: RetentionStateObservation,
    pub pins: PinStateObservation,
    pub reclaim_eligibility: ReclaimEligibility,
    pub historical_availability: HistoricalAvailabilityObservation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetentionInspectionSummary {
    pub current_version_id: VersionId,
    pub active_snapshot_count: usize,
    pub branch_pinned_entities: usize,
    pub replay_pinned_entities: usize,
    pub snapshot_pinned_entities: usize,
    pub branch_pinned_relations: usize,
    pub replay_pinned_relations: usize,
    pub snapshot_pinned_relations: usize,
    pub reclaimable_entities: usize,
    pub reclaimable_relations: usize,
    pub origin: InspectionOrigin,
    pub access_path: InspectionAccessPath,
    pub availability: InspectionAvailability,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
    pub changed_aspects: CanonicalAspectSet,
    pub origin: InspectionOrigin,
    pub access_path: InspectionAccessPath,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecentCommitInspectionRequest {
    pub branch_id: Option<BranchId>,
    pub limit: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecentCommitInspectionWindow {
    pub branch_head: Option<CommitReference>,
    pub commits: Vec<CommitInspection>,
    pub origin: InspectionOrigin,
    pub access_path: InspectionAccessPath,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TransactionIntentCounts {
    pub create_count: usize,
    pub entity_mutation_count: usize,
    pub relation_mutation_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavepointInspectionSurface {
    pub savepoint_id: SavepointId,
    pub retained_batch_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransactionInspectionSurface {
    pub transaction_id: TransactionId,
    pub target_branch: Option<BranchId>,
    pub batch_count: usize,
    pub savepoints: Vec<SavepointInspectionSurface>,
    pub touched_records: Vec<RecordRef>,
    pub intent_counts: TransactionIntentCounts,
    pub reserved_bulk_entity_slots: usize,
    pub reserved_bulk_relation_slots: usize,
    pub contains_lineage_affecting_intents: bool,
    pub origin: InspectionOrigin,
    pub access_path: InspectionAccessPath,
    pub availability: InspectionAvailability,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetentionExecutionInspection {
    pub outcome: RetentionPassOutcome,
    pub origin: InspectionOrigin,
    pub access_path: InspectionAccessPath,
}
