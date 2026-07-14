use crate::adapter::TruthWritebackReceipt;
use crate::facade::{
    BridgeWritebackError, BridgeWritebackFeedbackContext, BridgeWritebackIdempotenceBasis,
    BridgeWritebackLoopPreventionReport, BridgeWritebackReplayBundle,
};
use crate::writeback::{
    AdmittedBridgeWritebackContract, BridgeDerivedWritebackEffect, BridgeWritebackExecutionRecord,
    BridgeWritebackFamilyAdmissionRecord, BridgeWritebackReplayRecord,
};

mod mapper_and_shadow;
mod replay_isolation;

pub(in crate::harness::adapter::adapter_impl) use mapper_and_shadow::{
    FamilyExtensionMapperParityProof, FamilyExtensionShadowProtocolRejection,
};
pub(in crate::harness::adapter::adapter_impl) use replay_isolation::{
    FamilyExtensionChangedCausalityIsolation, FamilyExtensionCrossFamilyReplayIsolation,
    FamilyExtensionLoopIsolation, FamilyExtensionSameFamilyEquivalence,
};

pub(in crate::harness::adapter::adapter_impl) struct WritebackFamilyExtensionMatrixEvidence<'a> {
    pub projected_contract: &'a AdmittedBridgeWritebackContract,
    pub aspect_contract: &'a AdmittedBridgeWritebackContract,
    pub projected_admission_record: &'a BridgeWritebackFamilyAdmissionRecord,
    pub aspect_admission_record: &'a BridgeWritebackFamilyAdmissionRecord,
    pub projected_effect: &'a BridgeDerivedWritebackEffect,
    pub aspect_effect: &'a BridgeDerivedWritebackEffect,
    pub projected_idempotence: &'a BridgeWritebackIdempotenceBasis,
    pub aspect_idempotence: &'a BridgeWritebackIdempotenceBasis,
    pub projected_bundle: &'a BridgeWritebackReplayBundle,
    pub aspect_bundle: &'a BridgeWritebackReplayBundle,
    pub projected_receipt: &'a TruthWritebackReceipt,
    pub aspect_receipt: &'a TruthWritebackReceipt,
    pub cross_family_replay_error: &'a BridgeWritebackError,
    pub cross_family_replay_record: &'a BridgeWritebackReplayRecord,
    pub rebuilt_projected_effect: &'a BridgeDerivedWritebackEffect,
    pub rebuilt_projected_bundle: &'a BridgeWritebackReplayBundle,
    pub rebuilt_execution_record: &'a BridgeWritebackExecutionRecord,
    pub changed_projected_bundle: &'a BridgeWritebackReplayBundle,
    pub same_family_drift_error: &'a BridgeWritebackError,
    pub same_family_drift_replay_record: &'a BridgeWritebackReplayRecord,
    pub projected_feedback_context: &'a BridgeWritebackFeedbackContext,
    pub cross_family_loop_prevention: &'a BridgeWritebackLoopPreventionReport,
    pub projected_mapper_envelope_retained: bool,
    pub aspect_mapper_envelope_retained: bool,
    pub projected_mapped_input_retained: bool,
    pub aspect_mapped_input_retained: bool,
    pub projected_execution_record: &'a BridgeWritebackExecutionRecord,
    pub aspect_execution_record: &'a BridgeWritebackExecutionRecord,
    pub shadow_protocol_error: &'a BridgeWritebackError,
}

pub(in crate::harness::adapter::adapter_impl) struct WritebackFamilyExtensionMatrix {
    projected_family: FamilyExtensionFamilyEvidence,
    aspect_family: FamilyExtensionFamilyEvidence,
    cross_family_replay_isolation: FamilyExtensionCrossFamilyReplayIsolation,
    same_family_equivalence: FamilyExtensionSameFamilyEquivalence,
    same_family_changed_causality: FamilyExtensionChangedCausalityIsolation,
    cross_family_loop_isolation: FamilyExtensionLoopIsolation,
    mapper_parity_proof: FamilyExtensionMapperParityProof,
    shadow_protocol_rejection: FamilyExtensionShadowProtocolRejection,
}

pub(in crate::harness::adapter::adapter_impl) struct FamilyExtensionFamilyEvidence {
    admission_record: BridgeWritebackFamilyAdmissionRecord,
    contract: AdmittedBridgeWritebackContract,
    effect: BridgeDerivedWritebackEffect,
    idempotence: BridgeWritebackIdempotenceBasis,
    replay_bundle: BridgeWritebackReplayBundle,
    authority_receipt: TruthWritebackReceipt,
}

impl WritebackFamilyExtensionMatrix {
    pub(in crate::harness::adapter::adapter_impl) fn from_family_extension_evidence(
        evidence: WritebackFamilyExtensionMatrixEvidence<'_>,
    ) -> Self {
        Self {
            projected_family: FamilyExtensionFamilyEvidence::from_family_evidence(
                evidence.projected_contract,
                evidence.projected_admission_record,
                evidence.projected_effect,
                evidence.projected_idempotence,
                evidence.projected_bundle,
                evidence.projected_receipt,
            ),
            aspect_family: FamilyExtensionFamilyEvidence::from_family_evidence(
                evidence.aspect_contract,
                evidence.aspect_admission_record,
                evidence.aspect_effect,
                evidence.aspect_idempotence,
                evidence.aspect_bundle,
                evidence.aspect_receipt,
            ),
            cross_family_replay_isolation:
                FamilyExtensionCrossFamilyReplayIsolation::from_replay_error(
                    evidence.projected_bundle,
                    evidence.aspect_bundle,
                    evidence.cross_family_replay_error,
                    evidence.cross_family_replay_record,
                ),
            same_family_equivalence: FamilyExtensionSameFamilyEquivalence::from_rebuilt_family(
                evidence.projected_effect,
                evidence.projected_bundle,
                evidence.rebuilt_projected_effect,
                evidence.rebuilt_projected_bundle,
                evidence.rebuilt_execution_record,
            ),
            same_family_changed_causality:
                FamilyExtensionChangedCausalityIsolation::from_changed_causality(
                    evidence.projected_bundle,
                    evidence.changed_projected_bundle,
                    evidence.same_family_drift_error,
                    evidence.same_family_drift_replay_record,
                ),
            cross_family_loop_isolation: FamilyExtensionLoopIsolation::from_loop_prevention(
                evidence.projected_feedback_context,
                evidence.cross_family_loop_prevention,
            ),
            mapper_parity_proof: FamilyExtensionMapperParityProof::from_mapper_evidence(&evidence),
            shadow_protocol_rejection: FamilyExtensionShadowProtocolRejection::from_shadow_error(
                evidence.shadow_protocol_error,
                evidence.projected_admission_record,
                evidence.aspect_admission_record,
            ),
        }
    }

    pub(in crate::harness::adapter::adapter_impl) fn projected_family(
        &self,
    ) -> &FamilyExtensionFamilyEvidence {
        &self.projected_family
    }

    pub(in crate::harness::adapter::adapter_impl) fn aspect_family(
        &self,
    ) -> &FamilyExtensionFamilyEvidence {
        &self.aspect_family
    }

    pub(in crate::harness::adapter::adapter_impl) fn cross_family_replay_isolation(
        &self,
    ) -> &FamilyExtensionCrossFamilyReplayIsolation {
        &self.cross_family_replay_isolation
    }

    pub(in crate::harness::adapter::adapter_impl) fn same_family_equivalence(
        &self,
    ) -> &FamilyExtensionSameFamilyEquivalence {
        &self.same_family_equivalence
    }

    pub(in crate::harness::adapter::adapter_impl) fn same_family_changed_causality(
        &self,
    ) -> &FamilyExtensionChangedCausalityIsolation {
        &self.same_family_changed_causality
    }

    pub(in crate::harness::adapter::adapter_impl) fn cross_family_loop_isolation(
        &self,
    ) -> &FamilyExtensionLoopIsolation {
        &self.cross_family_loop_isolation
    }

    pub(in crate::harness::adapter::adapter_impl) fn mapper_parity_proof(
        &self,
    ) -> &FamilyExtensionMapperParityProof {
        &self.mapper_parity_proof
    }

    pub(in crate::harness::adapter::adapter_impl) fn shadow_protocol_rejection(
        &self,
    ) -> &FamilyExtensionShadowProtocolRejection {
        &self.shadow_protocol_rejection
    }
}

impl FamilyExtensionFamilyEvidence {
    fn from_family_evidence(
        contract: &AdmittedBridgeWritebackContract,
        admission_record: &BridgeWritebackFamilyAdmissionRecord,
        effect: &BridgeDerivedWritebackEffect,
        idempotence: &BridgeWritebackIdempotenceBasis,
        bundle: &BridgeWritebackReplayBundle,
        receipt: &TruthWritebackReceipt,
    ) -> Self {
        Self {
            admission_record: admission_record.clone(),
            contract: contract.clone(),
            effect: effect.clone(),
            idempotence: idempotence.clone(),
            replay_bundle: bundle.clone(),
            authority_receipt: receipt.clone(),
        }
    }

    pub(in crate::harness::adapter::adapter_impl) fn admission_record_digest(&self) -> &str {
        self.admission_record.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn contract_digest(&self) -> &str {
        self.contract.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn writeback_effect_artifact_digest(
        &self,
    ) -> &str {
        self.effect.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn effect_intent_digest(&self) -> &str {
        self.effect.effect_intent_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn effect_intent_patch_canonical_basis(
        &self,
    ) -> &str {
        self.effect.effect_intent().patch_canonical_basis()
    }

    pub(in crate::harness::adapter::adapter_impl) fn mapped_input_digest(&self) -> &str {
        self.effect.mapped_input_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn mapper_envelope_digest(&self) -> &str {
        self.effect.mapper_envelope_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn causality_digest(&self) -> &str {
        self.effect.causality_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn idempotence_digest(&self) -> &str {
        self.idempotence.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn replay_bundle_digest(&self) -> &str {
        self.replay_bundle.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn replay_semantic_digest(&self) -> &str {
        self.replay_bundle.semantic_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn authority_receipt_digest(&self) -> &str {
        self.authority_receipt.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn contract(
        &self,
    ) -> &AdmittedBridgeWritebackContract {
        &self.contract
    }

    pub(in crate::harness::adapter::adapter_impl) fn effect(
        &self,
    ) -> &BridgeDerivedWritebackEffect {
        &self.effect
    }

    pub(in crate::harness::adapter::adapter_impl) fn idempotence(
        &self,
    ) -> &BridgeWritebackIdempotenceBasis {
        &self.idempotence
    }

    pub(in crate::harness::adapter::adapter_impl) fn replay_bundle(
        &self,
    ) -> &BridgeWritebackReplayBundle {
        &self.replay_bundle
    }

    pub(in crate::harness::adapter::adapter_impl) fn authority_receipt(
        &self,
    ) -> &TruthWritebackReceipt {
        &self.authority_receipt
    }
}
