use crate::{
    authority::digest_from_string,
    authority::{FetchedAuthoritativeCommit, PersistedAuthoritativeCommit},
    bulk::{
        BulkChunkCommitWitness, BulkPlanKind, DeterministicChunkPlan,
        FrozenBulkSourceManifest, FrozenTransformBasis, FrozenTransformTargetPartition,
        ProgramChunkWitnessIndex, PublishedBulkProgressCheckpoint,
    },
    delta::{BranchDeltaLayerId, BRANCH_DELTA_FAMILY_VERSION},
    layout::{
        AdmittedAspectLayoutReadPlan, ChunkDeterminismWitness, ChunkModelFrozenPhysicalLayout,
        DedupAdmittedBlockReuse, Milestone6LayoutMaterialization,
        Milestone7IndependentLayoutReference, Milestone9PhysicalChunkReference,
    },
    snapshot::{SnapshotId, SnapshotImageBundle},
    wal::WalRecord,
};
use forge_relational::facade::history::{BranchId, CommitId};
use forge_relational::facade::lineage::{
    LineageArtifactCounters, LineageDecisionLogDigestBasis, LineageDigestBasis,
    LineageEventBatchDigestBasis, LineageEventRecord,
};
use forge_relational::facade::replay::CanonicalCommitEnvelope;
use forge_relational::facade::schema::{
    DescriptorSemanticsVersion, SchemaContinuationDescriptor, SchemaReconciliationDescriptor,
    SchemaTransitionArtifact, SchemaVersionId,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use std::collections::BTreeMap;
use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchRecord {
    pub branch_id: BranchId,
    pub created_from_branch: Option<BranchId>,
    pub created_from_commit_id: Option<CommitId>,
    pub created_at_commit_sequence: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchHeadRecord {
    pub branch_id: BranchId,
    pub head_commit_id: Option<CommitId>,
    pub head_commit_digest: Option<String>,
    pub head_update_sequence: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoredCommitEnvelope {
    pub envelope: CanonicalCommitEnvelope,
    pub envelope_digest: String,
    pub canonicalization_version: u32,
    pub commit_sequence: u64,
}

impl StoredCommitEnvelope {
    pub fn into_persisted(self) -> PersistedAuthoritativeCommit {
        PersistedAuthoritativeCommit::new(
            self.envelope,
            digest_from_string(self.envelope_digest),
            self.canonicalization_version,
            self.commit_sequence,
        )
    }

    pub fn into_fetched(self) -> FetchedAuthoritativeCommit {
        FetchedAuthoritativeCommit::new(
            self.envelope,
            digest_from_string(self.envelope_digest),
            self.canonicalization_version,
            self.commit_sequence,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommitParentRecord {
    pub commit_id: CommitId,
    pub parent_position: usize,
    pub parent_commit_id: CommitId,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct AuthoritativeArtifactDigestRecord {
    pub artifact_family: AuthoritativeArtifactFamily,
    pub artifact_id: String,
    pub canonicalization_version: u32,
    pub digest_algorithm: String,
    pub artifact_digest: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum AuthoritativeArtifactFamily {
    BranchRecord,
    BranchHeadRecord,
    CommitEnvelope,
    CommitParentRecord,
    CommitSupportSummary,
    SchemaSupportRecord,
    LineageSupportRecord,
    DurableCursorIdentityRecord,
    SubscriberCheckpointRecord,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CommitSupportSummaryRecord {
    pub commit_id: CommitId,
    pub branch_id: BranchId,
    pub schema_support_artifact_id: Option<String>,
    pub lineage_support_artifact_id: Option<String>,
    #[serde(default)]
    pub milestone_6_published_layout_request_artifact_ids: Vec<String>,
    pub emitted_schema_artifact: bool,
    pub emitted_lineage_artifact: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SchemaSupportRecord {
    pub artifact_id: String,
    pub commit_id: CommitId,
    pub branch_id: BranchId,
    pub schema_version_id: SchemaVersionId,
    pub descriptor_semantics_version: DescriptorSemanticsVersion,
    pub schema_transition: Option<SchemaTransitionArtifact>,
    pub schema_continuation_descriptor: Option<SchemaContinuationDescriptor>,
    pub schema_reconciliation_descriptor: Option<SchemaReconciliationDescriptor>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LineageSupportRecord {
    pub artifact_id: String,
    pub commit_id: CommitId,
    pub branch_id: BranchId,
    pub lineage_event_ids: Vec<u64>,
    pub lineage_events: Vec<LineageEventRecord>,
    pub lineage_digest_basis: LineageDigestBasis,
    pub event_batch_digest_basis: LineageEventBatchDigestBasis,
    pub decision_log_digest_basis: LineageDecisionLogDigestBasis,
    pub lineage_artifact_counters: LineageArtifactCounters,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DurableCursorIdentityRecord {
    pub artifact_id: String,
    pub cursor_id: String,
    pub subscriber_id: String,
    pub branch_id: BranchId,
    pub feed_shape_id: String,
    pub schema_interpretation_id: String,
    pub cursor_semantics_version: u32,
    pub latest_checkpoint_sequence: u64,
    pub latest_basis_commit_id: CommitId,
    pub latest_schema_support_artifact_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SubscriberCheckpointRecord {
    pub artifact_id: String,
    pub cursor_id: String,
    pub subscriber_id: String,
    pub branch_id: BranchId,
    pub feed_shape_id: String,
    pub schema_interpretation_id: String,
    pub cursor_semantics_version: u32,
    pub checkpoint_sequence: u64,
    pub basis_commit_id: CommitId,
    pub schema_support_artifact_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BranchSharedBaseRecord {
    pub branch_id: BranchId,
    pub source_branch_id: BranchId,
    pub source_frontier_commit_id: Option<CommitId>,
    pub delta_family_version: u32,
    pub authority_basis_digest: String,
}

impl Default for BranchSharedBaseRecord {
    fn default() -> Self {
        Self {
            branch_id: BranchId(String::new()),
            source_branch_id: BranchId(String::new()),
            source_frontier_commit_id: None,
            delta_family_version: BRANCH_DELTA_FAMILY_VERSION,
            authority_basis_digest: String::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct BranchDeltaLayerArtifacts {
    pub commit_envelopes: Vec<StoredCommitEnvelope>,
    pub commit_parent_records: Vec<CommitParentRecord>,
    pub commit_support_summaries: Vec<CommitSupportSummaryRecord>,
    pub schema_support_records: Vec<SchemaSupportRecord>,
    pub lineage_support_records: Vec<LineageSupportRecord>,
}

impl BranchDeltaLayerArtifacts {
    pub fn is_empty(&self) -> bool {
        self.commit_envelopes.is_empty()
            && self.commit_parent_records.is_empty()
            && self.commit_support_summaries.is_empty()
            && self.schema_support_records.is_empty()
            && self.lineage_support_records.is_empty()
    }

    pub fn canonicalize_order(&mut self) {
        self.commit_envelopes
            .sort_by_key(|record| record.commit_sequence);
        self.commit_parent_records.sort_by(|left, right| {
            left.commit_id
                .cmp(&right.commit_id)
                .then(left.parent_position.cmp(&right.parent_position))
                .then(left.parent_commit_id.cmp(&right.parent_commit_id))
        });
        self.commit_support_summaries
            .sort_by(|left, right| left.commit_id.cmp(&right.commit_id));
        self.schema_support_records
            .sort_by(|left, right| left.commit_id.cmp(&right.commit_id));
        self.lineage_support_records
            .sort_by(|left, right| left.commit_id.cmp(&right.commit_id));
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BranchDeltaReplacementProofEntry {
    pub layer_id: BranchDeltaLayerId,
    pub branch_id: BranchId,
    pub base_frontier_commit_id: Option<CommitId>,
    pub target_frontier_commit_id: CommitId,
    pub commit_ids: Vec<CommitId>,
    pub delta_family_version: u32,
    pub authority_basis_digest: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BranchDeltaLayerRecord {
    pub branch_delta_layer_id: BranchDeltaLayerId,
    pub branch_id: BranchId,
    pub base_frontier_commit_id: Option<CommitId>,
    pub target_frontier_commit_id: CommitId,
    pub commit_ids: Vec<CommitId>,
    pub delta_family_version: u32,
    pub authority_basis_digest: String,
    #[serde(default)]
    pub artifacts: BranchDeltaLayerArtifacts,
    pub replacement_of_layer_ids: Vec<BranchDeltaLayerId>,
    #[serde(default)]
    pub replacement_lineage_proof: Vec<BranchDeltaReplacementProofEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EmbeddedCheckpointClassification {
    DerivedDurable,
    Ephemeral,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EmbeddedCheckpointRecord {
    pub checkpoint_id: String,
    pub source_runtime_id: String,
    pub basis_branch_id: Option<BranchId>,
    pub basis_commit_id: Option<CommitId>,
    pub classification: EmbeddedCheckpointClassification,
    pub contained_commit_ids: Vec<CommitId>,
    pub metadata: Value,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Milestone6LayoutMaterializationRecord {
    pub artifact_id: String,
    pub materialization: Milestone6LayoutMaterialization,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Milestone6CommitCoupledLayoutSeedRecord {
    pub artifact_id: String,
    pub request: crate::AspectLayoutReadRequest,
    pub layout_materialization_artifact_id: String,
    pub authority_basis_commit_id: CommitId,
    pub authority_basis_commit_digest: String,
    pub authority_basis_commit_sequence: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Milestone6ScopeSliceMembershipRecord {
    pub artifact_id: String,
    pub branch_id: BranchId,
    pub frontier_commit_id: CommitId,
    pub scope_class: String,
    pub projection_digest: String,
    pub slice_ids: Vec<crate::AspectLayoutSliceId>,
    pub layout_materialization_artifact_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Milestone6ChunkMembershipRecord {
    pub artifact_id: String,
    pub physical_chunk_id: crate::PhysicalChunkId,
    pub chunk_shape_version: crate::ChunkShapeVersion,
    pub determinism_digest: String,
    pub slice_ids: Vec<crate::AspectLayoutSliceId>,
    pub layout_materialization_artifact_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Milestone6StructuralBlockRecord {
    pub artifact_id: String,
    pub structural_block_id: crate::StructuralBlockId,
    pub scope_class: String,
    pub equivalence_contract_version: crate::EquivalenceContractVersion,
    pub slice_ids: Vec<crate::AspectLayoutSliceId>,
    pub supporting_layout_materialization_artifact_ids: Vec<String>,
}

impl Serialize for Milestone6LayoutMaterializationRecord {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        PersistedMilestone6LayoutMaterializationRecord::from(self).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Milestone6LayoutMaterializationRecord {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let persisted = PersistedMilestone6LayoutMaterializationRecord::deserialize(deserializer)?;
        Self::try_from(persisted).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct PersistedMilestone6LayoutMaterializationRecord {
    artifact_id: String,
    materialization: PersistedMilestone6LayoutMaterialization,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct PersistedMilestone6LayoutMaterialization {
    artifact_id: String,
    admitted_plan: PersistedAdmittedAspectLayoutReadPlan,
    block_reuse: PersistedDedupAdmittedBlockReuse,
    frozen_layout: PersistedChunkModelFrozenPhysicalLayout,
    milestone_7_reference: PersistedMilestone7IndependentLayoutReference,
    milestone_9_reference: PersistedMilestone9PhysicalChunkReference,
    semantic_truth_digest: String,
    authoritative_commit_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct PersistedAdmittedAspectLayoutReadPlan {
    request: crate::AspectLayoutReadRequest,
    slice_ids: Vec<crate::AspectLayoutSliceId>,
    structural_block_id: crate::StructuralBlockId,
    performance: crate::AspectLayoutPerformanceEnvelope,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct PersistedDedupAdmittedBlockReuse {
    branch_id: BranchId,
    frontier_commit_id: CommitId,
    scope_class: String,
    structural_block_id: crate::StructuralBlockId,
    equivalence_contract_version: crate::EquivalenceContractVersion,
    slice_ids: Vec<crate::AspectLayoutSliceId>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct PersistedChunkDeterminismWitness {
    physical_chunk_id: crate::PhysicalChunkId,
    chunk_shape_version: crate::ChunkShapeVersion,
    determinism_digest: String,
    ordered_slice_ids: Vec<crate::AspectLayoutSliceId>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct PersistedChunkModelFrozenPhysicalLayout {
    request: crate::AspectLayoutReadRequest,
    chunk_width: u64,
    witness: PersistedChunkDeterminismWitness,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct PersistedMilestone7IndependentLayoutReference {
    branch_id: BranchId,
    frontier_commit_id: CommitId,
    scope_class: String,
    projection_digest: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct PersistedMilestone9PhysicalChunkReference {
    physical_chunk_id: crate::PhysicalChunkId,
    chunk_shape_version: crate::ChunkShapeVersion,
    determinism_digest: String,
    chunk_member_count: usize,
}

impl From<&Milestone6LayoutMaterializationRecord> for PersistedMilestone6LayoutMaterializationRecord {
    fn from(record: &Milestone6LayoutMaterializationRecord) -> Self {
        Self {
            artifact_id: record.artifact_id.clone(),
            materialization: PersistedMilestone6LayoutMaterialization::from(&record.materialization),
        }
    }
}

impl From<&Milestone6LayoutMaterialization> for PersistedMilestone6LayoutMaterialization {
    fn from(materialization: &Milestone6LayoutMaterialization) -> Self {
        Self {
            artifact_id: materialization.artifact_id().to_string(),
            admitted_plan: PersistedAdmittedAspectLayoutReadPlan::from(materialization.admitted_plan()),
            block_reuse: PersistedDedupAdmittedBlockReuse::from(materialization.block_reuse()),
            frozen_layout: PersistedChunkModelFrozenPhysicalLayout::from(materialization.frozen_layout()),
            milestone_7_reference: PersistedMilestone7IndependentLayoutReference::from(
                materialization.milestone_7_reference(),
            ),
            milestone_9_reference: PersistedMilestone9PhysicalChunkReference::from(
                materialization.milestone_9_reference(),
            ),
            semantic_truth_digest: materialization.semantic_truth_digest().to_string(),
            authoritative_commit_count: materialization.authoritative_commit_count(),
        }
    }
}

impl From<&AdmittedAspectLayoutReadPlan> for PersistedAdmittedAspectLayoutReadPlan {
    fn from(plan: &AdmittedAspectLayoutReadPlan) -> Self {
        Self {
            request: plan.request().clone(),
            slice_ids: plan.slice_ids().to_vec(),
            structural_block_id: plan.structural_block_id().clone(),
            performance: plan.performance().clone(),
        }
    }
}

impl From<&DedupAdmittedBlockReuse> for PersistedDedupAdmittedBlockReuse {
    fn from(reuse: &DedupAdmittedBlockReuse) -> Self {
        Self {
            branch_id: reuse.branch_id().clone(),
            frontier_commit_id: reuse.frontier_commit_id(),
            scope_class: reuse.scope_class().to_string(),
            structural_block_id: reuse.structural_block_id().clone(),
            equivalence_contract_version: reuse.equivalence_contract_version(),
            slice_ids: reuse.slice_ids().to_vec(),
        }
    }
}

impl From<&ChunkDeterminismWitness> for PersistedChunkDeterminismWitness {
    fn from(witness: &ChunkDeterminismWitness) -> Self {
        Self {
            physical_chunk_id: witness.physical_chunk_id().clone(),
            chunk_shape_version: witness.chunk_shape_version(),
            determinism_digest: witness.determinism_digest().to_string(),
            ordered_slice_ids: witness.ordered_slice_ids().to_vec(),
        }
    }
}

impl From<&ChunkModelFrozenPhysicalLayout> for PersistedChunkModelFrozenPhysicalLayout {
    fn from(frozen: &ChunkModelFrozenPhysicalLayout) -> Self {
        Self {
            request: frozen.request().clone(),
            chunk_width: frozen.chunk_width(),
            witness: PersistedChunkDeterminismWitness::from(frozen.witness()),
        }
    }
}

impl From<&Milestone7IndependentLayoutReference> for PersistedMilestone7IndependentLayoutReference {
    fn from(reference: &Milestone7IndependentLayoutReference) -> Self {
        Self {
            branch_id: reference.branch_id().clone(),
            frontier_commit_id: reference.frontier_commit_id(),
            scope_class: reference.scope_class().to_string(),
            projection_digest: reference.projection_digest().to_string(),
        }
    }
}

impl From<&Milestone9PhysicalChunkReference> for PersistedMilestone9PhysicalChunkReference {
    fn from(reference: &Milestone9PhysicalChunkReference) -> Self {
        Self {
            physical_chunk_id: reference.physical_chunk_id().clone(),
            chunk_shape_version: reference.chunk_shape_version(),
            determinism_digest: reference.determinism_digest().to_string(),
            chunk_member_count: reference.chunk_member_count(),
        }
    }
}

impl TryFrom<PersistedMilestone6LayoutMaterializationRecord> for Milestone6LayoutMaterializationRecord {
    type Error = String;

    fn try_from(record: PersistedMilestone6LayoutMaterializationRecord) -> Result<Self, Self::Error> {
        validate_persisted_milestone_6_layout_materialization_record(&record)?;
        Ok(Self {
            artifact_id: record.artifact_id,
            materialization: Milestone6LayoutMaterialization::try_from(record.materialization)?,
        })
    }
}

impl TryFrom<PersistedMilestone6LayoutMaterialization> for Milestone6LayoutMaterialization {
    type Error = String;

    fn try_from(materialization: PersistedMilestone6LayoutMaterialization) -> Result<Self, Self::Error> {
        validate_persisted_milestone_6_layout_materialization(&materialization)?;
        let admitted_plan = AdmittedAspectLayoutReadPlan::new(
            materialization.admitted_plan.request,
            materialization.admitted_plan.slice_ids,
            materialization.admitted_plan.structural_block_id,
            materialization.admitted_plan.performance,
        );
        let block_reuse = DedupAdmittedBlockReuse::from_parts(
            materialization.block_reuse.branch_id,
            materialization.block_reuse.frontier_commit_id,
            materialization.block_reuse.scope_class,
            materialization.block_reuse.structural_block_id,
            materialization.block_reuse.equivalence_contract_version,
            materialization.block_reuse.slice_ids,
        );
        let witness = ChunkDeterminismWitness::new(
            materialization.frozen_layout.witness.physical_chunk_id,
            materialization.frozen_layout.witness.chunk_shape_version,
            materialization.frozen_layout.witness.determinism_digest,
            materialization.frozen_layout.witness.ordered_slice_ids,
        );
        let frozen_layout = ChunkModelFrozenPhysicalLayout::new(
            materialization.frozen_layout.request,
            materialization.frozen_layout.chunk_width,
            witness,
        );
        let milestone_7_reference = Milestone7IndependentLayoutReference::new(
            materialization.milestone_7_reference.branch_id,
            materialization.milestone_7_reference.frontier_commit_id,
            materialization.milestone_7_reference.scope_class,
            materialization.milestone_7_reference.projection_digest,
        );
        let milestone_9_reference = Milestone9PhysicalChunkReference::new(
            materialization.milestone_9_reference.physical_chunk_id,
            materialization.milestone_9_reference.chunk_shape_version,
            materialization.milestone_9_reference.determinism_digest,
            materialization.milestone_9_reference.chunk_member_count,
        );
        Ok(Milestone6LayoutMaterialization::new(
            materialization.artifact_id,
            admitted_plan,
            block_reuse,
            frozen_layout,
            milestone_7_reference,
            milestone_9_reference,
            materialization.semantic_truth_digest,
            materialization.authoritative_commit_count,
        ))
    }
}

fn validate_persisted_milestone_6_layout_materialization_record(
    record: &PersistedMilestone6LayoutMaterializationRecord,
) -> Result<(), String> {
    validate_persisted_milestone_6_layout_materialization(&record.materialization)?;
    if record.artifact_id != record.materialization.artifact_id {
        return Err(format!(
            "persisted milestone 6 materialization record key `{}` drifted from payload artifact id `{}`",
            record.artifact_id, record.materialization.artifact_id
        ));
    }
    Ok(())
}

fn validate_persisted_milestone_6_layout_materialization(
    materialization: &PersistedMilestone6LayoutMaterialization,
) -> Result<(), String> {
    let expected_plan = match crate::layout::classify_layout_request(
        materialization.admitted_plan.request.clone(),
    )
    .map_err(|error| error.to_string())?
    {
        crate::AspectLayoutReadPlanDecision::Admitted(plan) => plan,
        crate::AspectLayoutReadPlanDecision::Fallback(plan) => {
            return Err(format!(
                "persisted milestone 6 materialization `{}` referenced a request that now classifies as fallback: {}",
                materialization.artifact_id,
                plan.reason()
            ));
        }
        crate::AspectLayoutReadPlanDecision::Rejected(plan) => {
            return Err(format!(
                "persisted milestone 6 materialization `{}` referenced a request that now classifies as rejected: {}",
                materialization.artifact_id,
                plan.reason()
            ));
        }
    };
    let expected_block_reuse = DedupAdmittedBlockReuse::new(
        &expected_plan,
        materialization.block_reuse.equivalence_contract_version,
    );
    let expected_frozen_layout =
        crate::layout::freeze_chunk_model_from_plan(&expected_plan).map_err(|error| error.to_string())?;
    let expected_milestone_7_reference =
        crate::layout::admit_milestone_7_reference_from_plan(&expected_plan)
            .map_err(|error| error.to_string())?;
    let expected_milestone_9_reference =
        crate::layout::admit_milestone_9_reference_from_frozen(&expected_frozen_layout);
    let expected_artifact_id = crate::layout::layout_materialization_artifact_id(&expected_plan);

    if materialization.artifact_id != expected_artifact_id {
        return Err(format!(
            "persisted milestone 6 materialization artifact id `{}` did not match expected `{expected_artifact_id}`",
            materialization.artifact_id
        ));
    }

    let expected_plan_persisted = PersistedAdmittedAspectLayoutReadPlan::from(&expected_plan);
    if materialization.admitted_plan != expected_plan_persisted {
        return Err(format!(
            "persisted milestone 6 materialization `{}` drifted from the canonical admitted layout plan for its request",
            materialization.artifact_id
        ));
    }

    let expected_block_reuse_persisted = PersistedDedupAdmittedBlockReuse::from(&expected_block_reuse);
    if materialization.block_reuse != expected_block_reuse_persisted {
        return Err(format!(
            "persisted milestone 6 materialization `{}` drifted from the canonical structural block reuse witness for its admitted plan",
            materialization.artifact_id
        ));
    }

    let expected_frozen_layout_persisted =
        PersistedChunkModelFrozenPhysicalLayout::from(&expected_frozen_layout);
    if materialization.frozen_layout != expected_frozen_layout_persisted {
        return Err(format!(
            "persisted milestone 6 materialization `{}` drifted from the canonical frozen chunk layout for its admitted plan",
            materialization.artifact_id
        ));
    }

    let expected_milestone_7_reference_persisted =
        PersistedMilestone7IndependentLayoutReference::from(&expected_milestone_7_reference);
    if materialization.milestone_7_reference != expected_milestone_7_reference_persisted {
        return Err(format!(
            "persisted milestone 6 materialization `{}` drifted from the canonical Milestone 7 reference for its admitted plan",
            materialization.artifact_id
        ));
    }

    let expected_milestone_9_reference_persisted =
        PersistedMilestone9PhysicalChunkReference::from(&expected_milestone_9_reference);
    if materialization.milestone_9_reference != expected_milestone_9_reference_persisted {
        return Err(format!(
            "persisted milestone 6 materialization `{}` drifted from the canonical Milestone 9 physical chunk reference for its frozen layout",
            materialization.artifact_id
        ));
    }
    if materialization.semantic_truth_digest.is_empty() {
        return Err(format!(
            "persisted milestone 6 materialization `{}` was missing semantic truth digest",
            materialization.artifact_id
        ));
    }
    if materialization.authoritative_commit_count == 0 {
        return Err(format!(
            "persisted milestone 6 materialization `{}` was missing authoritative commit count",
            materialization.artifact_id
        ));
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BulkProgramIdentityRecord {
    pub artifact_id: String,
    pub family_version: u32,
    pub kind: BulkPlanKind,
    pub program_id: String,
    pub source_identity: String,
    pub target_branch_scope: BranchId,
    pub basis_commit_id: Option<CommitId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrozenBulkManifestRecord {
    pub artifact_id: String,
    pub family_version: u32,
    pub program_id: String,
    pub manifest: FrozenBulkSourceManifest,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrozenTransformBasisRecord {
    pub artifact_id: String,
    pub family_version: u32,
    pub program_id: String,
    pub basis: FrozenTransformBasis,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrozenTransformPartitionRecord {
    pub artifact_id: String,
    pub family_version: u32,
    pub program_id: String,
    pub partition: FrozenTransformTargetPartition,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BulkDeterministicPlanRecord {
    pub artifact_id: String,
    pub family_version: u32,
    pub program_id: String,
    pub plan: DeterministicChunkPlan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BulkProgressCheckpointRecord {
    pub artifact_id: String,
    pub family_version: u32,
    pub program_id: String,
    pub plan_id: String,
    pub checkpoint: PublishedBulkProgressCheckpoint,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BulkChunkWitnessRecord {
    pub artifact_id: String,
    pub family_version: u32,
    pub program_id: String,
    pub plan_id: String,
    pub witness: BulkChunkCommitWitness,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProgramChunkWitnessIndexRecord {
    pub artifact_id: String,
    pub family_version: u32,
    pub program_id: String,
    pub plan_id: String,
    pub index: ProgramChunkWitnessIndex,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotBasisRecord {
    pub snapshot_id: SnapshotId,
    pub snapshot_family_version: u32,
    pub snapshot_basis_version: u32,
    pub snapshot_image_format_version: u32,
    pub snapshot_branch_id: BranchId,
    pub snapshot_frontier_commit_id: CommitId,
    pub snapshot_history_range: Vec<CommitId>,
    pub snapshot_canonicalization_version: u32,
    pub snapshot_authority_digest: String,
    pub snapshot_image_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotImageRecord {
    pub snapshot_id: SnapshotId,
    pub image: SnapshotImageBundle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct StoreState {
    pub canonicalization_version: u32,
    pub next_commit_sequence: u64,
    pub next_head_update_sequence: u64,
    pub branch_records: BTreeMap<String, BranchRecord>,
    pub branch_head_records: BTreeMap<String, BranchHeadRecord>,
    pub commit_envelopes: BTreeMap<u64, StoredCommitEnvelope>,
    pub commit_parent_records: BTreeMap<String, CommitParentRecord>,
    pub authoritative_artifact_digests: BTreeMap<String, AuthoritativeArtifactDigestRecord>,
    #[serde(default)]
    pub commit_support_summaries: BTreeMap<u64, CommitSupportSummaryRecord>,
    #[serde(default)]
    pub schema_support_records: BTreeMap<String, SchemaSupportRecord>,
    #[serde(default)]
    pub lineage_support_records: BTreeMap<String, LineageSupportRecord>,
    #[serde(default)]
    pub durable_cursor_identity_records: BTreeMap<String, DurableCursorIdentityRecord>,
    #[serde(default)]
    pub subscriber_checkpoint_records: BTreeMap<String, SubscriberCheckpointRecord>,
    #[serde(default)]
    pub branch_shared_base_records: BTreeMap<String, BranchSharedBaseRecord>,
    #[serde(default)]
    pub next_branch_delta_layer_id: u64,
    #[serde(default)]
    pub branch_delta_layer_records: BTreeMap<u64, BranchDeltaLayerRecord>,
    #[serde(default)]
    pub embedded_checkpoint_records: BTreeMap<String, EmbeddedCheckpointRecord>,
    #[serde(default)]
    pub milestone_6_layout_materialization_records:
        BTreeMap<String, Milestone6LayoutMaterializationRecord>,
    #[serde(default)]
    pub milestone_6_commit_coupled_layout_seed_records:
        BTreeMap<String, Milestone6CommitCoupledLayoutSeedRecord>,
    #[serde(default)]
    pub milestone_6_scope_slice_membership_records:
        BTreeMap<String, Milestone6ScopeSliceMembershipRecord>,
    #[serde(default)]
    pub milestone_6_chunk_membership_records:
        BTreeMap<String, Milestone6ChunkMembershipRecord>,
    #[serde(default)]
    pub milestone_6_structural_block_records:
        BTreeMap<String, Milestone6StructuralBlockRecord>,
    #[serde(default)]
    pub bulk_program_identity_records: BTreeMap<String, BulkProgramIdentityRecord>,
    #[serde(default)]
    pub frozen_bulk_manifest_records: BTreeMap<String, FrozenBulkManifestRecord>,
    #[serde(default)]
    pub frozen_transform_basis_records: BTreeMap<String, FrozenTransformBasisRecord>,
    #[serde(default)]
    pub frozen_transform_partition_records: BTreeMap<String, FrozenTransformPartitionRecord>,
    #[serde(default)]
    pub bulk_deterministic_plan_records: BTreeMap<String, BulkDeterministicPlanRecord>,
    #[serde(default)]
    pub bulk_progress_checkpoint_records: BTreeMap<String, BulkProgressCheckpointRecord>,
    #[serde(default)]
    pub bulk_chunk_witness_records: BTreeMap<String, BulkChunkWitnessRecord>,
    #[serde(default)]
    pub program_chunk_witness_index_records: BTreeMap<String, ProgramChunkWitnessIndexRecord>,
    #[serde(default)]
    pub next_snapshot_id: u64,
    #[serde(default)]
    pub snapshot_basis_records: BTreeMap<u64, SnapshotBasisRecord>,
    #[serde(default)]
    pub snapshot_image_records: BTreeMap<u64, SnapshotImageRecord>,
    #[serde(default)]
    pub next_durable_mutation_id: u64,
    #[serde(default)]
    pub next_wal_sequence: u64,
    #[serde(default)]
    pub wal_records: BTreeMap<u64, WalRecord>,
}
