use crate::writeback::{
    BridgeWritebackIdempotenceBasis, BridgeWritebackIdempotenceClass,
    BridgeWritebackLoopDisposition, BridgeWritebackLoopPreventionReport,
    BridgeWritebackOutcomeClass, BridgeWritebackReplayBundle, BridgeWritebackRetryDisposition,
    BridgeWritebackStrategyClass, BridgeWritebackStrategyDescriptorBasis,
};

pub(in crate::harness::adapter::adapter_impl) struct DuplicateReplayBundleReport {
    replay_bundle: BridgeWritebackReplayBundle,
}

pub(in crate::harness::adapter::adapter_impl) struct DuplicateIdempotenceReport {
    first_digest: String,
    repeated_digest: String,
    idempotence_class: BridgeWritebackIdempotenceClass,
    authoritative_state_before: String,
    authoritative_state_after_first_commit: String,
    lowered_policy_digest: String,
    strategy_descriptor_basis: BridgeWritebackStrategyDescriptorBasis,
}

pub(in crate::harness::adapter::adapter_impl) struct DuplicateLoopPreventionReport {
    first_loop_prevention: BridgeWritebackLoopPreventionReport,
    repeated_loop_prevention: BridgeWritebackLoopPreventionReport,
}

impl DuplicateReplayBundleReport {
    pub(super) fn from_replay_bundle(replay_bundle: &BridgeWritebackReplayBundle) -> Self {
        Self {
            replay_bundle: replay_bundle.clone(),
        }
    }

    pub(in crate::harness::adapter::adapter_impl) fn digest(&self) -> &str {
        self.replay_bundle.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn semantic_digest(&self) -> &str {
        self.replay_bundle.semantic_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn replay_bundle(
        &self,
    ) -> &BridgeWritebackReplayBundle {
        &self.replay_bundle
    }

    pub(in crate::harness::adapter::adapter_impl) fn effect_intent_digest(&self) -> &str {
        self.replay_bundle.effect_intent_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn effect_intent_patch_canonical_basis(
        &self,
    ) -> &str {
        self.replay_bundle.effect_intent_patch_canonical_basis()
    }

    pub(in crate::harness::adapter::adapter_impl) fn strategy_class(
        &self,
    ) -> BridgeWritebackStrategyClass {
        self.replay_bundle.strategy_class()
    }

    pub(in crate::harness::adapter::adapter_impl) fn strategy_descriptor_digest(&self) -> &str {
        self.replay_bundle.strategy_descriptor_basis().digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn causality_digest(&self) -> &str {
        self.replay_bundle.causality_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn lowered_policy_digest(&self) -> &str {
        self.replay_bundle.lowered_policy_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn retry_disposition(
        &self,
    ) -> BridgeWritebackRetryDisposition {
        self.replay_bundle.retry_disposition()
    }

    pub(in crate::harness::adapter::adapter_impl) fn outcome_class(
        &self,
    ) -> BridgeWritebackOutcomeClass {
        self.replay_bundle.outcome_class()
    }

    pub(in crate::harness::adapter::adapter_impl) fn authoritative_artifact_digest(&self) -> &str {
        self.replay_bundle.authoritative_artifact_digest()
    }
}

impl DuplicateIdempotenceReport {
    pub(super) fn from_idempotence_attempts(
        first_idempotence: &BridgeWritebackIdempotenceBasis,
        repeated_idempotence: &BridgeWritebackIdempotenceBasis,
    ) -> Self {
        Self {
            first_digest: first_idempotence.digest().to_owned(),
            repeated_digest: repeated_idempotence.digest().to_owned(),
            idempotence_class: first_idempotence.idempotence_class(),
            authoritative_state_before: first_idempotence.authoritative_state_digest().to_owned(),
            authoritative_state_after_first_commit: repeated_idempotence
                .authoritative_state_digest()
                .to_owned(),
            lowered_policy_digest: first_idempotence.lowered_policy_digest().to_owned(),
            strategy_descriptor_basis: first_idempotence.strategy_descriptor_basis().clone(),
        }
    }

    pub(in crate::harness::adapter::adapter_impl) fn first_digest(&self) -> &str {
        &self.first_digest
    }

    pub(in crate::harness::adapter::adapter_impl) fn repeated_digest(&self) -> &str {
        &self.repeated_digest
    }

    pub(in crate::harness::adapter::adapter_impl) fn idempotence_class(
        &self,
    ) -> BridgeWritebackIdempotenceClass {
        self.idempotence_class
    }

    pub(in crate::harness::adapter::adapter_impl) fn authoritative_state_before(&self) -> &str {
        &self.authoritative_state_before
    }

    pub(in crate::harness::adapter::adapter_impl) fn authoritative_state_after_first_commit(
        &self,
    ) -> &str {
        &self.authoritative_state_after_first_commit
    }

    pub(in crate::harness::adapter::adapter_impl) fn lowered_policy_digest(&self) -> &str {
        &self.lowered_policy_digest
    }

    pub(in crate::harness::adapter::adapter_impl) fn strategy_descriptor_digest(&self) -> &str {
        self.strategy_descriptor_basis.digest()
    }
}

impl DuplicateLoopPreventionReport {
    pub(super) fn from_loop_prevention_attempts(
        first_loop_prevention: &BridgeWritebackLoopPreventionReport,
        repeated_loop_prevention: &BridgeWritebackLoopPreventionReport,
    ) -> Self {
        Self {
            first_loop_prevention: first_loop_prevention.clone(),
            repeated_loop_prevention: repeated_loop_prevention.clone(),
        }
    }

    pub(in crate::harness::adapter::adapter_impl) fn first_loop_prevention(
        &self,
    ) -> &BridgeWritebackLoopPreventionReport {
        &self.first_loop_prevention
    }

    pub(in crate::harness::adapter::adapter_impl) fn repeated_loop_prevention(
        &self,
    ) -> &BridgeWritebackLoopPreventionReport {
        &self.repeated_loop_prevention
    }

    pub(in crate::harness::adapter::adapter_impl) fn first_digest(&self) -> &str {
        self.first_loop_prevention.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn first_disposition(
        &self,
    ) -> BridgeWritebackLoopDisposition {
        self.first_loop_prevention.disposition()
    }

    pub(in crate::harness::adapter::adapter_impl) fn repeated_digest(&self) -> &str {
        self.repeated_loop_prevention.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn repeated_disposition(
        &self,
    ) -> BridgeWritebackLoopDisposition {
        self.repeated_loop_prevention.disposition()
    }

    pub(in crate::harness::adapter::adapter_impl) fn current_feedback_provenance_digest(
        &self,
    ) -> &str {
        self.first_loop_prevention
            .current_feedback_provenance_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn current_causality_digest(&self) -> &str {
        self.first_loop_prevention.current_causality_digest()
    }
}
