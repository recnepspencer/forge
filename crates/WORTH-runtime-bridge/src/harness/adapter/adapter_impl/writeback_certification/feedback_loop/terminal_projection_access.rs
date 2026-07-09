use super::{
    FeedbackAuthorityBoundaryMatrix, FeedbackChangedEffectMatrix, FeedbackIdempotenceReport,
    FeedbackLoopPreventionReport, FeedbackReplayBundleReport, WritebackFeedbackLoopMatrix,
};
use crate::{
    facade::BridgeWritebackErrorKind,
    routing::canonicalization::digest_string,
    writeback::{
        BridgeWritebackOutcomeClass, BridgeWritebackRetryDisposition, BridgeWritebackStrategyClass,
    },
};

mod restart_projection_access;
mod truth_interleaving_projection_access;

impl WritebackFeedbackLoopMatrix {
    pub(in crate::harness::adapter::adapter_impl) fn feedback_provenance_digest(&self) -> &str {
        self.feedback_provenance.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn carried_causality_digest(&self) -> &str {
        self.carried_feedback_context.causality_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn carried_feedback_provenance_digest(
        &self,
    ) -> &str {
        self.carried_feedback_context.provenance_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn initial_causality_digest(&self) -> &str {
        self.causality.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn feedback_route_digest(&self) -> String {
        digest_string(
            "bridge-writeback-feedback-route",
            self.feedback_route_identity.as_str(),
        )
        .to_string()
    }

    pub(in crate::harness::adapter::adapter_impl) fn writeback_effect_artifact_digest(
        &self,
    ) -> &str {
        self.effect.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn effect_intent_patch_canonical_basis(
        &self,
    ) -> &str {
        self.effect.effect_intent().patch_canonical_basis()
    }

    pub(in crate::harness::adapter::adapter_impl) fn initial_outcome(
        &self,
    ) -> &crate::facade::BridgeWritebackAuthorityOutcome {
        &self.initial_outcome
    }

    pub(in crate::harness::adapter::adapter_impl) fn loop_prevention_digest(&self) -> &str {
        self.loop_prevention_report.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn loop_prevention_disposition(&self) -> String {
        self.loop_prevention_report.disposition()
    }
}

impl FeedbackLoopPreventionReport {
    pub(in crate::harness::adapter::adapter_impl) fn digest(&self) -> &str {
        self.report.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn disposition(&self) -> String {
        format!("{:?}", self.report.disposition())
    }

    pub(in crate::harness::adapter::adapter_impl) fn current_feedback_provenance_digest(
        &self,
    ) -> &str {
        self.report.current_feedback_provenance_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn current_causality_digest(&self) -> &str {
        self.report.current_causality_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn incoming_feedback_provenance_digest(
        &self,
    ) -> Option<&str> {
        self.report.incoming_feedback_provenance_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn incoming_feedback_causality_digest(
        &self,
    ) -> Option<&str> {
        self.report.incoming_feedback_causality_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn report(
        &self,
    ) -> &crate::facade::BridgeWritebackLoopPreventionReport {
        &self.report
    }
}

impl FeedbackReplayBundleReport {
    pub(in crate::harness::adapter::adapter_impl) fn strategy_class(
        &self,
    ) -> BridgeWritebackStrategyClass {
        self.strategy_class
    }

    pub(in crate::harness::adapter::adapter_impl) fn retry_disposition(
        &self,
    ) -> BridgeWritebackRetryDisposition {
        self.retry_disposition
    }

    pub(in crate::harness::adapter::adapter_impl) fn outcome_class(
        &self,
    ) -> BridgeWritebackOutcomeClass {
        self.outcome_class
    }

    pub(in crate::harness::adapter::adapter_impl) fn digest(&self) -> &str {
        self.replay_bundle.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn semantic_digest(&self) -> &str {
        self.replay_bundle.semantic_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn effect_intent_digest(&self) -> &str {
        self.replay_bundle.effect_intent_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn effect_intent_patch_canonical_basis(
        &self,
    ) -> &str {
        self.replay_bundle.effect_intent_patch_canonical_basis()
    }

    pub(in crate::harness::adapter::adapter_impl) fn causality_digest(&self) -> &str {
        self.replay_bundle.causality_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn lowered_policy_digest(&self) -> &str {
        self.replay_bundle.lowered_policy_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn authoritative_artifact_digest(&self) -> &str {
        self.replay_bundle.authoritative_artifact_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn strategy_descriptor_digest(&self) -> &str {
        self.strategy_descriptor_basis.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn replay_bundle(
        &self,
    ) -> &crate::facade::BridgeWritebackReplayBundle {
        &self.replay_bundle
    }
}

impl FeedbackIdempotenceReport {
    pub(in crate::harness::adapter::adapter_impl) fn initial_digest(&self) -> &str {
        self.initial_idempotence.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn replayed_digest(&self) -> &str {
        self.replayed_idempotence.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn idempotence_class(&self) -> String {
        format!("{:?}", self.initial_idempotence.idempotence_class())
    }

    pub(in crate::harness::adapter::adapter_impl) fn initial_authoritative_state_digest(
        &self,
    ) -> &str {
        self.initial_idempotence.authoritative_state_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn replayed_authoritative_state_digest(
        &self,
    ) -> &str {
        self.replayed_idempotence.authoritative_state_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn lowered_policy_digest(&self) -> &str {
        self.initial_idempotence.lowered_policy_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn strategy_descriptor_digest(&self) -> &str {
        self.initial_idempotence.strategy_descriptor_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn initial_idempotence(
        &self,
    ) -> &crate::facade::BridgeWritebackIdempotenceBasis {
        &self.initial_idempotence
    }

    pub(in crate::harness::adapter::adapter_impl) fn replayed_idempotence(
        &self,
    ) -> &crate::facade::BridgeWritebackIdempotenceBasis {
        &self.replayed_idempotence
    }
}

impl FeedbackAuthorityBoundaryMatrix {
    pub(in crate::harness::adapter::adapter_impl) fn contract_digest(&self) -> &str {
        self.contract.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn strategy_basis_digest(&self) -> &str {
        self.contract
            .validated_declaration()
            .strategy_basis()
            .expect("admitted writeback contract should preserve strategy basis")
            .digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn strategy_coherence_digest(&self) -> &str {
        self.strategy_coherence.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn strategy_coherence_disposition(
        &self,
    ) -> String {
        format!("{:?}", self.strategy_coherence.disposition())
    }

    pub(in crate::harness::adapter::adapter_impl) fn candidate_digest(&self) -> Option<&str> {
        self.candidate.as_ref().map(|candidate| candidate.digest())
    }

    pub(in crate::harness::adapter::adapter_impl) fn contract(
        &self,
    ) -> &crate::facade::AdmittedBridgeWritebackContract {
        &self.contract
    }

    pub(in crate::harness::adapter::adapter_impl) fn strategy_coherence(
        &self,
    ) -> &crate::facade::BridgeWritebackStrategyCoherenceReport {
        &self.strategy_coherence
    }

    pub(in crate::harness::adapter::adapter_impl) fn candidate(
        &self,
    ) -> Option<&crate::facade::BridgeValidatedWritebackCandidate> {
        self.candidate.as_ref()
    }

    pub(in crate::harness::adapter::adapter_impl) fn authority_request_digest(
        &self,
    ) -> Option<&str> {
        self.authority_request
            .as_ref()
            .map(|request| request.digest())
    }

    pub(in crate::harness::adapter::adapter_impl) fn authority_receipt_digest(
        &self,
    ) -> Option<&str> {
        self.authority_receipt
            .as_ref()
            .map(|receipt| receipt.digest())
    }

    pub(in crate::harness::adapter::adapter_impl) fn authority_request(
        &self,
    ) -> Option<&crate::adapter::TruthWritebackRequest> {
        self.authority_request.as_ref()
    }

    pub(in crate::harness::adapter::adapter_impl) fn authority_receipt(
        &self,
    ) -> Option<&crate::adapter::TruthWritebackReceipt> {
        self.authority_receipt.as_ref()
    }
}

impl FeedbackChangedEffectMatrix {
    pub(in crate::harness::adapter::adapter_impl) fn failure_digest(&self) -> String {
        digest_string(
            "bridge-writeback-feedback-changed-effect-failure",
            &format!("{:?}|{}", self.failure.kind(), self.failure),
        )
        .to_string()
    }

    pub(in crate::harness::adapter::adapter_impl) fn writeback_effect_artifact_digest(
        &self,
    ) -> &str {
        self.changed_effect.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn effect_intent_digest(&self) -> &str {
        self.changed_effect.effect_intent_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn effect_intent_patch_canonical_basis(
        &self,
    ) -> &str {
        self.changed_effect.effect_intent().patch_canonical_basis()
    }

    pub(in crate::harness::adapter::adapter_impl) fn causality_digest(&self) -> &str {
        self.changed_effect.causality_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn idempotence_digest(&self) -> &str {
        self.changed_idempotence.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn failure_kind(
        &self,
    ) -> BridgeWritebackErrorKind {
        self.failure.kind()
    }

    pub(in crate::harness::adapter::adapter_impl) fn changed_effect(
        &self,
    ) -> &crate::writeback::BridgeDerivedWritebackEffect {
        &self.changed_effect
    }

    pub(in crate::harness::adapter::adapter_impl) fn changed_idempotence(
        &self,
    ) -> &crate::facade::BridgeWritebackIdempotenceBasis {
        &self.changed_idempotence
    }

    pub(in crate::harness::adapter::adapter_impl) fn failure(
        &self,
    ) -> &crate::facade::BridgeWritebackError {
        &self.failure
    }

    pub(in crate::harness::adapter::adapter_impl) fn same_causality_as_initial(&self) -> bool {
        self.same_causality_as_initial
    }

    pub(in crate::harness::adapter::adapter_impl) fn same_feedback_provenance_as_initial(
        &self,
    ) -> bool {
        self.same_feedback_provenance_as_initial
    }
}
