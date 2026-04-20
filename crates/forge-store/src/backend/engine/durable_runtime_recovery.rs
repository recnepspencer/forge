use crate::bulk::ChunkOrdinal;
use crate::failure::{StoreError, StoreErrorKind};
use crate::recovery::{
    build_recovery_plan, evaluate_recovery_for_mutation, DurableRecoveryDecision,
    DurableRecoveryOutcome, DurableRecoveryPlan, DurableRetryResolution, RecoveryAction,
    RecoverySourceKind,
};
use crate::wal::{DurableMutationId, DurablePublicationPhase, RecoveryDecisionClass, WalRecord, WalRecordPayload};

use super::{StateBackedStoreBackend, StatePersistence};

impl<P: StatePersistence> StateBackedStoreBackend<P> {
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
            Ok(DurableRetryResolution::RetryRequiresOperatorOrHigherLevelPolicy {
                durable_mutation_id,
            })
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
                    let persisted = self.append(self.verify_append(crate::authority::canonicalize(
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
    wal_records.iter().rev().find_map(|record| match &record.payload {
        WalRecordPayload::BulkCheckpointPublicationIntent(intent) => intent.checkpoint_sequence,
        _ => None,
    })
}
