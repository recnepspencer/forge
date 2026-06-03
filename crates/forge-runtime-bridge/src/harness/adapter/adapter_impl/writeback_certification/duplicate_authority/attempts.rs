use crate::adapter::TruthWritebackReceipt;
use crate::writeback::{
    BridgeWritebackAuthorityOutcome, BridgeWritebackIdempotenceBasis, BridgeWritebackOutcomeClass,
    BridgeWritebackReplayBundle,
};

pub(in crate::harness::adapter::adapter_impl) struct DuplicateAttemptReport {
    idempotence: BridgeWritebackIdempotenceBasis,
    outcome: BridgeWritebackAuthorityOutcome,
    replay_bundle: BridgeWritebackReplayBundle,
    receipt: TruthWritebackReceipt,
}

pub(in crate::harness::adapter::adapter_impl) struct DuplicateBoundednessProof {
    authoritative_commit_count: usize,
    canonical_noop_count: usize,
    duplicate_causality_detected: bool,
    loop_converged: bool,
}

impl DuplicateAttemptReport {
    pub(super) fn from_attempt(
        idempotence: &BridgeWritebackIdempotenceBasis,
        outcome: &BridgeWritebackAuthorityOutcome,
        replay_bundle: &BridgeWritebackReplayBundle,
        receipt: &TruthWritebackReceipt,
    ) -> Self {
        Self {
            idempotence: idempotence.clone(),
            outcome: outcome.clone(),
            replay_bundle: replay_bundle.clone(),
            receipt: receipt.clone(),
        }
    }

    pub(in crate::harness::adapter::adapter_impl) fn idempotence(
        &self,
    ) -> &BridgeWritebackIdempotenceBasis {
        &self.idempotence
    }

    pub(in crate::harness::adapter::adapter_impl) fn outcome(
        &self,
    ) -> &BridgeWritebackAuthorityOutcome {
        &self.outcome
    }

    pub(in crate::harness::adapter::adapter_impl) fn replay_bundle(
        &self,
    ) -> &BridgeWritebackReplayBundle {
        &self.replay_bundle
    }

    pub(in crate::harness::adapter::adapter_impl) fn receipt(&self) -> &TruthWritebackReceipt {
        &self.receipt
    }

    pub(in crate::harness::adapter::adapter_impl) fn idempotence_digest(&self) -> &str {
        self.idempotence.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn outcome_digest(&self) -> &str {
        self.outcome.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn replay_bundle_digest(&self) -> &str {
        self.replay_bundle.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn outcome_class(
        &self,
    ) -> BridgeWritebackOutcomeClass {
        self.receipt.outcome_class()
    }
}

impl DuplicateBoundednessProof {
    pub(super) fn new(authoritative_commit_count: usize, canonical_noop_count: usize) -> Self {
        Self {
            authoritative_commit_count,
            canonical_noop_count,
            duplicate_causality_detected: true,
            loop_converged: authoritative_commit_count == 1 && canonical_noop_count == 1,
        }
    }

    pub(in crate::harness::adapter::adapter_impl) fn authoritative_commit_count(&self) -> usize {
        self.authoritative_commit_count
    }

    pub(in crate::harness::adapter::adapter_impl) fn canonical_noop_count(&self) -> usize {
        self.canonical_noop_count
    }

    pub(in crate::harness::adapter::adapter_impl) fn duplicate_causality_detected(&self) -> bool {
        self.duplicate_causality_detected
    }

    pub(in crate::harness::adapter::adapter_impl) fn loop_converged(&self) -> bool {
        self.loop_converged
    }
}
