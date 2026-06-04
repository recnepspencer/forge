mod attempts;
mod authority_boundary;
mod replay_and_idempotence;

use crate::adapter::{TruthWritebackReceipt, TruthWritebackRequest};
use crate::writeback::{
    AdmittedBridgeWritebackContract, BridgeDerivedWritebackEffect,
    BridgeValidatedWritebackCandidate, BridgeWritebackAuthorityOutcome,
    BridgeWritebackIdempotenceBasis, BridgeWritebackLoopPreventionReport,
    BridgeWritebackNativeCausalityInputs, BridgeWritebackReplayBundle,
    BridgeWritebackStrategyCoherenceReport,
};

pub(in crate::harness::adapter::adapter_impl) use attempts::{
    DuplicateAttemptReport, DuplicateBoundednessProof,
};
pub(in crate::harness::adapter::adapter_impl) use authority_boundary::{
    DuplicateAuthorityBoundaryEvidence, DuplicateAuthorityBoundaryMatrix,
};
pub(in crate::harness::adapter::adapter_impl) use replay_and_idempotence::{
    DuplicateIdempotenceReport, DuplicateLoopPreventionReport, DuplicateReplayBundleReport,
};

pub(in crate::harness::adapter::adapter_impl) struct WritebackDuplicateAuthorityMatrixEvidence<'a> {
    pub(in crate::harness::adapter::adapter_impl) contract: &'a AdmittedBridgeWritebackContract,
    pub(in crate::harness::adapter::adapter_impl) effect: &'a BridgeDerivedWritebackEffect,
    pub(in crate::harness::adapter::adapter_impl) causality:
        &'a BridgeWritebackNativeCausalityInputs,
    pub(in crate::harness::adapter::adapter_impl) replay_bundle: &'a BridgeWritebackReplayBundle,
    pub(in crate::harness::adapter::adapter_impl) first_bundle: &'a BridgeWritebackReplayBundle,
    pub(in crate::harness::adapter::adapter_impl) repeated_bundle: &'a BridgeWritebackReplayBundle,
    pub(in crate::harness::adapter::adapter_impl) first_idempotence:
        &'a BridgeWritebackIdempotenceBasis,
    pub(in crate::harness::adapter::adapter_impl) repeated_idempotence:
        &'a BridgeWritebackIdempotenceBasis,
    pub(in crate::harness::adapter::adapter_impl) first_loop_prevention:
        &'a BridgeWritebackLoopPreventionReport,
    pub(in crate::harness::adapter::adapter_impl) repeated_loop_prevention:
        &'a BridgeWritebackLoopPreventionReport,
    pub(in crate::harness::adapter::adapter_impl) first_strategy_coherence:
        &'a BridgeWritebackStrategyCoherenceReport,
    pub(in crate::harness::adapter::adapter_impl) repeated_strategy_coherence:
        &'a BridgeWritebackStrategyCoherenceReport,
    pub(in crate::harness::adapter::adapter_impl) first_candidate:
        &'a BridgeValidatedWritebackCandidate,
    pub(in crate::harness::adapter::adapter_impl) repeated_candidate:
        &'a BridgeValidatedWritebackCandidate,
    pub(in crate::harness::adapter::adapter_impl) first_authority_request:
        &'a TruthWritebackRequest,
    pub(in crate::harness::adapter::adapter_impl) repeated_authority_request:
        &'a TruthWritebackRequest,
    pub(in crate::harness::adapter::adapter_impl) first_receipt: &'a TruthWritebackReceipt,
    pub(in crate::harness::adapter::adapter_impl) repeated_receipt: &'a TruthWritebackReceipt,
    pub(in crate::harness::adapter::adapter_impl) first_outcome:
        &'a BridgeWritebackAuthorityOutcome,
    pub(in crate::harness::adapter::adapter_impl) repeated_outcome:
        &'a BridgeWritebackAuthorityOutcome,
    pub(in crate::harness::adapter::adapter_impl) commit_count: usize,
    pub(in crate::harness::adapter::adapter_impl) noop_count: usize,
}

pub(in crate::harness::adapter::adapter_impl) struct WritebackDuplicateAuthorityMatrix {
    writeback_digest: String,
    effect: BridgeDerivedWritebackEffect,
    causality: BridgeWritebackNativeCausalityInputs,
    replay_digest: String,
    mutation_plan_digest: String,
    replay_bundle_report: DuplicateReplayBundleReport,
    idempotence_report: DuplicateIdempotenceReport,
    loop_prevention_report: DuplicateLoopPreventionReport,
    authority_boundary_matrix: DuplicateAuthorityBoundaryMatrix,
    first_attempt: DuplicateAttemptReport,
    repeated_attempt: DuplicateAttemptReport,
    boundedness_proof: DuplicateBoundednessProof,
}

impl WritebackDuplicateAuthorityMatrix {
    pub(in crate::harness::adapter::adapter_impl) fn from_duplicate_attempts(
        evidence: WritebackDuplicateAuthorityMatrixEvidence<'_>,
    ) -> Self {
        let strategy_basis = evidence
            .contract
            .validated_declaration()
            .strategy_basis()
            .expect("admitted writeback contract should preserve strategy basis");
        Self {
            writeback_digest: evidence.repeated_bundle.digest().to_owned(),
            effect: evidence.effect.clone(),
            causality: evidence.causality.clone(),
            replay_digest: evidence.replay_bundle.digest().to_owned(),
            mutation_plan_digest: evidence
                .first_receipt
                .authoritative_artifact_digest()
                .to_owned(),
            replay_bundle_report: DuplicateReplayBundleReport::from_replay_bundle(
                evidence.replay_bundle,
            ),
            idempotence_report: DuplicateIdempotenceReport::from_idempotence_attempts(
                evidence.first_idempotence,
                evidence.repeated_idempotence,
            ),
            loop_prevention_report: DuplicateLoopPreventionReport::from_loop_prevention_attempts(
                evidence.first_loop_prevention,
                evidence.repeated_loop_prevention,
            ),
            authority_boundary_matrix:
                DuplicateAuthorityBoundaryMatrix::from_authority_boundary_evidence(
                    DuplicateAuthorityBoundaryEvidence {
                        contract: evidence.contract,
                        strategy_basis,
                        first_strategy_coherence: evidence.first_strategy_coherence,
                        repeated_strategy_coherence: evidence.repeated_strategy_coherence,
                        first_candidate: evidence.first_candidate,
                        repeated_candidate: evidence.repeated_candidate,
                        first_authority_request: evidence.first_authority_request,
                        repeated_authority_request: evidence.repeated_authority_request,
                        first_receipt: evidence.first_receipt,
                        repeated_receipt: evidence.repeated_receipt,
                    },
                ),
            first_attempt: DuplicateAttemptReport::from_attempt(
                evidence.first_idempotence,
                evidence.first_outcome,
                evidence.first_bundle,
                evidence.first_receipt,
            ),
            repeated_attempt: DuplicateAttemptReport::from_attempt(
                evidence.repeated_idempotence,
                evidence.repeated_outcome,
                evidence.repeated_bundle,
                evidence.repeated_receipt,
            ),
            boundedness_proof: DuplicateBoundednessProof::new(
                evidence.commit_count,
                evidence.noop_count,
            ),
        }
    }

    pub(in crate::harness::adapter::adapter_impl) fn writeback_digest(&self) -> &str {
        &self.writeback_digest
    }

    pub(in crate::harness::adapter::adapter_impl) fn writeback_effect_artifact_digest(
        &self,
    ) -> &str {
        self.effect.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn effect(
        &self,
    ) -> &BridgeDerivedWritebackEffect {
        &self.effect
    }

    pub(in crate::harness::adapter::adapter_impl) fn effect_intent_digest(&self) -> &str {
        self.effect.effect_intent_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn effect_intent_patch_canonical_basis(
        &self,
    ) -> &str {
        self.effect.effect_intent().patch_canonical_basis()
    }

    pub(in crate::harness::adapter::adapter_impl) fn causality(
        &self,
    ) -> &BridgeWritebackNativeCausalityInputs {
        &self.causality
    }

    pub(in crate::harness::adapter::adapter_impl) fn causality_digest(&self) -> &str {
        self.causality.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn replay_digest(&self) -> &str {
        &self.replay_digest
    }

    pub(in crate::harness::adapter::adapter_impl) fn mutation_plan_digest(&self) -> &str {
        &self.mutation_plan_digest
    }

    pub(in crate::harness::adapter::adapter_impl) fn replay_bundle_report(
        &self,
    ) -> &DuplicateReplayBundleReport {
        &self.replay_bundle_report
    }

    pub(in crate::harness::adapter::adapter_impl) fn idempotence_report(
        &self,
    ) -> &DuplicateIdempotenceReport {
        &self.idempotence_report
    }

    pub(in crate::harness::adapter::adapter_impl) fn loop_prevention_report(
        &self,
    ) -> &DuplicateLoopPreventionReport {
        &self.loop_prevention_report
    }

    pub(in crate::harness::adapter::adapter_impl) fn authority_boundary_matrix(
        &self,
    ) -> &DuplicateAuthorityBoundaryMatrix {
        &self.authority_boundary_matrix
    }

    pub(in crate::harness::adapter::adapter_impl) fn truth_trigger_digest(&self) -> &str {
        self.causality.truth_trigger_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn route_digest(&self) -> &str {
        self.causality.route_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn first_attempt(
        &self,
    ) -> &DuplicateAttemptReport {
        &self.first_attempt
    }

    pub(in crate::harness::adapter::adapter_impl) fn repeated_attempt(
        &self,
    ) -> &DuplicateAttemptReport {
        &self.repeated_attempt
    }

    pub(in crate::harness::adapter::adapter_impl) fn boundedness_proof(
        &self,
    ) -> &DuplicateBoundednessProof {
        &self.boundedness_proof
    }
}
