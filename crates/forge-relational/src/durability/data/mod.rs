use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::config::data::RelationalRuntimeProfile;
use crate::history::data::{BranchHead, CommitId, CommitReference};
use crate::identity::data::{
    EntityId, KindId, LineageId, PartitionId, RelationId, StructuralFingerprint, VersionId,
};
use crate::indexes::data::{DerivedIndexDefinition, DerivedIndexGeneration};
use crate::lineage::data::{CorrespondenceCandidate, LineageEventRecord, LineageNode};
use crate::payloads::data::RecordPayload;
use crate::replay::data::CanonicalCommitEnvelope;
use crate::schema::data::SchemaVersionId;
use crate::storage::data::RecordLifecycleState;
use crate::symbols::data::{Symbol, SymbolTableSnapshot};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DurabilityMode {
    InMemoryCanonical,
    PersistedSegmentedLocalFs,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableStoreLayout {
    pub root_path: PathBuf,
    pub segment_commit_capacity: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DurableSegmentId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DurableCheckpointId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DurableIntegrityStatus {
    Verified,
    Corrupt,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckpointCoverage {
    pub up_to_commit: Option<CommitReference>,
    pub up_to_version: Option<VersionId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableSegmentManifest {
    pub segment_id: DurableSegmentId,
    pub path: PathBuf,
    pub first_commit_id: Option<CommitId>,
    pub last_commit_id: Option<CommitId>,
    pub commit_count: usize,
    pub runtime_name: String,
    pub profile: RelationalRuntimeProfile,
    pub schema_version: SchemaVersionId,
    pub integrity: DurableIntegrityStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableCheckpointManifest {
    pub checkpoint_id: DurableCheckpointId,
    pub path: PathBuf,
    pub coverage: CheckpointCoverage,
    pub partition_count: usize,
    pub runtime_name: String,
    pub profile: RelationalRuntimeProfile,
    pub schema_version: SchemaVersionId,
    pub integrity: DurableIntegrityStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableStore {
    pub layout: DurableStoreLayout,
    pub segments: Vec<DurableSegmentManifest>,
    pub checkpoints: Vec<DurableCheckpointManifest>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableBitSet {
    pub words: Vec<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionedPayloadImage {
    pub effective_at: VersionId,
    pub retired_at: Option<VersionId>,
    pub value: RecordPayload,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityArenaCheckpointImage {
    pub generations: Vec<u32>,
    pub lifecycle: Vec<RecordLifecycleState>,
    pub kind_ids: Vec<Option<KindId>>,
    pub payloads: Vec<Option<RecordPayload>>,
    pub payload_history: Vec<Vec<VersionedPayloadImage>>,
    pub created_at: Vec<VersionId>,
    pub retired_at: Vec<Option<VersionId>>,
    pub aspect_versions: Vec<std::collections::BTreeMap<Symbol, u64>>,
    pub structural_fingerprints: Vec<Option<StructuralFingerprint>>,
    pub lineage_ids: Vec<Option<LineageId>>,
    pub diagnostics_enrichment: Vec<std::collections::BTreeMap<Symbol, String>>,
    pub branch_pins: Vec<u32>,
    pub replay_pins: Vec<u32>,
    pub snapshot_pins: Vec<u32>,
    pub live_bitset: DurableBitSet,
    pub reclaimable_bitset: DurableBitSet,
    pub free_list: Vec<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationEndpointsImage {
    pub source: EntityId,
    pub target: EntityId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationArenaCheckpointImage {
    pub generations: Vec<u32>,
    pub lifecycle: Vec<RecordLifecycleState>,
    pub kind_ids: Vec<Option<KindId>>,
    pub payloads: Vec<Option<RecordPayload>>,
    pub payload_history: Vec<(usize, Vec<VersionedPayloadImage>)>,
    pub created_at: Vec<VersionId>,
    pub retired_at: Vec<Option<VersionId>>,
    pub endpoints: Vec<Option<RelationEndpointsImage>>,
    pub diagnostics_enrichment: Vec<std::collections::BTreeMap<Symbol, String>>,
    pub snapshot_pins: Vec<u32>,
    pub live_bitset: DurableBitSet,
    pub reclaimable_bitset: DurableBitSet,
    pub free_list: Vec<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PartitionCheckpointImage {
    pub partition_id: PartitionId,
    pub entity_arena: EntityArenaCheckpointImage,
    pub relation_arena: RelationArenaCheckpointImage,
    pub adjacency: Vec<Vec<RelationId>>,
    pub reverse_adjacency: Vec<Vec<RelationId>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableCommitEnvelope {
    pub envelope: CanonicalCommitEnvelope,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableCheckpoint {
    pub coverage: CheckpointCoverage,
    pub branches: Vec<BranchHead>,
    pub envelopes: Vec<CanonicalCommitEnvelope>,
    pub partition_images: Vec<PartitionCheckpointImage>,
    pub lineage_nodes: Vec<LineageNode>,
    pub lineage_events: Vec<LineageEventRecord>,
    pub correspondence_candidates: Vec<CorrespondenceCandidate>,
    pub index_definitions: Vec<DerivedIndexDefinition>,
    pub index_generations: Vec<DerivedIndexGeneration>,
    pub symbol_table: SymbolTableSnapshot,
    pub runtime_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryCursor {
    pub checkpoint_id: Option<DurableCheckpointId>,
    pub segment_ids: Vec<DurableSegmentId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryCoverage {
    pub checkpoint_commits: usize,
    pub replayed_tail_commits: usize,
    pub recovered_through_commit: Option<CommitReference>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryIntegrityReport {
    pub selected_checkpoint_id: Option<DurableCheckpointId>,
    pub skipped_corrupt_checkpoints: Vec<DurableCheckpointId>,
    pub verified_segment_ids: Vec<DurableSegmentId>,
    pub corrupt_segment_id: Option<DurableSegmentId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryCompatibilityCheck {
    pub schema_match: bool,
    pub profile_match: bool,
    pub runtime_name_match: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryPlan {
    pub config: crate::logic::runtime::RelationalRuntimeConfig,
    pub store: Option<DurableStore>,
    pub checkpoint_manifest: Option<DurableCheckpointManifest>,
    pub checkpoint: Option<DurableCheckpoint>,
    pub tail_log: Vec<DurableCommitEnvelope>,
    pub cursor: RecoveryCursor,
    pub integrity_report: RecoveryIntegrityReport,
    pub compatibility: RecoveryCompatibilityCheck,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecoveryFailureClass {
    SchemaMismatch,
    ProfileMismatch,
    RuntimeNameMismatch,
    CorruptCheckpoint,
    CorruptSegment,
    MissingParentChain,
    ReplayFailure,
    DurableIoFailure,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurabilityError {
    pub class: RecoveryFailureClass,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryOutcome {
    pub recovered_commits: usize,
    pub latest_commit: Option<crate::history::data::CommitReference>,
    pub restored_branches: usize,
    pub cursor: RecoveryCursor,
    pub coverage: RecoveryCoverage,
    pub integrity_report: RecoveryIntegrityReport,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompactionPlan {
    pub checkpoint_id: DurableCheckpointId,
    pub removable_segments: Vec<DurableSegmentId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompactionOutcome {
    pub removed_segments: Vec<DurableSegmentId>,
    pub retained_segments: Vec<DurableSegmentId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompactionPolicy {
    pub remove_fully_covered_segments: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SegmentRetentionClass {
    CoveredByCheckpoint,
    RequiredForRecovery,
}
