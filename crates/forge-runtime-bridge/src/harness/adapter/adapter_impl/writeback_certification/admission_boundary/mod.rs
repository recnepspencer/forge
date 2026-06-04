use crate::facade::{
    BridgeWritebackAuthorityOutcome, BridgeWritebackError, BridgeWritebackIdempotenceBasis,
    BridgeWritebackReplayBundle,
};
use crate::routing::canonicalization::digest_string;
use crate::writeback::{
    AdmittedBridgeWritebackContract, BridgeDerivedWritebackEffect, BridgeWritebackEffectClass,
};

mod authority_boundary;

pub(in crate::harness::adapter::adapter_impl) use authority_boundary::{
    AdmissionBoundaryAuthorityProof, AdmissionBoundaryShadowProtocolRejection,
};

pub(in crate::harness::adapter::adapter_impl) struct WritebackAdmissionBoundaryMatrixEvidence<'a> {
    pub projected_contract: &'a AdmittedBridgeWritebackContract,
    pub aspect_contract: &'a AdmittedBridgeWritebackContract,
    pub projected_admission_record_digest: &'a str,
    pub aspect_admission_record_digest: &'a str,
    pub projected_effect: &'a BridgeDerivedWritebackEffect,
    pub aspect_effect: &'a BridgeDerivedWritebackEffect,
    pub projected_idempotence: &'a BridgeWritebackIdempotenceBasis,
    pub aspect_idempotence: &'a BridgeWritebackIdempotenceBasis,
    pub projected_bundle: &'a BridgeWritebackReplayBundle,
    pub aspect_bundle: &'a BridgeWritebackReplayBundle,
    pub projected_authority_outcome: &'a BridgeWritebackAuthorityOutcome,
    pub aspect_authority_outcome: &'a BridgeWritebackAuthorityOutcome,
    pub shadow_protocol_error: &'a BridgeWritebackError,
}

pub(in crate::harness::adapter::adapter_impl) struct WritebackAdmissionBoundaryMatrix {
    projected_family: AdmissionBoundaryFamilyEvidence,
    aspect_family: AdmissionBoundaryFamilyEvidence,
    family_admission_proof: AdmissionBoundaryFamilyAdmissionProof,
    authority_boundary_proof: AdmissionBoundaryAuthorityProof,
    shadow_protocol_rejection: AdmissionBoundaryShadowProtocolRejection,
}

pub(in crate::harness::adapter::adapter_impl) struct AdmissionBoundaryFamilyEvidence {
    admission_record_digest: String,
    contract: AdmittedBridgeWritebackContract,
    effect: BridgeDerivedWritebackEffect,
    idempotence: BridgeWritebackIdempotenceBasis,
    replay_bundle: BridgeWritebackReplayBundle,
}

pub(in crate::harness::adapter::adapter_impl) struct AdmissionBoundaryFamilyAdmissionProof {
    projected_family_admitted: bool,
    aspect_family_admitted: bool,
    projected_admission_record_digest: String,
    aspect_admission_record_digest: String,
    projected_contract: AdmittedBridgeWritebackContract,
    aspect_contract: AdmittedBridgeWritebackContract,
    family_digest_separated: bool,
    projected_strategy_matches_family: bool,
    aspect_strategy_matches_family: bool,
    decision_trace_digest: String,
}

impl WritebackAdmissionBoundaryMatrix {
    pub(in crate::harness::adapter::adapter_impl) fn from_admission_boundary_evidence(
        evidence: WritebackAdmissionBoundaryMatrixEvidence<'_>,
    ) -> Self {
        let projected_family = AdmissionBoundaryFamilyEvidence::from_family_evidence(
            evidence.projected_contract,
            evidence.projected_admission_record_digest,
            evidence.projected_effect,
            evidence.projected_idempotence,
            evidence.projected_bundle,
        );
        let aspect_family = AdmissionBoundaryFamilyEvidence::from_family_evidence(
            evidence.aspect_contract,
            evidence.aspect_admission_record_digest,
            evidence.aspect_effect,
            evidence.aspect_idempotence,
            evidence.aspect_bundle,
        );
        let failure_digest = digest_string(
            "bridge-writeback-family-admission-boundary-shadow-protocol",
            &evidence.shadow_protocol_error.to_string(),
        )
        .to_string();
        let shadow_error_kind = evidence.shadow_protocol_error.kind();
        Self {
            family_admission_proof: AdmissionBoundaryFamilyAdmissionProof::from_family_evidence(
                &projected_family,
                &aspect_family,
                evidence.projected_effect,
                evidence.aspect_effect,
            ),
            authority_boundary_proof: AdmissionBoundaryAuthorityProof::from_authority_evidence(
                evidence.projected_authority_outcome,
                evidence.aspect_authority_outcome,
                shadow_error_kind,
                &failure_digest,
            ),
            shadow_protocol_rejection:
                AdmissionBoundaryShadowProtocolRejection::from_shadow_protocol_error(
                    shadow_error_kind,
                    failure_digest,
                    evidence.projected_admission_record_digest,
                    evidence.aspect_admission_record_digest,
                ),
            projected_family,
            aspect_family,
        }
    }

    pub(in crate::harness::adapter::adapter_impl) fn projected_family(
        &self,
    ) -> &AdmissionBoundaryFamilyEvidence {
        &self.projected_family
    }

    pub(in crate::harness::adapter::adapter_impl) fn aspect_family(
        &self,
    ) -> &AdmissionBoundaryFamilyEvidence {
        &self.aspect_family
    }

    pub(in crate::harness::adapter::adapter_impl) fn family_admission_proof(
        &self,
    ) -> &AdmissionBoundaryFamilyAdmissionProof {
        &self.family_admission_proof
    }

    pub(in crate::harness::adapter::adapter_impl) fn authority_boundary_proof(
        &self,
    ) -> &AdmissionBoundaryAuthorityProof {
        &self.authority_boundary_proof
    }

    pub(in crate::harness::adapter::adapter_impl) fn shadow_protocol_rejection(
        &self,
    ) -> &AdmissionBoundaryShadowProtocolRejection {
        &self.shadow_protocol_rejection
    }
}

impl AdmissionBoundaryFamilyEvidence {
    fn from_family_evidence(
        contract: &AdmittedBridgeWritebackContract,
        admission_record_digest: &str,
        effect: &BridgeDerivedWritebackEffect,
        idempotence: &BridgeWritebackIdempotenceBasis,
        bundle: &BridgeWritebackReplayBundle,
    ) -> Self {
        Self {
            admission_record_digest: admission_record_digest.to_owned(),
            contract: contract.clone(),
            effect: effect.clone(),
            idempotence: idempotence.clone(),
            replay_bundle: bundle.clone(),
        }
    }

    pub(in crate::harness::adapter::adapter_impl) fn admission_record_digest(&self) -> &str {
        &self.admission_record_digest
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
}

impl AdmissionBoundaryFamilyAdmissionProof {
    fn from_family_evidence(
        projected_family: &AdmissionBoundaryFamilyEvidence,
        aspect_family: &AdmissionBoundaryFamilyEvidence,
        projected_effect: &BridgeDerivedWritebackEffect,
        aspect_effect: &BridgeDerivedWritebackEffect,
    ) -> Self {
        Self {
            projected_family_admitted: !projected_family.admission_record_digest().is_empty(),
            aspect_family_admitted: !aspect_family.admission_record_digest().is_empty(),
            projected_admission_record_digest: projected_family
                .admission_record_digest()
                .to_owned(),
            aspect_admission_record_digest: aspect_family.admission_record_digest().to_owned(),
            projected_contract: projected_family.contract().clone(),
            aspect_contract: aspect_family.contract().clone(),
            family_digest_separated: projected_family.contract_digest()
                != aspect_family.contract_digest(),
            projected_strategy_matches_family: projected_effect.effect_class()
                == BridgeWritebackEffectClass::ProjectedStateDiff,
            aspect_strategy_matches_family: aspect_effect.effect_class()
                == BridgeWritebackEffectClass::AspectReconciliation,
            decision_trace_digest: digest_string(
                "bridge-writeback-family-admission-boundary-trace",
                &format!(
                    "projected-admission={}|aspect-admission={}|projected-contract={}|aspect-contract={}",
                    projected_family.admission_record_digest(),
                    aspect_family.admission_record_digest(),
                    projected_family.contract_digest(),
                    aspect_family.contract_digest(),
                ),
            )
            .to_string(),
        }
    }

    pub(in crate::harness::adapter::adapter_impl) fn projected_family_admitted(&self) -> bool {
        self.projected_family_admitted
    }

    pub(in crate::harness::adapter::adapter_impl) fn aspect_family_admitted(&self) -> bool {
        self.aspect_family_admitted
    }

    pub(in crate::harness::adapter::adapter_impl) fn projected_admission_record_digest(
        &self,
    ) -> &str {
        &self.projected_admission_record_digest
    }

    pub(in crate::harness::adapter::adapter_impl) fn aspect_admission_record_digest(&self) -> &str {
        &self.aspect_admission_record_digest
    }

    pub(in crate::harness::adapter::adapter_impl) fn projected_contract_digest(&self) -> &str {
        self.projected_contract.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn aspect_contract_digest(&self) -> &str {
        self.aspect_contract.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn projected_contract(
        &self,
    ) -> &AdmittedBridgeWritebackContract {
        &self.projected_contract
    }

    pub(in crate::harness::adapter::adapter_impl) fn aspect_contract(
        &self,
    ) -> &AdmittedBridgeWritebackContract {
        &self.aspect_contract
    }

    pub(in crate::harness::adapter::adapter_impl) fn family_digest_separated(&self) -> bool {
        self.family_digest_separated
    }

    pub(in crate::harness::adapter::adapter_impl) fn projected_strategy_matches_family(
        &self,
    ) -> bool {
        self.projected_strategy_matches_family
    }

    pub(in crate::harness::adapter::adapter_impl) fn aspect_strategy_matches_family(&self) -> bool {
        self.aspect_strategy_matches_family
    }

    pub(in crate::harness::adapter::adapter_impl) fn decision_trace_digest(&self) -> &str {
        &self.decision_trace_digest
    }
}
