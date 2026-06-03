use crate::adapter::{TruthWritebackReceipt, TruthWritebackRequest};
use crate::facade::{
    BridgeRouteIdentity, BridgeWritebackAuthorityOutcome, BridgeWritebackCausalityBasis,
    BridgeWritebackError, BridgeWritebackFeedbackContext, BridgeWritebackFeedbackProvenance,
    BridgeWritebackIdempotenceBasis, BridgeWritebackLoopPreventionReport,
    BridgeWritebackReplayBundle, TruthCommitIdentity,
};
use crate::writeback::{
    AdmittedBridgeWritebackContract, BridgeDerivedWritebackEffect,
    BridgeValidatedWritebackCandidate, BridgeWritebackOutcomeClass,
    BridgeWritebackRetryDisposition, BridgeWritebackStrategyClass,
    BridgeWritebackStrategyCoherenceReport, BridgeWritebackStrategyDescriptorBasis,
};

mod terminal_projection_access;

pub(in crate::harness::adapter::adapter_impl) struct WritebackFeedbackLoopMatrixEvidence<'a> {
    pub contract: &'a AdmittedBridgeWritebackContract,
    pub effect: &'a BridgeDerivedWritebackEffect,
    pub original_causality: &'a BridgeWritebackCausalityBasis,
    pub replayed_bundle: &'a BridgeWritebackReplayBundle,
    pub initial_outcome: &'a BridgeWritebackAuthorityOutcome,
    pub initial_idempotence: &'a BridgeWritebackIdempotenceBasis,
    pub replayed_idempotence: &'a BridgeWritebackIdempotenceBasis,
    pub loop_prevention: &'a BridgeWritebackLoopPreventionReport,
    pub replayed_strategy_coherence: &'a BridgeWritebackStrategyCoherenceReport,
    pub replayed_candidate: Option<&'a BridgeValidatedWritebackCandidate>,
    pub feedback_authority_request: Option<&'a TruthWritebackRequest>,
    pub replayed_receipt: Option<&'a TruthWritebackReceipt>,
    pub changed_effect: &'a BridgeDerivedWritebackEffect,
    pub changed_idempotence: &'a BridgeWritebackIdempotenceBasis,
    pub changed_effect_error: &'a BridgeWritebackError,
    pub changed_effect_feedback_matches_initial: bool,
    pub ordinary_truth_commit_identity: &'a TruthCommitIdentity,
    pub ordinary_route_identity: &'a BridgeRouteIdentity,
    pub rebuilt_contract: &'a AdmittedBridgeWritebackContract,
    pub rebuilt_effect: &'a BridgeDerivedWritebackEffect,
    pub rebuilt_idempotence: &'a BridgeWritebackIdempotenceBasis,
    pub rebuilt_loop_prevention: &'a BridgeWritebackLoopPreventionReport,
    pub rebuilt_outcome: &'a BridgeWritebackAuthorityOutcome,
    pub rebuilt_replay_bundle: &'a BridgeWritebackReplayBundle,
    pub rebuilt_receipt: Option<&'a TruthWritebackReceipt>,
    pub feedback_provenance: &'a BridgeWritebackFeedbackProvenance,
    pub carried_feedback_context: &'a BridgeWritebackFeedbackContext,
    pub feedback_commit_identity: &'a TruthCommitIdentity,
    pub feedback_route_identity: &'a BridgeRouteIdentity,
    pub authoritative_commit_count: usize,
}

pub(in crate::harness::adapter::adapter_impl) struct WritebackFeedbackLoopMatrix {
    effect: BridgeDerivedWritebackEffect,
    causality: BridgeWritebackCausalityBasis,
    initial_outcome: BridgeWritebackAuthorityOutcome,
    replay_bundle_report: FeedbackReplayBundleReport,
    idempotence_report: FeedbackIdempotenceReport,
    loop_prevention_report: FeedbackLoopPreventionReport,
    authority_boundary_matrix: FeedbackAuthorityBoundaryMatrix,
    changed_effect_feedback_matrix: FeedbackChangedEffectMatrix,
    interleaved_truth_matrix: FeedbackInterleavedTruthMatrix,
    restart_replay_matrix: FeedbackRestartReplayMatrix,
    feedback_provenance: BridgeWritebackFeedbackProvenance,
    carried_feedback_context: BridgeWritebackFeedbackContext,
    feedback_route_identity: BridgeRouteIdentity,
    boundedness_proof: FeedbackBoundednessProof,
}

pub(in crate::harness::adapter::adapter_impl) struct FeedbackReplayBundleReport {
    replay_bundle: BridgeWritebackReplayBundle,
    strategy_class: BridgeWritebackStrategyClass,
    strategy_descriptor_basis: BridgeWritebackStrategyDescriptorBasis,
    retry_disposition: BridgeWritebackRetryDisposition,
    outcome_class: BridgeWritebackOutcomeClass,
}

pub(in crate::harness::adapter::adapter_impl) struct FeedbackIdempotenceReport {
    initial_idempotence: BridgeWritebackIdempotenceBasis,
    replayed_idempotence: BridgeWritebackIdempotenceBasis,
}

pub(in crate::harness::adapter::adapter_impl) struct FeedbackLoopPreventionReport {
    report: BridgeWritebackLoopPreventionReport,
}

pub(in crate::harness::adapter::adapter_impl) struct FeedbackAuthorityBoundaryMatrix {
    contract: AdmittedBridgeWritebackContract,
    strategy_coherence: BridgeWritebackStrategyCoherenceReport,
    candidate: Option<BridgeValidatedWritebackCandidate>,
    authority_request: Option<TruthWritebackRequest>,
    authority_receipt: Option<TruthWritebackReceipt>,
}

pub(in crate::harness::adapter::adapter_impl) struct FeedbackChangedEffectMatrix {
    changed_effect: BridgeDerivedWritebackEffect,
    changed_idempotence: BridgeWritebackIdempotenceBasis,
    failure: BridgeWritebackError,
    same_causality_as_initial: bool,
    same_feedback_provenance_as_initial: bool,
}

pub(in crate::harness::adapter::adapter_impl) struct FeedbackInterleavedTruthMatrix {
    ordinary_truth_commit_identity: TruthCommitIdentity,
    ordinary_truth_route_identity: BridgeRouteIdentity,
    bridge_feedback_commit_identity: TruthCommitIdentity,
    interleaving_preserved_single_authoritative_commit: bool,
}

pub(in crate::harness::adapter::adapter_impl) struct FeedbackRestartReplayMatrix {
    rebuilt_contract: AdmittedBridgeWritebackContract,
    rebuilt_effect: BridgeDerivedWritebackEffect,
    rebuilt_idempotence: BridgeWritebackIdempotenceBasis,
    rebuilt_loop_prevention: BridgeWritebackLoopPreventionReport,
    rebuilt_outcome: BridgeWritebackAuthorityOutcome,
    rebuilt_replay_bundle: BridgeWritebackReplayBundle,
    rebuilt_receipt: Option<TruthWritebackReceipt>,
    replay_equivalent_to_live_feedback: bool,
}

pub(in crate::harness::adapter::adapter_impl) struct FeedbackBoundednessProof {
    authoritative_commit_count: usize,
    replayed_feedback_outcome_class: BridgeWritebackOutcomeClass,
    changed_effect_retrigger_failure_kind: crate::facade::BridgeWritebackErrorKind,
    feedback_publication_routed: bool,
    ordinary_truth_interleaved: bool,
    feedback_converged: bool,
    restart_replay_converged: bool,
    replayed_authority_receipt_present: bool,
}

impl WritebackFeedbackLoopMatrix {
    pub(in crate::harness::adapter::adapter_impl) fn from_feedback_evidence(
        evidence: WritebackFeedbackLoopMatrixEvidence<'_>,
    ) -> Self {
        Self {
            effect: evidence.effect.clone(),
            causality: evidence.original_causality.clone(),
            initial_outcome: evidence.initial_outcome.clone(),
            replay_bundle_report: FeedbackReplayBundleReport::from_replay_bundle(
                evidence.replayed_bundle,
            ),
            idempotence_report: FeedbackIdempotenceReport::from_idempotence(
                evidence.initial_idempotence,
                evidence.replayed_idempotence,
            ),
            loop_prevention_report: FeedbackLoopPreventionReport::from_loop_prevention(
                evidence.loop_prevention,
            ),
            authority_boundary_matrix: FeedbackAuthorityBoundaryMatrix {
                contract: evidence.contract.clone(),
                strategy_coherence: evidence.replayed_strategy_coherence.clone(),
                candidate: evidence.replayed_candidate.cloned(),
                authority_request: evidence.feedback_authority_request.cloned(),
                authority_receipt: evidence.replayed_receipt.cloned(),
            },
            changed_effect_feedback_matrix: FeedbackChangedEffectMatrix {
                changed_effect: evidence.changed_effect.clone(),
                changed_idempotence: evidence.changed_idempotence.clone(),
                failure: evidence.changed_effect_error.clone(),
                same_causality_as_initial: evidence.changed_effect.causality_digest()
                    == evidence.original_causality.digest(),
                same_feedback_provenance_as_initial: evidence
                    .changed_effect_feedback_matches_initial,
            },
            interleaved_truth_matrix: FeedbackInterleavedTruthMatrix {
                ordinary_truth_commit_identity: evidence.ordinary_truth_commit_identity.clone(),
                ordinary_truth_route_identity: evidence.ordinary_route_identity.clone(),
                bridge_feedback_commit_identity: evidence.feedback_commit_identity.clone(),
                interleaving_preserved_single_authoritative_commit: evidence
                    .authoritative_commit_count
                    == 1,
            },
            restart_replay_matrix: FeedbackRestartReplayMatrix {
                rebuilt_contract: evidence.rebuilt_contract.clone(),
                rebuilt_effect: evidence.rebuilt_effect.clone(),
                rebuilt_idempotence: evidence.rebuilt_idempotence.clone(),
                rebuilt_loop_prevention: evidence.rebuilt_loop_prevention.clone(),
                rebuilt_outcome: evidence.rebuilt_outcome.clone(),
                rebuilt_replay_bundle: evidence.rebuilt_replay_bundle.clone(),
                rebuilt_receipt: evidence.rebuilt_receipt.cloned(),
                replay_equivalent_to_live_feedback: evidence.rebuilt_replay_bundle.digest()
                    == evidence.replayed_bundle.digest(),
            },
            feedback_provenance: evidence.feedback_provenance.clone(),
            carried_feedback_context: evidence.carried_feedback_context.clone(),
            feedback_route_identity: evidence.feedback_route_identity.clone(),
            boundedness_proof: FeedbackBoundednessProof {
                authoritative_commit_count: evidence.authoritative_commit_count,
                replayed_feedback_outcome_class: BridgeWritebackOutcomeClass::CanonicalNoop,
                changed_effect_retrigger_failure_kind: evidence.changed_effect_error.kind(),
                feedback_publication_routed: true,
                ordinary_truth_interleaved: true,
                feedback_converged: true,
                restart_replay_converged: evidence.rebuilt_replay_bundle.digest()
                    == evidence.replayed_bundle.digest(),
                replayed_authority_receipt_present: evidence.replayed_receipt.is_some(),
            },
        }
    }

    pub(in crate::harness::adapter::adapter_impl) fn writeback_digest(&self) -> &str {
        self.replay_bundle_report.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn effect_intent_digest(&self) -> &str {
        self.effect.effect_intent_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn effect(
        &self,
    ) -> &BridgeDerivedWritebackEffect {
        &self.effect
    }

    pub(in crate::harness::adapter::adapter_impl) fn causality_digest(&self) -> &str {
        self.causality.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn causality(
        &self,
    ) -> &BridgeWritebackCausalityBasis {
        &self.causality
    }

    pub(in crate::harness::adapter::adapter_impl) fn feedback_provenance(
        &self,
    ) -> &BridgeWritebackFeedbackProvenance {
        &self.feedback_provenance
    }

    pub(in crate::harness::adapter::adapter_impl) fn carried_feedback_context(
        &self,
    ) -> &BridgeWritebackFeedbackContext {
        &self.carried_feedback_context
    }

    pub(in crate::harness::adapter::adapter_impl) fn feedback_route_identity(
        &self,
    ) -> &BridgeRouteIdentity {
        &self.feedback_route_identity
    }

    pub(in crate::harness::adapter::adapter_impl) fn replay_digest(&self) -> &str {
        self.replay_bundle_report.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn mutation_plan_digest(&self) -> &str {
        self.initial_outcome.authoritative_artifact_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn replay_bundle_report(
        &self,
    ) -> &FeedbackReplayBundleReport {
        &self.replay_bundle_report
    }

    pub(in crate::harness::adapter::adapter_impl) fn idempotence_report(
        &self,
    ) -> &FeedbackIdempotenceReport {
        &self.idempotence_report
    }

    pub(in crate::harness::adapter::adapter_impl) fn loop_prevention_report(
        &self,
    ) -> &FeedbackLoopPreventionReport {
        &self.loop_prevention_report
    }

    pub(in crate::harness::adapter::adapter_impl) fn authority_boundary_matrix(
        &self,
    ) -> &FeedbackAuthorityBoundaryMatrix {
        &self.authority_boundary_matrix
    }

    pub(in crate::harness::adapter::adapter_impl) fn changed_effect_feedback_matrix(
        &self,
    ) -> &FeedbackChangedEffectMatrix {
        &self.changed_effect_feedback_matrix
    }

    pub(in crate::harness::adapter::adapter_impl) fn boundedness_proof(
        &self,
    ) -> &FeedbackBoundednessProof {
        &self.boundedness_proof
    }
}

impl FeedbackReplayBundleReport {
    fn from_replay_bundle(bundle: &BridgeWritebackReplayBundle) -> Self {
        Self {
            replay_bundle: bundle.clone(),
            strategy_class: bundle.strategy_class(),
            strategy_descriptor_basis: bundle.strategy_descriptor_basis().clone(),
            retry_disposition: bundle.retry_disposition(),
            outcome_class: bundle.outcome_class(),
        }
    }
}

impl FeedbackIdempotenceReport {
    fn from_idempotence(
        initial: &BridgeWritebackIdempotenceBasis,
        replayed: &BridgeWritebackIdempotenceBasis,
    ) -> Self {
        Self {
            initial_idempotence: initial.clone(),
            replayed_idempotence: replayed.clone(),
        }
    }
}

impl FeedbackLoopPreventionReport {
    fn from_loop_prevention(report: &BridgeWritebackLoopPreventionReport) -> Self {
        Self {
            report: report.clone(),
        }
    }
}
