use std::marker::PhantomData;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::config::data::RelationalRuntimeProfile;
use crate::history::data::{BranchHead, CommitId, CommitReference};
use crate::identity::data::{
    EntityId, KindId, LineageId, PartitionId, RelationId, StructuralFingerprint, VersionId,
};
use crate::indexes::data::{DerivedIndexDefinition, DerivedIndexGeneration};
use crate::lineage::data::LineageCheckpointArtifact;
use crate::payloads::data::RecordPayload;
use crate::replay::data::CanonicalCommitEnvelope;
use crate::replay::data::ReplayVerificationLayer;
use crate::schema::data::{DescriptorSemanticsVersion, SchemaVersionId};
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
    pub generation: u32,
    pub value: RecordPayload,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionedEntityMetadataImage {
    pub effective_at: VersionId,
    pub retired_at: Option<VersionId>,
    pub generation: u32,
    pub kind_id: KindId,
    pub lineage_id: Option<LineageId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityExtraImage {
    pub structural_fingerprint: Option<StructuralFingerprint>,
    pub lineage_id: Option<LineageId>,
}

pub trait RecordArenaCheckpointKind: Clone + PartialEq + Eq {
    type MetaImage: Clone + PartialEq + Eq;
    type ExtraImage: Clone + PartialEq + Eq;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityCheckpointImageKind;

impl RecordArenaCheckpointKind for EntityCheckpointImageKind {
    type MetaImage = VersionedEntityMetadataImage;
    type ExtraImage = EntityExtraImage;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationCheckpointImageKind;

impl RecordArenaCheckpointKind for RelationCheckpointImageKind {
    type MetaImage = VersionedRelationMetadataImage;
    type ExtraImage = Option<RelationEndpointsImage>;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound(
    serialize = "K::MetaImage: Serialize, K::ExtraImage: Serialize",
    deserialize = "K::MetaImage: Deserialize<'de>, K::ExtraImage: Deserialize<'de>"
))]
pub struct RecordArenaCheckpointImage<K: RecordArenaCheckpointKind> {
    pub generations: Vec<u32>,
    pub lifecycle: Vec<RecordLifecycleState>,
    pub kind_ids: Vec<Option<KindId>>,
    pub payloads: Vec<Option<RecordPayload>>,
    pub payload_history: Vec<Vec<VersionedPayloadImage>>,
    pub metadata_history: Vec<Vec<K::MetaImage>>,
    pub created_at: Vec<VersionId>,
    pub retired_at: Vec<Option<VersionId>>,
    pub aspect_versions: Vec<std::collections::BTreeMap<Symbol, u64>>,
    pub extra: Vec<K::ExtraImage>,
    pub diagnostics_enrichment: Vec<std::collections::BTreeMap<Symbol, String>>,
    pub branch_pins: Vec<u32>,
    pub replay_pins: Vec<u32>,
    pub snapshot_pins: Vec<u32>,
    pub live_bitset: DurableBitSet,
    pub reclaimable_bitset: DurableBitSet,
    pub free_list: Vec<u64>,
    #[serde(skip)]
    pub marker: PhantomData<K>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationEndpointsImage {
    pub source: EntityId,
    pub target: EntityId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionedRelationMetadataImage {
    pub effective_at: VersionId,
    pub retired_at: Option<VersionId>,
    pub generation: u32,
    pub kind_id: KindId,
    pub endpoints: RelationEndpointsImage,
}

pub type EntityArenaCheckpointImage = RecordArenaCheckpointImage<EntityCheckpointImageKind>;
pub type RelationArenaCheckpointImage = RecordArenaCheckpointImage<RelationCheckpointImageKind>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PartitionCheckpointImage {
    pub partition_id: PartitionId,
    pub entity_arena: EntityArenaCheckpointImage,
    pub relation_arena: RelationArenaCheckpointImage,
    pub adjacency: Vec<Vec<RelationId>>,
    pub reverse_adjacency: Vec<Vec<RelationId>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableCheckpoint {
    pub coverage: CheckpointCoverage,
    pub branches: Vec<BranchHead>,
    pub envelopes: Vec<CanonicalCommitEnvelope>,
    pub partition_images: Vec<PartitionCheckpointImage>,
    pub lineage: LineageCheckpointArtifact,
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

mod recovery_errors;

pub use recovery_errors::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryCompatibilityCheck {
    pub schema_parity: RecoveryAuthorityParity,
    pub profile_parity: RecoveryAuthorityParity,
    pub runtime_name_parity: RecoveryAuthorityParity,
    pub descriptor_version_parity: RecoveryAuthorityParity,
    pub schema_transition_parity: RecoveryAuthorityParity,
    pub continuation_descriptor_parity: RecoveryAuthorityParity,
    pub reconciliation_descriptor_parity: RecoveryAuthorityParity,
    pub schema_lineage_parity: RecoveryAuthorityParity,
    pub verification_outcome: RecoveryVerificationOutcome,
    pub first_mismatch: Option<RecoveryCompatibilityMismatch>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecoveryVerificationMode {
    NormalRecoveryVerification,
    AuditRecoveryVerification,
    CorruptionDiagnosisReplay,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecoveryVerificationPlan {
    Normal(NormalRecoveryVerificationPlan),
    Audit(AuditRecoveryVerificationPlan),
    CorruptionDiagnosis(CorruptionDiagnosisReplayPlan),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalRecoveryVerificationPlan;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditRecoveryVerificationPlan;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CorruptionDiagnosisReplayPlan;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecoveryVerificationOutcome {
    VerifiedAtLayer(ReplayVerificationLayer),
    Rejected {
        layer: ReplayVerificationLayer,
        detail: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecoveryAuthorityParity {
    VerifiedAtLayer(ReplayVerificationLayer),
    Drift,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryPlan {
    pub config: crate::logic::runtime::RelationalRuntimeConfig,
    pub store: Option<DurableStore>,
    pub checkpoint_manifest: Option<DurableCheckpointManifest>,
    pub checkpoint: Option<DurableCheckpoint>,
    pub tail_log: Vec<CanonicalCommitEnvelope>,
    pub cursor: RecoveryCursor,
    pub integrity_report: RecoveryIntegrityReport,
    pub compatibility: RecoveryCompatibilityCheck,
    pub verification_plan: RecoveryVerificationPlan,
    pub descriptor_semantics_version: DescriptorSemanticsVersion,
}

impl RecoveryVerificationPlan {
    pub fn from_mode(mode: RecoveryVerificationMode) -> Self {
        match mode {
            RecoveryVerificationMode::NormalRecoveryVerification => {
                Self::Normal(NormalRecoveryVerificationPlan)
            }
            RecoveryVerificationMode::AuditRecoveryVerification => {
                Self::Audit(AuditRecoveryVerificationPlan)
            }
            RecoveryVerificationMode::CorruptionDiagnosisReplay => {
                Self::CorruptionDiagnosis(CorruptionDiagnosisReplayPlan)
            }
        }
    }

    pub fn allows_deep_artifact_parity(&self) -> bool {
        !matches!(self, Self::Normal(_))
    }

    pub fn mode(&self) -> RecoveryVerificationMode {
        match self {
            Self::Normal(_) => RecoveryVerificationMode::NormalRecoveryVerification,
            Self::Audit(_) => RecoveryVerificationMode::AuditRecoveryVerification,
            Self::CorruptionDiagnosis(_) => RecoveryVerificationMode::CorruptionDiagnosisReplay,
        }
    }
}

impl RecoveryAuthorityParity {
    pub fn verified_at(layer: ReplayVerificationLayer) -> Self {
        Self::VerifiedAtLayer(layer)
    }

    pub fn drift() -> Self {
        Self::Drift
    }

    pub fn is_verified(&self) -> bool {
        matches!(self, Self::VerifiedAtLayer(_))
    }
}

impl RecoveryCompatibilityCheck {
    pub fn verified_at(layer: ReplayVerificationLayer) -> Self {
        Self {
            schema_parity: RecoveryAuthorityParity::verified_at(layer),
            profile_parity: RecoveryAuthorityParity::verified_at(layer),
            runtime_name_parity: RecoveryAuthorityParity::verified_at(layer),
            descriptor_version_parity: RecoveryAuthorityParity::verified_at(layer),
            schema_transition_parity: RecoveryAuthorityParity::verified_at(layer),
            continuation_descriptor_parity: RecoveryAuthorityParity::verified_at(layer),
            reconciliation_descriptor_parity: RecoveryAuthorityParity::verified_at(layer),
            schema_lineage_parity: RecoveryAuthorityParity::verified_at(layer),
            verification_outcome: RecoveryVerificationOutcome::VerifiedAtLayer(layer),
            first_mismatch: None,
        }
    }
}

impl RecoveryPlan {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        config: crate::logic::runtime::RelationalRuntimeConfig,
        store: Option<DurableStore>,
        checkpoint_manifest: Option<DurableCheckpointManifest>,
        checkpoint: Option<DurableCheckpoint>,
        tail_log: Vec<CanonicalCommitEnvelope>,
        cursor: RecoveryCursor,
        integrity_report: RecoveryIntegrityReport,
        compatibility: RecoveryCompatibilityCheck,
        verification_mode: RecoveryVerificationMode,
        descriptor_semantics_version: DescriptorSemanticsVersion,
    ) -> Self {
        Self {
            config,
            store,
            checkpoint_manifest,
            checkpoint,
            tail_log,
            cursor,
            integrity_report,
            compatibility,
            verification_plan: RecoveryVerificationPlan::from_mode(verification_mode),
            descriptor_semantics_version,
        }
    }

    pub fn verification_mode(&self) -> RecoveryVerificationMode {
        self.verification_plan.mode()
    }
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
