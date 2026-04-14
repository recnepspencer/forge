use crate::{
    authority::{
        AuthoritativeBranchHeadRecord, AuthoritativeExportBundle, CanonicalizedCommitEnvelope,
        FetchedAuthoritativeCommit, PersistedAuthoritativeCommit, VerifiedAuthoritativeAppend,
    },
    evidence::{CanonicalizationMetrics, StoreCounterSnapshot, StoreCounters},
    failure::{StoreError, StoreErrorKind},
    recovery::{
        build_recovery_plan, evaluate_recovery_for_mutation, DurableRecoveryDecision,
        DurableRecoveryOutcome, DurableRecoveryPlan, DurableRetryResolution, RecoveryAction,
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
    fn persist_state(&mut self, state: &StoreState) -> Result<(), StoreError>;
}

#[derive(Debug)]
pub(crate) struct StateBackedStoreBackend<P> {
    persistence: P,
    state: StoreState,
    counters: StoreCounters,
}

impl<P: StatePersistence> StateBackedStoreBackend<P> {
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
        let bundle = bundle.into_canonicalized();
        let mut state = StoreState::default();
        state.canonicalization_version = bundle.canonicalization_version;
        for branch_record in bundle.branch_records {
            let branch_id = branch_record.branch_id.0.clone();
            if state
                .branch_records
                .insert(branch_id.clone(), branch_record)
                .is_some()
            {
                return Err(StoreError::new(
                    StoreErrorKind::DuplicateArtifactIdentity,
                    format!("duplicate branch record `{branch_id}` in authoritative export"),
                ));
            }
        }
        for branch_head_record in bundle.branch_head_records {
            let branch_id = branch_head_record.branch_id.0.clone();
            if state
                .branch_head_records
                .insert(branch_id.clone(), branch_head_record)
                .is_some()
            {
                return Err(StoreError::new(
                    StoreErrorKind::DuplicateArtifactIdentity,
                    format!("duplicate branch head record `{branch_id}` in authoritative export"),
                ));
            }
        }
        for commit_envelope in bundle.commit_envelopes {
            let commit_id = commit_envelope.envelope.commit.commit_id.0;
            if state
                .commit_envelopes
                .insert(commit_id, commit_envelope)
                .is_some()
            {
                return Err(StoreError::duplicate_conflict(CommitId(commit_id)));
            }
        }
        for parent_record in bundle.commit_parent_records {
            let artifact_id = super::integrity::parent_artifact_id(
                parent_record.commit_id,
                parent_record.parent_position,
            );
            if state
                .commit_parent_records
                .insert(artifact_id.clone(), parent_record)
                .is_some()
            {
                return Err(StoreError::new(
                    StoreErrorKind::DuplicateArtifactIdentity,
                    format!(
                        "duplicate commit parent record `{artifact_id}` in authoritative export"
                    ),
                ));
            }
        }
        for digest_record in bundle.authoritative_artifact_digests {
            let artifact_key = format!(
                "{:?}:{}:v{}",
                digest_record.artifact_family,
                digest_record.artifact_id,
                digest_record.canonicalization_version
            );
            if state
                .authoritative_artifact_digests
                .insert(artifact_key.clone(), digest_record)
                .is_some()
            {
                return Err(StoreError::new(
                    StoreErrorKind::DuplicateArtifactIdentity,
                    format!("duplicate digest record `{artifact_key}` in authoritative export"),
                ));
            }
        }
        state.next_commit_sequence = state
            .commit_envelopes
            .values()
            .map(|record| record.commit_sequence)
            .max()
            .map(|sequence| sequence + 1)
            .unwrap_or(1);
        state.next_head_update_sequence = state
            .branch_head_records
            .values()
            .map(|record| record.head_update_sequence)
            .max()
            .map(|sequence| sequence + 1)
            .unwrap_or(1);
        state.verify_integrity()?;
        persistence.persist_state(&state)?;
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
        let next = self.state.stage_branch_creation(new_branch, from_branch)?;
        self.commit_state(next)?;
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
        let next = self.state.stage_verified_append(&verified)?;
        self.commit_state(next)?;
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

        let mut next = self.state.clone();
        next.embedded_checkpoint_records
            .insert(record.checkpoint_id.clone(), record.clone());
        self.commit_state(next)?;
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

    pub fn admit_durable_mutation(
        &mut self,
        runtime_session_id: &str,
        operation_name: &str,
    ) -> Result<DurableMutationId, StoreError> {
        let mut next = self.state.clone();
        let durable_mutation_id = next.allocate_durable_mutation_id();
        let record = WalRecord::durable_mutation_intent(
            next.next_wal_sequence,
            durable_mutation_id,
            runtime_session_id,
            operation_name,
        )?;
        next.append_wal_record(record)?;
        self.commit_state(next)?;
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
        let mut next = self.state.clone();
        let record = WalRecord::hosted_runtime_commit_result(
            next.next_wal_sequence,
            durable_mutation_id,
            runtime_session_id,
            envelope,
        )?;
        next.append_wal_record(record)?;
        self.commit_state(next)?;
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
        let mut next = self.state.clone();
        let record = WalRecord::durable_publication_progress(
            next.next_wal_sequence,
            durable_mutation_id,
            runtime_session_id,
            phase,
            commit_id,
        )?;
        next.append_wal_record(record)?;
        self.commit_state(next)?;
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
        let wal_sequences: Vec<u64> = self.state.wal_records.keys().copied().collect();
        self.counters.record_wal_scan(wal_sequences.len());
        let mut decisions = Vec::new();

        for durable_mutation_id in plan.pending_durable_mutation_ids {
            let wal_records = self.state.wal_records_for_mutation(durable_mutation_id);
            let evaluation = match evaluate_recovery_for_mutation(
                &self.state,
                durable_mutation_id,
                &wal_records,
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
            }

            let decision = DurableRecoveryDecision {
                durable_mutation_id,
                decision,
                commit_id,
            };

            let mut next = self.state.clone();
            let record = WalRecord::recovery_decision(
                next.next_wal_sequence,
                durable_mutation_id,
                runtime_session_id,
                decision.decision,
                decision.commit_id,
            )?;
            next.append_wal_record(record)?;
            self.commit_state(next)?;
            self.counters.record_wal_append();
            decisions.push(decision);
        }

        Ok(DurableRecoveryOutcome { decisions })
    }

    fn commit_state(&mut self, next: StoreState) -> Result<(), StoreError> {
        next.verify_integrity()?;
        self.persistence.persist_state(&next)?;
        self.state = next;
        Ok(())
    }
}
