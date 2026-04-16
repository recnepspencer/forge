use crate::{
    authority::{
        canonicalize, AuthoritativeBranchHeadRecord, AuthoritativeExportBundle,
        AuthoritativeExportRestoreRequest, AdvanceCursorWitness,
        DurableCursorAcknowledgeRequest, DurableCursorResumePlan, DurableCursorResumeRequest,
        FetchedAuthoritativeCommit, FetchedDurableCursorIdentity, FetchedLineageSupportArtifact,
        FetchedSchemaBoundaryArtifact, FetchedSchemaSupportArtifact, HistoricalIdentityRequest,
        HistoricalIdentityResolution, PersistedAuthoritativeCommit, PersistedEmbeddedCheckpoint,
        PersistedSubscriberCheckpoint, RawRuntimeCommitEnvelope, ResumeAdmittedCursor,
        EmbeddedCheckpointFetchRequest, CURRENT_CANONICALIZATION_VERSION,
    },
    backend::{records::EmbeddedCheckpointRecord, StoreBackend, StoreBackendMode},
    bulk::{
        BudgetAdmittedChunkPlan, BulkCanonicalChunkExecutionRequest, BulkChunkCommitWitness,
        BulkChunkExecutionOutcome, BulkIngestSourceRequest, BulkTransformRequest,
        ChunkMaterializationReceipt, ChunkOrdinal, ChunkWidthBudget, DeterministicChunkPlan,
        DurablyExecutedBulkChunk, FrozenBulkSourceManifest, FrozenTransformBasis,
        FrozenTransformTargetPartition, ProgramChunkWitnessIndex,
        PublishedBulkProgressCheckpoint, RecoveredBulkChunkResume, ResumeBoundaryCandidate,
        ResumeReadyBulkProgram,
    },
    delta::{
        BranchDeltaAutoCompactOutcome, BranchDeltaReadPlan, BranchDeltaReadRequest,
        BranchDeltaReadResult, BranchDeltaRebuildReceipt, BranchDeltaRewritePlan,
        BranchDeltaRewriteReceipt, BranchDeltaRewriteRecommendation, BranchDeltaRewriteRequest,
        SameBranchDescendantWitness, SharedBaseBranchCreationReceipt,
        SharedBaseBranchCreationRequest, SharedBaseBranchCreationWitness,
    },
    evidence::{
        Milestone1CertificationBundle, Milestone35CertificationBundle,
        Milestone4CertificationBundle, Milestone6CertificationBundle,
        Milestone7CertificationBundle, ObservedPublicationFailure, OperatingModeLane,
        PersistedModeLaneEvidence, StoreCounterSnapshot,
    },
    failure::StoreError,
    layout::{
        AdmittedAspectLayoutReadPlan, AspectLayoutReadPlanDecision, AspectLayoutReadRequest,
        ChunkModelFrozenPhysicalLayout, DedupAdmittedBlockReuse,
        Milestone7IndependentLayoutReference, Milestone9PhysicalChunkReference,
    },
    media::DurableMediaReport,
    publication::PublicationWriteOutcome,
    recovery::{
        BackupRestoreCompatibilityReport, DurableRecoveryOutcome, DurableRecoveryPlan,
        MaintenanceRecoveryReport, ResumeEligibleRecoveredBulkChunk,
        SnapshotMaintenanceRecoveryReport,
    },
    snapshot::{
        PublishedSnapshotHandle, SnapshotCaptureRequest, SnapshotId, SnapshotImageBundle,
        SnapshotReadRequest, SnapshotReadResult, SnapshotRestoreOutcome, SnapshotRestorePlan,
        SnapshotRestoreRequest,
    },
    wal::{DurableMutationId, DurablePublicationPhase},
};
use forge_relational::facade::{
    history::{BranchId, CommitId},
    replay::CanonicalCommitEnvelope,
};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ForgeStoreBuilder {
    backend_mode: StoreBackendMode,
}

impl Default for ForgeStoreBuilder {
    fn default() -> Self {
        Self {
            backend_mode: StoreBackendMode::InMemory,
        }
    }
}

impl ForgeStoreBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn in_memory(mut self) -> Self {
        self.backend_mode = StoreBackendMode::InMemory;
        self
    }

    pub fn local_file(mut self, path: impl Into<PathBuf>) -> Self {
        self.backend_mode = StoreBackendMode::LocalFile(path.into());
        self
    }

    pub fn sqlite_file(mut self, path: impl Into<PathBuf>) -> Self {
        self.backend_mode = StoreBackendMode::SqliteFile(path.into());
        self
    }

    pub fn build(self) -> Result<ForgeStore, StoreError> {
        Ok(ForgeStore {
            backend: StoreBackend::open(self.backend_mode)?,
        })
    }

    pub(crate) fn build_for_durable_recovery(self) -> Result<ForgeStore, StoreError> {
        Ok(ForgeStore {
            backend: StoreBackend::open_for_durable_recovery(self.backend_mode)?,
        })
    }

    pub fn embedded_mode(self) -> crate::EmbeddedModeBuilder {
        crate::EmbeddedModeBuilder::new(self)
    }

    pub fn durable_mode(
        self,
        runtime: forge_relational::facade::runtime::RelationalRuntime,
    ) -> crate::DurableModeBuilder {
        crate::DurableModeBuilder::new(self, runtime)
    }
}

#[derive(Debug)]
pub struct ForgeStore {
    backend: StoreBackend,
}

impl ForgeStore {
    pub(crate) fn append_runtime_envelope(
        &mut self,
        envelope: forge_relational::facade::replay::CanonicalCommitEnvelope,
    ) -> Result<PersistedAuthoritativeCommit, StoreError> {
        let raw = RawRuntimeCommitEnvelope::new(envelope);
        let canonical = canonicalize(raw, CURRENT_CANONICALIZATION_VERSION)?;
        self.backend.record_canonicalization(*canonical.metrics());
        let verified = self.backend.verify_append(canonical)?;
        self.backend.append(verified)
    }

    pub(crate) fn persist_embedded_checkpoint_record(
        &mut self,
        record: EmbeddedCheckpointRecord,
    ) -> Result<EmbeddedCheckpointRecord, StoreError> {
        self.backend.persist_embedded_checkpoint(record)
    }

    pub(crate) fn record_durable_mode_selection(&self) {
        self.backend.record_durable_mode_selection();
    }

    pub(crate) fn record_embedded_mode_selection(&self) {
        self.backend.record_embedded_mode_selection();
    }

    pub(crate) fn record_hosted_runtime_start(&self) {
        self.backend.record_hosted_runtime_start();
    }

    pub(crate) fn record_hosted_runtime_stop(&self) {
        self.backend.record_hosted_runtime_stop();
    }

    pub(crate) fn record_external_commit_intake(&self) {
        self.backend.record_external_commit_intake();
    }

    pub(crate) fn record_external_checkpoint_intake(&self) {
        self.backend.record_external_checkpoint_intake();
    }

    #[cfg(test)]
    pub(crate) fn record_embedded_checkpoint_authority_rejection(&self) {
        self.backend
            .record_embedded_checkpoint_authority_rejection();
    }

    #[cfg(test)]
    pub(crate) fn record_mode_misuse_rejection(&self) {
        self.backend.record_mode_misuse_rejection();
    }

    pub(crate) fn admit_durable_mutation(
        &mut self,
        runtime_session_id: &str,
        operation_name: &str,
    ) -> Result<DurableMutationId, StoreError> {
        self.backend
            .admit_durable_mutation(runtime_session_id, operation_name)
    }

    pub(crate) fn record_hosted_runtime_commit_result(
        &mut self,
        runtime_session_id: &str,
        durable_mutation_id: DurableMutationId,
        envelope: CanonicalCommitEnvelope,
    ) -> Result<(), StoreError> {
        self.backend.record_hosted_runtime_commit_result(
            runtime_session_id,
            durable_mutation_id,
            envelope,
        )
    }

    pub(crate) fn record_publication_phase(
        &mut self,
        runtime_session_id: &str,
        durable_mutation_id: DurableMutationId,
        phase: DurablePublicationPhase,
        commit_id: Option<CommitId>,
    ) -> Result<(), StoreError> {
        self.backend.record_publication_phase(
            runtime_session_id,
            durable_mutation_id,
            phase,
            commit_id,
        )
    }

    pub(crate) fn record_bulk_checkpoint_publication_intent(
        &mut self,
        runtime_session_id: &str,
        durable_mutation_id: DurableMutationId,
        checkpoint_sequence: Option<u64>,
    ) -> Result<(), StoreError> {
        self.backend.record_bulk_checkpoint_publication_intent(
            runtime_session_id,
            durable_mutation_id,
            checkpoint_sequence,
        )
    }

    pub(crate) fn recover_durable_runtime(
        &mut self,
        runtime_session_id: &str,
    ) -> Result<DurableRecoveryOutcome, StoreError> {
        self.backend.recover_durable_runtime(runtime_session_id)
    }

    pub(crate) fn plan_durable_recovery(&self) -> DurableRecoveryPlan {
        self.backend.plan_durable_recovery()
    }

    pub(crate) fn resolve_durable_retry(
        &self,
        durable_mutation_id: DurableMutationId,
    ) -> Result<crate::DurableRetryResolution, StoreError> {
        self.backend.resolve_retry(durable_mutation_id)
    }

    pub(crate) fn record_durable_commit_acknowledged(&self) {
        self.backend.record_durable_commit_acknowledged();
    }

    pub(crate) fn classify_durable_publication(
        &self,
        durable_mutation_id: DurableMutationId,
        expected_commit_id: Option<CommitId>,
    ) -> Result<PublicationWriteOutcome, StoreError> {
        self.backend
            .classify_durable_publication(durable_mutation_id, expected_commit_id)
    }

    pub fn create_branch(
        &mut self,
        new_branch: BranchId,
        from_branch: Option<&BranchId>,
    ) -> Result<AuthoritativeBranchHeadRecord, StoreError> {
        self.backend.create_branch(new_branch, from_branch)
    }

    pub fn create_shared_base_branch(
        &mut self,
        request: SharedBaseBranchCreationRequest,
    ) -> Result<SharedBaseBranchCreationReceipt, StoreError> {
        self.backend.create_shared_base_branch(request)
    }

    pub fn admit_shared_base_branch_creation(
        &self,
        request: SharedBaseBranchCreationRequest,
    ) -> Result<SharedBaseBranchCreationWitness, StoreError> {
        self.backend.admit_shared_base_branch_creation(request)
    }

    pub fn append_canonical_commit(
        &mut self,
        envelope: CanonicalCommitEnvelope,
    ) -> Result<PersistedAuthoritativeCommit, StoreError> {
        self.append_runtime_envelope(envelope)
    }

    pub fn fetch_canonical_commit(
        &self,
        commit_id: CommitId,
    ) -> Result<FetchedAuthoritativeCommit, StoreError> {
        self.backend.fetch_commit(commit_id)
    }

    pub fn fetch_branch_head(
        &self,
        branch_id: &BranchId,
    ) -> Result<AuthoritativeBranchHeadRecord, StoreError> {
        self.backend.fetch_branch_head(branch_id)
    }

    pub fn plan_branch_delta_read(
        &self,
        request: BranchDeltaReadRequest,
    ) -> Result<BranchDeltaReadPlan, StoreError> {
        self.backend.plan_branch_delta_read(request)
    }

    pub fn admit_same_branch_descendant(
        &self,
        request: BranchDeltaReadRequest,
    ) -> Result<SameBranchDescendantWitness, StoreError> {
        self.backend.admit_same_branch_descendant(request)
    }

    pub fn admit_milestone_7_independent_reference(
        &self,
        request: BranchDeltaReadRequest,
    ) -> Result<crate::Milestone7IndependentReference, StoreError> {
        self.backend
            .admit_milestone_7_independent_reference(request)
    }

    pub fn read_branch_delta(
        &self,
        witness: SameBranchDescendantWitness,
    ) -> Result<BranchDeltaReadResult, StoreError> {
        self.backend.read_branch_delta(witness)
    }

    pub fn read_branch_delta_control(
        &self,
        witness: SameBranchDescendantWitness,
    ) -> Result<BranchDeltaReadResult, StoreError> {
        self.backend.read_branch_delta_control(witness)
    }

    pub fn read_branch_delta_control_from_milestone_7_reference(
        &self,
        reference: crate::Milestone7IndependentReference,
    ) -> Result<BranchDeltaReadResult, StoreError> {
        self.backend
            .read_branch_delta_control_from_milestone_7_reference(reference)
    }

    pub fn plan_aspect_layout_read(
        &self,
        request: AspectLayoutReadRequest,
    ) -> Result<AspectLayoutReadPlanDecision, StoreError> {
        self.backend.plan_aspect_layout_read(request)
    }

    pub fn admit_structural_block_reuse(
        &self,
        plan: AdmittedAspectLayoutReadPlan,
    ) -> Result<DedupAdmittedBlockReuse, StoreError> {
        self.backend.admit_structural_block_reuse(plan)
    }

    pub fn freeze_chunk_model(
        &self,
        plan: AdmittedAspectLayoutReadPlan,
    ) -> Result<ChunkModelFrozenPhysicalLayout, StoreError> {
        self.backend.freeze_chunk_model(plan)
    }

    pub fn admit_milestone_7_independent_layout_reference(
        &self,
        plan: AdmittedAspectLayoutReadPlan,
    ) -> Result<Milestone7IndependentLayoutReference, StoreError> {
        self.backend
            .admit_milestone_7_independent_layout_reference(plan)
    }

    pub fn admit_milestone_9_physical_chunk_reference(
        &self,
        frozen: ChunkModelFrozenPhysicalLayout,
    ) -> Result<Milestone9PhysicalChunkReference, StoreError> {
        self.backend.admit_milestone_9_physical_chunk_reference(frozen)
    }

    pub fn materialize_milestone_6_layout_support(
        &mut self,
        request: AspectLayoutReadRequest,
    ) -> Result<crate::Milestone6LayoutMaterialization, StoreError> {
        self.backend.materialize_milestone_6_layout_support(request)
    }

    pub fn fetch_milestone_6_layout_support(
        &self,
        request: AspectLayoutReadRequest,
    ) -> Result<crate::Milestone6LayoutMaterialization, StoreError> {
        self.backend.fetch_milestone_6_layout_support(request)
    }

    pub fn plan_delta_rewrite(
        &self,
        request: BranchDeltaRewriteRequest,
    ) -> Result<BranchDeltaRewritePlan, StoreError> {
        self.backend.plan_delta_rewrite(request)
    }

    pub fn recommend_delta_rewrite(
        &self,
        request: BranchDeltaRewriteRequest,
    ) -> Result<BranchDeltaRewriteRecommendation, StoreError> {
        self.backend.recommend_delta_rewrite(request)
    }

    pub fn auto_compact_branch_delta(
        &mut self,
        request: BranchDeltaRewriteRequest,
    ) -> Result<BranchDeltaAutoCompactOutcome, StoreError> {
        self.backend.auto_compact_branch_delta(request)
    }

    pub fn rewrite_branch_delta(
        &mut self,
        plan: BranchDeltaRewritePlan,
    ) -> Result<BranchDeltaRewriteReceipt, StoreError> {
        self.backend.rewrite_branch_delta(plan)
    }

    pub fn rebuild_branch_delta_artifacts(
        &mut self,
        branch_id: BranchId,
    ) -> Result<BranchDeltaRebuildReceipt, StoreError> {
        self.backend.rebuild_branch_delta_artifacts(branch_id)
    }

    pub fn fetch_schema_support(
        &self,
        commit_id: CommitId,
    ) -> Result<FetchedSchemaSupportArtifact, StoreError> {
        self.backend.fetch_schema_support(commit_id)
    }

    pub fn fetch_schema_boundary(
        &self,
        commit_id: CommitId,
    ) -> Result<FetchedSchemaBoundaryArtifact, StoreError> {
        self.backend.fetch_schema_support(commit_id)
    }

    pub fn fetch_lineage_support(
        &self,
        commit_id: CommitId,
    ) -> Result<FetchedLineageSupportArtifact, StoreError> {
        self.backend.fetch_lineage_support(commit_id)
    }

    pub fn fetch_lineage_history(
        &self,
        request: HistoricalIdentityRequest,
    ) -> Result<HistoricalIdentityResolution, StoreError> {
        self.backend.fetch_lineage_history(request)
    }

    pub fn acknowledge_cursor(
        &mut self,
        request: DurableCursorAcknowledgeRequest,
    ) -> Result<PersistedSubscriberCheckpoint, StoreError> {
        let witness = self.admit_cursor_advance(request)?;
        self.acknowledge_cursor_progress(witness)
    }

    pub fn fetch_durable_cursor_identity(
        &self,
        cursor_id: &str,
    ) -> Result<FetchedDurableCursorIdentity, StoreError> {
        self.backend.fetch_durable_cursor_identity(cursor_id)
    }

    pub fn plan_cursor_resume(
        &self,
        request: DurableCursorResumeRequest,
    ) -> Result<DurableCursorResumePlan, StoreError> {
        self.backend.plan_cursor_resume(request)
    }

    pub fn admit_cursor_resume(
        &self,
        request: DurableCursorResumeRequest,
    ) -> Result<ResumeAdmittedCursor, StoreError> {
        Ok(ResumeAdmittedCursor::new(self.backend.plan_cursor_resume(request)?))
    }

    pub fn admit_cursor_advance(
        &self,
        request: DurableCursorAcknowledgeRequest,
    ) -> Result<AdvanceCursorWitness, StoreError> {
        Ok(AdvanceCursorWitness::new(request))
    }

    pub fn admit_resumed_cursor_advance(
        &self,
        resumed: &ResumeAdmittedCursor,
        request: DurableCursorAcknowledgeRequest,
    ) -> Result<AdvanceCursorWitness, StoreError> {
        let identity = resumed.identity();
        if identity.cursor_id != request.cursor_id()
            || identity.subscriber_id != request.subscriber_id()
            || identity.branch_id != *request.branch_id()
            || identity.feed_shape_id != request.feed_shape_id()
            || identity.schema_interpretation_id != request.schema_interpretation_id()
            || identity.cursor_semantics_version != request.cursor_semantics_version()
        {
            return Err(StoreError::new(
                crate::StoreErrorKind::CursorEquivalenceViolation,
                "cursor advance witness does not match the admitted resume identity basis",
            ));
        }
        Ok(AdvanceCursorWitness::new(request))
    }

    pub fn acknowledge_cursor_progress(
        &mut self,
        witness: AdvanceCursorWitness,
    ) -> Result<PersistedSubscriberCheckpoint, StoreError> {
        self.backend.acknowledge_cursor(witness.into_request())
    }

    pub fn acknowledge_resumed_cursor_progress(
        &mut self,
        resumed: &ResumeAdmittedCursor,
        witness: AdvanceCursorWitness,
    ) -> Result<PersistedSubscriberCheckpoint, StoreError> {
        if resumed.identity().cursor_id != witness.cursor_id() {
            return Err(StoreError::new(
                crate::StoreErrorKind::CursorEquivalenceViolation,
                "resume-admitted cursor and advance witness must reference the same cursor identity",
            ));
        }
        self.backend.acknowledge_cursor(witness.into_request())
    }

    pub fn freeze_bulk_ingest_source(
        &mut self,
        request: BulkIngestSourceRequest,
    ) -> Result<FrozenBulkSourceManifest, StoreError> {
        let manifest = FrozenBulkSourceManifest::freeze(request)?;
        self.backend.record_bulk_source_manifest(
            manifest.ordered_members().len() as u64,
            1,
        );
        self.backend.persist_frozen_bulk_manifest(manifest)
    }

    pub fn plan_bulk_ingest(
        &mut self,
        manifest: FrozenBulkSourceManifest,
        chunk_width_budget: ChunkWidthBudget,
    ) -> Result<DeterministicChunkPlan, StoreError> {
        let plan = DeterministicChunkPlan::for_ingest(&manifest, chunk_width_budget)?;
        self.backend
            .record_bulk_chunk_plan(plan.chunk_count() as u64);
        self.backend.persist_bulk_chunk_plan(plan)
    }

    pub fn freeze_bulk_transform_basis(
        &mut self,
        request: BulkTransformRequest,
    ) -> Result<FrozenTransformBasis, StoreError> {
        let basis = FrozenTransformBasis::freeze(&request)?;
        self.backend.persist_frozen_transform_basis(basis)
    }

    pub fn freeze_bulk_transform_target_partition(
        &mut self,
        request: BulkTransformRequest,
        basis: &FrozenTransformBasis,
    ) -> Result<FrozenTransformTargetPartition, StoreError> {
        let partition = FrozenTransformTargetPartition::freeze(&request, basis)?;
        self.backend.persist_frozen_transform_partition(partition)
    }

    pub fn plan_bulk_transform(
        &mut self,
        basis: &FrozenTransformBasis,
        partition: &FrozenTransformTargetPartition,
        chunk_width_budget: ChunkWidthBudget,
    ) -> Result<DeterministicChunkPlan, StoreError> {
        let plan = DeterministicChunkPlan::for_transform(basis, partition, chunk_width_budget)?;
        self.backend
            .record_bulk_chunk_plan(plan.chunk_count() as u64);
        self.backend.persist_bulk_chunk_plan(plan)
    }

    pub fn fetch_frozen_bulk_manifest(
        &self,
        program_id: &str,
        manifest_digest: &str,
    ) -> Result<FrozenBulkSourceManifest, StoreError> {
        self.backend
            .fetch_frozen_bulk_manifest(program_id, manifest_digest)
    }

    pub fn fetch_frozen_transform_basis(
        &self,
        program_id: &str,
        basis_digest: &str,
    ) -> Result<FrozenTransformBasis, StoreError> {
        self.backend
            .fetch_frozen_transform_basis(program_id, basis_digest)
    }

    pub fn fetch_frozen_transform_partition(
        &self,
        program_id: &str,
        partition_digest: &str,
    ) -> Result<FrozenTransformTargetPartition, StoreError> {
        self.backend
            .fetch_frozen_transform_partition(program_id, partition_digest)
    }

    pub fn admit_bulk_ingest_chunk(
        &self,
        plan: &DeterministicChunkPlan,
        ordinal: ChunkOrdinal,
        admitted_memory_units: u64,
    ) -> Result<BudgetAdmittedChunkPlan, StoreError> {
        BudgetAdmittedChunkPlan::admit(plan, ordinal, admitted_memory_units)
    }

    pub fn admit_bulk_transform_chunk(
        &self,
        plan: &DeterministicChunkPlan,
        ordinal: ChunkOrdinal,
        admitted_memory_units: u64,
    ) -> Result<BudgetAdmittedChunkPlan, StoreError> {
        BudgetAdmittedChunkPlan::admit(plan, ordinal, admitted_memory_units)
    }

    pub fn materialize_bulk_ingest_chunk(
        &self,
        admitted: &BudgetAdmittedChunkPlan,
    ) -> Result<ChunkMaterializationReceipt, StoreError> {
        let receipt = ChunkMaterializationReceipt::from_admitted_chunk(admitted);
        self.backend.record_bulk_chunk_execute(
            receipt.admitted_width_units(),
            receipt.memory_units(),
            receipt.materialization_breadth_units(),
            receipt.execution_path(),
        );
        Ok(receipt)
    }

    pub fn publish_bulk_chunk_witness(
        &mut self,
        admitted: &BudgetAdmittedChunkPlan,
        canonical_commit_id: CommitId,
    ) -> Result<BulkChunkCommitWitness, StoreError> {
        let witness = BulkChunkCommitWitness::publish(admitted, canonical_commit_id)?;
        self.backend.publish_bulk_chunk_witness(witness)
    }

    pub fn publish_bulk_progress_checkpoint(
        &mut self,
        witness: &BulkChunkCommitWitness,
    ) -> Result<PublishedBulkProgressCheckpoint, StoreError> {
        self.backend
            .publish_bulk_progress_checkpoint(witness.clone())
    }

    pub fn fetch_bulk_progress_checkpoint(
        &self,
        program_id: &str,
        plan_id: &str,
    ) -> Result<PublishedBulkProgressCheckpoint, StoreError> {
        self.backend.fetch_bulk_progress_checkpoint(program_id, plan_id)
    }

    pub fn fetch_bulk_chunk_plan(
        &self,
        program_id: &str,
        plan_id: &str,
    ) -> Result<DeterministicChunkPlan, StoreError> {
        self.backend.fetch_bulk_chunk_plan(program_id, plan_id)
    }

    pub fn fetch_program_chunk_witness_index(
        &self,
        program_id: &str,
        plan_id: &str,
    ) -> Result<ProgramChunkWitnessIndex, StoreError> {
        self.backend.fetch_program_chunk_witness_index(program_id, plan_id)
    }

    pub fn fetch_latest_bulk_resume_boundary(
        &self,
        program_id: &str,
        plan_id: &str,
    ) -> Result<ResumeBoundaryCandidate, StoreError> {
        self.backend.fetch_latest_resume_boundary(program_id, plan_id)
    }

    pub fn admit_bulk_ingest_resume(
        &self,
        program_id: &str,
        plan_id: &str,
        manifest_digest: &str,
    ) -> Result<ResumeReadyBulkProgram, StoreError> {
        let manifest = self.fetch_frozen_bulk_manifest(program_id, manifest_digest)?;
        let (plan, witness_index, latest_checkpoint, resume_boundary) =
            self.load_bulk_resume_artifacts(program_id, plan_id)?;
        self.backend.record_bulk_chunk_resume();
        ResumeReadyBulkProgram::admit_ingest(
            &manifest,
            plan,
            witness_index,
            latest_checkpoint,
            resume_boundary,
        )
    }

    pub fn admit_bulk_transform_resume(
        &self,
        program_id: &str,
        plan_id: &str,
        basis_digest: &str,
        partition_digest: &str,
    ) -> Result<ResumeReadyBulkProgram, StoreError> {
        let basis = self.fetch_frozen_transform_basis(program_id, basis_digest)?;
        let partition = self.fetch_frozen_transform_partition(program_id, partition_digest)?;
        let (plan, witness_index, latest_checkpoint, resume_boundary) =
            self.load_bulk_resume_artifacts(program_id, plan_id)?;
        self.backend.record_bulk_chunk_resume();
        ResumeReadyBulkProgram::admit_transform(
            &basis,
            &partition,
            plan,
            witness_index,
            latest_checkpoint,
            resume_boundary,
        )
    }

    pub fn admit_recovered_bulk_chunk_resume(
        &self,
        recovered: &ResumeEligibleRecoveredBulkChunk,
    ) -> Result<RecoveredBulkChunkResume, StoreError> {
        let recovered = recovered.recovered();
        let (plan, witness_index, latest_checkpoint, resume_boundary) =
            self.load_bulk_resume_artifacts(recovered.program_id(), recovered.plan_id())?;
        self.backend.record_bulk_chunk_resume();
        let resumed_program = self.admit_resume_ready_bulk_program(
            recovered.program_id(),
            recovered.plan_id(),
            plan,
            witness_index,
            latest_checkpoint,
            resume_boundary,
        )?;

        Ok(RecoveredBulkChunkResume::new(
            ChunkOrdinal::new(recovered.chunk_ordinal()),
            resumed_program,
        ))
    }

    pub fn finalize_bulk_chunk_execution(
        &mut self,
        admitted: &BudgetAdmittedChunkPlan,
        canonical_commit_id: CommitId,
        publish_checkpoint: bool,
    ) -> Result<BulkChunkExecutionOutcome, StoreError> {
        let materialization_receipt = self.materialize_bulk_ingest_chunk(admitted)?;
        let witness = self.publish_bulk_chunk_witness(admitted, canonical_commit_id)?;
        let published_checkpoint = match publish_checkpoint {
            true => Some(self.publish_bulk_progress_checkpoint(&witness)?),
            false => None,
        };
        self.backend.record_bulk_chunk_commit();
        Ok(BulkChunkExecutionOutcome::new(
            materialization_receipt,
            witness,
            published_checkpoint,
        ))
    }

    pub fn admit_bulk_canonical_chunk_execution(
        &self,
        admitted: BudgetAdmittedChunkPlan,
        canonical_envelope: CanonicalCommitEnvelope,
    ) -> Result<BulkCanonicalChunkExecutionRequest, StoreError> {
        BulkCanonicalChunkExecutionRequest::admit(admitted, canonical_envelope)
    }

    pub fn execute_bulk_canonical_chunk(
        &mut self,
        request: BulkCanonicalChunkExecutionRequest,
        publish_checkpoint: bool,
    ) -> Result<BulkChunkExecutionOutcome, StoreError> {
        let (admitted, canonical_envelope) = request.into_parts();
        let persisted = self.append_canonical_commit(canonical_envelope)?;
        self.finalize_bulk_chunk_execution(
            &admitted,
            persisted.envelope().commit.commit_id,
            publish_checkpoint,
        )
    }

    pub fn execute_bulk_canonical_chunk_durably(
        &mut self,
        request: BulkCanonicalChunkExecutionRequest,
        publish_checkpoint: bool,
    ) -> Result<DurablyExecutedBulkChunk, StoreError> {
        let runtime_session_id = request.runtime_session_id();
        let operation_name = request.operation_name();
        let canonical_commit_id = request.canonical_envelope().commit.commit_id;
        let next_checkpoint_sequence = if publish_checkpoint {
            Some(self.next_bulk_checkpoint_sequence(
                request.admitted_chunk().program_id(),
                request.admitted_chunk().plan_id(),
            )?)
        } else {
            None
        };
        let durable_mutation_id =
            self.admit_durable_mutation(&runtime_session_id, &operation_name)?;
        self.record_hosted_runtime_commit_result(
            &runtime_session_id,
            durable_mutation_id,
            request.canonical_envelope().clone(),
        )?;
        self.record_bulk_checkpoint_publication_intent(
            &runtime_session_id,
            durable_mutation_id,
            next_checkpoint_sequence,
        )?;
        self.record_publication_phase(
            &runtime_session_id,
            durable_mutation_id,
            DurablePublicationPhase::CanonicalCommitProduced,
            Some(canonical_commit_id),
        )?;
        let (admitted, canonical_envelope) = request.into_parts();
        let persisted = self.append_canonical_commit(canonical_envelope)?;
        let persisted_commit_id = persisted.envelope().commit.commit_id;
        self.record_publication_phase(
            &runtime_session_id,
            durable_mutation_id,
            DurablePublicationPhase::AuthoritativeAppendPublished,
            Some(persisted_commit_id),
        )?;
        let outcome =
            self.finalize_bulk_chunk_execution(&admitted, persisted_commit_id, publish_checkpoint)?;
        self.record_publication_phase(
            &runtime_session_id,
            durable_mutation_id,
            DurablePublicationPhase::AcknowledgmentEligible,
            Some(persisted_commit_id),
        )?;
        self.record_durable_commit_acknowledged();
        Ok(DurablyExecutedBulkChunk::new(durable_mutation_id, outcome))
    }

    pub fn execute_next_resumed_bulk_chunk(
        &mut self,
        resumed: &ResumeReadyBulkProgram,
        admitted_memory_units: u64,
        canonical_envelope: CanonicalCommitEnvelope,
        publish_checkpoint: bool,
    ) -> Result<Option<BulkChunkExecutionOutcome>, StoreError> {
        let Some(admitted) = resumed.admit_next_chunk(admitted_memory_units)? else {
            return Ok(None);
        };
        let request = self.admit_bulk_canonical_chunk_execution(admitted, canonical_envelope)?;
        self.execute_bulk_canonical_chunk(request, publish_checkpoint)
            .map(Some)
    }

    pub fn execute_next_resumed_bulk_chunk_durably(
        &mut self,
        resumed: &ResumeReadyBulkProgram,
        admitted_memory_units: u64,
        canonical_envelope: CanonicalCommitEnvelope,
        publish_checkpoint: bool,
    ) -> Result<Option<DurablyExecutedBulkChunk>, StoreError> {
        let Some(admitted) = resumed.admit_next_chunk(admitted_memory_units)? else {
            return Ok(None);
        };
        let request = self.admit_bulk_canonical_chunk_execution(admitted, canonical_envelope)?;
        self.execute_bulk_canonical_chunk_durably(request, publish_checkpoint)
            .map(Some)
    }

    fn load_bulk_resume_artifacts(
        &self,
        program_id: &str,
        plan_id: &str,
    ) -> Result<
        (
            DeterministicChunkPlan,
            Option<ProgramChunkWitnessIndex>,
            Option<PublishedBulkProgressCheckpoint>,
            ResumeBoundaryCandidate,
        ),
        StoreError,
    > {
        let plan = self.fetch_bulk_chunk_plan(program_id, plan_id)?;
        let resume_boundary = self.fetch_latest_bulk_resume_boundary(program_id, plan_id)?;
        let witness_index = match self.fetch_program_chunk_witness_index(program_id, plan_id) {
            Ok(index) => Some(index),
            Err(error) if matches!(error.kind(), crate::StoreErrorKind::BulkChunkWitnessGap) => None,
            Err(error) => return Err(error),
        };
        let latest_checkpoint = match resume_boundary.latest_checkpoint_sequence() {
            Some(_) => Some(self.fetch_bulk_progress_checkpoint(program_id, plan_id)?),
            None => None,
        };
        Ok((plan, witness_index, latest_checkpoint, resume_boundary))
    }

    fn next_bulk_checkpoint_sequence(
        &self,
        program_id: &str,
        plan_id: &str,
    ) -> Result<u64, StoreError> {
        match self.fetch_program_chunk_witness_index(program_id, plan_id) {
            Ok(index) => Ok(index
                .latest_checkpoint_sequence()
                .map(|sequence| sequence + 1)
                .unwrap_or(1)),
            Err(error) if matches!(error.kind(), crate::StoreErrorKind::BulkChunkWitnessGap) => {
                Ok(1)
            }
            Err(error) => Err(error),
        }
    }

    fn admit_resume_ready_bulk_program(
        &self,
        program_id: &str,
        plan_id: &str,
        plan: DeterministicChunkPlan,
        witness_index: Option<ProgramChunkWitnessIndex>,
        latest_checkpoint: Option<PublishedBulkProgressCheckpoint>,
        resume_boundary: ResumeBoundaryCandidate,
    ) -> Result<ResumeReadyBulkProgram, StoreError> {
        match plan.kind() {
            crate::BulkPlanKind::Ingest => {
                let manifest = self.fetch_frozen_bulk_manifest(program_id, plan.input_digest())?;
                ResumeReadyBulkProgram::admit_ingest(
                    &manifest,
                    plan,
                    witness_index,
                    latest_checkpoint,
                    resume_boundary,
                )
            }
            crate::BulkPlanKind::Transform => {
                let basis_commit_id = plan.basis_commit_id().ok_or_else(|| {
                    StoreError::new(
                        crate::StoreErrorKind::BulkTransformBasisDrift,
                        format!(
                            "bulk transform plan `{plan_id}` for program `{program_id}` is missing a locked basis commit"
                        ),
                    )
                })?;
                let basis = self.backend.find_frozen_transform_basis_for_plan(
                    program_id,
                    plan.target_branch_scope(),
                    basis_commit_id,
                )?;
                let partition =
                    self.fetch_frozen_transform_partition(program_id, plan.input_digest())?;
                ResumeReadyBulkProgram::admit_transform(
                    &basis,
                    &partition,
                    plan,
                    witness_index,
                    latest_checkpoint,
                    resume_boundary,
                )
            }
        }
    }

    pub fn fetch_embedded_checkpoint(
        &self,
        request: EmbeddedCheckpointFetchRequest,
    ) -> Result<PersistedEmbeddedCheckpoint, StoreError> {
        Ok(PersistedEmbeddedCheckpoint::new(
            self.backend.fetch_embedded_checkpoint(request.checkpoint_id())?,
        ))
    }

    pub fn counters(&self) -> StoreCounterSnapshot {
        self.backend.counter_snapshot()
    }

    pub fn export_authoritative_records(&self) -> AuthoritativeExportBundle {
        self.backend.export_bundle()
    }

    pub fn durable_media_report(&self) -> DurableMediaReport {
        self.backend.durable_media_report()
    }

    pub fn milestone_7_access_structure_verification(
        &self,
    ) -> crate::Milestone7AccessStructureVerification {
        self.backend.milestone_7_access_structure_verification()
    }

    pub fn milestone_6_access_structure_verification(
        &self,
    ) -> crate::Milestone6AccessStructureVerification {
        self.backend.milestone_6_access_structure_verification()
    }

    pub fn durable_publication_report(
        &self,
        durable_mutation_id: DurableMutationId,
        expected_commit_id: Option<CommitId>,
    ) -> Result<PublicationWriteOutcome, StoreError> {
        self.backend
            .classify_durable_publication(durable_mutation_id, expected_commit_id)
    }

    pub fn snapshot_publication_report(
        &self,
        snapshot_id: SnapshotId,
    ) -> Result<PublicationWriteOutcome, StoreError> {
        self.backend.classify_snapshot_publication(snapshot_id)
    }

    pub fn snapshot_maintenance_recovery_report(
        &self,
        snapshot_id: SnapshotId,
    ) -> Result<SnapshotMaintenanceRecoveryReport, StoreError> {
        self.backend
            .classify_snapshot_maintenance_recovery(snapshot_id)
    }

    pub fn maintenance_recovery_report(&self) -> Result<MaintenanceRecoveryReport, StoreError> {
        self.backend.maintenance_recovery_report()
    }

    pub fn support_artifact_recovery_report(&self) -> crate::SupportArtifactRecoveryReport {
        self.backend.support_artifact_recovery_report()
    }

    pub(crate) fn record_support_artifact_recovery_gap(&self, count: u64) {
        self.backend.record_support_artifact_recovery_gap(count);
    }

    pub fn backup_restore_compatibility_report(
        &self,
    ) -> Result<BackupRestoreCompatibilityReport, StoreError> {
        self.backend.backup_restore_compatibility_report()
    }

    pub fn capture_snapshot(
        &mut self,
        request: SnapshotCaptureRequest,
    ) -> Result<PublishedSnapshotHandle, StoreError> {
        self.backend.capture_snapshot(request)
    }

    pub fn read_snapshot(
        &self,
        request: SnapshotReadRequest,
    ) -> Result<SnapshotReadResult, StoreError> {
        self.backend.read_snapshot(request)
    }

    pub fn plan_snapshot_restore(
        &self,
        request: SnapshotRestoreRequest,
    ) -> Result<SnapshotRestorePlan, StoreError> {
        self.backend.plan_snapshot_restore(request)
    }

    pub fn execute_snapshot_restore(
        &self,
        plan: SnapshotRestorePlan,
    ) -> Result<SnapshotRestoreOutcome, StoreError> {
        self.backend.execute_snapshot_restore(plan)
    }

    pub fn restore_snapshot(
        &self,
        snapshot_id: SnapshotId,
        target_commit_id: CommitId,
    ) -> Result<SnapshotRestoreOutcome, StoreError> {
        let plan =
            self.plan_snapshot_restore(SnapshotRestoreRequest::new(snapshot_id, target_commit_id))?;
        self.execute_snapshot_restore(plan)
    }

    pub fn rebuild_snapshot(
        &self,
        snapshot_id: SnapshotId,
    ) -> Result<SnapshotImageBundle, StoreError> {
        self.backend.rebuild_snapshot(snapshot_id)
    }

    #[cfg(test)]
    pub(crate) fn remove_snapshot_image_for_test(
        &mut self,
        snapshot_id: SnapshotId,
    ) -> Result<(), StoreError> {
        self.backend.remove_snapshot_image_for_test(snapshot_id)
    }

    #[cfg(test)]
    pub(crate) fn remove_snapshot_basis_for_test(
        &mut self,
        snapshot_id: SnapshotId,
    ) -> Result<(), StoreError> {
        self.backend.remove_snapshot_basis_for_test(snapshot_id)
    }

    #[cfg(test)]
    pub(crate) fn clear_branch_heads_for_test(&mut self) -> Result<(), StoreError> {
        self.backend.clear_branch_heads_for_test()
    }

    #[cfg(test)]
    pub(crate) fn corrupt_snapshot_basis_digest_for_test(
        &mut self,
        snapshot_id: SnapshotId,
    ) -> Result<(), StoreError> {
        self.backend
            .corrupt_snapshot_basis_digest_for_test(snapshot_id)
    }

    pub fn milestone_1_certification_bundle(&self) -> Milestone1CertificationBundle {
        let export = self.export_authoritative_records();
        Milestone1CertificationBundle::from_export(&export, self.counters())
    }

    pub fn milestone_3_5_certification_bundle(
        &self,
        ack_boundary_report: PublicationWriteOutcome,
        failures: &[ObservedPublicationFailure],
    ) -> Milestone35CertificationBundle {
        Milestone35CertificationBundle::new(
            self.durable_media_report(),
            ack_boundary_report,
            self.counters(),
            failures,
        )
    }

    pub fn milestone_4_certification_bundle(
        &self,
        truth_image: &SnapshotImageBundle,
        restored_image: &SnapshotImageBundle,
        rebuilt_image: &SnapshotImageBundle,
    ) -> Milestone4CertificationBundle {
        Milestone4CertificationBundle::new(
            truth_image,
            restored_image,
            rebuilt_image,
            self.counters(),
        )
    }

    pub fn milestone_5_certification_bundle(
        &self,
        request: BranchDeltaReadRequest,
    ) -> Result<crate::Milestone5CertificationBundle, StoreError> {
        let witness = self.admit_same_branch_descendant(request.clone())?;
        let direct = self.read_branch_delta(witness)?;
        let reference = self.admit_milestone_7_independent_reference(request.clone())?;
        let control = self.read_branch_delta_control_from_milestone_7_reference(reference)?;
        let delta_storage_report = self.backend.milestone_5_delta_storage_report(
            request.branch_id,
            request.target_commit_id,
            &direct.plan,
            &control.plan,
        )?;
        Ok(crate::Milestone5CertificationBundle::new(
            direct.authoritative_export(),
            control.authoritative_export(),
            delta_storage_report,
            self.counters(),
        ))
    }

    fn require_admitted_aspect_layout_plan(
        &self,
        request: AspectLayoutReadRequest,
        operation_name: &str,
    ) -> Result<crate::AdmittedAspectLayoutReadPlan, StoreError> {
        match self.plan_aspect_layout_read(request)? {
            crate::AspectLayoutReadPlanDecision::Admitted(plan) => Ok(plan),
            crate::AspectLayoutReadPlanDecision::Fallback(plan) => Err(StoreError::new(
                crate::StoreErrorKind::AspectLayoutFallbackRequired,
                format!(
                    "{operation_name} requires an admitted layout read, but request fell back: {}",
                    plan.reason()
                ),
            )),
            crate::AspectLayoutReadPlanDecision::Rejected(plan) => Err(StoreError::new(
                crate::StoreErrorKind::AspectScopeUnsupported,
                format!(
                    "{operation_name} requires an admitted layout read, but request was rejected: {}",
                    plan.reason()
                ),
            )),
        }
    }

    pub fn milestone_6_certification_bundle(
        &self,
        request: AspectLayoutReadRequest,
    ) -> Result<Milestone6CertificationBundle, StoreError> {
        let plan =
            self.require_admitted_aspect_layout_plan(request.clone(), "milestone 6 certification")?;
        let artifact_id = crate::layout::layout_materialization_artifact_id(&plan);
        match self
            .backend
            .fetch_existing_milestone_6_layout_support(&artifact_id)
        {
            Ok(materialization) => {
                return Ok(Milestone6CertificationBundle::from_materialization(
                    &materialization,
                    self.milestone_6_access_structure_verification(),
                    self.counters(),
                ));
            }
            Err(error)
                if matches!(error.kind(), crate::StoreErrorKind::AspectLayoutArtifactMissing) => {}
            Err(error) => return Err(error),
        }
        let reuse = self.admit_structural_block_reuse(plan.clone())?;
        let frozen = self.freeze_chunk_model(plan.clone())?;
        let milestone_7 = self.admit_milestone_7_independent_layout_reference(plan.clone())?;
        let milestone_9 = self.admit_milestone_9_physical_chunk_reference(frozen.clone())?;
        Ok(Milestone6CertificationBundle::new(
            &plan,
            &reuse,
            &frozen,
            &milestone_7,
            &milestone_9,
            self.milestone_6_access_structure_verification(),
            self.counters(),
        ))
    }

    pub fn milestone_7_certification_bundle(
        &self,
        control_export: &AuthoritativeExportBundle,
    ) -> Milestone7CertificationBundle {
        Milestone7CertificationBundle::new(
            &self.export_authoritative_records(),
            control_export,
            self.durable_media_report(),
            self.support_artifact_recovery_report(),
            self.milestone_7_access_structure_verification(),
            self.counters(),
        )
    }

    pub(crate) fn milestone_2_lane_evidence(
        &self,
        lane: OperatingModeLane,
    ) -> PersistedModeLaneEvidence {
        let export = self.export_authoritative_records();
        PersistedModeLaneEvidence::from_export(lane, &export, self.counters())
    }

    pub fn restore_from_authoritative_export(
        request: AuthoritativeExportRestoreRequest,
    ) -> Result<Self, StoreError> {
        Ok(Self {
            backend: StoreBackend::from_export_bundle(request.into_bundle())?,
        })
    }
}
