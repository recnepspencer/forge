use crate::{
    authority::{
        AuthoritativeBranchHeadRecord, AuthoritativeExportBundle, CanonicalizedCommitEnvelope,
        DurableCursorAcknowledgeRequest, DurableCursorResumePlan, DurableCursorResumeRequest,
        FetchedAuthoritativeCommit, FetchedDurableCursorIdentity, FetchedLineageSupportArtifact,
        FetchedSchemaSupportArtifact, HistoricalIdentityRequest, HistoricalIdentityResolution,
        PersistedAuthoritativeCommit, PersistedSubscriberCheckpoint, VerifiedAuthoritativeAppend,
    },
    bulk::{
        BulkChunkCommitWitness, BulkExecutionPath, DeterministicChunkPlan,
        FrozenBulkSourceManifest, FrozenTransformBasis, FrozenTransformTargetPartition,
        ProgramChunkWitnessIndex, PublishedBulkProgressCheckpoint, ResumeBoundaryCandidate,
    },
    delta::{
        BranchDeltaAutoCompactOutcome, BranchDeltaReadPlan, BranchDeltaReadRequest,
        BranchDeltaReadResult, BranchDeltaRebuildReceipt, BranchDeltaRewritePlan,
        BranchDeltaRewriteReceipt, BranchDeltaRewriteRecommendation, BranchDeltaRewriteRequest,
        SameBranchDescendantWitness, SharedBaseBranchCreationReceipt,
        SharedBaseBranchCreationRequest, SharedBaseBranchCreationWitness,
    },
    evidence::{
        CanonicalizationMetrics, Milestone7AccessStructureVerification, StoreCounterSnapshot,
    },
    failure::StoreError,
    layout::{
        AdmittedAspectLayoutReadPlan, AspectLayoutReadExecutionDecision,
        AspectLayoutReadPlanDecision, AspectLayoutReadRequest, ChunkModelFrozenPhysicalLayout,
        DedupAdmittedBlockReuse, DedupBackedReadResult, Milestone7IndependentLayoutReference,
        Milestone9PhysicalChunkReference, StructuralBlockLookup, StructuralBlockLookupResult,
    },
    media::DurableMediaReport,
    publication::PublicationWriteOutcome,
    recovery::{
        BackupRestoreCompatibilityReport, DurableRecoveryOutcome, DurableRecoveryPlan,
        DurableRetryResolution, MaintenanceRecoveryReport, SnapshotMaintenanceRecoveryReport,
    },
    snapshot::{
        PublishedSnapshotHandle, SnapshotCaptureRequest, SnapshotId, SnapshotImageBundle,
        SnapshotReadRequest, SnapshotReadResult, SnapshotRestoreOutcome, SnapshotRestorePlan,
        SnapshotRestoreRequest,
    },
    wal::{DurableMutationId, DurablePublicationPhase},
};
use forge_relational::facade::history::{BranchId, CommitId};
use std::path::PathBuf;

use super::{
    embedded::{EmbeddedBackendMode, EmbeddedStoreBackend},
    records::EmbeddedCheckpointRecord,
    sqlite::SqliteStoreBackend,
};

#[derive(Debug, Clone)]
pub enum StoreBackendMode {
    InMemory,
    LocalFile(PathBuf),
    SqliteFile(PathBuf),
}

#[derive(Debug)]
pub enum StoreBackend {
    Embedded(EmbeddedStoreBackend),
    Sqlite(SqliteStoreBackend),
}

impl StoreBackend {
    pub fn open(mode: StoreBackendMode) -> Result<Self, StoreError> {
        match mode {
            StoreBackendMode::InMemory => Ok(Self::Embedded(EmbeddedStoreBackend::open(
                EmbeddedBackendMode::InMemory,
            )?)),
            StoreBackendMode::LocalFile(path) => Ok(Self::Embedded(EmbeddedStoreBackend::open(
                EmbeddedBackendMode::LocalFile(path),
            )?)),
            StoreBackendMode::SqliteFile(path) => Ok(Self::Sqlite(SqliteStoreBackend::open(path)?)),
        }
    }

    pub fn open_for_durable_recovery(mode: StoreBackendMode) -> Result<Self, StoreError> {
        match mode {
            StoreBackendMode::InMemory => Ok(Self::Embedded(
                EmbeddedStoreBackend::open_for_durable_recovery(EmbeddedBackendMode::InMemory)?,
            )),
            StoreBackendMode::LocalFile(path) => Ok(Self::Embedded(
                EmbeddedStoreBackend::open_for_durable_recovery(EmbeddedBackendMode::LocalFile(
                    path,
                ))?,
            )),
            StoreBackendMode::SqliteFile(path) => Ok(Self::Sqlite(
                SqliteStoreBackend::open_for_durable_recovery(path)?,
            )),
        }
    }

    pub fn from_export_bundle(bundle: AuthoritativeExportBundle) -> Result<Self, StoreError> {
        Ok(Self::Embedded(EmbeddedStoreBackend::from_export_bundle(
            bundle,
        )?))
    }

    pub fn create_branch(
        &mut self,
        new_branch: BranchId,
        from_branch: Option<&BranchId>,
    ) -> Result<AuthoritativeBranchHeadRecord, StoreError> {
        match self {
            Self::Embedded(backend) => backend.create_branch(new_branch, from_branch),
            Self::Sqlite(backend) => backend.create_branch(new_branch, from_branch),
        }
    }

    pub fn create_shared_base_branch(
        &mut self,
        request: SharedBaseBranchCreationRequest,
    ) -> Result<SharedBaseBranchCreationReceipt, StoreError> {
        match self {
            Self::Embedded(backend) => backend.create_shared_base_branch(request),
            Self::Sqlite(backend) => backend.create_shared_base_branch(request),
        }
    }

    pub fn admit_shared_base_branch_creation(
        &self,
        request: SharedBaseBranchCreationRequest,
    ) -> Result<SharedBaseBranchCreationWitness, StoreError> {
        match self {
            Self::Embedded(backend) => backend.admit_shared_base_branch_creation(request),
            Self::Sqlite(backend) => backend.admit_shared_base_branch_creation(request),
        }
    }

    pub fn verify_append(
        &self,
        append: CanonicalizedCommitEnvelope,
    ) -> Result<VerifiedAuthoritativeAppend, StoreError> {
        match self {
            Self::Embedded(backend) => backend.verify_append(append),
            Self::Sqlite(backend) => backend.verify_append(append),
        }
    }

    pub fn append(
        &mut self,
        verified: VerifiedAuthoritativeAppend,
    ) -> Result<PersistedAuthoritativeCommit, StoreError> {
        match self {
            Self::Embedded(backend) => backend.append(verified),
            Self::Sqlite(backend) => backend.append(verified),
        }
    }

    pub fn fetch_commit(
        &self,
        commit_id: CommitId,
    ) -> Result<FetchedAuthoritativeCommit, StoreError> {
        match self {
            Self::Embedded(backend) => backend.fetch_commit(commit_id),
            Self::Sqlite(backend) => backend.fetch_commit(commit_id),
        }
    }

    pub fn fetch_branch_head(
        &self,
        branch_id: &BranchId,
    ) -> Result<AuthoritativeBranchHeadRecord, StoreError> {
        match self {
            Self::Embedded(backend) => backend.fetch_branch_head(branch_id),
            Self::Sqlite(backend) => backend.fetch_branch_head(branch_id),
        }
    }

    pub fn plan_branch_delta_read(
        &self,
        request: BranchDeltaReadRequest,
    ) -> Result<BranchDeltaReadPlan, StoreError> {
        match self {
            Self::Embedded(backend) => backend.plan_branch_delta_read(request),
            Self::Sqlite(backend) => backend.plan_branch_delta_read(request),
        }
    }

    pub fn admit_same_branch_descendant(
        &self,
        request: BranchDeltaReadRequest,
    ) -> Result<SameBranchDescendantWitness, StoreError> {
        match self {
            Self::Embedded(backend) => backend.admit_same_branch_descendant(request),
            Self::Sqlite(backend) => backend.admit_same_branch_descendant(request),
        }
    }

    pub fn admit_milestone_7_independent_reference(
        &self,
        request: BranchDeltaReadRequest,
    ) -> Result<crate::Milestone7IndependentReference, StoreError> {
        match self {
            Self::Embedded(backend) => backend.admit_milestone_7_independent_reference(request),
            Self::Sqlite(backend) => backend.admit_milestone_7_independent_reference(request),
        }
    }

    pub fn plan_aspect_layout_read(
        &self,
        request: AspectLayoutReadRequest,
    ) -> Result<AspectLayoutReadPlanDecision, StoreError> {
        match self {
            Self::Embedded(backend) => backend.plan_aspect_layout_read(request),
            Self::Sqlite(backend) => backend.plan_aspect_layout_read(request),
        }
    }

    pub fn admit_structural_block_reuse(
        &self,
        plan: AdmittedAspectLayoutReadPlan,
    ) -> Result<DedupAdmittedBlockReuse, StoreError> {
        match self {
            Self::Embedded(backend) => backend.admit_structural_block_reuse(plan),
            Self::Sqlite(backend) => backend.admit_structural_block_reuse(plan),
        }
    }

    pub fn freeze_chunk_model(
        &self,
        plan: AdmittedAspectLayoutReadPlan,
    ) -> Result<ChunkModelFrozenPhysicalLayout, StoreError> {
        match self {
            Self::Embedded(backend) => backend.freeze_chunk_model(plan),
            Self::Sqlite(backend) => backend.freeze_chunk_model(plan),
        }
    }

    pub fn admit_milestone_7_independent_layout_reference(
        &self,
        plan: AdmittedAspectLayoutReadPlan,
    ) -> Result<Milestone7IndependentLayoutReference, StoreError> {
        match self {
            Self::Embedded(backend) => backend.admit_milestone_7_independent_layout_reference(plan),
            Self::Sqlite(backend) => backend.admit_milestone_7_independent_layout_reference(plan),
        }
    }

    pub fn admit_milestone_9_physical_chunk_reference(
        &self,
        frozen: ChunkModelFrozenPhysicalLayout,
    ) -> Result<Milestone9PhysicalChunkReference, StoreError> {
        match self {
            Self::Embedded(backend) => backend.admit_milestone_9_physical_chunk_reference(frozen),
            Self::Sqlite(backend) => backend.admit_milestone_9_physical_chunk_reference(frozen),
        }
    }

    pub fn materialize_milestone_6_layout_support(
        &mut self,
        request: AspectLayoutReadRequest,
    ) -> Result<crate::Milestone6LayoutMaterialization, StoreError> {
        match self {
            Self::Embedded(backend) => backend.materialize_milestone_6_layout_support(request),
            Self::Sqlite(backend) => backend.materialize_milestone_6_layout_support(request),
        }
    }

    pub fn fetch_milestone_6_layout_support(
        &self,
        request: AspectLayoutReadRequest,
    ) -> Result<crate::Milestone6LayoutMaterialization, StoreError> {
        match self {
            Self::Embedded(backend) => backend.fetch_milestone_6_layout_support(request),
            Self::Sqlite(backend) => backend.fetch_milestone_6_layout_support(request),
        }
    }

    pub(crate) fn note_milestone_6_scope_prepare(
        &mut self,
        request: &AspectLayoutReadRequest,
    ) -> Result<u64, StoreError> {
        match self {
            Self::Embedded(backend) => backend.note_milestone_6_scope_prepare(request),
            Self::Sqlite(backend) => backend.note_milestone_6_scope_prepare(request),
        }
    }

    pub(crate) fn milestone_6_branch_has_materialized_support(&self, branch_id: &BranchId) -> bool {
        match self {
            Self::Embedded(backend) => {
                backend.milestone_6_branch_has_materialized_support(branch_id)
            }
            Self::Sqlite(backend) => backend.milestone_6_branch_has_materialized_support(branch_id),
        }
    }

    pub(crate) fn record_milestone_6_proof_only_prepare(&self) {
        match self {
            Self::Embedded(backend) => backend.record_milestone_6_proof_only_prepare(),
            Self::Sqlite(backend) => backend.record_milestone_6_proof_only_prepare(),
        }
    }

    pub(crate) fn record_milestone_6_on_demand_materialize(&self) {
        match self {
            Self::Embedded(backend) => backend.record_milestone_6_on_demand_materialize(),
            Self::Sqlite(backend) => backend.record_milestone_6_on_demand_materialize(),
        }
    }

    pub(crate) fn record_milestone_6_policy_eager_resolution(&self) {
        match self {
            Self::Embedded(backend) => backend.record_milestone_6_policy_eager_resolution(),
            Self::Sqlite(backend) => backend.record_milestone_6_policy_eager_resolution(),
        }
    }

    pub(crate) fn record_milestone_6_policy_eager_publish(&self) {
        match self {
            Self::Embedded(backend) => backend.record_milestone_6_policy_eager_publish(),
            Self::Sqlite(backend) => backend.record_milestone_6_policy_eager_publish(),
        }
    }

    pub(crate) fn record_milestone_6_policy_eager_reuse_existing(&self) {
        match self {
            Self::Embedded(backend) => backend.record_milestone_6_policy_eager_reuse_existing(),
            Self::Sqlite(backend) => backend.record_milestone_6_policy_eager_reuse_existing(),
        }
    }

    pub(crate) fn fetch_existing_milestone_6_layout_support(
        &self,
        artifact_id: &str,
    ) -> Result<crate::Milestone6LayoutMaterialization, StoreError> {
        match self {
            Self::Embedded(backend) => {
                backend.fetch_existing_milestone_6_layout_support(artifact_id)
            }
            Self::Sqlite(backend) => backend.fetch_existing_milestone_6_layout_support(artifact_id),
        }
    }

    pub fn rebuild_milestone_6_derived_artifacts_from_materializations(
        &mut self,
    ) -> Result<crate::Milestone6DerivedArtifactRebuildReport, StoreError> {
        match self {
            Self::Embedded(backend) => {
                backend.rebuild_milestone_6_derived_artifacts_from_materializations()
            }
            Self::Sqlite(backend) => {
                backend.rebuild_milestone_6_derived_artifacts_from_materializations()
            }
        }
    }

    pub fn rebuild_milestone_6_derived_artifacts_from_authority(
        &mut self,
    ) -> Result<crate::Milestone6DerivedArtifactRebuildReport, StoreError> {
        match self {
            Self::Embedded(backend) => {
                backend.rebuild_milestone_6_derived_artifacts_from_authority()
            }
            Self::Sqlite(backend) => backend.rebuild_milestone_6_derived_artifacts_from_authority(),
        }
    }

    pub fn structural_block_lookup(
        &self,
        lookup: StructuralBlockLookup,
    ) -> Result<StructuralBlockLookupResult, StoreError> {
        match self {
            Self::Embedded(backend) => backend.structural_block_lookup(lookup),
            Self::Sqlite(backend) => backend.structural_block_lookup(lookup),
        }
    }

    pub fn execute_aspect_layout_read(
        &self,
        request: AspectLayoutReadRequest,
    ) -> Result<AspectLayoutReadExecutionDecision, StoreError> {
        match self {
            Self::Embedded(backend) => backend.execute_aspect_layout_read(request),
            Self::Sqlite(backend) => backend.execute_aspect_layout_read(request),
        }
    }

    pub fn execute_dedup_backed_read(
        &self,
        request: AspectLayoutReadRequest,
    ) -> Result<DedupBackedReadResult, StoreError> {
        match self {
            Self::Embedded(backend) => backend.execute_dedup_backed_read(request),
            Self::Sqlite(backend) => backend.execute_dedup_backed_read(request),
        }
    }

    pub fn read_branch_delta(
        &self,
        witness: SameBranchDescendantWitness,
    ) -> Result<BranchDeltaReadResult, StoreError> {
        match self {
            Self::Embedded(backend) => backend.read_branch_delta(witness),
            Self::Sqlite(backend) => backend.read_branch_delta(witness),
        }
    }

    pub fn read_branch_delta_control(
        &self,
        witness: SameBranchDescendantWitness,
    ) -> Result<BranchDeltaReadResult, StoreError> {
        match self {
            Self::Embedded(backend) => backend.read_branch_delta_control(witness),
            Self::Sqlite(backend) => backend.read_branch_delta_control(witness),
        }
    }

    pub fn read_branch_delta_control_from_milestone_7_reference(
        &self,
        reference: crate::Milestone7IndependentReference,
    ) -> Result<BranchDeltaReadResult, StoreError> {
        match self {
            Self::Embedded(backend) => {
                backend.read_branch_delta_control_from_milestone_7_reference(reference)
            }
            Self::Sqlite(backend) => {
                backend.read_branch_delta_control_from_milestone_7_reference(reference)
            }
        }
    }

    pub fn plan_delta_rewrite(
        &self,
        request: BranchDeltaRewriteRequest,
    ) -> Result<BranchDeltaRewritePlan, StoreError> {
        match self {
            Self::Embedded(backend) => backend.plan_delta_rewrite(request),
            Self::Sqlite(backend) => backend.plan_delta_rewrite(request),
        }
    }

    pub fn recommend_delta_rewrite(
        &self,
        request: BranchDeltaRewriteRequest,
    ) -> Result<BranchDeltaRewriteRecommendation, StoreError> {
        match self {
            Self::Embedded(backend) => backend.recommend_delta_rewrite(request),
            Self::Sqlite(backend) => backend.recommend_delta_rewrite(request),
        }
    }

    pub fn auto_compact_branch_delta(
        &mut self,
        request: BranchDeltaRewriteRequest,
    ) -> Result<BranchDeltaAutoCompactOutcome, StoreError> {
        match self {
            Self::Embedded(backend) => backend.auto_compact_branch_delta(request),
            Self::Sqlite(backend) => backend.auto_compact_branch_delta(request),
        }
    }

    pub fn rewrite_branch_delta(
        &mut self,
        plan: BranchDeltaRewritePlan,
    ) -> Result<BranchDeltaRewriteReceipt, StoreError> {
        match self {
            Self::Embedded(backend) => backend.rewrite_branch_delta(plan),
            Self::Sqlite(backend) => backend.rewrite_branch_delta(plan),
        }
    }

    pub fn rebuild_branch_delta_artifacts(
        &mut self,
        branch_id: BranchId,
    ) -> Result<BranchDeltaRebuildReceipt, StoreError> {
        match self {
            Self::Embedded(backend) => backend.rebuild_branch_delta_artifacts(branch_id),
            Self::Sqlite(backend) => backend.rebuild_branch_delta_artifacts(branch_id),
        }
    }

    pub fn fetch_schema_support(
        &self,
        commit_id: CommitId,
    ) -> Result<FetchedSchemaSupportArtifact, StoreError> {
        match self {
            Self::Embedded(backend) => backend.fetch_schema_support(commit_id),
            Self::Sqlite(backend) => backend.fetch_schema_support(commit_id),
        }
    }

    pub fn fetch_lineage_support(
        &self,
        commit_id: CommitId,
    ) -> Result<FetchedLineageSupportArtifact, StoreError> {
        match self {
            Self::Embedded(backend) => backend.fetch_lineage_support(commit_id),
            Self::Sqlite(backend) => backend.fetch_lineage_support(commit_id),
        }
    }

    pub fn fetch_lineage_history(
        &self,
        request: HistoricalIdentityRequest,
    ) -> Result<HistoricalIdentityResolution, StoreError> {
        match self {
            Self::Embedded(backend) => backend.fetch_lineage_history(request),
            Self::Sqlite(backend) => backend.fetch_lineage_history(request),
        }
    }

    pub fn acknowledge_cursor(
        &mut self,
        request: DurableCursorAcknowledgeRequest,
    ) -> Result<PersistedSubscriberCheckpoint, StoreError> {
        match self {
            Self::Embedded(backend) => backend.acknowledge_cursor(request),
            Self::Sqlite(backend) => backend.acknowledge_cursor(request),
        }
    }

    pub fn fetch_durable_cursor_identity(
        &self,
        cursor_id: &str,
    ) -> Result<FetchedDurableCursorIdentity, StoreError> {
        match self {
            Self::Embedded(backend) => backend.fetch_durable_cursor_identity(cursor_id),
            Self::Sqlite(backend) => backend.fetch_durable_cursor_identity(cursor_id),
        }
    }

    pub fn plan_cursor_resume(
        &self,
        request: DurableCursorResumeRequest,
    ) -> Result<DurableCursorResumePlan, StoreError> {
        match self {
            Self::Embedded(backend) => backend.plan_cursor_resume(request),
            Self::Sqlite(backend) => backend.plan_cursor_resume(request),
        }
    }

    pub fn record_canonicalization(&self, metrics: CanonicalizationMetrics) {
        match self {
            Self::Embedded(backend) => backend.record_canonicalization(metrics),
            Self::Sqlite(backend) => backend.record_canonicalization(metrics),
        }
    }

    pub fn record_bulk_source_manifest(&self, member_count: u64, stream_pass_count: u64) {
        match self {
            Self::Embedded(backend) => {
                backend.record_bulk_source_manifest(member_count, stream_pass_count)
            }
            Self::Sqlite(backend) => {
                backend.record_bulk_source_manifest(member_count, stream_pass_count)
            }
        }
    }

    pub fn record_bulk_chunk_plan(&self, chunk_count: u64) {
        match self {
            Self::Embedded(backend) => backend.record_bulk_chunk_plan(chunk_count),
            Self::Sqlite(backend) => backend.record_bulk_chunk_plan(chunk_count),
        }
    }

    pub fn record_bulk_checkpoint_publication_intent(
        &mut self,
        runtime_session_id: &str,
        durable_mutation_id: DurableMutationId,
        checkpoint_sequence: Option<u64>,
    ) -> Result<(), StoreError> {
        match self {
            Self::Embedded(backend) => backend.record_bulk_checkpoint_publication_intent(
                runtime_session_id,
                durable_mutation_id,
                checkpoint_sequence,
            ),
            Self::Sqlite(backend) => backend.record_bulk_checkpoint_publication_intent(
                runtime_session_id,
                durable_mutation_id,
                checkpoint_sequence,
            ),
        }
    }

    pub fn record_bulk_chunk_execute(
        &self,
        width_units: u64,
        memory_units: u64,
        fallback_breadth_units: u64,
        execution_path: BulkExecutionPath,
    ) {
        let used_fallback_path = matches!(execution_path, BulkExecutionPath::ExplicitFallbackPath);
        match self {
            Self::Embedded(backend) => backend.record_bulk_chunk_execute(
                width_units,
                memory_units,
                fallback_breadth_units,
                used_fallback_path,
            ),
            Self::Sqlite(backend) => backend.record_bulk_chunk_execute(
                width_units,
                memory_units,
                fallback_breadth_units,
                used_fallback_path,
            ),
        }
    }

    pub fn record_bulk_chunk_resume(&self) {
        match self {
            Self::Embedded(backend) => backend.record_bulk_chunk_resume(),
            Self::Sqlite(backend) => backend.record_bulk_chunk_resume(),
        }
    }

    pub fn record_bulk_chunk_commit(&self) {
        match self {
            Self::Embedded(backend) => backend.record_bulk_chunk_commit(),
            Self::Sqlite(backend) => backend.record_bulk_chunk_commit(),
        }
    }

    pub fn persist_frozen_bulk_manifest(
        &mut self,
        manifest: FrozenBulkSourceManifest,
    ) -> Result<FrozenBulkSourceManifest, StoreError> {
        match self {
            Self::Embedded(backend) => backend.persist_frozen_bulk_manifest(manifest),
            Self::Sqlite(backend) => backend.persist_frozen_bulk_manifest(manifest),
        }
    }

    pub fn persist_frozen_transform_basis(
        &mut self,
        basis: FrozenTransformBasis,
    ) -> Result<FrozenTransformBasis, StoreError> {
        match self {
            Self::Embedded(backend) => backend.persist_frozen_transform_basis(basis),
            Self::Sqlite(backend) => backend.persist_frozen_transform_basis(basis),
        }
    }

    pub fn persist_frozen_transform_partition(
        &mut self,
        partition: FrozenTransformTargetPartition,
    ) -> Result<FrozenTransformTargetPartition, StoreError> {
        match self {
            Self::Embedded(backend) => backend.persist_frozen_transform_partition(partition),
            Self::Sqlite(backend) => backend.persist_frozen_transform_partition(partition),
        }
    }

    pub fn persist_bulk_chunk_plan(
        &mut self,
        plan: DeterministicChunkPlan,
    ) -> Result<DeterministicChunkPlan, StoreError> {
        match self {
            Self::Embedded(backend) => backend.persist_bulk_chunk_plan(plan),
            Self::Sqlite(backend) => backend.persist_bulk_chunk_plan(plan),
        }
    }

    pub fn fetch_frozen_bulk_manifest(
        &self,
        program_id: &str,
        manifest_digest: &str,
    ) -> Result<FrozenBulkSourceManifest, StoreError> {
        match self {
            Self::Embedded(backend) => {
                backend.fetch_frozen_bulk_manifest(program_id, manifest_digest)
            }
            Self::Sqlite(backend) => {
                backend.fetch_frozen_bulk_manifest(program_id, manifest_digest)
            }
        }
    }

    pub fn fetch_frozen_transform_basis(
        &self,
        program_id: &str,
        basis_digest: &str,
    ) -> Result<FrozenTransformBasis, StoreError> {
        match self {
            Self::Embedded(backend) => {
                backend.fetch_frozen_transform_basis(program_id, basis_digest)
            }
            Self::Sqlite(backend) => backend.fetch_frozen_transform_basis(program_id, basis_digest),
        }
    }

    pub fn fetch_frozen_transform_partition(
        &self,
        program_id: &str,
        partition_digest: &str,
    ) -> Result<FrozenTransformTargetPartition, StoreError> {
        match self {
            Self::Embedded(backend) => {
                backend.fetch_frozen_transform_partition(program_id, partition_digest)
            }
            Self::Sqlite(backend) => {
                backend.fetch_frozen_transform_partition(program_id, partition_digest)
            }
        }
    }

    pub fn find_frozen_transform_basis_for_plan(
        &self,
        program_id: &str,
        target_branch_scope: &forge_relational::facade::history::BranchId,
        basis_commit_id: forge_relational::facade::history::CommitId,
    ) -> Result<FrozenTransformBasis, StoreError> {
        match self {
            Self::Embedded(backend) => backend.find_frozen_transform_basis_for_plan(
                program_id,
                target_branch_scope,
                basis_commit_id,
            ),
            Self::Sqlite(backend) => backend.find_frozen_transform_basis_for_plan(
                program_id,
                target_branch_scope,
                basis_commit_id,
            ),
        }
    }

    pub fn fetch_bulk_chunk_plan(
        &self,
        program_id: &str,
        plan_id: &str,
    ) -> Result<DeterministicChunkPlan, StoreError> {
        match self {
            Self::Embedded(backend) => backend.fetch_bulk_chunk_plan(program_id, plan_id),
            Self::Sqlite(backend) => backend.fetch_bulk_chunk_plan(program_id, plan_id),
        }
    }

    pub fn publish_bulk_chunk_witness(
        &mut self,
        witness: BulkChunkCommitWitness,
    ) -> Result<BulkChunkCommitWitness, StoreError> {
        match self {
            Self::Embedded(backend) => backend.publish_bulk_chunk_witness(witness),
            Self::Sqlite(backend) => backend.publish_bulk_chunk_witness(witness),
        }
    }

    pub fn publish_bulk_progress_checkpoint(
        &mut self,
        witness: BulkChunkCommitWitness,
    ) -> Result<PublishedBulkProgressCheckpoint, StoreError> {
        match self {
            Self::Embedded(backend) => backend.publish_bulk_progress_checkpoint(witness),
            Self::Sqlite(backend) => backend.publish_bulk_progress_checkpoint(witness),
        }
    }

    pub fn fetch_bulk_progress_checkpoint(
        &self,
        program_id: &str,
        plan_id: &str,
    ) -> Result<PublishedBulkProgressCheckpoint, StoreError> {
        match self {
            Self::Embedded(backend) => backend.fetch_bulk_progress_checkpoint(program_id, plan_id),
            Self::Sqlite(backend) => backend.fetch_bulk_progress_checkpoint(program_id, plan_id),
        }
    }

    pub fn fetch_program_chunk_witness_index(
        &self,
        program_id: &str,
        plan_id: &str,
    ) -> Result<ProgramChunkWitnessIndex, StoreError> {
        match self {
            Self::Embedded(backend) => {
                backend.fetch_program_chunk_witness_index(program_id, plan_id)
            }
            Self::Sqlite(backend) => backend.fetch_program_chunk_witness_index(program_id, plan_id),
        }
    }

    pub fn fetch_latest_resume_boundary(
        &self,
        program_id: &str,
        plan_id: &str,
    ) -> Result<ResumeBoundaryCandidate, StoreError> {
        match self {
            Self::Embedded(backend) => backend.fetch_latest_resume_boundary(program_id, plan_id),
            Self::Sqlite(backend) => backend.fetch_latest_resume_boundary(program_id, plan_id),
        }
    }

    pub fn counter_snapshot(&self) -> StoreCounterSnapshot {
        match self {
            Self::Embedded(backend) => backend.counter_snapshot(),
            Self::Sqlite(backend) => backend.counter_snapshot(),
        }
    }

    pub(crate) fn record_physical_chunk_export(&self, chunk_width: u64) {
        match self {
            Self::Embedded(backend) => backend.record_physical_chunk_export(chunk_width),
            Self::Sqlite(backend) => backend.record_physical_chunk_export(chunk_width),
        }
    }

    pub fn export_bundle(&self) -> AuthoritativeExportBundle {
        match self {
            Self::Embedded(backend) => backend.export_bundle(),
            Self::Sqlite(backend) => backend.export_bundle(),
        }
    }

    pub fn durable_media_report(&self) -> DurableMediaReport {
        match self {
            Self::Embedded(backend) => backend.durable_media_report(),
            Self::Sqlite(backend) => backend.durable_media_report(),
        }
    }

    pub fn milestone_7_access_structure_verification(
        &self,
    ) -> Milestone7AccessStructureVerification {
        match self {
            Self::Embedded(backend) => backend.milestone_7_access_structure_verification(),
            Self::Sqlite(backend) => backend.milestone_7_access_structure_verification(),
        }
    }

    pub fn milestone_6_access_structure_verification(
        &self,
    ) -> crate::Milestone6AccessStructureVerification {
        match self {
            Self::Embedded(backend) => backend.milestone_6_access_structure_verification(),
            Self::Sqlite(backend) => backend.milestone_6_access_structure_verification(),
        }
    }

    pub fn classify_durable_publication(
        &self,
        durable_mutation_id: DurableMutationId,
        expected_commit_id: Option<CommitId>,
    ) -> Result<PublicationWriteOutcome, StoreError> {
        match self {
            Self::Embedded(backend) => {
                backend.classify_durable_publication(durable_mutation_id, expected_commit_id)
            }
            Self::Sqlite(backend) => {
                backend.classify_durable_publication(durable_mutation_id, expected_commit_id)
            }
        }
    }

    pub fn classify_snapshot_publication(
        &self,
        snapshot_id: SnapshotId,
    ) -> Result<PublicationWriteOutcome, StoreError> {
        match self {
            Self::Embedded(backend) => backend.classify_snapshot_publication(snapshot_id),
            Self::Sqlite(backend) => backend.classify_snapshot_publication(snapshot_id),
        }
    }

    pub fn classify_snapshot_maintenance_recovery(
        &self,
        snapshot_id: SnapshotId,
    ) -> Result<SnapshotMaintenanceRecoveryReport, StoreError> {
        match self {
            Self::Embedded(backend) => backend.classify_snapshot_maintenance_recovery(snapshot_id),
            Self::Sqlite(backend) => backend.classify_snapshot_maintenance_recovery(snapshot_id),
        }
    }

    pub fn maintenance_recovery_report(&self) -> Result<MaintenanceRecoveryReport, StoreError> {
        match self {
            Self::Embedded(backend) => backend.maintenance_recovery_report(),
            Self::Sqlite(backend) => backend.maintenance_recovery_report(),
        }
    }

    pub fn support_artifact_recovery_report(&self) -> crate::SupportArtifactRecoveryReport {
        match self {
            Self::Embedded(backend) => backend.support_artifact_recovery_report(),
            Self::Sqlite(backend) => backend.support_artifact_recovery_report(),
        }
    }

    pub fn record_support_artifact_recovery_gap(&self, count: u64) {
        match self {
            Self::Embedded(backend) => backend
                .counters()
                .record_support_artifact_recovery_gap(count),
            Self::Sqlite(backend) => backend
                .counters()
                .record_support_artifact_recovery_gap(count),
        }
    }

    pub(crate) fn milestone_5_delta_storage_report(
        &self,
        branch_id: BranchId,
        target_commit_id: CommitId,
        direct_plan: &BranchDeltaReadPlan,
        control_plan: &BranchDeltaReadPlan,
    ) -> Result<crate::Milestone5DeltaStorageReport, StoreError> {
        match self {
            Self::Embedded(backend) => backend.milestone_5_delta_storage_report(
                branch_id,
                target_commit_id,
                direct_plan,
                control_plan,
            ),
            Self::Sqlite(backend) => backend.milestone_5_delta_storage_report(
                branch_id,
                target_commit_id,
                direct_plan,
                control_plan,
            ),
        }
    }

    pub fn backup_restore_compatibility_report(
        &self,
    ) -> Result<BackupRestoreCompatibilityReport, StoreError> {
        match self {
            Self::Embedded(backend) => backend.backup_restore_compatibility_report(),
            Self::Sqlite(backend) => backend.backup_restore_compatibility_report(),
        }
    }

    pub fn persist_embedded_checkpoint(
        &mut self,
        record: EmbeddedCheckpointRecord,
    ) -> Result<EmbeddedCheckpointRecord, StoreError> {
        match self {
            Self::Embedded(backend) => backend.persist_embedded_checkpoint(record),
            Self::Sqlite(backend) => backend.persist_embedded_checkpoint(record),
        }
    }

    pub fn fetch_embedded_checkpoint(
        &self,
        checkpoint_id: &str,
    ) -> Result<EmbeddedCheckpointRecord, StoreError> {
        match self {
            Self::Embedded(backend) => backend.fetch_embedded_checkpoint(checkpoint_id),
            Self::Sqlite(backend) => backend.fetch_embedded_checkpoint(checkpoint_id),
        }
    }

    pub fn capture_snapshot(
        &mut self,
        request: SnapshotCaptureRequest,
    ) -> Result<PublishedSnapshotHandle, StoreError> {
        match self {
            Self::Embedded(backend) => backend.capture_snapshot(request),
            Self::Sqlite(backend) => backend.capture_snapshot(request),
        }
    }

    pub fn read_snapshot(
        &self,
        request: SnapshotReadRequest,
    ) -> Result<SnapshotReadResult, StoreError> {
        match self {
            Self::Embedded(backend) => backend.read_snapshot(request),
            Self::Sqlite(backend) => backend.read_snapshot(request),
        }
    }

    pub fn plan_snapshot_restore(
        &self,
        request: SnapshotRestoreRequest,
    ) -> Result<SnapshotRestorePlan, StoreError> {
        match self {
            Self::Embedded(backend) => backend.plan_snapshot_restore(request),
            Self::Sqlite(backend) => backend.plan_snapshot_restore(request),
        }
    }

    pub fn execute_snapshot_restore(
        &self,
        plan: SnapshotRestorePlan,
    ) -> Result<SnapshotRestoreOutcome, StoreError> {
        match self {
            Self::Embedded(backend) => backend.execute_snapshot_restore(plan),
            Self::Sqlite(backend) => backend.execute_snapshot_restore(plan),
        }
    }

    pub fn rebuild_snapshot(
        &self,
        snapshot_id: SnapshotId,
    ) -> Result<SnapshotImageBundle, StoreError> {
        match self {
            Self::Embedded(backend) => backend.rebuild_snapshot(snapshot_id),
            Self::Sqlite(backend) => backend.rebuild_snapshot(snapshot_id),
        }
    }

    #[cfg(test)]
    pub fn remove_snapshot_image_for_test(
        &mut self,
        snapshot_id: SnapshotId,
    ) -> Result<(), StoreError> {
        match self {
            Self::Embedded(backend) => backend.remove_snapshot_image_for_test(snapshot_id),
            Self::Sqlite(backend) => backend.remove_snapshot_image_for_test(snapshot_id),
        }
    }

    #[cfg(test)]
    pub fn remove_snapshot_basis_for_test(
        &mut self,
        snapshot_id: SnapshotId,
    ) -> Result<(), StoreError> {
        match self {
            Self::Embedded(backend) => backend.remove_snapshot_basis_for_test(snapshot_id),
            Self::Sqlite(backend) => backend.remove_snapshot_basis_for_test(snapshot_id),
        }
    }

    #[cfg(test)]
    pub fn clear_branch_heads_for_test(&mut self) -> Result<(), StoreError> {
        match self {
            Self::Embedded(backend) => backend.clear_branch_heads_for_test(),
            Self::Sqlite(backend) => backend.clear_branch_heads_for_test(),
        }
    }

    #[cfg(test)]
    pub fn corrupt_snapshot_basis_digest_for_test(
        &mut self,
        snapshot_id: SnapshotId,
    ) -> Result<(), StoreError> {
        match self {
            Self::Embedded(backend) => backend.corrupt_snapshot_basis_digest_for_test(snapshot_id),
            Self::Sqlite(backend) => backend.corrupt_snapshot_basis_digest_for_test(snapshot_id),
        }
    }

    pub fn record_durable_mode_selection(&self) {
        match self {
            Self::Embedded(backend) => backend.counters().record_durable_mode_selection(),
            Self::Sqlite(backend) => backend.counters().record_durable_mode_selection(),
        }
    }

    pub fn record_embedded_mode_selection(&self) {
        match self {
            Self::Embedded(backend) => backend.counters().record_embedded_mode_selection(),
            Self::Sqlite(backend) => backend.counters().record_embedded_mode_selection(),
        }
    }

    pub fn record_hosted_runtime_start(&self) {
        match self {
            Self::Embedded(backend) => backend.counters().record_hosted_runtime_start(),
            Self::Sqlite(backend) => backend.counters().record_hosted_runtime_start(),
        }
    }

    pub fn record_hosted_runtime_stop(&self) {
        match self {
            Self::Embedded(backend) => backend.counters().record_hosted_runtime_stop(),
            Self::Sqlite(backend) => backend.counters().record_hosted_runtime_stop(),
        }
    }

    pub fn record_external_commit_intake(&self) {
        match self {
            Self::Embedded(backend) => backend.counters().record_external_commit_intake(),
            Self::Sqlite(backend) => backend.counters().record_external_commit_intake(),
        }
    }

    pub fn record_external_checkpoint_intake(&self) {
        match self {
            Self::Embedded(backend) => backend.counters().record_external_checkpoint_intake(),
            Self::Sqlite(backend) => backend.counters().record_external_checkpoint_intake(),
        }
    }

    #[cfg(test)]
    pub fn record_embedded_checkpoint_authority_rejection(&self) {
        match self {
            Self::Embedded(backend) => backend
                .counters()
                .record_embedded_checkpoint_authority_rejection(),
            Self::Sqlite(backend) => backend
                .counters()
                .record_embedded_checkpoint_authority_rejection(),
        }
    }

    #[cfg(test)]
    pub fn record_mode_misuse_rejection(&self) {
        match self {
            Self::Embedded(backend) => backend.counters().record_mode_misuse_rejection(),
            Self::Sqlite(backend) => backend.counters().record_mode_misuse_rejection(),
        }
    }

    pub fn record_durable_commit_acknowledged(&self) {
        match self {
            Self::Embedded(backend) => backend.counters().record_durable_commit_acknowledged(),
            Self::Sqlite(backend) => backend.counters().record_durable_commit_acknowledged(),
        }
    }

    pub fn admit_durable_mutation(
        &mut self,
        runtime_session_id: &str,
        operation_name: &str,
    ) -> Result<DurableMutationId, StoreError> {
        match self {
            Self::Embedded(backend) => {
                backend.admit_durable_mutation(runtime_session_id, operation_name)
            }
            Self::Sqlite(backend) => {
                backend.admit_durable_mutation(runtime_session_id, operation_name)
            }
        }
    }

    pub fn record_hosted_runtime_commit_result(
        &mut self,
        runtime_session_id: &str,
        durable_mutation_id: DurableMutationId,
        envelope: forge_relational::facade::replay::CanonicalCommitEnvelope,
    ) -> Result<(), StoreError> {
        match self {
            Self::Embedded(backend) => backend.record_hosted_runtime_commit_result(
                runtime_session_id,
                durable_mutation_id,
                envelope,
            ),
            Self::Sqlite(backend) => backend.record_hosted_runtime_commit_result(
                runtime_session_id,
                durable_mutation_id,
                envelope,
            ),
        }
    }

    pub fn record_publication_phase(
        &mut self,
        runtime_session_id: &str,
        durable_mutation_id: DurableMutationId,
        phase: DurablePublicationPhase,
        commit_id: Option<CommitId>,
    ) -> Result<(), StoreError> {
        match self {
            Self::Embedded(backend) => backend.record_publication_phase(
                runtime_session_id,
                durable_mutation_id,
                phase,
                commit_id,
            ),
            Self::Sqlite(backend) => backend.record_publication_phase(
                runtime_session_id,
                durable_mutation_id,
                phase,
                commit_id,
            ),
        }
    }

    pub fn resolve_retry(
        &self,
        durable_mutation_id: DurableMutationId,
    ) -> Result<DurableRetryResolution, StoreError> {
        match self {
            Self::Embedded(backend) => backend.resolve_retry(durable_mutation_id),
            Self::Sqlite(backend) => backend.resolve_retry(durable_mutation_id),
        }
    }

    pub fn recover_durable_runtime(
        &mut self,
        runtime_session_id: &str,
    ) -> Result<DurableRecoveryOutcome, StoreError> {
        match self {
            Self::Embedded(backend) => backend.recover_durable_runtime(runtime_session_id),
            Self::Sqlite(backend) => backend.recover_durable_runtime(runtime_session_id),
        }
    }

    pub fn plan_durable_recovery(&self) -> DurableRecoveryPlan {
        match self {
            Self::Embedded(backend) => backend.plan_durable_recovery(),
            Self::Sqlite(backend) => backend.plan_durable_recovery(),
        }
    }
}
