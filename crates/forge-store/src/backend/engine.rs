use crate::{
    authority::{
        AuthoritativeBranchHeadRecord, AuthoritativeExportBundle, CanonicalizedCommitEnvelope,
        FetchedAuthoritativeCommit, PersistedAuthoritativeCommit, VerifiedAuthoritativeAppend,
    },
    evidence::{CanonicalizationMetrics, StoreCounterSnapshot, StoreCounters},
    failure::{StoreError, StoreErrorKind},
    media::DurableMediaReport,
    publication::{
        classify_durable_publication, classify_snapshot_publication, durable_publication_facts,
        PublicationWriteOutcome,
    },
    recovery::{
        build_backup_restore_compatibility_report, build_maintenance_recovery_report,
        build_recovery_plan, classify_snapshot_maintenance_recovery,
        evaluate_recovery_for_mutation, BackupRestoreCompatibilityReport, DurableRecoveryDecision,
        DurableRecoveryOutcome, DurableRecoveryPlan, DurableRetryResolution,
        MaintenanceRecoveryReport, RecoveryAction, RecoverySourceKind,
        SnapshotMaintenanceRecoveryReport,
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

use super::{
    integrity::branch_key,
    records::{EmbeddedCheckpointRecord, StoreState},
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
    counters: StoreCounters,
}

impl<P: StatePersistence> StateBackedStoreBackend<P> {
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
        Ok(Self {
            persistence,
            state,
            counters: StoreCounters::default(),
        })
    }

    pub fn from_export_bundle_with_persistence(
        mut persistence: P,
        bundle: AuthoritativeExportBundle,
    ) -> Result<Self, StoreError> {
        let state = StoreState::from_authoritative_export_bundle(bundle)?;
        let _ = persistence.persist_state(&state)?;
        Ok(Self {
            persistence,
            state,
            counters: StoreCounters::default(),
        })
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
        self.fetch_branch_head(&created_branch_id)
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
        let digest_writes = verified.envelope().commit.parents.len() as u64
            + if branch_already_exists { 2 } else { 4 };
        let branch_head_writes = if branch_already_exists { 1 } else { 2 };
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
        self.counters.record_state_delta_apply(
            if branch_already_exists { 3 } else { 4 },
            verified.envelope().commit.parents.len() as u64
                + if branch_already_exists { 2 } else { 3 },
        );
        self.counters.record_append(
            verified.envelope().commit.parents.len(),
            digest_writes,
            branch_head_writes,
        );
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

    pub fn record_canonicalization(&self, metrics: CanonicalizationMetrics) {
        self.counters.record_canonicalization(metrics);
    }

    pub fn counter_snapshot(&self) -> StoreCounterSnapshot {
        self.counters.snapshot()
    }

    pub fn durable_media_report(&self) -> DurableMediaReport {
        self.persistence.durable_media_report()
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
        self.state
            .embedded_checkpoint_records
            .get(checkpoint_id)
            .cloned()
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::CommitNotFound,
                    format!("embedded checkpoint `{checkpoint_id}` not found"),
                )
            })
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
            let wal_records = self.state.wal_records_for_mutation(durable_mutation_id);
            let evaluation = match evaluate_recovery_for_mutation(
                &self.state,
                durable_mutation_id,
                &wal_records,
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
