mod closeout_readiness;
mod digest_basis;
mod support_evidence;

use closeout_readiness::{
    standard_deferred_boundaries, standard_ready_capabilities, standard_verification_gates,
};
use digest_basis::{closeout_digest_from_typed_evidence, support_digest_from_typed_evidence};
use support_evidence::{
    standard_aggregate_evidence_digests, standard_carry_forward_sections,
    standard_continuity_mutation_families, standard_existing_truth_binding_families,
    standard_naming_mutation_families, standard_symbolic_target_reference_families,
};

pub use closeout_readiness::{
    BridgeAuthorityEvidenceDeferredBoundary, BridgeAuthorityEvidenceReadyCapability,
    BridgeAuthorityEvidenceVerificationGate,
};
pub use support_evidence::{
    BridgeAggregateMutationEvidenceDigest, BridgeMutationEvidenceCarryForwardSection,
    BridgeMutationEvidenceContinuityFamily, BridgeMutationEvidenceExistingTruthBindingFamily,
    BridgeMutationEvidenceNamingFamily, BridgeMutationEvidenceSymbolicTargetReferenceFamily,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeAuthoritativeMutationEvidenceSupport {
    carry_forward_sections: Vec<BridgeMutationEvidenceCarryForwardSection>,
    existing_truth_binding_families: Vec<BridgeMutationEvidenceExistingTruthBindingFamily>,
    symbolic_target_reference_families: Vec<BridgeMutationEvidenceSymbolicTargetReferenceFamily>,
    naming_mutation_families: Vec<BridgeMutationEvidenceNamingFamily>,
    continuity_mutation_families: Vec<BridgeMutationEvidenceContinuityFamily>,
    aggregate_evidence_digests: Vec<BridgeAggregateMutationEvidenceDigest>,
    support_digest: String,
}

impl BridgeAuthoritativeMutationEvidenceSupport {
    pub fn standard() -> Self {
        let carry_forward_sections = standard_carry_forward_sections();
        let existing_truth_binding_families = standard_existing_truth_binding_families();
        let symbolic_target_reference_families = standard_symbolic_target_reference_families();
        let naming_mutation_families = standard_naming_mutation_families();
        let continuity_mutation_families = standard_continuity_mutation_families();
        let aggregate_evidence_digests = standard_aggregate_evidence_digests();
        let support_digest = support_digest_from_typed_evidence(
            &carry_forward_sections,
            &existing_truth_binding_families,
            &symbolic_target_reference_families,
            &naming_mutation_families,
            &continuity_mutation_families,
            &aggregate_evidence_digests,
        );
        Self {
            carry_forward_sections,
            existing_truth_binding_families,
            symbolic_target_reference_families,
            naming_mutation_families,
            continuity_mutation_families,
            aggregate_evidence_digests,
            support_digest,
        }
    }

    pub fn carry_forward_sections(&self) -> &[BridgeMutationEvidenceCarryForwardSection] {
        &self.carry_forward_sections
    }

    pub fn existing_truth_binding_families(
        &self,
    ) -> &[BridgeMutationEvidenceExistingTruthBindingFamily] {
        &self.existing_truth_binding_families
    }

    pub fn symbolic_target_reference_families(
        &self,
    ) -> &[BridgeMutationEvidenceSymbolicTargetReferenceFamily] {
        &self.symbolic_target_reference_families
    }

    pub fn naming_mutation_families(&self) -> &[BridgeMutationEvidenceNamingFamily] {
        &self.naming_mutation_families
    }

    pub fn continuity_mutation_families(&self) -> &[BridgeMutationEvidenceContinuityFamily] {
        &self.continuity_mutation_families
    }

    pub fn aggregate_evidence_digests(&self) -> &[BridgeAggregateMutationEvidenceDigest] {
        &self.aggregate_evidence_digests
    }

    pub fn support_digest(&self) -> &str {
        &self.support_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeAuthoritativeMutationEvidenceCloseout {
    support_digest: String,
    ready_capabilities: Vec<BridgeAuthorityEvidenceReadyCapability>,
    deferred_boundaries: Vec<BridgeAuthorityEvidenceDeferredBoundary>,
    verification_gates: Vec<BridgeAuthorityEvidenceVerificationGate>,
    closeout_digest: String,
}

impl BridgeAuthoritativeMutationEvidenceCloseout {
    pub fn derive(support: &BridgeAuthoritativeMutationEvidenceSupport) -> Self {
        let ready_capabilities = standard_ready_capabilities();
        let deferred_boundaries = standard_deferred_boundaries();
        let verification_gates = standard_verification_gates();
        let closeout_digest = closeout_digest_from_typed_evidence(
            support.support_digest(),
            &ready_capabilities,
            &deferred_boundaries,
            &verification_gates,
        );
        Self {
            support_digest: support.support_digest().to_string(),
            ready_capabilities,
            deferred_boundaries,
            verification_gates,
            closeout_digest,
        }
    }

    pub fn support_digest(&self) -> &str {
        &self.support_digest
    }

    pub fn ready_capabilities(&self) -> &[BridgeAuthorityEvidenceReadyCapability] {
        &self.ready_capabilities
    }

    pub fn deferred_boundaries(&self) -> &[BridgeAuthorityEvidenceDeferredBoundary] {
        &self.deferred_boundaries
    }

    pub fn verification_gates(&self) -> &[BridgeAuthorityEvidenceVerificationGate] {
        &self.verification_gates
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }
}
