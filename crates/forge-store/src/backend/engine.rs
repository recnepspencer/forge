use crate::{
    authority::{
        AuthoritativeBranchHeadRecord, AuthoritativeExportBundle, CanonicalizedCommitEnvelope,
        DurableCursorAcknowledgeRequest, DurableCursorResumePlan, DurableCursorResumeRequest,
        FetchedAuthoritativeCommit, FetchedDurableCursorIdentity, FetchedLineageSupportArtifact,
        FetchedSchemaSupportArtifact, HistoricalIdentityRequest, HistoricalIdentityResolution,
        PersistedAuthoritativeCommit, PersistedSubscriberCheckpoint, VerifiedAuthoritativeAppend,
    },
    bulk::{
        BudgetAdmittedChunkPlan, BulkChunkCommitWitness, BulkPlanKind,
        BulkProgressCheckpointRecordInput, ChunkOrdinal, DeterministicChunkPlan,
        FrozenBulkSourceManifest, FrozenTransformBasis, FrozenTransformTargetPartition,
        ProgramChunkWitnessIndex, PublishedBulkProgressCheckpoint, ResumeBoundaryCandidate,
        ResumeReadyBulkProgram, BULK_FAMILY_VERSION,
    },
    delta::{
        BranchDeltaAutoCompactDisposition, BranchDeltaAutoCompactOutcome, BranchDeltaFallbackClass,
        BranchDeltaReadPlan, BranchDeltaReadRequest, BranchDeltaReadResult,
        BranchDeltaReadStrategy, BranchDeltaRebuildReceipt, BranchDeltaRewritePlan,
        BranchDeltaRewriteReceipt, BranchDeltaRewriteRecommendation, BranchDeltaRewriteRequest,
        BranchDeltaRewriteStrategy, SameBranchDescendantWitness, SharedBaseBranchCreationReceipt,
        SharedBaseBranchCreationRequest, SharedBaseBranchCreationWitness,
    },
    evidence::{
        CanonicalizationMetrics, Milestone6AccessStructureVerification,
        Milestone6AccessStructureVerificationPath, Milestone7AccessStructureVerification,
        Milestone7AccessStructureVerificationPath, StoreCounterSnapshot, StoreCounters,
    },
    failure::{StoreError, StoreErrorKind},
    layout::{
        AdmittedAspectLayoutReadPlan, AspectLayoutReadExecutionDecision,
        AspectLayoutReadPlanDecision, AspectLayoutReadRequest, ChunkModelFrozenPhysicalLayout,
        DedupAdmittedBlockReuse, DedupBackedReadResult, Milestone6LayoutMaterialization,
        Milestone7IndependentLayoutReference, Milestone9PhysicalChunkReference,
        StructuralBlockLookup, StructuralBlockLookupResult,
    },
    media::DurableMediaReport,
    publication::{
        classify_durable_publication, classify_snapshot_publication, durable_publication_facts,
        PublicationWriteOutcome,
    },
    recovery::{
        build_backup_restore_compatibility_report, build_maintenance_recovery_report,
        build_recovery_plan, build_support_artifact_recovery_report,
        classify_snapshot_maintenance_recovery, evaluate_recovery_for_mutation,
        BackupRestoreCompatibilityReport, DurableRecoveryDecision, DurableRecoveryOutcome,
        DurableRecoveryPlan, DurableRetryResolution, MaintenanceRecoveryReport, RecoveryAction,
        RecoverySourceKind, SnapshotMaintenanceRecoveryReport, SupportArtifactRecoveryReport,
    },
    snapshot::{
        PublishedSnapshotHandle, SnapshotCaptureRequest, SnapshotId, SnapshotImageBundle,
        SnapshotReadRequest, SnapshotReadResult, SnapshotRestoreOutcome, SnapshotRestorePlan,
        SnapshotRestoreRequest,
    },
    wal::{
        DurableMutationId, DurablePublicationPhase, RecoveryDecisionClass, WalRecord,
        WalRecordPayload,
    },
};
use forge_relational::facade::history::{BranchId, CommitId};
use forge_relational::facade::identity::LineageId;
use forge_relational::facade::lineage::LineageEventRecord;
use std::collections::HashMap;

use super::{
    integrity::{
        branch_key, bulk_checkpoint_artifact_id, bulk_plan_artifact_id, bulk_program_artifact_id,
        bulk_witness_artifact_id, bulk_witness_index_artifact_id,
        commit_support_summary_artifact_id, durable_cursor_identity_artifact_id,
        frozen_bulk_manifest_artifact_id, frozen_transform_basis_artifact_id,
        frozen_transform_partition_artifact_id, stable_structural_digest,
        subscriber_checkpoint_artifact_id,
    },
    records::{
        BulkChunkWitnessRecord, BulkDeterministicPlanRecord, BulkProgramIdentityRecord,
        BulkProgressCheckpointRecord, DurableCursorIdentityRecord, EmbeddedCheckpointRecord,
        FrozenBulkManifestRecord, FrozenTransformBasisRecord, FrozenTransformPartitionRecord,
        LineageSupportRecord, Milestone6ChunkMembershipRecord,
        Milestone6LayoutMaterializationRecord, Milestone6ScopeSliceMembershipRecord,
        Milestone6StructuralBlockRecord, ProgramChunkWitnessIndexRecord, SchemaSupportRecord,
        StoreState, SubscriberCheckpointRecord,
    },
};

pub(crate) trait StatePersistence: std::fmt::Debug {
    fn load_state(&mut self) -> Result<StoreState, StoreError>;
    fn persist_state(&mut self, state: &StoreState) -> Result<DurableMediaReport, StoreError>;
    fn durable_media_report(&self) -> DurableMediaReport;
}

#[derive(Debug)]
pub(crate) struct StateBackedStoreBackend<P> {
    persistence: P,
    state: StoreState,
    milestone_6_access_structure_verification: Milestone6AccessStructureVerification,
    milestone_7_access_structure_verification: Milestone7AccessStructureVerification,
    milestone_6_scope_prepare_counts: HashMap<String, u64>,
    counters: StoreCounters,
}

impl<P: StatePersistence> StateBackedStoreBackend<P> {
    fn refresh_access_structure_verifications(&mut self) {
        let media_report = self.persistence.durable_media_report();
        self.milestone_6_access_structure_verification =
            verify_milestone_6_access_structures(&self.state, media_report);
        self.milestone_7_access_structure_verification =
            verify_milestone_7_access_structures(&self.state, media_report);
    }

    fn commit_replacement_state(&mut self, next: StoreState) -> Result<(), StoreError> {
        next.verify_integrity()?;
        let report = self.persistence.persist_state(&next)?;
        if report.content_barrier() < report.ack_required_barrier() {
            self.counters.record_durable_ack_barrier_violation();
            return Err(StoreError::new(
                StoreErrorKind::DurableBarrierContractViolation,
                format!(
                    "backend {:?} reported content barrier {:?} below required acknowledgment barrier {:?}",
                    report.backend_family(),
                    report.content_barrier(),
                    report.ack_required_barrier()
                ),
            ));
        }
        self.counters.record_durable_barrier_verified();
        self.state = next;
        self.refresh_access_structure_verifications();
        Ok(())
    }

    fn require_admitted_aspect_layout_plan(
        &self,
        request: AspectLayoutReadRequest,
        operation_name: &str,
    ) -> Result<AdmittedAspectLayoutReadPlan, StoreError> {
        match self.plan_aspect_layout_read(request)? {
            AspectLayoutReadPlanDecision::Admitted(plan) => Ok(plan),
            AspectLayoutReadPlanDecision::Fallback(plan) => Err(StoreError::new(
                StoreErrorKind::AspectLayoutFallbackRequired,
                format!(
                    "{operation_name} requires an admitted Milestone 6 layout request, but planning fell back: {}",
                    plan.reason()
                ),
            )),
            AspectLayoutReadPlanDecision::Rejected(plan) => Err(StoreError::new(
                StoreErrorKind::AspectScopeUnsupported,
                format!(
                    "{operation_name} requires an admitted Milestone 6 layout request, but planning rejected the request: {}",
                    plan.reason()
                ),
            )),
        }
    }

    fn append_wal_record_committed(&mut self, record: WalRecord) -> Result<(), StoreError> {
        let inserted_sequence = record.wal_sequence;
        self.state.append_wal_record(record)?;

        if let Err(error) = self.state.verify_wal_record_family() {
            self.state.wal_records.remove(&inserted_sequence);
            self.state.next_wal_sequence = inserted_sequence;
            return Err(error);
        }

        let report = match self.persistence.persist_state(&self.state) {
            Ok(report) => report,
            Err(error) => {
                self.state.wal_records.remove(&inserted_sequence);
                self.state.next_wal_sequence = inserted_sequence;
                return Err(error);
            }
        };

        if report.content_barrier() < report.ack_required_barrier() {
            self.state.wal_records.remove(&inserted_sequence);
            self.state.next_wal_sequence = inserted_sequence;
            self.counters.record_durable_ack_barrier_violation();
            return Err(StoreError::new(
                StoreErrorKind::DurableBarrierContractViolation,
                format!(
                    "backend {:?} reported content barrier {:?} below required acknowledgment barrier {:?}",
                    report.backend_family(),
                    report.content_barrier(),
                    report.ack_required_barrier()
                ),
            ));
        }

        self.counters.record_durable_barrier_verified();
        self.counters.record_state_delta_apply(1, 1);
        Ok(())
    }

    pub fn open_with_persistence(mut persistence: P) -> Result<Self, StoreError> {
        let state = persistence.load_state()?;
        state.verify_integrity()?;
        let milestone_6_access_structure_verification =
            verify_milestone_6_access_structures(&state, persistence.durable_media_report());
        let milestone_7_access_structure_verification =
            verify_milestone_7_access_structures(&state, persistence.durable_media_report());
        Ok(Self {
            persistence,
            state,
            milestone_6_access_structure_verification,
            milestone_7_access_structure_verification,
            milestone_6_scope_prepare_counts: HashMap::new(),
            counters: StoreCounters::default(),
        })
    }

    pub fn open_with_persistence_for_durable_recovery(
        mut persistence: P,
    ) -> Result<Self, StoreError> {
        let state = persistence.load_state()?;
        state.verify_integrity_for_durable_recovery()?;
        let milestone_6_access_structure_verification =
            verify_milestone_6_access_structures(&state, persistence.durable_media_report());
        let milestone_7_access_structure_verification =
            verify_milestone_7_access_structures(&state, persistence.durable_media_report());
        Ok(Self {
            persistence,
            state,
            milestone_6_access_structure_verification,
            milestone_7_access_structure_verification,
            milestone_6_scope_prepare_counts: HashMap::new(),
            counters: StoreCounters::default(),
        })
    }

    pub fn from_export_bundle_with_persistence(
        mut persistence: P,
        bundle: AuthoritativeExportBundle,
    ) -> Result<Self, StoreError> {
        let state = StoreState::from_authoritative_export_bundle(bundle)?;
        let _ = persistence.persist_state(&state)?;
        let milestone_6_access_structure_verification =
            verify_milestone_6_access_structures(&state, persistence.durable_media_report());
        let milestone_7_access_structure_verification =
            verify_milestone_7_access_structures(&state, persistence.durable_media_report());
        Ok(Self {
            persistence,
            state,
            milestone_6_access_structure_verification,
            milestone_7_access_structure_verification,
            milestone_6_scope_prepare_counts: HashMap::new(),
            counters: StoreCounters::default(),
        })
    }

    pub(crate) fn note_milestone_6_scope_prepare(
        &mut self,
        request: &AspectLayoutReadRequest,
    ) -> Result<u64, StoreError> {
        let artifact_id = crate::layout::published_layout_request_artifact_id(request)?;
        let entry = self
            .milestone_6_scope_prepare_counts
            .entry(artifact_id)
            .or_insert(0);
        *entry += 1;
        Ok(*entry)
    }

    pub(crate) fn milestone_6_branch_has_materialized_support(&self, branch_id: &BranchId) -> bool {
        self.state
            .milestone_6_layout_materialization_records
            .values()
            .any(|record| {
                record
                    .materialization
                    .admitted_plan()
                    .request()
                    .target()
                    .branch_id()
                    == branch_id
            })
    }

    pub(crate) fn record_milestone_6_proof_only_prepare(&self) {
        self.counters.record_milestone_6_proof_only_prepare();
    }

    pub(crate) fn record_milestone_6_on_demand_materialize(&self) {
        self.counters.record_milestone_6_on_demand_materialize();
    }

    pub(crate) fn record_milestone_6_policy_eager_resolution(&self) {
        self.counters.record_milestone_6_policy_eager_resolution();
    }

    pub(crate) fn record_milestone_6_policy_eager_publish(&self) {
        self.counters.record_milestone_6_policy_eager_publish();
    }

    pub(crate) fn record_milestone_6_policy_eager_reuse_existing(&self) {
        self.counters
            .record_milestone_6_policy_eager_reuse_existing();
    }

    pub fn create_branch(
        &mut self,
        new_branch: BranchId,
        from_branch: Option<&BranchId>,
    ) -> Result<AuthoritativeBranchHeadRecord, StoreError> {
        let created_branch_id = new_branch.clone();
        let applied = self
            .state
            .apply_branch_creation_in_place(new_branch, from_branch)?;
        if let Err(error) = self.state.verify_applied_branch_creation(&applied) {
            self.state.rollback_branch_creation(applied);
            return Err(error);
        }
        let report = match self.persistence.persist_state(&self.state) {
            Ok(report) => report,
            Err(error) => {
                self.state.rollback_branch_creation(applied);
                return Err(error);
            }
        };
        if report.content_barrier() < report.ack_required_barrier() {
            self.state.rollback_branch_creation(applied);
            self.counters.record_durable_ack_barrier_violation();
            return Err(StoreError::new(
                StoreErrorKind::DurableBarrierContractViolation,
                format!(
                    "backend {:?} reported content barrier {:?} below required acknowledgment barrier {:?}",
                    report.backend_family(),
                    report.content_barrier(),
                    report.ack_required_barrier()
                ),
            ));
        }
        self.counters.record_durable_barrier_verified();
        self.counters.record_state_delta_apply(2, 2);
        self.counters.record_branch_create();
        self.fetch_branch_head(&created_branch_id)
    }

    pub fn create_shared_base_branch(
        &mut self,
        request: SharedBaseBranchCreationRequest,
    ) -> Result<SharedBaseBranchCreationReceipt, StoreError> {
        let (applied, receipt) = self
            .state
            .apply_shared_base_branch_creation_in_place(request)?;
        if let Err(error) = self
            .state
            .verify_applied_shared_base_branch_creation(&applied)
        {
            self.state.rollback_shared_base_branch_creation(applied);
            return Err(error);
        }
        let report = match self.persistence.persist_state(&self.state) {
            Ok(report) => report,
            Err(error) => {
                self.state.rollback_shared_base_branch_creation(applied);
                return Err(error);
            }
        };
        if report.content_barrier() < report.ack_required_barrier() {
            self.state.rollback_shared_base_branch_creation(applied);
            self.counters.record_durable_ack_barrier_violation();
            return Err(StoreError::new(
                StoreErrorKind::DurableBarrierContractViolation,
                format!(
                    "backend {:?} reported content barrier {:?} below required acknowledgment barrier {:?}",
                    report.backend_family(),
                    report.content_barrier(),
                    report.ack_required_barrier()
                ),
            ));
        }
        self.counters.record_durable_barrier_verified();
        self.counters.record_state_delta_apply(3, 3);
        self.counters.record_branch_create();
        self.counters.record_branch_base_reuse();
        Ok(receipt)
    }

    pub fn admit_shared_base_branch_creation(
        &self,
        request: SharedBaseBranchCreationRequest,
    ) -> Result<SharedBaseBranchCreationWitness, StoreError> {
        self.state.admit_shared_base_branch_creation(request)
    }

    pub fn verify_append(
        &self,
        append: CanonicalizedCommitEnvelope,
    ) -> Result<VerifiedAuthoritativeAppend, StoreError> {
        self.state.verify_authoritative_append(append)
    }

    pub fn append(
        &mut self,
        verified: VerifiedAuthoritativeAppend,
    ) -> Result<PersistedAuthoritativeCommit, StoreError> {
        let commit_id = verified.envelope().commit.commit_id;
        if let Some(existing) = self.state.commit_record(commit_id) {
            return Ok(existing.clone().into_persisted());
        }

        let branch_already_exists = self
            .state
            .branch_exists(&verified.envelope().branch_context);
        let emits_schema_support = verified.envelope().schema_transition.is_some()
            || verified.envelope().schema_continuation_descriptor.is_some()
            || verified
                .envelope()
                .schema_reconciliation_descriptor
                .is_some();
        let emits_lineage_support = !verified.envelope().lineage_event_ids().is_empty()
            || !verified.envelope().lineage_events().is_empty();
        let support_family_writes =
            1 + u64::from(emits_schema_support) + u64::from(emits_lineage_support);
        let digest_writes = verified.envelope().commit.parents.len() as u64
            + support_family_writes
            + if branch_already_exists { 2 } else { 4 };
        let branch_head_writes = if branch_already_exists { 1 } else { 2 };
        let touched_families = if branch_already_exists { 3 } else { 4 }
            + usize::from(emits_schema_support)
            + usize::from(emits_lineage_support)
            + 1;
        let touched_records = verified.envelope().commit.parents.len()
            + support_family_writes as usize
            + if branch_already_exists { 2 } else { 3 };
        let applied = self.state.apply_verified_append_in_place(&verified)?;
        if let Err(error) = self.state.verify_applied_authoritative_append(&applied) {
            self.state.rollback_verified_append(applied);
            return Err(error);
        }
        let report = match self.persistence.persist_state(&self.state) {
            Ok(report) => report,
            Err(error) => {
                self.state.rollback_verified_append(applied);
                return Err(error);
            }
        };
        if report.content_barrier() < report.ack_required_barrier() {
            self.state.rollback_verified_append(applied);
            self.counters.record_durable_ack_barrier_violation();
            return Err(StoreError::new(
                StoreErrorKind::DurableBarrierContractViolation,
                format!(
                    "backend {:?} reported content barrier {:?} below required acknowledgment barrier {:?}",
                    report.backend_family(),
                    report.content_barrier(),
                    report.ack_required_barrier()
                ),
            ));
        }
        self.counters.record_durable_barrier_verified();
        self.counters
            .record_state_delta_apply(touched_families as u64, touched_records as u64);
        self.counters.record_append(
            verified.envelope().commit.parents.len(),
            digest_writes,
            branch_head_writes,
        );
        self.counters.record_commit_support_summary_build();
        self.counters.record_commit_support_publication();
        self.state
            .commit_envelopes
            .get(&commit_id.0)
            .cloned()
            .map(super::records::StoredCommitEnvelope::into_persisted)
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::AuthoritativeAppendAtomicityViolation,
                    format!("commit {} missing after successful append", commit_id.0),
                )
            })
    }

    pub fn fetch_commit(
        &self,
        commit_id: CommitId,
    ) -> Result<FetchedAuthoritativeCommit, StoreError> {
        let stored = self.state.commit_record(commit_id).ok_or_else(|| {
            StoreError::new(
                StoreErrorKind::CommitNotFound,
                format!("commit {} not found", commit_id.0),
            )
        })?;
        let verification = self.state.verify_commit_record(stored);
        self.counters
            .record_fetch_verification(verification.is_ok());
        verification?;
        Ok(stored.clone().into_fetched())
    }

    pub fn fetch_branch_head(
        &self,
        branch_id: &BranchId,
    ) -> Result<AuthoritativeBranchHeadRecord, StoreError> {
        let record = self
            .state
            .branch_head_records
            .get(&branch_key(branch_id))
            .ok_or_else(|| StoreError::unknown_branch(branch_id))?;
        let head = match record.head_commit_id {
            Some(head_commit_id) => {
                let stored = self
                    .state
                    .commit_envelopes
                    .get(&head_commit_id.0)
                    .ok_or_else(|| {
                        StoreError::backend_integrity(format!(
                            "branch `{}` points at missing head commit {}",
                            branch_id.0, head_commit_id.0
                        ))
                    })?;
                Some(stored.envelope.commit.clone())
            }
            None => None,
        };
        Ok(AuthoritativeBranchHeadRecord::new(
            record.branch_id.clone(),
            head,
            record.head_update_sequence,
        ))
    }

    pub fn plan_branch_delta_read(
        &self,
        request: BranchDeltaReadRequest,
    ) -> Result<BranchDeltaReadPlan, StoreError> {
        match self.state.plan_branch_delta_read(request) {
            Ok(plan) => {
                self.counters.record_branch_delta_read(
                    plan.performance.layers_traversed,
                    plan.performance.records_decoded.max(plan.commit_ids.len()),
                    plan.performance.replay_commit_count,
                    matches!(
                        plan.performance.fallback_class,
                        BranchDeltaFallbackClass::RequiresAuthorityReplayControlLane
                    ),
                );
                Ok(plan)
            }
            Err(error) => {
                if matches!(
                    error.kind(),
                    StoreErrorKind::BranchDeltaTargetRequiresMergeAwareWidening
                ) {
                    self.counters.record_branch_delta_merge_path_search();
                }
                if matches!(
                    error.kind(),
                    StoreErrorKind::BranchDeltaDigestMismatch
                        | StoreErrorKind::BranchDeltaPublicationGap
                        | StoreErrorKind::BranchDeltaIntegrityFailure
                ) {
                    self.counters.record_branch_delta_integrity_failure();
                }
                Err(error)
            }
        }
    }

    pub fn admit_same_branch_descendant(
        &self,
        request: BranchDeltaReadRequest,
    ) -> Result<SameBranchDescendantWitness, StoreError> {
        self.state.admit_same_branch_descendant(request)
    }

    pub fn admit_milestone_7_independent_reference(
        &self,
        request: BranchDeltaReadRequest,
    ) -> Result<crate::Milestone7IndependentReference, StoreError> {
        self.state.admit_milestone_7_independent_reference(request)
    }

    pub fn plan_aspect_layout_read(
        &self,
        request: AspectLayoutReadRequest,
    ) -> Result<AspectLayoutReadPlanDecision, StoreError> {
        let decision = self.state.plan_aspect_layout_read(request)?;
        match &decision {
            AspectLayoutReadPlanDecision::Admitted(plan) => {
                self.counters.record_aspect_layout_plan(
                    true,
                    false,
                    false,
                    plan.performance().layout_slices_read,
                    plan.performance().blocks_decoded,
                    plan.performance().control_replay_breadth,
                )
            }
            AspectLayoutReadPlanDecision::Fallback(plan) => {
                self.counters.record_aspect_layout_plan(
                    false,
                    true,
                    false,
                    plan.performance().layout_slices_read,
                    plan.performance().blocks_decoded,
                    plan.performance().control_replay_breadth,
                )
            }
            AspectLayoutReadPlanDecision::Rejected(_) => self
                .counters
                .record_aspect_layout_plan(false, false, true, 0, 0, 0),
        }
        Ok(decision)
    }

    pub fn admit_structural_block_reuse(
        &self,
        plan: AdmittedAspectLayoutReadPlan,
    ) -> Result<DedupAdmittedBlockReuse, StoreError> {
        let admitted = self.state.admit_structural_block_reuse(plan)?;
        self.counters.record_structural_block_reuse_admission();
        Ok(admitted)
    }

    pub fn freeze_chunk_model(
        &self,
        plan: AdmittedAspectLayoutReadPlan,
    ) -> Result<ChunkModelFrozenPhysicalLayout, StoreError> {
        match self.state.freeze_chunk_model(plan) {
            Ok(frozen) => {
                self.counters.record_chunk_model_freeze();
                Ok(frozen)
            }
            Err(error) => {
                if matches!(
                    error.kind(),
                    StoreErrorKind::PhysicalChunkDeterminismViolation
                ) {
                    self.counters.record_physical_chunk_determinism_violation();
                }
                Err(error)
            }
        }
    }

    pub fn admit_milestone_7_independent_layout_reference(
        &self,
        plan: AdmittedAspectLayoutReadPlan,
    ) -> Result<Milestone7IndependentLayoutReference, StoreError> {
        let reference = self
            .state
            .admit_milestone_7_independent_layout_reference(plan)?;
        self.counters
            .record_milestone_7_layout_reference_admission();
        Ok(reference)
    }

    pub fn admit_milestone_9_physical_chunk_reference(
        &self,
        frozen: ChunkModelFrozenPhysicalLayout,
    ) -> Result<Milestone9PhysicalChunkReference, StoreError> {
        let reference = self
            .state
            .admit_milestone_9_physical_chunk_reference(frozen)?;
        self.counters
            .record_milestone_9_physical_chunk_reference_admission();
        Ok(reference)
    }

    pub fn materialize_milestone_6_layout_support(
        &mut self,
        request: AspectLayoutReadRequest,
    ) -> Result<Milestone6LayoutMaterialization, StoreError> {
        let plan = self.require_admitted_aspect_layout_plan(request, "layout materialization")?;
        let block_reuse = self.admit_structural_block_reuse(plan.clone())?;
        let frozen_layout = self.freeze_chunk_model(plan.clone())?;
        let milestone_7_reference =
            self.admit_milestone_7_independent_layout_reference(plan.clone())?;
        let milestone_9_reference =
            self.admit_milestone_9_physical_chunk_reference(frozen_layout.clone())?;
        let control = self
            .state
            .read_branch_delta_control_from_milestone_7_reference(
                crate::Milestone7IndependentReference::new(
                    milestone_7_reference.branch_id().clone(),
                    milestone_7_reference.frontier_commit_id(),
                ),
            )?;
        let artifact_id = crate::layout::layout_materialization_artifact_id(&plan);
        let materialization = Milestone6LayoutMaterialization::new(
            artifact_id.clone(),
            plan,
            block_reuse,
            frozen_layout,
            milestone_7_reference,
            milestone_9_reference,
            crate::layout::stable_layout_truth_digest(control.authoritative_export()),
            control.authoritative_export().commit_envelopes.len(),
        );
        let authority_basis_commit = self
            .state
            .commit_record(materialization.admitted_plan().request().target().frontier_commit_id())
            .ok_or_else(|| {
                StoreError::backend_integrity(format!(
                    "milestone 6 layout materialization `{}` targeted missing authority frontier commit `{}`",
                    materialization.artifact_id(),
                    materialization
                        .admitted_plan()
                        .request()
                        .target()
                        .frontier_commit_id()
                        .0
                ))
            })?;
        let commit_coupled_seed_record = milestone_6_commit_coupled_layout_seed_record(
            &materialization,
            authority_basis_commit,
        )?;
        let scope_membership_record = milestone_6_scope_slice_membership_record(&materialization)?;
        let chunk_membership_record = milestone_6_chunk_membership_record(&materialization);
        let structural_block_record = milestone_6_structural_block_record(&materialization);

        let mut next = self.state.clone();
        next.milestone_6_layout_materialization_records.insert(
            artifact_id.clone(),
            Milestone6LayoutMaterializationRecord {
                artifact_id,
                materialization: materialization.clone(),
            },
        );
        next.milestone_6_commit_coupled_layout_seed_records.insert(
            commit_coupled_seed_record.artifact_id.clone(),
            commit_coupled_seed_record,
        );
        attach_milestone_6_commit_coupled_layout_seed_to_commit_support_summary(
            &mut next,
            materialization
                .admitted_plan()
                .request()
                .target()
                .frontier_commit_id(),
            &materialization,
        )?;
        next.milestone_6_scope_slice_membership_records.insert(
            scope_membership_record.artifact_id.clone(),
            scope_membership_record,
        );
        next.milestone_6_chunk_membership_records.insert(
            chunk_membership_record.artifact_id.clone(),
            chunk_membership_record,
        );
        merge_milestone_6_structural_block_record(&mut next, structural_block_record);
        self.commit_replacement_state(next)?;
        Ok(materialization)
    }

    pub fn fetch_milestone_6_layout_support(
        &self,
        request: AspectLayoutReadRequest,
    ) -> Result<Milestone6LayoutMaterialization, StoreError> {
        let plan =
            self.require_admitted_aspect_layout_plan(request, "layout materialization fetch")?;
        let artifact_id = crate::layout::layout_materialization_artifact_id(&plan);
        self.fetch_existing_milestone_6_layout_support(&artifact_id)
    }

    pub(crate) fn fetch_existing_milestone_6_layout_support(
        &self,
        artifact_id: &str,
    ) -> Result<Milestone6LayoutMaterialization, StoreError> {
        self.state
            .milestone_6_layout_materialization_records
            .get(artifact_id)
            .map(|record| record.materialization.clone())
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::AspectLayoutArtifactMissing,
                    format!("milestone 6 layout materialization `{artifact_id}` not found"),
                )
            })
    }

    pub fn rebuild_milestone_6_derived_artifacts_from_materializations(
        &mut self,
    ) -> Result<crate::Milestone6DerivedArtifactRebuildReport, StoreError> {
        let commit_coupled_seeds =
            milestone_6_commit_coupled_layout_seed_rebuild_records(&self.state)?;
        let mut next = self.state.clone();
        next.milestone_6_scope_slice_membership_records.clear();
        next.milestone_6_chunk_membership_records.clear();
        next.milestone_6_structural_block_records.clear();

        for commit_coupled_seed in &commit_coupled_seeds {
            let plan = match self
                .state
                .plan_aspect_layout_read(commit_coupled_seed.request.clone())?
            {
                crate::AspectLayoutReadPlanDecision::Admitted(plan) => plan,
                crate::AspectLayoutReadPlanDecision::Fallback(plan) => {
                    return Err(StoreError::backend_integrity(format!(
                        "commit-coupled milestone 6 layout seed `{}` no longer admits during rebuild: {}",
                        commit_coupled_seed.artifact_id,
                        plan.reason()
                    )))
                }
                crate::AspectLayoutReadPlanDecision::Rejected(plan) => {
                    return Err(StoreError::backend_integrity(format!(
                        "commit-coupled milestone 6 layout seed `{}` was rejected during rebuild: {}",
                        commit_coupled_seed.artifact_id,
                        plan.reason()
                    )))
                }
            };
            let expected_materialization_artifact_id =
                crate::layout::layout_materialization_artifact_id(&plan);
            if commit_coupled_seed.layout_materialization_artifact_id
                != expected_materialization_artifact_id
            {
                return Err(StoreError::backend_integrity(format!(
                    "commit-coupled milestone 6 layout seed `{}` drifted from expected materialization `{expected_materialization_artifact_id}`",
                    commit_coupled_seed.artifact_id
                )));
            }
            let materialization = self.fetch_existing_milestone_6_layout_support(
                &commit_coupled_seed.layout_materialization_artifact_id,
            )?;
            if materialization.admitted_plan() != &plan {
                return Err(StoreError::backend_integrity(format!(
                    "persisted milestone 6 materialization `{}` drifted from rebuild admission plan",
                    materialization.artifact_id()
                )));
            }
            let scope_membership_record =
                milestone_6_scope_slice_membership_record(&materialization)?;
            let chunk_membership_record = milestone_6_chunk_membership_record(&materialization);
            let structural_block_record = milestone_6_structural_block_record(&materialization);
            next.milestone_6_scope_slice_membership_records.insert(
                scope_membership_record.artifact_id.clone(),
                scope_membership_record,
            );
            next.milestone_6_chunk_membership_records.insert(
                chunk_membership_record.artifact_id.clone(),
                chunk_membership_record,
            );
            merge_milestone_6_structural_block_record(&mut next, structural_block_record);
        }

        self.commit_replacement_state(next)?;
        Ok(crate::Milestone6DerivedArtifactRebuildReport::new(
            self.state.milestone_6_layout_materialization_records.len(),
            self.state.milestone_6_scope_slice_membership_records.len(),
            self.state.milestone_6_structural_block_records.len(),
            self.state.milestone_6_chunk_membership_records.len(),
        ))
    }

    pub fn rebuild_milestone_6_derived_artifacts_from_authority(
        &mut self,
    ) -> Result<crate::Milestone6DerivedArtifactRebuildReport, StoreError> {
        let commit_coupled_seeds =
            milestone_6_commit_coupled_layout_seed_rebuild_records(&self.state)?;
        let mut next = self.state.clone();
        next.milestone_6_layout_materialization_records.clear();
        next.milestone_6_scope_slice_membership_records.clear();
        next.milestone_6_chunk_membership_records.clear();
        next.milestone_6_structural_block_records.clear();

        for commit_coupled_seed in &commit_coupled_seeds {
            let plan = match self
                .state
                .plan_aspect_layout_read(commit_coupled_seed.request.clone())?
            {
                crate::AspectLayoutReadPlanDecision::Admitted(plan) => plan,
                crate::AspectLayoutReadPlanDecision::Fallback(plan) => {
                    return Err(StoreError::backend_integrity(format!(
                        "commit-coupled milestone 6 layout seed `{}` no longer admits during authority rebuild: {}",
                        commit_coupled_seed.artifact_id,
                        plan.reason()
                    )))
                }
                crate::AspectLayoutReadPlanDecision::Rejected(plan) => {
                    return Err(StoreError::backend_integrity(format!(
                        "commit-coupled milestone 6 layout seed `{}` was rejected during authority rebuild: {}",
                        commit_coupled_seed.artifact_id,
                        plan.reason()
                    )))
                }
            };
            let artifact_id = crate::layout::layout_materialization_artifact_id(&plan);
            if commit_coupled_seed.layout_materialization_artifact_id != artifact_id {
                return Err(StoreError::backend_integrity(format!(
                    "commit-coupled milestone 6 layout seed `{}` drifted from expected authority-rebuilt materialization `{artifact_id}`",
                    commit_coupled_seed.artifact_id
                )));
            }
            let block_reuse = self.state.admit_structural_block_reuse(plan.clone())?;
            let frozen_layout = self.state.freeze_chunk_model(plan.clone())?;
            let milestone_7_reference = self
                .state
                .admit_milestone_7_independent_layout_reference(plan.clone())?;
            let milestone_9_reference = self
                .state
                .admit_milestone_9_physical_chunk_reference(frozen_layout.clone())?;
            let control = self
                .state
                .read_branch_delta_control_from_milestone_7_reference(
                    crate::Milestone7IndependentReference::new(
                        milestone_7_reference.branch_id().clone(),
                        milestone_7_reference.frontier_commit_id(),
                    ),
                )?;
            let materialization = Milestone6LayoutMaterialization::new(
                artifact_id.clone(),
                plan,
                block_reuse,
                frozen_layout,
                milestone_7_reference,
                milestone_9_reference,
                crate::layout::stable_layout_truth_digest(control.authoritative_export()),
                control.authoritative_export().commit_envelopes.len(),
            );
            let scope_membership_record =
                milestone_6_scope_slice_membership_record(&materialization)?;
            let chunk_membership_record = milestone_6_chunk_membership_record(&materialization);
            let structural_block_record = milestone_6_structural_block_record(&materialization);
            next.milestone_6_layout_materialization_records.insert(
                artifact_id.clone(),
                Milestone6LayoutMaterializationRecord {
                    artifact_id,
                    materialization,
                },
            );
            next.milestone_6_scope_slice_membership_records.insert(
                scope_membership_record.artifact_id.clone(),
                scope_membership_record,
            );
            next.milestone_6_chunk_membership_records.insert(
                chunk_membership_record.artifact_id.clone(),
                chunk_membership_record,
            );
            merge_milestone_6_structural_block_record(&mut next, structural_block_record);
        }

        self.commit_replacement_state(next)?;
        Ok(crate::Milestone6DerivedArtifactRebuildReport::new(
            self.state.milestone_6_layout_materialization_records.len(),
            self.state.milestone_6_scope_slice_membership_records.len(),
            self.state.milestone_6_structural_block_records.len(),
            self.state.milestone_6_chunk_membership_records.len(),
        ))
    }

    pub fn structural_block_lookup(
        &self,
        lookup: StructuralBlockLookup,
    ) -> Result<StructuralBlockLookupResult, StoreError> {
        match self.state.structural_block_lookup(lookup) {
            Ok(result) => {
                self.counters.record_structural_block_lookup(true);
                Ok(result)
            }
            Err(error) => {
                if matches!(error.kind(), StoreErrorKind::AspectLayoutArtifactMissing) {
                    self.counters.record_structural_block_lookup(false);
                }
                Err(error)
            }
        }
    }

    pub fn execute_aspect_layout_read(
        &self,
        request: AspectLayoutReadRequest,
    ) -> Result<AspectLayoutReadExecutionDecision, StoreError> {
        self.state.execute_aspect_layout_read(request)
    }

    pub fn execute_dedup_backed_read(
        &self,
        request: AspectLayoutReadRequest,
    ) -> Result<DedupBackedReadResult, StoreError> {
        let read = match self.execute_aspect_layout_read(request)? {
            AspectLayoutReadExecutionDecision::Admitted(read) => read,
            AspectLayoutReadExecutionDecision::Fallback(plan) => {
                return Err(StoreError::new(
                    StoreErrorKind::AspectLayoutFallbackRequired,
                    plan.reason().to_string(),
                ))
            }
            AspectLayoutReadExecutionDecision::Rejected(plan) => {
                return Err(StoreError::new(
                    StoreErrorKind::AspectScopeUnsupported,
                    plan.reason().to_string(),
                ))
            }
        };
        let lookup = self.structural_block_lookup(StructuralBlockLookup::new(
            read.plan().structural_block_id().clone(),
        ))?;
        if lookup.slice_ids() != read.plan().slice_ids() {
            return Err(StoreError::backend_integrity(
                "dedup-backed read structural block lookup drifted from admitted plan slice ids",
            ));
        }
        Ok(DedupBackedReadResult::new(read, lookup))
    }

    pub fn read_branch_delta(
        &self,
        witness: SameBranchDescendantWitness,
    ) -> Result<BranchDeltaReadResult, StoreError> {
        let result = self.state.read_branch_delta(witness)?;
        self.counters.record_branch_delta_read(
            result.plan.performance.layers_traversed,
            branch_delta_result_record_count(&result),
            result.plan.performance.replay_commit_count,
            matches!(
                result.plan.performance.fallback_class,
                BranchDeltaFallbackClass::RequiresAuthorityReplayControlLane
            ),
        );
        Ok(result)
    }

    pub fn read_branch_delta_control(
        &self,
        witness: SameBranchDescendantWitness,
    ) -> Result<BranchDeltaReadResult, StoreError> {
        let result = self.state.read_branch_delta_control(witness)?;
        self.counters.record_branch_delta_read(
            result.plan.performance.layers_traversed,
            branch_delta_result_record_count(&result),
            result.plan.performance.replay_commit_count,
            matches!(
                result.plan.strategy,
                BranchDeltaReadStrategy::AuthorityReplayControl
            ),
        );
        Ok(result)
    }

    pub fn read_branch_delta_control_from_milestone_7_reference(
        &self,
        reference: crate::Milestone7IndependentReference,
    ) -> Result<BranchDeltaReadResult, StoreError> {
        let result = self
            .state
            .read_branch_delta_control_from_milestone_7_reference(reference)?;
        self.counters.record_branch_delta_read(
            result.plan.performance.layers_traversed,
            branch_delta_result_record_count(&result),
            result.plan.performance.replay_commit_count,
            matches!(
                result.plan.strategy,
                BranchDeltaReadStrategy::AuthorityReplayControl
            ),
        );
        Ok(result)
    }

    pub(crate) fn milestone_5_delta_storage_report(
        &self,
        branch_id: BranchId,
        target_commit_id: CommitId,
        direct_plan: &BranchDeltaReadPlan,
        control_plan: &BranchDeltaReadPlan,
    ) -> Result<crate::Milestone5DeltaStorageReport, StoreError> {
        self.state.milestone_5_delta_storage_report(
            branch_id,
            target_commit_id,
            direct_plan,
            control_plan,
        )
    }

    pub fn plan_delta_rewrite(
        &self,
        request: BranchDeltaRewriteRequest,
    ) -> Result<BranchDeltaRewritePlan, StoreError> {
        self.state.plan_delta_rewrite(request)
    }

    pub fn recommend_delta_rewrite(
        &self,
        request: BranchDeltaRewriteRequest,
    ) -> Result<BranchDeltaRewriteRecommendation, StoreError> {
        self.state.recommend_delta_rewrite(request)
    }

    pub fn auto_compact_branch_delta(
        &mut self,
        request: BranchDeltaRewriteRequest,
    ) -> Result<BranchDeltaAutoCompactOutcome, StoreError> {
        let recommendation = self.state.recommend_delta_rewrite(request.clone())?;
        match recommendation.decision {
            crate::BranchDeltaRewritePolicyDecision::CompactNow => {
                let rewrite_receipt = self.rewrite_branch_delta(recommendation.plan.clone())?;
                Ok(BranchDeltaAutoCompactOutcome {
                    disposition: BranchDeltaAutoCompactDisposition::Compacted,
                    recommendation,
                    rewrite_receipt: Some(rewrite_receipt),
                })
            }
            crate::BranchDeltaRewritePolicyDecision::NoAction => {
                Ok(BranchDeltaAutoCompactOutcome {
                    disposition: BranchDeltaAutoCompactDisposition::NoAction,
                    recommendation,
                    rewrite_receipt: None,
                })
            }
            crate::BranchDeltaRewritePolicyDecision::Defer => Ok(BranchDeltaAutoCompactOutcome {
                disposition: BranchDeltaAutoCompactDisposition::Deferred,
                recommendation,
                rewrite_receipt: None,
            }),
            crate::BranchDeltaRewritePolicyDecision::RejectAsTooBroad => {
                Ok(BranchDeltaAutoCompactOutcome {
                    disposition: BranchDeltaAutoCompactDisposition::RejectedAsTooBroad,
                    recommendation,
                    rewrite_receipt: None,
                })
            }
        }
    }

    pub fn rewrite_branch_delta(
        &mut self,
        plan: BranchDeltaRewritePlan,
    ) -> Result<BranchDeltaRewriteReceipt, StoreError> {
        if !matches!(
            plan.strategy(),
            BranchDeltaRewriteStrategy::ReplaceContiguousSegment
        ) {
            return Err(StoreError::new(
                StoreErrorKind::BranchDeltaRewriteTargetIllegal,
                "branch delta rewrite execution requires an admitted rewrite plan",
            ));
        }
        let rewrite_record_count = plan
            .segment()
            .map(|segment| segment.commit_ids().len())
            .unwrap_or(0);
        let replaced_layer_count = plan
            .segment()
            .map(|segment| segment.layer_ids().len())
            .unwrap_or(0);
        let (applied, receipt) = self.state.apply_delta_rewrite_plan_in_place(plan)?;
        if let Err(error) = self.state.verify_applied_delta_rewrite(&applied) {
            self.state.rollback_delta_rewrite(applied);
            return Err(error);
        }
        let report = match self.persistence.persist_state(&self.state) {
            Ok(report) => report,
            Err(error) => {
                self.state.rollback_delta_rewrite(applied);
                return Err(error);
            }
        };
        if report.content_barrier() < report.ack_required_barrier() {
            self.state.rollback_delta_rewrite(applied);
            self.counters.record_durable_ack_barrier_violation();
            return Err(StoreError::new(
                StoreErrorKind::DurableBarrierContractViolation,
                format!(
                    "backend {:?} reported content barrier {:?} below required acknowledgment barrier {:?}",
                    report.backend_family(),
                    report.content_barrier(),
                    report.ack_required_barrier()
                ),
            ));
        }
        self.counters.record_durable_barrier_verified();
        self.counters
            .record_state_delta_apply(1, (replaced_layer_count + 1) as u64);
        self.counters.record_branch_delta_rewrite(
            replaced_layer_count,
            rewrite_record_count,
            false,
        );
        Ok(receipt)
    }

    pub fn rebuild_branch_delta_artifacts(
        &mut self,
        branch_id: BranchId,
    ) -> Result<BranchDeltaRebuildReceipt, StoreError> {
        let (applied, receipt) = self.state.apply_branch_delta_rebuild_in_place(branch_id)?;
        if let Err(error) = self.state.verify_applied_branch_delta_rebuild(&applied) {
            self.state.rollback_branch_delta_rebuild(applied);
            return Err(error);
        }
        let report = match self.persistence.persist_state(&self.state) {
            Ok(report) => report,
            Err(error) => {
                self.state.rollback_branch_delta_rebuild(applied);
                return Err(error);
            }
        };
        if report.content_barrier() < report.ack_required_barrier() {
            self.state.rollback_branch_delta_rebuild(applied);
            self.counters.record_durable_ack_barrier_violation();
            return Err(StoreError::new(
                StoreErrorKind::DurableBarrierContractViolation,
                format!(
                    "backend {:?} reported content barrier {:?} below required acknowledgment barrier {:?}",
                    report.backend_family(),
                    report.content_barrier(),
                    report.ack_required_barrier()
                ),
            ));
        }
        self.counters.record_durable_barrier_verified();
        self.counters
            .record_state_delta_apply(1, receipt.rebuilt_layer_count as u64);
        self.counters
            .record_branch_delta_rebuild(receipt.rebuilt_layer_count);
        Ok(receipt)
    }

    pub fn fetch_schema_support(
        &self,
        commit_id: CommitId,
    ) -> Result<FetchedSchemaSupportArtifact, StoreError> {
        let record = self.fetch_verified_schema_support_record(commit_id)?;
        self.counters.record_schema_boundary_fetch(1, 1);
        Ok(FetchedSchemaSupportArtifact::new(record))
    }

    pub fn fetch_lineage_support(
        &self,
        commit_id: CommitId,
    ) -> Result<FetchedLineageSupportArtifact, StoreError> {
        let record = self.fetch_verified_lineage_support_record(commit_id)?;
        self.record_lineage_lookup(&record);
        Ok(FetchedLineageSupportArtifact::new(record))
    }

    pub fn fetch_lineage_history(
        &self,
        request: HistoricalIdentityRequest,
    ) -> Result<HistoricalIdentityResolution, StoreError> {
        let record = self.fetch_verified_lineage_support_record(request.commit_id())?;
        self.record_lineage_lookup(&record);

        if record.branch_id != *request.branch_id() {
            return Err(StoreError::new(
                StoreErrorKind::HistoricalIdentityResolutionGap,
                format!(
                    "historical identity request for commit {} expected branch `{}` but durable lineage support belongs to `{}`",
                    request.commit_id().0,
                    request.branch_id().0,
                    record.branch_id.0
                ),
            ));
        }

        let matching_events = record
            .lineage_events
            .iter()
            .filter(|event| lineage_event_touches(event, request.lineage_id()))
            .cloned()
            .collect::<Vec<_>>();
        if matching_events.is_empty() {
            return Err(StoreError::new(
                StoreErrorKind::HistoricalIdentityResolutionGap,
                format!(
                    "historical identity request for lineage {} found no durable lineage neighborhood in commit {} on branch `{}`",
                    request.lineage_id().0,
                    request.commit_id().0,
                    request.branch_id().0
                ),
            ));
        }

        let mut resolved_lineage_ids = matching_events
            .iter()
            .flat_map(|event: &LineageEventRecord| {
                event
                    .sources()
                    .iter()
                    .chain(event.targets().iter())
                    .copied()
            })
            .collect::<Vec<_>>();
        resolved_lineage_ids.sort_unstable();
        resolved_lineage_ids.dedup();

        Ok(HistoricalIdentityResolution::new(
            request.commit_id(),
            request.branch_id().clone(),
            request.lineage_id(),
            record.artifact_id,
            resolved_lineage_ids,
            matching_events,
            record.lineage_digest_basis,
            record.event_batch_digest_basis,
            record.decision_log_digest_basis,
        ))
    }

    fn fetch_verified_schema_support_record(
        &self,
        commit_id: CommitId,
    ) -> Result<SchemaSupportRecord, StoreError> {
        let artifact_id = super::integrity::schema_support_artifact_id(commit_id);
        let record = self
            .state
            .schema_support_records
            .get(&artifact_id)
            .cloned()
            .ok_or_else(|| {
                self.counters.record_commit_support_publication_gap();
                StoreError::new(
                    StoreErrorKind::SchemaBoundaryArtifactMissing,
                    format!(
                        "schema support artifact for commit {} not found",
                        commit_id.0
                    ),
                )
            })?;
        let verification = self.state.verify_schema_support_record(&record);
        if verification.is_err() {
            self.counters.record_commit_support_publication_gap();
        }
        verification?;
        Ok(record)
    }

    fn fetch_verified_lineage_support_record(
        &self,
        commit_id: CommitId,
    ) -> Result<LineageSupportRecord, StoreError> {
        let artifact_id = super::integrity::lineage_support_artifact_id(commit_id);
        let record = self
            .state
            .lineage_support_records
            .get(&artifact_id)
            .cloned()
            .ok_or_else(|| {
                self.counters.record_commit_support_publication_gap();
                StoreError::new(
                    StoreErrorKind::LineageArtifactMissing,
                    format!(
                        "lineage support artifact for commit {} not found",
                        commit_id.0
                    ),
                )
            })?;
        let verification = self.state.verify_lineage_support_record(&record);
        if verification.is_err() {
            self.counters.record_commit_support_publication_gap();
        }
        verification?;
        Ok(record)
    }

    fn record_lineage_lookup(&self, record: &LineageSupportRecord) {
        self.counters
            .record_lineage_lookup(1, record.lineage_events.len() as u64);
    }

    pub fn acknowledge_cursor(
        &mut self,
        request: DurableCursorAcknowledgeRequest,
    ) -> Result<PersistedSubscriberCheckpoint, StoreError> {
        let cursor_id = request.cursor_id().to_string();
        let cursor_artifact_id = durable_cursor_identity_artifact_id(&cursor_id);
        self.counters.record_cursor_identity_lookup();

        let basis_commit = self
            .state
            .commit_record(request.basis_commit_id())
            .cloned()
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::CursorBasisMismatch,
                    format!(
                        "durable cursor `{}` references missing basis commit {}",
                        request.cursor_id(),
                        request.basis_commit_id().0
                    ),
                )
            })?;
        if basis_commit.envelope.branch_context != *request.branch_id() {
            return Err(StoreError::new(
                StoreErrorKind::CursorBasisMismatch,
                format!(
                    "durable cursor `{}` basis commit {} belongs to branch `{}` not `{}`",
                    request.cursor_id(),
                    request.basis_commit_id().0,
                    basis_commit.envelope.branch_context.0,
                    request.branch_id().0
                ),
            ));
        }

        let schema_support_artifact_id = request.schema_support_artifact_id().map(str::to_string);
        if let Some(schema_support_id) = &schema_support_artifact_id {
            let schema_support = self
                .state
                .schema_support_records
                .get(schema_support_id)
                .ok_or_else(|| {
                    StoreError::new(
                        StoreErrorKind::CursorSchemaBasisMismatch,
                        format!(
                            "durable cursor `{}` references missing schema support artifact `{schema_support_id}`",
                            request.cursor_id()
                        ),
                    )
                })?;
            if schema_support.branch_id != *request.branch_id() {
                return Err(StoreError::new(
                    StoreErrorKind::CursorSchemaBasisMismatch,
                    format!(
                        "durable cursor `{}` references schema support artifact `{schema_support_id}` on a different branch",
                        request.cursor_id()
                    ),
                ));
            }
        }

        let previous_identity = self
            .state
            .durable_cursor_identity_records
            .get(&cursor_artifact_id)
            .cloned();
        let next_checkpoint_sequence = if let Some(identity) = &previous_identity {
            if identity.subscriber_id != request.subscriber_id()
                || identity.branch_id != *request.branch_id()
                || identity.feed_shape_id != request.feed_shape_id()
                || identity.schema_interpretation_id != request.schema_interpretation_id()
                || identity.cursor_semantics_version != request.cursor_semantics_version()
            {
                self.counters.record_cursor_equivalence_reject();
                return Err(StoreError::new(
                    StoreErrorKind::CursorEquivalenceViolation,
                    format!(
                        "durable cursor `{}` cannot be reused with a different subscriber, branch scope, feed shape, schema interpretation, or semantics version",
                        request.cursor_id()
                    ),
                ));
            }
            let latest_basis_commit = self
                .state
                .commit_record(identity.latest_basis_commit_id)
                .ok_or_else(|| {
                    StoreError::new(
                        StoreErrorKind::CursorBasisMismatch,
                        format!(
                            "durable cursor `{}` latest basis commit {} is missing",
                            request.cursor_id(),
                            identity.latest_basis_commit_id.0
                        ),
                    )
                })?;
            if basis_commit.commit_sequence < latest_basis_commit.commit_sequence {
                self.counters.record_cursor_regression_reject();
                return Err(StoreError::new(
                    StoreErrorKind::CursorRegression,
                    format!(
                        "durable cursor `{}` cannot regress from commit {} to commit {}",
                        request.cursor_id(),
                        identity.latest_basis_commit_id.0,
                        request.basis_commit_id().0
                    ),
                ));
            }
            identity.latest_checkpoint_sequence + 1
        } else {
            1
        };

        let checkpoint_artifact_id =
            subscriber_checkpoint_artifact_id(&cursor_id, next_checkpoint_sequence);
        let checkpoint_record = SubscriberCheckpointRecord {
            artifact_id: checkpoint_artifact_id.clone(),
            cursor_id: cursor_id.clone(),
            subscriber_id: request.subscriber_id().to_string(),
            branch_id: request.branch_id().clone(),
            feed_shape_id: request.feed_shape_id().to_string(),
            schema_interpretation_id: request.schema_interpretation_id().to_string(),
            cursor_semantics_version: request.cursor_semantics_version(),
            checkpoint_sequence: next_checkpoint_sequence,
            basis_commit_id: request.basis_commit_id(),
            schema_support_artifact_id: schema_support_artifact_id.clone(),
        };
        let identity_record = DurableCursorIdentityRecord {
            artifact_id: cursor_artifact_id.clone(),
            cursor_id: cursor_id.clone(),
            subscriber_id: request.subscriber_id().to_string(),
            branch_id: request.branch_id().clone(),
            feed_shape_id: request.feed_shape_id().to_string(),
            schema_interpretation_id: request.schema_interpretation_id().to_string(),
            cursor_semantics_version: request.cursor_semantics_version(),
            latest_checkpoint_sequence: next_checkpoint_sequence,
            latest_basis_commit_id: request.basis_commit_id(),
            latest_schema_support_artifact_id: schema_support_artifact_id,
        };

        self.state
            .subscriber_checkpoint_records
            .insert(checkpoint_artifact_id.clone(), checkpoint_record.clone());
        self.state
            .durable_cursor_identity_records
            .insert(cursor_artifact_id.clone(), identity_record.clone());
        self.state.upsert_digest_record(
            super::records::AuthoritativeArtifactFamily::SubscriberCheckpointRecord,
            checkpoint_artifact_id.clone(),
            stable_structural_digest(&checkpoint_record)?,
        );
        self.state.upsert_digest_record(
            super::records::AuthoritativeArtifactFamily::DurableCursorIdentityRecord,
            cursor_artifact_id.clone(),
            stable_structural_digest(&identity_record)?,
        );

        if let Err(error) = self.state.verify_cursor_record_family() {
            self.state
                .subscriber_checkpoint_records
                .remove(&checkpoint_artifact_id);
            self.state.authoritative_artifact_digests.remove(
                &super::integrity::digest_artifact_key(
                    &super::records::AuthoritativeArtifactFamily::SubscriberCheckpointRecord,
                    &checkpoint_artifact_id,
                    self.state.canonicalization_version,
                ),
            );
            match previous_identity {
                Some(previous) => {
                    self.state
                        .durable_cursor_identity_records
                        .insert(cursor_artifact_id.clone(), previous.clone());
                    self.state.upsert_digest_record(
                        super::records::AuthoritativeArtifactFamily::DurableCursorIdentityRecord,
                        cursor_artifact_id.clone(),
                        stable_structural_digest(&previous)?,
                    );
                }
                None => {
                    self.state
                        .durable_cursor_identity_records
                        .remove(&cursor_artifact_id);
                    self.state
                        .authoritative_artifact_digests
                        .remove(&super::integrity::digest_artifact_key(
                        &super::records::AuthoritativeArtifactFamily::DurableCursorIdentityRecord,
                        &cursor_artifact_id,
                        self.state.canonicalization_version,
                    ));
                }
            }
            if matches!(error.kind(), StoreErrorKind::CheckpointShapeViolation) {
                self.counters.record_checkpoint_shape_reject();
            }
            return Err(error);
        }

        let report = match self.persistence.persist_state(&self.state) {
            Ok(report) => report,
            Err(error) => {
                self.state
                    .subscriber_checkpoint_records
                    .remove(&checkpoint_artifact_id);
                self.state.authoritative_artifact_digests.remove(
                    &super::integrity::digest_artifact_key(
                        &super::records::AuthoritativeArtifactFamily::SubscriberCheckpointRecord,
                        &checkpoint_artifact_id,
                        self.state.canonicalization_version,
                    ),
                );
                match previous_identity {
                    Some(previous) => {
                        self.state
                            .durable_cursor_identity_records
                            .insert(cursor_artifact_id.clone(), previous.clone());
                        self.state.upsert_digest_record(
                            super::records::AuthoritativeArtifactFamily::DurableCursorIdentityRecord,
                            cursor_artifact_id.clone(),
                            stable_structural_digest(&previous)?,
                        );
                    }
                    None => {
                        self.state
                            .durable_cursor_identity_records
                            .remove(&cursor_artifact_id);
                        self.state.authoritative_artifact_digests.remove(
                            &super::integrity::digest_artifact_key(
                                &super::records::AuthoritativeArtifactFamily::DurableCursorIdentityRecord,
                                &cursor_artifact_id,
                                self.state.canonicalization_version,
                            ),
                        );
                    }
                }
                return Err(error);
            }
        };
        if report.content_barrier() < report.ack_required_barrier() {
            self.counters.record_durable_ack_barrier_violation();
            return Err(StoreError::new(
                StoreErrorKind::DurableBarrierContractViolation,
                format!(
                    "backend {:?} reported content barrier {:?} below required acknowledgment barrier {:?}",
                    report.backend_family(),
                    report.content_barrier(),
                    report.ack_required_barrier()
                ),
            ));
        }
        self.counters.record_durable_barrier_verified();
        self.counters.record_state_delta_apply(2, 2);
        self.counters.record_cursor_ack();
        self.counters.record_subscriber_checkpoint_write();
        Ok(PersistedSubscriberCheckpoint::new(checkpoint_record))
    }

    pub fn fetch_durable_cursor_identity(
        &self,
        cursor_id: &str,
    ) -> Result<FetchedDurableCursorIdentity, StoreError> {
        self.counters.record_cursor_identity_lookup();
        let artifact_id = durable_cursor_identity_artifact_id(cursor_id);
        let record = self
            .state
            .durable_cursor_identity_records
            .get(&artifact_id)
            .cloned()
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::CursorCheckpointMissing,
                    format!("durable cursor `{cursor_id}` not found"),
                )
            })?;
        self.state.verify_durable_cursor_identity_record(&record)?;
        Ok(FetchedDurableCursorIdentity::new(record))
    }

    pub fn plan_cursor_resume(
        &self,
        request: DurableCursorResumeRequest,
    ) -> Result<DurableCursorResumePlan, StoreError> {
        self.counters.record_cursor_identity_lookup();
        let identity = self
            .state
            .durable_cursor_identity_records
            .get(&durable_cursor_identity_artifact_id(request.cursor_id()))
            .cloned()
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::CursorCheckpointMissing,
                    format!("durable cursor `{}` not found", request.cursor_id()),
                )
            })?;
        if identity.subscriber_id != request.subscriber_id()
            || identity.branch_id != *request.branch_id()
            || identity.feed_shape_id != request.feed_shape_id()
            || identity.schema_interpretation_id != request.schema_interpretation_id()
            || identity.cursor_semantics_version != request.cursor_semantics_version()
        {
            self.counters.record_cursor_equivalence_reject();
            return Err(StoreError::new(
                StoreErrorKind::CursorEquivalenceViolation,
                format!(
                    "durable cursor `{}` does not match the requested resume identity basis",
                    request.cursor_id()
                ),
            ));
        }
        let latest_checkpoint = self
            .state
            .subscriber_checkpoint_records
            .get(&subscriber_checkpoint_artifact_id(
                request.cursor_id(),
                identity.latest_checkpoint_sequence,
            ))
            .cloned()
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::CursorCheckpointMissing,
                    format!(
                        "durable cursor `{}` is missing checkpoint sequence {}",
                        request.cursor_id(),
                        identity.latest_checkpoint_sequence
                    ),
                )
            })?;
        self.state
            .verify_durable_cursor_identity_record(&identity)?;
        self.state
            .verify_subscriber_checkpoint_record(&latest_checkpoint)?;
        self.counters.record_cursor_resume(2, 1);
        Ok(DurableCursorResumePlan::new(identity, latest_checkpoint))
    }

    pub fn record_canonicalization(&self, metrics: CanonicalizationMetrics) {
        self.counters.record_canonicalization(metrics);
    }

    pub fn record_bulk_source_manifest(&self, member_count: u64, stream_pass_count: u64) {
        self.counters
            .record_bulk_source_manifest(member_count, stream_pass_count);
    }

    pub fn record_bulk_chunk_plan(&self, chunk_count: u64) {
        self.counters.record_bulk_chunk_plan(chunk_count);
    }

    pub fn record_bulk_chunk_execute(
        &self,
        width_units: u64,
        memory_units: u64,
        fallback_breadth_units: u64,
        used_fallback_path: bool,
    ) {
        self.counters.record_bulk_chunk_execute(
            width_units,
            memory_units,
            fallback_breadth_units,
            used_fallback_path,
        );
    }

    pub fn record_bulk_chunk_resume(&self) {
        self.counters.record_bulk_chunk_resume();
    }

    pub fn record_bulk_chunk_commit(&self) {
        self.counters.record_bulk_chunk_commit();
    }

    pub fn persist_frozen_bulk_manifest(
        &mut self,
        manifest: FrozenBulkSourceManifest,
    ) -> Result<FrozenBulkSourceManifest, StoreError> {
        let mut next = self.state.clone();
        let program_artifact_id = bulk_program_artifact_id(manifest.program_id());
        next.bulk_program_identity_records.insert(
            program_artifact_id.clone(),
            BulkProgramIdentityRecord {
                artifact_id: program_artifact_id,
                family_version: BULK_FAMILY_VERSION,
                kind: BulkPlanKind::Ingest,
                program_id: manifest.program_id().to_string(),
                source_identity: manifest.source_identity().to_string(),
                target_branch_scope: manifest.target_branch_scope().clone(),
                basis_commit_id: None,
            },
        );
        let artifact_id =
            frozen_bulk_manifest_artifact_id(manifest.program_id(), manifest.manifest_digest());
        next.frozen_bulk_manifest_records.insert(
            artifact_id.clone(),
            FrozenBulkManifestRecord {
                artifact_id,
                family_version: BULK_FAMILY_VERSION,
                program_id: manifest.program_id().to_string(),
                manifest: manifest.clone(),
            },
        );
        self.commit_replacement_state(next)?;
        Ok(manifest)
    }

    pub fn persist_frozen_transform_basis(
        &mut self,
        basis: FrozenTransformBasis,
    ) -> Result<FrozenTransformBasis, StoreError> {
        let mut next = self.state.clone();
        let program_artifact_id = bulk_program_artifact_id(basis.program_id());
        next.bulk_program_identity_records.insert(
            program_artifact_id.clone(),
            BulkProgramIdentityRecord {
                artifact_id: program_artifact_id,
                family_version: BULK_FAMILY_VERSION,
                kind: BulkPlanKind::Transform,
                program_id: basis.program_id().to_string(),
                source_identity: basis.transform_identity().to_string(),
                target_branch_scope: basis.target_branch_scope().clone(),
                basis_commit_id: Some(basis.basis_commit_id()),
            },
        );
        let artifact_id =
            frozen_transform_basis_artifact_id(basis.program_id(), basis.basis_digest());
        next.frozen_transform_basis_records.insert(
            artifact_id.clone(),
            FrozenTransformBasisRecord {
                artifact_id,
                family_version: BULK_FAMILY_VERSION,
                program_id: basis.program_id().to_string(),
                basis: basis.clone(),
            },
        );
        self.commit_replacement_state(next)?;
        Ok(basis)
    }

    pub fn persist_frozen_transform_partition(
        &mut self,
        partition: FrozenTransformTargetPartition,
    ) -> Result<FrozenTransformTargetPartition, StoreError> {
        let mut next = self.state.clone();
        let artifact_id = frozen_transform_partition_artifact_id(
            partition.program_id(),
            partition.partition_digest(),
        );
        next.frozen_transform_partition_records.insert(
            artifact_id.clone(),
            FrozenTransformPartitionRecord {
                artifact_id,
                family_version: BULK_FAMILY_VERSION,
                program_id: partition.program_id().to_string(),
                partition: partition.clone(),
            },
        );
        self.commit_replacement_state(next)?;
        self.counters.record_bulk_transform_partition(1);
        Ok(partition)
    }

    pub fn persist_bulk_chunk_plan(
        &mut self,
        plan: DeterministicChunkPlan,
    ) -> Result<DeterministicChunkPlan, StoreError> {
        let mut next = self.state.clone();
        let artifact_id = bulk_plan_artifact_id(plan.program_id(), plan.plan_id());
        next.bulk_deterministic_plan_records.insert(
            artifact_id.clone(),
            BulkDeterministicPlanRecord {
                artifact_id,
                family_version: BULK_FAMILY_VERSION,
                program_id: plan.program_id().to_string(),
                plan: plan.clone(),
            },
        );
        self.commit_replacement_state(next)?;
        Ok(plan)
    }

    pub fn fetch_frozen_bulk_manifest(
        &self,
        program_id: &str,
        manifest_digest: &str,
    ) -> Result<FrozenBulkSourceManifest, StoreError> {
        let artifact_id = frozen_bulk_manifest_artifact_id(program_id, manifest_digest);
        self.state
            .frozen_bulk_manifest_records
            .get(&artifact_id)
            .map(|record| record.manifest.clone())
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::BulkChunkContractUnsupported,
                    format!("bulk manifest `{artifact_id}` not found"),
                )
            })
    }

    pub fn fetch_frozen_transform_basis(
        &self,
        program_id: &str,
        basis_digest: &str,
    ) -> Result<FrozenTransformBasis, StoreError> {
        let artifact_id = frozen_transform_basis_artifact_id(program_id, basis_digest);
        self.state
            .frozen_transform_basis_records
            .get(&artifact_id)
            .map(|record| record.basis.clone())
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::BulkTransformBasisDrift,
                    format!("bulk transform basis `{artifact_id}` not found"),
                )
            })
    }

    pub fn fetch_frozen_transform_partition(
        &self,
        program_id: &str,
        partition_digest: &str,
    ) -> Result<FrozenTransformTargetPartition, StoreError> {
        let artifact_id = frozen_transform_partition_artifact_id(program_id, partition_digest);
        self.state
            .frozen_transform_partition_records
            .get(&artifact_id)
            .map(|record| record.partition.clone())
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::BulkTransformBasisDrift,
                    format!("bulk transform partition `{artifact_id}` not found"),
                )
            })
    }

    pub fn find_frozen_transform_basis_for_plan(
        &self,
        program_id: &str,
        target_branch_scope: &BranchId,
        basis_commit_id: CommitId,
    ) -> Result<FrozenTransformBasis, StoreError> {
        self.state
            .frozen_transform_basis_records
            .values()
            .find(|record| {
                record.program_id == program_id
                    && record.basis.target_branch_scope() == target_branch_scope
                    && record.basis.basis_commit_id() == basis_commit_id
            })
            .map(|record| record.basis.clone())
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::BulkTransformBasisDrift,
                    format!(
                        "bulk transform basis for program `{program_id}` branch `{}` commit {} not found",
                        target_branch_scope.0,
                        basis_commit_id.0
                    ),
                )
            })
    }

    pub fn fetch_bulk_chunk_plan(
        &self,
        program_id: &str,
        plan_id: &str,
    ) -> Result<DeterministicChunkPlan, StoreError> {
        let artifact_id = bulk_plan_artifact_id(program_id, plan_id);
        self.state
            .bulk_deterministic_plan_records
            .get(&artifact_id)
            .map(|record| record.plan.clone())
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::BulkChunkContractUnsupported,
                    format!("bulk plan `{artifact_id}` not found"),
                )
            })
    }

    pub fn publish_bulk_chunk_witness(
        &mut self,
        witness: BulkChunkCommitWitness,
    ) -> Result<BulkChunkCommitWitness, StoreError> {
        let mut next = self.state.clone();
        let artifact_id = bulk_witness_artifact_id(
            witness.program_id(),
            witness.plan_id(),
            witness.chunk_ordinal().value(),
        );
        if next.bulk_chunk_witness_records.contains_key(&artifact_id) {
            return Err(StoreError::new(
                StoreErrorKind::BulkChunkDuplicateCommit,
                format!("bulk witness `{artifact_id}` already exists"),
            ));
        }

        let existing_witnesses: Vec<_> = next
            .bulk_chunk_witness_records
            .values()
            .filter(|record| {
                record.program_id == witness.program_id() && record.plan_id == witness.plan_id()
            })
            .collect();
        let expected_ordinal = existing_witnesses.len() as u64;
        if witness.chunk_ordinal().value() != expected_ordinal {
            return Err(StoreError::new(
                StoreErrorKind::BulkChunkWitnessGap,
                format!(
                    "bulk witness ordinal {} was published before expected ordinal {}",
                    witness.chunk_ordinal().value(),
                    expected_ordinal
                ),
            ));
        }

        next.bulk_chunk_witness_records.insert(
            artifact_id.clone(),
            BulkChunkWitnessRecord {
                artifact_id,
                family_version: BULK_FAMILY_VERSION,
                program_id: witness.program_id().to_string(),
                plan_id: witness.plan_id().to_string(),
                witness: witness.clone(),
            },
        );
        let index_artifact_id =
            bulk_witness_index_artifact_id(witness.program_id(), witness.plan_id());
        let checkpoint_sequence = next
            .bulk_progress_checkpoint_records
            .values()
            .filter(|record| {
                record.program_id == witness.program_id() && record.plan_id == witness.plan_id()
            })
            .map(|record| record.checkpoint.checkpoint_sequence())
            .max();
        next.program_chunk_witness_index_records.insert(
            index_artifact_id.clone(),
            ProgramChunkWitnessIndexRecord {
                artifact_id: index_artifact_id,
                family_version: BULK_FAMILY_VERSION,
                program_id: witness.program_id().to_string(),
                plan_id: witness.plan_id().to_string(),
                index: ProgramChunkWitnessIndex::new(
                    witness.program_id().to_string(),
                    witness.plan_id().to_string(),
                    witness.chunk_ordinal(),
                    witness.canonical_commit_id(),
                    checkpoint_sequence,
                    expected_ordinal + 1,
                ),
            },
        );
        self.commit_replacement_state(next)?;
        self.counters.record_bulk_chunk_witness_write();
        Ok(witness)
    }

    pub fn publish_bulk_progress_checkpoint(
        &mut self,
        witness: BulkChunkCommitWitness,
    ) -> Result<PublishedBulkProgressCheckpoint, StoreError> {
        let mut next = self.state.clone();
        let latest_checkpoint = next
            .bulk_progress_checkpoint_records
            .values()
            .filter(|record| {
                record.program_id == witness.program_id() && record.plan_id == witness.plan_id()
            })
            .max_by_key(|record| record.checkpoint.checkpoint_sequence())
            .map(|record| record.checkpoint.clone());
        let latest_sequence = latest_checkpoint
            .as_ref()
            .map(|checkpoint| checkpoint.checkpoint_sequence());
        let input = BulkProgressCheckpointRecordInput::publish_next(latest_sequence, &witness)?;
        let witness_artifact_id = input.last_committed_chunk_witness_artifact_id().to_string();
        let witness = next
            .bulk_chunk_witness_records
            .get(&witness_artifact_id)
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::BulkCheckpointPublicationGap,
                    format!(
                        "bulk progress checkpoint referenced missing witness `{witness_artifact_id}`"
                    ),
                )
            })?;
        if witness.program_id != input.program_id() || witness.plan_id != input.plan_id() {
            return Err(StoreError::new(
                StoreErrorKind::BulkCheckpointPublicationGap,
                "bulk progress checkpoint must reference a witness from the same program and plan",
            ));
        }
        if let Some(previous_checkpoint) = latest_checkpoint {
            if input.completed_chunk_ordinal().value()
                < previous_checkpoint.next_chunk_ordinal().value()
            {
                return Err(StoreError::new(
                    StoreErrorKind::BulkCheckpointPublicationGap,
                    format!(
                        "bulk checkpoint for witness ordinal {} would not advance beyond prior checkpoint boundary {}",
                        input.completed_chunk_ordinal().value(),
                        previous_checkpoint.next_chunk_ordinal().value()
                    ),
                ));
            }
        }
        let checkpoint = PublishedBulkProgressCheckpoint::new(
            input.program_id().to_string(),
            input.plan_id().to_string(),
            input.checkpoint_sequence(),
            input.completed_chunk_ordinal(),
            input.next_chunk_ordinal(),
            witness_artifact_id.clone(),
            input.checkpoint_digest().to_string(),
        );
        let artifact_id = bulk_checkpoint_artifact_id(
            input.program_id(),
            input.plan_id(),
            input.checkpoint_sequence(),
        );
        next.bulk_progress_checkpoint_records.insert(
            artifact_id.clone(),
            BulkProgressCheckpointRecord {
                artifact_id,
                family_version: BULK_FAMILY_VERSION,
                program_id: input.program_id().to_string(),
                plan_id: input.plan_id().to_string(),
                checkpoint: checkpoint.clone(),
            },
        );
        if let Some(index) =
            next.program_chunk_witness_index_records
                .get_mut(&bulk_witness_index_artifact_id(
                    input.program_id(),
                    input.plan_id(),
                ))
        {
            index.index = ProgramChunkWitnessIndex::new(
                index.program_id.clone(),
                index.plan_id.clone(),
                index.index.highest_committed_chunk_ordinal(),
                index.index.highest_committed_commit_id(),
                Some(input.checkpoint_sequence()),
                index.index.witness_count(),
            );
        }
        self.commit_replacement_state(next)?;
        self.counters.record_bulk_checkpoint_write();
        Ok(checkpoint)
    }

    pub fn fetch_bulk_progress_checkpoint(
        &self,
        program_id: &str,
        plan_id: &str,
    ) -> Result<PublishedBulkProgressCheckpoint, StoreError> {
        let index = self.fetch_program_chunk_witness_index(program_id, plan_id)?;
        let checkpoint_sequence = index.latest_checkpoint_sequence().ok_or_else(|| {
            StoreError::new(
                StoreErrorKind::BulkCheckpointPublicationGap,
                format!("bulk checkpoint for `{program_id}:{plan_id}` not found"),
            )
        })?;
        let artifact_id = bulk_checkpoint_artifact_id(program_id, plan_id, checkpoint_sequence);
        self.state
            .bulk_progress_checkpoint_records
            .get(&artifact_id)
            .map(|record| record.checkpoint.clone())
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::BulkCheckpointPublicationGap,
                    format!("bulk checkpoint `{artifact_id}` not found"),
                )
            })
    }

    pub fn fetch_program_chunk_witness_index(
        &self,
        program_id: &str,
        plan_id: &str,
    ) -> Result<ProgramChunkWitnessIndex, StoreError> {
        self.counters.record_bulk_resume_index_lookup();
        self.fetch_program_chunk_witness_index_untracked(program_id, plan_id)
    }

    fn fetch_program_chunk_witness_index_untracked(
        &self,
        program_id: &str,
        plan_id: &str,
    ) -> Result<ProgramChunkWitnessIndex, StoreError> {
        let artifact_id = bulk_witness_index_artifact_id(program_id, plan_id);
        self.state
            .program_chunk_witness_index_records
            .get(&artifact_id)
            .map(|record| record.index.clone())
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::BulkChunkWitnessGap,
                    format!("bulk witness index `{artifact_id}` not found"),
                )
            })
    }

    pub fn fetch_latest_resume_boundary(
        &self,
        program_id: &str,
        plan_id: &str,
    ) -> Result<ResumeBoundaryCandidate, StoreError> {
        self.counters.record_bulk_resume_index_lookup();
        match self.fetch_program_chunk_witness_index_untracked(program_id, plan_id) {
            Ok(index) => Ok(ResumeBoundaryCandidate::new(
                program_id.to_string(),
                plan_id.to_string(),
                Some(index.highest_committed_chunk_ordinal()),
                crate::ChunkOrdinal::new(index.highest_committed_chunk_ordinal().value() + 1),
                index.latest_checkpoint_sequence(),
            )),
            Err(error) if matches!(error.kind(), StoreErrorKind::BulkChunkWitnessGap) => {
                Ok(ResumeBoundaryCandidate::new(
                    program_id.to_string(),
                    plan_id.to_string(),
                    None,
                    crate::ChunkOrdinal::new(0),
                    None,
                ))
            }
            Err(error) => Err(error),
        }
    }

    pub fn counter_snapshot(&self) -> StoreCounterSnapshot {
        self.counters.snapshot()
    }

    pub(crate) fn record_physical_chunk_export(&self, chunk_width: u64) {
        self.counters.record_physical_chunk_export(chunk_width);
    }

    pub fn durable_media_report(&self) -> DurableMediaReport {
        self.persistence.durable_media_report()
    }

    pub fn milestone_7_access_structure_verification(
        &self,
    ) -> Milestone7AccessStructureVerification {
        self.milestone_7_access_structure_verification.clone()
    }

    pub fn milestone_6_access_structure_verification(
        &self,
    ) -> Milestone6AccessStructureVerification {
        self.milestone_6_access_structure_verification.clone()
    }

    pub fn classify_durable_publication(
        &self,
        durable_mutation_id: DurableMutationId,
        expected_commit_id: Option<CommitId>,
    ) -> Result<PublicationWriteOutcome, StoreError> {
        let facts =
            durable_publication_facts(&self.state, durable_mutation_id, expected_commit_id)?;
        Ok(classify_durable_publication(
            self.persistence.durable_media_report(),
            facts,
        ))
    }

    pub fn classify_snapshot_publication(
        &self,
        snapshot_id: SnapshotId,
    ) -> Result<PublicationWriteOutcome, StoreError> {
        let basis = self
            .state
            .snapshot_basis_records
            .get(&snapshot_id.0)
            .cloned();
        let image = self
            .state
            .snapshot_image_records
            .get(&snapshot_id.0)
            .cloned();
        classify_snapshot_publication(self.persistence.durable_media_report(), basis, image)
    }

    pub fn classify_snapshot_maintenance_recovery(
        &self,
        snapshot_id: SnapshotId,
    ) -> Result<SnapshotMaintenanceRecoveryReport, StoreError> {
        classify_snapshot_maintenance_recovery(
            &self.state,
            snapshot_id,
            self.persistence.durable_media_report(),
        )
    }

    pub fn maintenance_recovery_report(&self) -> Result<MaintenanceRecoveryReport, StoreError> {
        build_maintenance_recovery_report(&self.state, self.persistence.durable_media_report())
    }

    pub fn support_artifact_recovery_report(&self) -> SupportArtifactRecoveryReport {
        build_support_artifact_recovery_report(&self.state)
    }

    pub fn backup_restore_compatibility_report(
        &self,
    ) -> Result<BackupRestoreCompatibilityReport, StoreError> {
        build_backup_restore_compatibility_report(
            &self.state,
            self.persistence.durable_media_report().backend_family(),
        )
    }

    pub fn export_bundle(&self) -> AuthoritativeExportBundle {
        self.state.authoritative_export_bundle()
    }

    pub fn persist_embedded_checkpoint(
        &mut self,
        record: EmbeddedCheckpointRecord,
    ) -> Result<EmbeddedCheckpointRecord, StoreError> {
        if self
            .state
            .embedded_checkpoint_records
            .contains_key(&record.checkpoint_id)
        {
            return Err(StoreError::new(
                StoreErrorKind::DuplicateArtifactIdentity,
                format!(
                    "embedded checkpoint `{}` already exists in forge-store",
                    record.checkpoint_id
                ),
            ));
        }

        self.state
            .embedded_checkpoint_records
            .insert(record.checkpoint_id.clone(), record.clone());
        if let Err(error) = self.state.verify_integrity() {
            self.state
                .embedded_checkpoint_records
                .remove(&record.checkpoint_id);
            return Err(error);
        }
        let report = match self.persistence.persist_state(&self.state) {
            Ok(report) => report,
            Err(error) => {
                self.state
                    .embedded_checkpoint_records
                    .remove(&record.checkpoint_id);
                return Err(error);
            }
        };
        if report.content_barrier() < report.ack_required_barrier() {
            self.state
                .embedded_checkpoint_records
                .remove(&record.checkpoint_id);
            self.counters.record_durable_ack_barrier_violation();
            return Err(StoreError::new(
                StoreErrorKind::DurableBarrierContractViolation,
                format!(
                    "backend {:?} reported content barrier {:?} below required acknowledgment barrier {:?}",
                    report.backend_family(),
                    report.content_barrier(),
                    report.ack_required_barrier()
                ),
            ));
        }
        self.counters.record_durable_barrier_verified();
        self.counters.record_state_delta_apply(1, 1);
        Ok(record)
    }

    pub fn fetch_embedded_checkpoint(
        &self,
        checkpoint_id: &str,
    ) -> Result<EmbeddedCheckpointRecord, StoreError> {
        let record = self
            .state
            .embedded_checkpoint_records
            .get(checkpoint_id)
            .cloned()
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::CommitNotFound,
                    format!("embedded checkpoint `{checkpoint_id}` not found"),
                )
            })?;
        let basis_reads = u64::from(record.basis_commit_id.is_some());
        let verification = self.state.verify_embedded_checkpoint_record(&record);
        if matches!(
            verification.as_ref().err().map(StoreError::kind),
            Some(StoreErrorKind::CheckpointShapeViolation)
        ) {
            self.counters.record_checkpoint_shape_reject();
        }
        verification?;
        self.counters.record_embedded_checkpoint_fetch(basis_reads);
        Ok(record)
    }

    pub fn counters(&self) -> &StoreCounters {
        &self.counters
    }

    pub fn capture_snapshot(
        &mut self,
        request: SnapshotCaptureRequest,
    ) -> Result<PublishedSnapshotHandle, StoreError> {
        let (applied, handle, record_count, byte_count) =
            self.state.apply_snapshot_capture_in_place(request)?;
        if let Err(error) = self.state.verify_applied_snapshot_capture(&applied) {
            self.state.rollback_snapshot_capture(applied);
            return Err(error);
        }
        let report = match self.persistence.persist_state(&self.state) {
            Ok(report) => report,
            Err(error) => {
                self.state.rollback_snapshot_capture(applied);
                return Err(error);
            }
        };
        if report.content_barrier() < report.ack_required_barrier() {
            self.state.rollback_snapshot_capture(applied);
            self.counters.record_durable_ack_barrier_violation();
            return Err(StoreError::new(
                StoreErrorKind::DurableBarrierContractViolation,
                format!(
                    "backend {:?} reported content barrier {:?} below required acknowledgment barrier {:?}",
                    report.backend_family(),
                    report.content_barrier(),
                    report.ack_required_barrier()
                ),
            ));
        }
        self.counters.record_durable_barrier_verified();
        self.counters.record_state_delta_apply(2, 2);
        self.counters
            .record_snapshot_capture(record_count, byte_count);
        Ok(handle)
    }

    pub fn read_snapshot(
        &self,
        request: SnapshotReadRequest,
    ) -> Result<SnapshotReadResult, StoreError> {
        match self.state.read_snapshot(request) {
            Ok((result, record_count, tail_commit_count, tail_replay_count)) => {
                self.counters.record_snapshot_read(
                    record_count,
                    tail_commit_count,
                    tail_replay_count,
                );
                Ok(result)
            }
            Err(error) => {
                if matches!(
                    error.kind(),
                    StoreErrorKind::SnapshotReadBasisMismatch
                        | StoreErrorKind::SnapshotRestoreTargetIllegal
                        | StoreErrorKind::SnapshotTailRangeGap
                ) {
                    self.counters.record_snapshot_basis_mismatch();
                }
                if matches!(
                    error.kind(),
                    StoreErrorKind::SnapshotDigestMismatch
                        | StoreErrorKind::SnapshotIntegrityFailure
                        | StoreErrorKind::SnapshotPublicationStateGap
                ) {
                    self.counters.record_snapshot_integrity_failure();
                }
                Err(error)
            }
        }
    }

    pub fn plan_snapshot_restore(
        &self,
        request: SnapshotRestoreRequest,
    ) -> Result<SnapshotRestorePlan, StoreError> {
        match self.state.plan_snapshot_restore(request) {
            Ok(plan) => Ok(plan),
            Err(error) => {
                if matches!(
                    error.kind(),
                    StoreErrorKind::SnapshotReadBasisMismatch
                        | StoreErrorKind::SnapshotRestoreTargetIllegal
                        | StoreErrorKind::SnapshotTailRangeGap
                ) {
                    self.counters.record_snapshot_basis_mismatch();
                }
                Err(error)
            }
        }
    }

    pub fn execute_snapshot_restore(
        &self,
        plan: SnapshotRestorePlan,
    ) -> Result<SnapshotRestoreOutcome, StoreError> {
        match self.state.execute_snapshot_restore(plan) {
            Ok((outcome, tail_commit_count, tail_replay_count)) => {
                self.counters
                    .record_snapshot_restore(tail_commit_count, tail_replay_count);
                Ok(outcome)
            }
            Err(error) => {
                if matches!(
                    error.kind(),
                    StoreErrorKind::SnapshotReadBasisMismatch
                        | StoreErrorKind::SnapshotRestoreTargetIllegal
                        | StoreErrorKind::SnapshotTailRangeGap
                ) {
                    self.counters.record_snapshot_basis_mismatch();
                }
                Err(error)
            }
        }
    }

    pub fn rebuild_snapshot(
        &self,
        snapshot_id: SnapshotId,
    ) -> Result<SnapshotImageBundle, StoreError> {
        match self.state.rebuild_snapshot(snapshot_id) {
            Ok((image, record_count)) => {
                self.counters.record_snapshot_rebuild(record_count);
                Ok(image)
            }
            Err(error) => {
                if matches!(
                    error.kind(),
                    StoreErrorKind::SnapshotDigestMismatch
                        | StoreErrorKind::SnapshotIntegrityFailure
                        | StoreErrorKind::SnapshotPublicationStateGap
                        | StoreErrorKind::SnapshotRebuildParityViolation
                ) {
                    self.counters.record_snapshot_integrity_failure();
                }
                Err(error)
            }
        }
    }

    #[cfg(test)]
    pub fn remove_snapshot_image_for_test(
        &mut self,
        snapshot_id: SnapshotId,
    ) -> Result<(), StoreError> {
        let mut next = self.state.clone();
        next.remove_snapshot_image(snapshot_id);
        let _ = self.persistence.persist_state(&next)?;
        self.state = next;
        Ok(())
    }

    #[cfg(test)]
    pub fn remove_snapshot_basis_for_test(
        &mut self,
        snapshot_id: SnapshotId,
    ) -> Result<(), StoreError> {
        let mut next = self.state.clone();
        next.remove_snapshot_basis(snapshot_id);
        let _ = self.persistence.persist_state(&next)?;
        self.state = next;
        Ok(())
    }

    #[cfg(test)]
    pub fn corrupt_snapshot_basis_digest_for_test(
        &mut self,
        snapshot_id: SnapshotId,
    ) -> Result<(), StoreError> {
        let mut next = self.state.clone();
        let basis = next
            .snapshot_basis_records
            .get_mut(&snapshot_id.0)
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::SnapshotBasisUnsupported,
                    format!("snapshot {} basis not found", snapshot_id.0),
                )
            })?;
        basis.snapshot_image_digest.push_str("-corrupt");
        let _ = self.persistence.persist_state(&next)?;
        self.state = next;
        Ok(())
    }

    #[cfg(test)]
    pub fn clear_branch_heads_for_test(&mut self) -> Result<(), StoreError> {
        let mut next = self.state.clone();
        next.branch_head_records.clear();
        let _ = self.persistence.persist_state(&next)?;
        self.state = next;
        Ok(())
    }

    pub fn admit_durable_mutation(
        &mut self,
        runtime_session_id: &str,
        operation_name: &str,
    ) -> Result<DurableMutationId, StoreError> {
        let durable_mutation_id = DurableMutationId(self.state.next_durable_mutation_id);
        let record = WalRecord::durable_mutation_intent(
            self.state.next_wal_sequence,
            durable_mutation_id,
            runtime_session_id,
            operation_name,
        )?;
        self.state.next_durable_mutation_id += 1;
        if let Err(error) = self.append_wal_record_committed(record) {
            self.state.next_durable_mutation_id = durable_mutation_id.0;
            return Err(error);
        }
        self.counters.record_durable_mutation_admit();
        self.counters.record_wal_append();
        Ok(durable_mutation_id)
    }

    pub fn record_hosted_runtime_commit_result(
        &mut self,
        runtime_session_id: &str,
        durable_mutation_id: DurableMutationId,
        envelope: forge_relational::facade::replay::CanonicalCommitEnvelope,
    ) -> Result<(), StoreError> {
        let record = WalRecord::hosted_runtime_commit_result(
            self.state.next_wal_sequence,
            durable_mutation_id,
            runtime_session_id,
            envelope,
        )?;
        self.append_wal_record_committed(record)?;
        self.counters.record_wal_append();
        Ok(())
    }

    pub fn record_publication_phase(
        &mut self,
        runtime_session_id: &str,
        durable_mutation_id: DurableMutationId,
        phase: DurablePublicationPhase,
        commit_id: Option<CommitId>,
    ) -> Result<(), StoreError> {
        let record = WalRecord::durable_publication_progress(
            self.state.next_wal_sequence,
            durable_mutation_id,
            runtime_session_id,
            phase,
            commit_id,
        )?;
        self.append_wal_record_committed(record)?;
        self.counters.record_wal_append();
        Ok(())
    }

    pub fn record_bulk_checkpoint_publication_intent(
        &mut self,
        runtime_session_id: &str,
        durable_mutation_id: DurableMutationId,
        checkpoint_sequence: Option<u64>,
    ) -> Result<(), StoreError> {
        let record = WalRecord::bulk_checkpoint_publication_intent(
            self.state.next_wal_sequence,
            durable_mutation_id,
            runtime_session_id,
            checkpoint_sequence,
        )?;
        self.append_wal_record_committed(record)?;
        self.counters.record_wal_append();
        Ok(())
    }

    pub fn resolve_retry(
        &self,
        durable_mutation_id: DurableMutationId,
    ) -> Result<DurableRetryResolution, StoreError> {
        let wal_records = self.state.wal_records_for_mutation(durable_mutation_id);
        let mut commit_id = None;
        let mut authoritative_published = false;
        let mut acknowledgment_eligible = false;

        for record in &wal_records {
            if let Some(record_commit_id) = record.canonical_commit_id() {
                commit_id = Some(record_commit_id);
            }
            if let WalRecordPayload::DurablePublicationProgress(progress) = &record.payload {
                if progress.phase == DurablePublicationPhase::AuthoritativeAppendPublished {
                    authoritative_published = true;
                }
                if progress.phase == DurablePublicationPhase::AcknowledgmentEligible {
                    acknowledgment_eligible = true;
                }
            }
        }

        if authoritative_published || acknowledgment_eligible {
            let commit_id = commit_id.ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::DurableRetryResolutionRequired,
                    format!(
                        "durable mutation {} reached publication without a recorded commit id",
                        durable_mutation_id.0
                    ),
                )
            })?;
            Ok(DurableRetryResolution::PreviouslyAcknowledgedEquivalentCommit { commit_id })
        } else if wal_records.is_empty() {
            Ok(DurableRetryResolution::NotPreviouslyPublished)
        } else {
            Ok(
                DurableRetryResolution::RetryRequiresOperatorOrHigherLevelPolicy {
                    durable_mutation_id,
                },
            )
        }
    }

    fn rebuild_bulk_resume_ready_program(
        &self,
        plan_kind: BulkPlanKind,
        program_id: &str,
        plan_id: &str,
    ) -> Result<ResumeReadyBulkProgram, StoreError> {
        let plan = self.fetch_bulk_chunk_plan(program_id, plan_id)?;
        let resume_boundary = self.fetch_latest_resume_boundary(program_id, plan_id)?;
        let witness_index =
            match self.fetch_program_chunk_witness_index_untracked(program_id, plan_id) {
                Ok(index) => Some(index),
                Err(error) if matches!(error.kind(), StoreErrorKind::BulkChunkWitnessGap) => None,
                Err(error) => return Err(error),
            };
        let latest_checkpoint = match resume_boundary.latest_checkpoint_sequence() {
            Some(_) => Some(self.fetch_bulk_progress_checkpoint(program_id, plan_id)?),
            None => None,
        };

        match plan_kind {
            BulkPlanKind::Ingest => {
                let manifest = self.fetch_frozen_bulk_manifest(program_id, plan.input_digest())?;
                ResumeReadyBulkProgram::admit_ingest(
                    &manifest,
                    plan,
                    witness_index,
                    latest_checkpoint,
                    resume_boundary,
                )
            }
            BulkPlanKind::Transform => {
                let basis_commit_id = plan.basis_commit_id().ok_or_else(|| {
                    StoreError::new(
                        StoreErrorKind::BulkTransformBasisDrift,
                        format!(
                            "bulk transform plan `{plan_id}` for program `{program_id}` is missing a locked basis commit"
                        ),
                    )
                })?;
                let basis = self.find_frozen_transform_basis_for_plan(
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

    fn finish_bulk_recovery_publication(
        &mut self,
        runtime_session_id: &str,
        durable_mutation_id: DurableMutationId,
        plan_kind: BulkPlanKind,
        program_id: &str,
        plan_id: &str,
        chunk_ordinal: ChunkOrdinal,
        canonical_commit_id: CommitId,
        checkpoint_sequence: Option<u64>,
    ) -> Result<(), StoreError> {
        let resumed_program =
            self.rebuild_bulk_resume_ready_program(plan_kind, program_id, plan_id)?;
        self.counters.record_bulk_chunk_resume();
        if resumed_program.next_chunk_ordinal() != chunk_ordinal {
            return Err(StoreError::new(
                StoreErrorKind::BulkResumeBoundaryAmbiguous,
                format!(
                    "bulk recovery for program `{program_id}` plan `{plan_id}` expected chunk {} but resume boundary resolved to {}",
                    chunk_ordinal.value(),
                    resumed_program.next_chunk_ordinal().value()
                ),
            ));
        }
        let admitted_memory_units = resumed_program
            .plan()
            .chunk_by_ordinal(chunk_ordinal)
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::BulkChunkContractUnsupported,
                    format!(
                        "bulk recovery chunk ordinal {} does not exist in deterministic plan `{plan_id}`",
                        chunk_ordinal.value()
                    ),
                )
            })?
            .width_units();
        let admitted: BudgetAdmittedChunkPlan = resumed_program
            .admit_next_chunk(admitted_memory_units)?
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::BulkResumeBoundaryAmbiguous,
                    format!(
                        "bulk recovery for program `{program_id}` plan `{plan_id}` resolved to a completed program before chunk {}",
                        chunk_ordinal.value()
                    ),
                )
            })?;
        let witness = self.publish_bulk_chunk_witness(BulkChunkCommitWitness::publish(
            &admitted,
            canonical_commit_id,
        )?)?;
        if let Some(sequence) = checkpoint_sequence {
            let latest_checkpoint_sequence = self
                .fetch_program_chunk_witness_index_untracked(program_id, plan_id)
                .ok()
                .and_then(|index| index.latest_checkpoint_sequence());
            if latest_checkpoint_sequence.unwrap_or(0) < sequence {
                self.publish_bulk_progress_checkpoint(witness.clone())?;
            }
        }
        self.record_publication_phase(
            runtime_session_id,
            durable_mutation_id,
            DurablePublicationPhase::AcknowledgmentEligible,
            Some(canonical_commit_id),
        )?;
        self.counters.record_bulk_chunk_commit();
        self.counters.record_durable_commit_acknowledged();
        Ok(())
    }

    fn reconcile_bulk_support_from_published_truth(
        &mut self,
        plan_kind: BulkPlanKind,
        program_id: &str,
        plan_id: &str,
        chunk_ordinal: ChunkOrdinal,
        canonical_commit_id: CommitId,
        checkpoint_sequence: Option<u64>,
    ) -> Result<(), StoreError> {
        let plan = self.fetch_bulk_chunk_plan(program_id, plan_id)?;
        if plan.kind() != plan_kind {
            return Err(StoreError::new(
                StoreErrorKind::BulkResumeBoundaryAmbiguous,
                format!(
                    "bulk recovery plan kind drift for program `{program_id}` plan `{plan_id}`"
                ),
            ));
        }
        let admitted_memory_units = plan
            .chunk_by_ordinal(chunk_ordinal)
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::BulkChunkContractUnsupported,
                    format!(
                        "bulk recovery chunk ordinal {} does not exist in deterministic plan `{plan_id}`",
                        chunk_ordinal.value()
                    ),
                )
            })?
            .width_units();
        let admitted = BudgetAdmittedChunkPlan::admit(&plan, chunk_ordinal, admitted_memory_units)?;

        let witness_index =
            match self.fetch_program_chunk_witness_index_untracked(program_id, plan_id) {
                Ok(index) => Some(index),
                Err(error) if matches!(error.kind(), StoreErrorKind::BulkChunkWitnessGap) => None,
                Err(error) => return Err(error),
            };
        let witness_present = witness_index
            .as_ref()
            .map(|index| index.highest_committed_chunk_ordinal().value() >= chunk_ordinal.value())
            .unwrap_or(false);
        let witness = if witness_present {
            None
        } else {
            Some(
                self.publish_bulk_chunk_witness(BulkChunkCommitWitness::publish(
                    &admitted,
                    canonical_commit_id,
                )?)?,
            )
        };

        if let Some(sequence) = checkpoint_sequence {
            let latest_checkpoint_sequence = match self
                .fetch_program_chunk_witness_index_untracked(program_id, plan_id)
            {
                Ok(index) => index.latest_checkpoint_sequence(),
                Err(error) if matches!(error.kind(), StoreErrorKind::BulkChunkWitnessGap) => None,
                Err(error) => return Err(error),
            };
            if latest_checkpoint_sequence.unwrap_or(0) < sequence {
                let checkpoint_witness = witness.as_ref().map_or_else(
                    || BulkChunkCommitWitness::publish(&admitted, canonical_commit_id),
                    |witness| Ok(witness.clone()),
                )?;
                self.publish_bulk_progress_checkpoint(checkpoint_witness)?;
            }
        }

        Ok(())
    }

    pub fn plan_durable_recovery(&self) -> DurableRecoveryPlan {
        build_recovery_plan(&self.state)
    }

    pub fn recover_durable_runtime(
        &mut self,
        runtime_session_id: &str,
    ) -> Result<DurableRecoveryOutcome, StoreError> {
        let plan = self.plan_durable_recovery();
        if plan.pending_durable_mutation_ids.is_empty() {
            self.counters.record_recovery_quiescent_restart();
        } else {
            self.counters.record_recovery_non_quiescent_restart();
        }
        let wal_sequences: Vec<u64> = self.state.wal_records.keys().copied().collect();
        self.counters.record_wal_scan(wal_sequences.len());
        let mut decisions = Vec::new();
        let mut degraded = Vec::new();
        let mut source_reports = Vec::new();

        for durable_mutation_id in plan.pending_durable_mutation_ids {
            let wal_records = self
                .state
                .wal_records_for_mutation(durable_mutation_id)
                .into_iter()
                .cloned()
                .collect::<Vec<_>>();
            let wal_record_refs = wal_records.iter().collect::<Vec<_>>();
            let bulk_checkpoint_sequence =
                bulk_checkpoint_sequence_intent_for_wal_records(&wal_record_refs);
            let evaluation = match evaluate_recovery_for_mutation(
                &self.state,
                durable_mutation_id,
                &wal_record_refs,
                self.persistence.durable_media_report(),
            ) {
                Ok(evaluation) => evaluation,
                Err(error) => {
                    if matches!(
                        error.kind(),
                        StoreErrorKind::WalDigestMismatch
                            | StoreErrorKind::WalCanonicalizationVersionUnsupported
                    ) {
                        self.counters.record_wal_decode_failure();
                    }
                    if matches!(error.kind(), StoreErrorKind::RecoveryRequiresFullRebuild) {
                        self.counters.record_recovery_requires_full_rebuild();
                    }
                    self.counters.record_recovery_failure();
                    return Err(error);
                }
            };
            self.counters.record_recovery_source_precedence_resolution();
            if !matches!(
                evaluation.source_kind,
                RecoverySourceKind::PublishedAuthoritativeTruth
            ) {
                self.counters.record_recovery_source_precedence_fallback();
            }
            source_reports.push(evaluation.source_report.clone());

            let DurableRecoveryDecision {
                durable_mutation_id,
                decision,
                commit_id,
            } = evaluation.decision;
            match evaluation.action {
                RecoveryAction::RetainPublishedTruth => {
                    if let crate::recovery::DurableMutationIdentity::BulkChunk {
                        plan_kind,
                        program_id,
                        plan_id,
                        chunk_ordinal,
                    } = evaluation.source_report.mutation_identity().clone()
                    {
                        let recovered_commit_id = commit_id.ok_or_else(|| {
                            StoreError::new(
                                StoreErrorKind::RecoveryRequiresFullRebuild,
                                format!(
                                    "bulk durable mutation {} published without a recoverable commit id",
                                    durable_mutation_id.0
                                ),
                            )
                        })?;
                        self.reconcile_bulk_support_from_published_truth(
                            plan_kind,
                            &program_id,
                            &plan_id,
                            ChunkOrdinal::new(chunk_ordinal),
                            recovered_commit_id,
                            bulk_checkpoint_sequence,
                        )?;
                    }
                    if decision == RecoveryDecisionClass::SuppressDuplicateReplay {
                        self.counters.record_durable_commit_duplicate_suppressed();
                    } else {
                        self.counters.record_durable_commit_recovered();
                    }
                }
                RecoveryAction::FinishPublicationFromCanonicalResult { canonical_envelope } => {
                    let persisted =
                        self.append(self.verify_append(crate::authority::canonicalize(
                            crate::authority::RawRuntimeCommitEnvelope::new(canonical_envelope),
                            crate::authority::CURRENT_CANONICALIZATION_VERSION,
                        )?)?)?;
                    let recovered_commit_id = persisted.envelope().commit.commit_id;
                    self.record_publication_phase(
                        runtime_session_id,
                        durable_mutation_id,
                        DurablePublicationPhase::AuthoritativeAppendPublished,
                        Some(recovered_commit_id),
                    )?;
                    if let crate::recovery::DurableMutationIdentity::BulkChunk {
                        plan_kind,
                        program_id,
                        plan_id,
                        chunk_ordinal,
                    } = evaluation.source_report.mutation_identity().clone()
                    {
                        self.finish_bulk_recovery_publication(
                            runtime_session_id,
                            durable_mutation_id,
                            plan_kind,
                            &program_id,
                            &plan_id,
                            ChunkOrdinal::new(chunk_ordinal),
                            recovered_commit_id,
                            bulk_checkpoint_sequence,
                        )?;
                    }
                    self.counters.record_durable_commit_recovered();
                }
                RecoveryAction::DiscardUnpublished => {
                    self.counters.record_durable_commit_unacknowledged_discard();
                }
                RecoveryAction::RequireRebuild => {
                    self.counters.record_recovery_requires_full_rebuild();
                }
                RecoveryAction::RequireQuarantine => {
                    self.counters.record_recovery_quarantine();
                }
            }

            let decision = DurableRecoveryDecision {
                durable_mutation_id,
                decision,
                commit_id,
            };
            if let Some(degraded_recovery) = evaluation.degraded {
                degraded.push(degraded_recovery);
            }

            let record = WalRecord::recovery_decision(
                self.state.next_wal_sequence,
                durable_mutation_id,
                runtime_session_id,
                decision.decision,
                decision.commit_id,
            )?;
            self.append_wal_record_committed(record)?;
            self.counters.record_wal_append();
            decisions.push(decision);
        }

        Ok(DurableRecoveryOutcome {
            decisions,
            degraded,
            source_reports,
        })
    }
}

fn bulk_checkpoint_sequence_intent_for_wal_records(wal_records: &[&WalRecord]) -> Option<u64> {
    wal_records
        .iter()
        .rev()
        .find_map(|record| match &record.payload {
            WalRecordPayload::BulkCheckpointPublicationIntent(intent) => intent.checkpoint_sequence,
            _ => None,
        })
}

fn verify_milestone_7_access_structures(
    state: &StoreState,
    media_report: DurableMediaReport,
) -> Milestone7AccessStructureVerification {
    let backend_family = media_report.backend_family();
    Milestone7AccessStructureVerification {
        backend_family,
        schema_boundary_fetch: verify_string_keyed_records(
            state.schema_support_records.iter().map(|(key, record)| {
                (
                    key.as_str(),
                    record.artifact_id.as_str(),
                    "schema support artifact id",
                )
            }),
            "loaded schema support map preserves exact artifact-id addressing",
        ),
        lineage_lookup: verify_string_keyed_records(
            state.lineage_support_records.iter().map(|(key, record)| {
                (
                    key.as_str(),
                    record.artifact_id.as_str(),
                    "lineage support artifact id",
                )
            }),
            "loaded lineage support map preserves exact artifact-id addressing",
        ),
        cursor_resume: verify_cursor_identity_keys(state),
        embedded_checkpoint_fetch: verify_string_keyed_records(
            state
                .embedded_checkpoint_records
                .iter()
                .map(|(key, record)| {
                    (
                        key.as_str(),
                        record.checkpoint_id.as_str(),
                        "embedded checkpoint id",
                    )
                }),
            "loaded embedded checkpoint map preserves exact checkpoint-id addressing",
        ),
        commit_coupled_support_publication: verify_commit_support_summary_keys(state),
        cursor_identity_admission: verify_subscriber_checkpoint_keys(state),
    }
}

fn verify_milestone_6_access_structures(
    state: &StoreState,
    media_report: DurableMediaReport,
) -> Milestone6AccessStructureVerification {
    Milestone6AccessStructureVerification {
        backend_family: media_report.backend_family(),
        aspect_layout_read: verify_milestone_6_scope_membership_keys(
            state,
            "aspect layout read remains proof-only until published Milestone 6 scope-to-slice membership records exist",
            "loaded Milestone 6 scope-to-slice membership records preserve canonical scope addressing for admitted aspect layout reads",
        ),
        structural_block_reuse: verify_milestone_6_structural_block_keys(
            state,
            "structural block reuse remains proof-only until published Milestone 6 structural-block records exist",
            "loaded Milestone 6 structural-block records preserve exact structural block identity and slice membership for reuse witnesses",
        ),
        chunk_model_freeze: verify_milestone_6_chunk_membership_keys(
            state,
            "chunk model freeze remains proof-only until published Milestone 6 chunk-membership records exist",
            "loaded Milestone 6 chunk-membership records preserve physical chunk addressing for frozen chunk witnesses",
        ),
        milestone_7_layout_reference: Milestone6AccessStructureVerificationPath::verified(
            "Milestone 7 layout references are compile-time isolated from slice, block, and placement internals",
        ),
        milestone_9_physical_chunk_reference: Milestone6AccessStructureVerificationPath::verified(
            "Milestone 9 physical chunk references are compile-time isolated from authority and mutation rights",
        ),
    }
}

fn verify_milestone_6_scope_membership_keys(
    state: &StoreState,
    missing_family_gap: &'static str,
    success_basis: &'static str,
) -> Milestone6AccessStructureVerificationPath {
    if state.milestone_6_scope_slice_membership_records.is_empty() {
        return Milestone6AccessStructureVerificationPath::debt(missing_family_gap);
    }
    for (stored_key, record) in &state.milestone_6_scope_slice_membership_records {
        if stored_key != record.artifact_id.as_str() {
            return Milestone6AccessStructureVerificationPath::debt(format!(
                "open-time access structure verification failed: stored key `{stored_key}` did not match Milestone 6 scope membership artifact id `{}`",
                record.artifact_id
            ));
        }
        if !state
            .milestone_6_layout_materialization_records
            .contains_key(&record.layout_materialization_artifact_id)
        {
            return Milestone6AccessStructureVerificationPath::debt(format!(
                "open-time access structure verification failed: Milestone 6 scope membership `{}` referenced missing layout materialization `{}`",
                record.artifact_id, record.layout_materialization_artifact_id
            ));
        }
    }
    Milestone6AccessStructureVerificationPath::verified(success_basis)
}

fn verify_milestone_6_structural_block_keys(
    state: &StoreState,
    missing_family_gap: &'static str,
    success_basis: &'static str,
) -> Milestone6AccessStructureVerificationPath {
    if state.milestone_6_structural_block_records.is_empty() {
        return Milestone6AccessStructureVerificationPath::debt(missing_family_gap);
    }
    for (stored_key, record) in &state.milestone_6_structural_block_records {
        if stored_key != record.artifact_id.as_str() {
            return Milestone6AccessStructureVerificationPath::debt(format!(
                "open-time access structure verification failed: stored key `{stored_key}` did not match Milestone 6 structural block artifact id `{}`",
                record.artifact_id
            ));
        }
        if record
            .supporting_layout_materialization_artifact_ids
            .is_empty()
        {
            return Milestone6AccessStructureVerificationPath::debt(format!(
                "open-time access structure verification failed: Milestone 6 structural block `{}` had no supporting layout materializations",
                record.artifact_id
            ));
        }
        for layout_materialization_artifact_id in
            &record.supporting_layout_materialization_artifact_ids
        {
            if !state
                .milestone_6_layout_materialization_records
                .contains_key(layout_materialization_artifact_id)
            {
                return Milestone6AccessStructureVerificationPath::debt(format!(
                    "open-time access structure verification failed: Milestone 6 structural block `{}` referenced missing layout materialization `{}`",
                    record.artifact_id, layout_materialization_artifact_id
                ));
            }
        }
    }
    Milestone6AccessStructureVerificationPath::verified(success_basis)
}

fn verify_milestone_6_chunk_membership_keys(
    state: &StoreState,
    missing_family_gap: &'static str,
    success_basis: &'static str,
) -> Milestone6AccessStructureVerificationPath {
    if state.milestone_6_chunk_membership_records.is_empty() {
        return Milestone6AccessStructureVerificationPath::debt(missing_family_gap);
    }
    for (stored_key, record) in &state.milestone_6_chunk_membership_records {
        if stored_key != record.artifact_id.as_str() {
            return Milestone6AccessStructureVerificationPath::debt(format!(
                "open-time access structure verification failed: stored key `{stored_key}` did not match Milestone 6 chunk membership artifact id `{}`",
                record.artifact_id
            ));
        }
        if !state
            .milestone_6_layout_materialization_records
            .contains_key(&record.layout_materialization_artifact_id)
        {
            return Milestone6AccessStructureVerificationPath::debt(format!(
                "open-time access structure verification failed: Milestone 6 chunk membership `{}` referenced missing layout materialization `{}`",
                record.artifact_id, record.layout_materialization_artifact_id
            ));
        }
    }
    Milestone6AccessStructureVerificationPath::verified(success_basis)
}

fn milestone_6_scope_slice_membership_record(
    materialization: &Milestone6LayoutMaterialization,
) -> Result<Milestone6ScopeSliceMembershipRecord, StoreError> {
    Ok(Milestone6ScopeSliceMembershipRecord {
        artifact_id: crate::layout::layout_scope_membership_artifact_id(
            materialization.admitted_plan().request(),
        )?,
        branch_id: materialization
            .admitted_plan()
            .request()
            .target()
            .branch_id()
            .clone(),
        frontier_commit_id: materialization
            .admitted_plan()
            .request()
            .target()
            .frontier_commit_id(),
        scope_class: materialization
            .admitted_plan()
            .request()
            .scope_class()
            .label()
            .to_string(),
        projection_digest: materialization
            .milestone_7_reference()
            .projection_digest()
            .to_string(),
        slice_ids: materialization.admitted_plan().slice_ids().to_vec(),
        layout_materialization_artifact_id: materialization.artifact_id().to_string(),
    })
}

fn milestone_6_commit_coupled_layout_seed_rebuild_records(
    state: &StoreState,
) -> Result<Vec<crate::backend::records::Milestone6CommitCoupledLayoutSeedRecord>, StoreError> {
    let mut artifact_ids = state
        .commit_support_summaries
        .values()
        .flat_map(|summary| {
            summary
                .milestone_6_published_layout_request_artifact_ids
                .iter()
                .cloned()
        })
        .collect::<Vec<_>>();
    artifact_ids.sort();
    artifact_ids.dedup();

    artifact_ids
        .into_iter()
        .map(|artifact_id| {
            state
                .milestone_6_commit_coupled_layout_seed_records
                .get(&artifact_id)
                .cloned()
                .ok_or_else(|| {
                    StoreError::backend_integrity(format!(
                        "milestone 6 rebuild seed `{artifact_id}` was listed by commit support publication but missing from commit-coupled layout seed storage"
                    ))
                })
        })
        .collect()
}

fn milestone_6_commit_coupled_layout_seed_record(
    materialization: &Milestone6LayoutMaterialization,
    authority_basis_commit: &crate::backend::records::StoredCommitEnvelope,
) -> Result<crate::backend::records::Milestone6CommitCoupledLayoutSeedRecord, StoreError> {
    Ok(
        crate::backend::records::Milestone6CommitCoupledLayoutSeedRecord {
            artifact_id: crate::layout::published_layout_request_artifact_id(
                materialization.admitted_plan().request(),
            )?,
            request: materialization.admitted_plan().request().clone(),
            layout_materialization_artifact_id: materialization.artifact_id().to_string(),
            authority_basis_commit_id: authority_basis_commit.envelope.commit.commit_id,
            authority_basis_commit_digest: authority_basis_commit.envelope_digest.clone(),
            authority_basis_commit_sequence: authority_basis_commit.commit_sequence,
        },
    )
}

fn milestone_6_chunk_membership_record(
    materialization: &Milestone6LayoutMaterialization,
) -> Milestone6ChunkMembershipRecord {
    Milestone6ChunkMembershipRecord {
        artifact_id: crate::layout::chunk_membership_artifact_id(materialization.frozen_layout()),
        physical_chunk_id: materialization
            .frozen_layout()
            .witness()
            .physical_chunk_id()
            .clone(),
        chunk_shape_version: materialization
            .frozen_layout()
            .witness()
            .chunk_shape_version(),
        determinism_digest: materialization
            .frozen_layout()
            .witness()
            .determinism_digest()
            .to_string(),
        slice_ids: materialization
            .frozen_layout()
            .witness()
            .ordered_slice_ids()
            .to_vec(),
        layout_materialization_artifact_id: materialization.artifact_id().to_string(),
    }
}

fn attach_milestone_6_commit_coupled_layout_seed_to_commit_support_summary(
    state: &mut StoreState,
    commit_id: CommitId,
    materialization: &Milestone6LayoutMaterialization,
) -> Result<(), StoreError> {
    let artifact_id = crate::layout::published_layout_request_artifact_id(
        materialization.admitted_plan().request(),
    )?;
    let summary_digest = {
        let summary = state
            .commit_support_summaries
            .get_mut(&commit_id.0)
            .ok_or_else(|| {
                StoreError::backend_integrity(format!(
                    "milestone 6 layout materialization `{}` targeted commit `{}` without a commit support summary",
                    materialization.artifact_id(),
                    commit_id.0
                ))
            })?;
        if !summary
            .milestone_6_published_layout_request_artifact_ids
            .contains(&artifact_id)
        {
            summary
                .milestone_6_published_layout_request_artifact_ids
                .push(artifact_id);
            summary
                .milestone_6_published_layout_request_artifact_ids
                .sort();
            summary
                .milestone_6_published_layout_request_artifact_ids
                .dedup();
        }
        stable_structural_digest(summary)?
    };
    state.upsert_digest_record(
        crate::backend::records::AuthoritativeArtifactFamily::CommitSupportSummary,
        commit_support_summary_artifact_id(commit_id),
        summary_digest,
    );
    let authoritative_summary = state
        .commit_support_summaries
        .get(&commit_id.0)
        .cloned()
        .ok_or_else(|| {
            StoreError::backend_integrity(format!(
                "milestone 6 commit support summary for commit `{}` disappeared during publication",
                commit_id.0
            ))
        })?;
    for layer in state.branch_delta_layer_records.values_mut() {
        let mut updated = false;
        for summary in &mut layer.artifacts.commit_support_summaries {
            if summary.commit_id == commit_id {
                *summary = authoritative_summary.clone();
                updated = true;
            }
        }
        if updated {
            layer.artifacts.canonicalize_order();
        }
    }
    Ok(())
}

fn milestone_6_structural_block_record(
    materialization: &Milestone6LayoutMaterialization,
) -> Milestone6StructuralBlockRecord {
    Milestone6StructuralBlockRecord {
        artifact_id: format!(
            "layout-structural-block:{}",
            materialization.block_reuse().structural_block_id().as_str()
        ),
        structural_block_id: materialization.block_reuse().structural_block_id().clone(),
        scope_class: materialization.block_reuse().scope_class().to_string(),
        equivalence_contract_version: materialization.block_reuse().equivalence_contract_version(),
        slice_ids: materialization.block_reuse().slice_ids().to_vec(),
        supporting_layout_materialization_artifact_ids: vec![materialization
            .artifact_id()
            .to_string()],
    }
}

fn merge_milestone_6_structural_block_record(
    state: &mut StoreState,
    mut record: Milestone6StructuralBlockRecord,
) {
    if let Some(existing) = state
        .milestone_6_structural_block_records
        .get_mut(&record.artifact_id)
    {
        for artifact_id in record
            .supporting_layout_materialization_artifact_ids
            .drain(..)
        {
            if !existing
                .supporting_layout_materialization_artifact_ids
                .contains(&artifact_id)
            {
                existing
                    .supporting_layout_materialization_artifact_ids
                    .push(artifact_id);
            }
        }
        existing
            .supporting_layout_materialization_artifact_ids
            .sort();
        existing
            .supporting_layout_materialization_artifact_ids
            .dedup();
        return;
    }

    state
        .milestone_6_structural_block_records
        .insert(record.artifact_id.clone(), record);
}

fn verify_string_keyed_records<'a>(
    records: impl Iterator<Item = (&'a str, &'a str, &'static str)>,
    success_basis: &'static str,
) -> Milestone7AccessStructureVerificationPath {
    for (stored_key, expected_key, label) in records {
        if stored_key != expected_key {
            return Milestone7AccessStructureVerificationPath::debt(format!(
                "open-time access structure verification failed: stored key `{stored_key}` did not match expected {label} `{expected_key}`"
            ));
        }
    }
    Milestone7AccessStructureVerificationPath::verified(success_basis)
}

fn verify_commit_support_summary_keys(
    state: &StoreState,
) -> Milestone7AccessStructureVerificationPath {
    for (commit_id, summary) in &state.commit_support_summaries {
        if *commit_id != summary.commit_id.0 {
            return Milestone7AccessStructureVerificationPath::debt(format!(
                "open-time access structure verification failed: commit support summary map key `{commit_id}` did not match summary commit id `{}`",
                summary.commit_id.0
            ));
        }
    }
    Milestone7AccessStructureVerificationPath::verified(
        "loaded commit support summary map preserves exact commit-id addressing",
    )
}

fn verify_cursor_identity_keys(state: &StoreState) -> Milestone7AccessStructureVerificationPath {
    for (stored_key, record) in &state.durable_cursor_identity_records {
        let expected_key = durable_cursor_identity_artifact_id(&record.cursor_id);
        if stored_key != &expected_key {
            return Milestone7AccessStructureVerificationPath::debt(format!(
                "open-time access structure verification failed: stored key `{stored_key}` did not match expected durable cursor identity artifact id `{expected_key}`"
            ));
        }
    }
    Milestone7AccessStructureVerificationPath::verified(
        "loaded durable cursor identity map preserves exact cursor-id addressing",
    )
}

fn verify_subscriber_checkpoint_keys(
    state: &StoreState,
) -> Milestone7AccessStructureVerificationPath {
    for (stored_key, record) in &state.subscriber_checkpoint_records {
        let expected_key =
            subscriber_checkpoint_artifact_id(&record.cursor_id, record.checkpoint_sequence);
        if stored_key != &expected_key {
            return Milestone7AccessStructureVerificationPath::debt(format!(
                "open-time access structure verification failed: stored key `{stored_key}` did not match expected subscriber checkpoint artifact id `{expected_key}`"
            ));
        }
    }
    Milestone7AccessStructureVerificationPath::verified(
        "loaded subscriber checkpoint map preserves exact cursor checkpoint addressing",
    )
}

fn branch_delta_result_record_count(result: &BranchDeltaReadResult) -> usize {
    let export = result.authoritative_export();
    export.commit_envelopes.len()
        + export.commit_parent_records.len()
        + export.commit_support_summaries.len()
        + export.schema_support_records.len()
        + export.lineage_support_records.len()
        + export.durable_cursor_identity_records.len()
        + export.subscriber_checkpoint_records.len()
        + export.branch_records.len()
        + export.branch_head_records.len()
        + export.authoritative_artifact_digests.len()
}

fn lineage_event_touches(
    event: &forge_relational::facade::lineage::LineageEventRecord,
    lineage_id: LineageId,
) -> bool {
    event.sources().contains(&lineage_id) || event.targets().contains(&lineage_id)
}
