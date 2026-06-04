use super::super::{
    FeedbackBoundednessProof, FeedbackRestartReplayMatrix, WritebackFeedbackLoopMatrix,
};

impl FeedbackRestartReplayMatrix {
    pub(in crate::harness::adapter::adapter_impl) fn rebuilt_contract_digest(&self) -> &str {
        self.rebuilt_contract.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn rebuilt_writeback_effect_artifact_digest(
        &self,
    ) -> &str {
        self.rebuilt_effect.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn rebuilt_effect_intent_digest(&self) -> &str {
        self.rebuilt_effect.effect_intent_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn rebuilt_idempotence_digest(&self) -> &str {
        self.rebuilt_idempotence.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn rebuilt_loop_prevention_digest(&self) -> &str {
        self.rebuilt_loop_prevention.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn rebuilt_loop_prevention_disposition(
        &self,
    ) -> String {
        format!("{:?}", self.rebuilt_loop_prevention.disposition())
    }

    pub(in crate::harness::adapter::adapter_impl) fn rebuilt_outcome_digest(&self) -> &str {
        self.rebuilt_outcome.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn rebuilt_replay_bundle_digest(&self) -> &str {
        self.rebuilt_replay_bundle.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn rebuilt_contract(
        &self,
    ) -> &crate::facade::AdmittedBridgeWritebackContract {
        &self.rebuilt_contract
    }

    pub(in crate::harness::adapter::adapter_impl) fn rebuilt_effect(
        &self,
    ) -> &crate::writeback::BridgeDerivedWritebackEffect {
        &self.rebuilt_effect
    }

    pub(in crate::harness::adapter::adapter_impl) fn rebuilt_idempotence(
        &self,
    ) -> &crate::facade::BridgeWritebackIdempotenceBasis {
        &self.rebuilt_idempotence
    }

    pub(in crate::harness::adapter::adapter_impl) fn rebuilt_loop_prevention(
        &self,
    ) -> &crate::facade::BridgeWritebackLoopPreventionReport {
        &self.rebuilt_loop_prevention
    }

    pub(in crate::harness::adapter::adapter_impl) fn rebuilt_outcome(
        &self,
    ) -> &crate::facade::BridgeWritebackAuthorityOutcome {
        &self.rebuilt_outcome
    }

    pub(in crate::harness::adapter::adapter_impl) fn rebuilt_replay_bundle(
        &self,
    ) -> &crate::facade::BridgeWritebackReplayBundle {
        &self.rebuilt_replay_bundle
    }

    pub(in crate::harness::adapter::adapter_impl) fn rebuilt_receipt(
        &self,
    ) -> Option<&crate::adapter::TruthWritebackReceipt> {
        self.rebuilt_receipt.as_ref()
    }

    pub(in crate::harness::adapter::adapter_impl) fn rebuilt_authority_receipt_present(
        &self,
    ) -> bool {
        self.rebuilt_receipt.is_some()
    }

    pub(in crate::harness::adapter::adapter_impl) fn replay_equivalent_to_live_feedback(
        &self,
    ) -> bool {
        self.replay_equivalent_to_live_feedback
    }
}

impl FeedbackBoundednessProof {
    pub(in crate::harness::adapter::adapter_impl) fn authoritative_commit_count(&self) -> usize {
        self.authoritative_commit_count
    }

    pub(in crate::harness::adapter::adapter_impl) fn replayed_feedback_outcome_class(
        &self,
    ) -> crate::writeback::BridgeWritebackOutcomeClass {
        self.replayed_feedback_outcome_class
    }

    pub(in crate::harness::adapter::adapter_impl) fn changed_effect_retrigger_failure_kind(
        &self,
    ) -> crate::facade::BridgeWritebackErrorKind {
        self.changed_effect_retrigger_failure_kind
    }

    pub(in crate::harness::adapter::adapter_impl) fn feedback_publication_routed(&self) -> bool {
        self.feedback_publication_routed
    }

    pub(in crate::harness::adapter::adapter_impl) fn ordinary_truth_interleaved(&self) -> bool {
        self.ordinary_truth_interleaved
    }

    pub(in crate::harness::adapter::adapter_impl) fn feedback_converged(&self) -> bool {
        self.feedback_converged
    }

    pub(in crate::harness::adapter::adapter_impl) fn restart_replay_converged(&self) -> bool {
        self.restart_replay_converged
    }

    pub(in crate::harness::adapter::adapter_impl) fn replayed_authority_receipt_present(
        &self,
    ) -> bool {
        self.replayed_authority_receipt_present
    }
}

impl WritebackFeedbackLoopMatrix {
    pub(in crate::harness::adapter::adapter_impl) fn restart_replay_matrix(
        &self,
    ) -> &FeedbackRestartReplayMatrix {
        &self.restart_replay_matrix
    }
}
