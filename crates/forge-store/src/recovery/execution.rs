use crate::{
    backend::records::StoreState,
    failure::StoreError,
    wal::{DurableMutationId, RecoveryDecisionClass, WalRecord},
};
use forge_relational::facade::replay::CanonicalCommitEnvelope;

use super::{
    build_recovery_source_set, select_recovery_source, DurableDegradedRecovery,
    DurableRecoveryDecision, DurableRecoveryDegradedKind, RecoveryQuarantineScope,
    RecoverySourceKind,
};

pub(crate) enum RecoveryAction {
    RetainPublishedTruth,
    FinishPublicationFromCanonicalResult {
        canonical_envelope: CanonicalCommitEnvelope,
    },
    DiscardUnpublished,
    RequireRebuild,
    RequireQuarantine,
}

pub(crate) struct RecoveryEvaluation {
    pub(crate) decision: DurableRecoveryDecision,
    pub(crate) action: RecoveryAction,
    pub(crate) source_kind: RecoverySourceKind,
    pub(crate) degraded: Option<DurableDegradedRecovery>,
    pub(crate) source_report: super::RecoverySourceReport,
}

pub(crate) fn evaluate_recovery_for_mutation(
    state: &StoreState,
    durable_mutation_id: DurableMutationId,
    wal_records: &[&WalRecord],
    backend_report: crate::media::DurableMediaReport,
) -> Result<RecoveryEvaluation, StoreError> {
    let source_set =
        build_recovery_source_set(state, durable_mutation_id, wal_records, backend_report)?;
    let selection = select_recovery_source(state, &source_set)?;
    match selection.source_kind() {
        RecoverySourceKind::PublishedAuthoritativeTruth => {
            let commit_id = selection
                .commit_id()
                .expect("published authoritative recovery source should carry commit id");
            let decision = if selection.acknowledgment_eligible() {
                DurableRecoveryDecision {
                    durable_mutation_id,
                    decision: RecoveryDecisionClass::SuppressDuplicateReplay,
                    commit_id: Some(commit_id),
                }
            } else {
                DurableRecoveryDecision {
                    durable_mutation_id,
                    decision: RecoveryDecisionClass::RetainPublishedTruth,
                    commit_id: Some(commit_id),
                }
            };
            Ok(RecoveryEvaluation {
                decision,
                action: RecoveryAction::RetainPublishedTruth,
                source_kind: RecoverySourceKind::PublishedAuthoritativeTruth,
                degraded: if selection.acknowledgment_eligible() {
                    None
                } else {
                    Some(DurableDegradedRecovery {
                        durable_mutation_id,
                        kind: DurableRecoveryDegradedKind::RetainedWithoutAcknowledgment,
                        scope: RecoveryQuarantineScope::ArtifactInstance,
                        commit_id: Some(commit_id),
                        reason: "authoritative truth is retained but the durable publication unit never reached clean acknowledgment eligibility".to_string(),
                    })
                },
                source_report: selection.report().clone(),
            })
        }
        RecoverySourceKind::HostedRuntimeCanonicalResult => {
            let canonical_envelope = selection
                .canonical_envelope()
                .expect("hosted runtime recovery source should carry canonical envelope")
                .clone();
            Ok(RecoveryEvaluation {
                decision: DurableRecoveryDecision {
                    durable_mutation_id,
                    decision: RecoveryDecisionClass::FinishPublicationFromCanonicalResult,
                    commit_id: Some(canonical_envelope.commit.commit_id),
                },
                action: RecoveryAction::FinishPublicationFromCanonicalResult { canonical_envelope },
                source_kind: RecoverySourceKind::HostedRuntimeCanonicalResult,
                degraded: None,
                source_report: selection.report().clone(),
            })
        }
        RecoverySourceKind::IntentOnly => Ok(RecoveryEvaluation {
            decision: DurableRecoveryDecision {
                durable_mutation_id,
                decision: RecoveryDecisionClass::DiscardUnpublished,
                commit_id: None,
            },
            action: RecoveryAction::DiscardUnpublished,
            source_kind: RecoverySourceKind::IntentOnly,
            degraded: None,
            source_report: selection.report().clone(),
        }),
        RecoverySourceKind::RequiresRebuild => Ok(RecoveryEvaluation {
            decision: DurableRecoveryDecision {
                durable_mutation_id,
                decision: RecoveryDecisionClass::RequiresFullRebuild,
                commit_id: selection.commit_id(),
            },
            action: RecoveryAction::RequireRebuild,
            source_kind: RecoverySourceKind::RequiresRebuild,
            degraded: Some(DurableDegradedRecovery {
                durable_mutation_id,
                kind: DurableRecoveryDegradedKind::RebuildRequired,
                scope: RecoveryQuarantineScope::ArtifactInstance,
                commit_id: selection.commit_id(),
                reason: selection.report().reason().to_string(),
            }),
            source_report: selection.report().clone(),
        }),
        RecoverySourceKind::RequiresQuarantine | RecoverySourceKind::MaintenanceResidue => {
            Ok(RecoveryEvaluation {
                decision: DurableRecoveryDecision {
                    durable_mutation_id,
                    decision: RecoveryDecisionClass::RequiresQuarantine,
                    commit_id: selection.commit_id(),
                },
                action: RecoveryAction::RequireQuarantine,
                source_kind: selection.source_kind(),
                degraded: Some(DurableDegradedRecovery {
                    durable_mutation_id,
                    kind: DurableRecoveryDegradedKind::QuarantineRequired,
                    scope: RecoveryQuarantineScope::ArtifactInstance,
                    commit_id: selection.commit_id(),
                    reason: selection.report().reason().to_string(),
                }),
                source_report: selection.report().clone(),
            })
        }
    }
}
