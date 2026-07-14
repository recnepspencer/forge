use crate::application::{
    WorthQueryDeclarationAspectContract, WorthQueryDeclarationAspectCoverage,
    WorthQueryDeclarationAspectCoverageBasis, WorthQueryDeclarationAspectFit,
    WorthQueryDeclarationEnvelope, WorthQueryDeclarationEnvelopeEvidenceOrigin,
    WorthQueryDeclarationFutureProjection, WorthQueryDeclarationInput,
    WorthQueryDeclarationPrimaryAuthorityFamily, WorthQueryDeclarationReceiptDenialCause,
    WorthQueryDeclarationRoutePlanDenialCause, WorthQueryDomainEntryMarker,
};
use crate::basis_lifecycle::BasisFamily;

use super::{
    contract::WorthQueryDeclarationSignalExecutionFamily,
    explain::WorthQueryDeclarationSignalCompatibilityExplanation,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDeclarationSignalCompatibilityClass {
    Compatible,
    Deferred,
    Denied,
    Failed,
}

pub struct WorthQueryDeclarationSignalCompatibility<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    class: WorthQueryDeclarationSignalCompatibilityClass,
    primary_authority_family: WorthQueryDeclarationPrimaryAuthorityFamily,
    execution_family: WorthQueryDeclarationSignalExecutionFamily,
    basis_families: Vec<BasisFamily>,
    aspect_contract: WorthQueryDeclarationAspectContract,
    aspect_coverage: WorthQueryDeclarationAspectCoverage,
    aspect_coverage_basis: WorthQueryDeclarationAspectCoverageBasis,
    aspect_fit: WorthQueryDeclarationAspectFit,
    dependency_aspects: WorthQueryDeclarationAspectContract,
    produced_aspects: WorthQueryDeclarationAspectContract,
    future_projection: WorthQueryDeclarationFutureProjection,
    envelope: WorthQueryDeclarationEnvelope<D, I>,
    signal_compatibility_digest: String,
    explanation: WorthQueryDeclarationSignalCompatibilityExplanation,
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryDeclarationSignalCompatibility<D, I>
{
    pub(crate) fn new(
        primary_authority_family: WorthQueryDeclarationPrimaryAuthorityFamily,
        execution_family: WorthQueryDeclarationSignalExecutionFamily,
        basis_families: Vec<BasisFamily>,
        aspect_contract: WorthQueryDeclarationAspectContract,
        aspect_coverage: WorthQueryDeclarationAspectCoverage,
        aspect_coverage_basis: WorthQueryDeclarationAspectCoverageBasis,
        aspect_fit: WorthQueryDeclarationAspectFit,
        dependency_aspects: WorthQueryDeclarationAspectContract,
        produced_aspects: WorthQueryDeclarationAspectContract,
        future_projection: WorthQueryDeclarationFutureProjection,
        envelope: WorthQueryDeclarationEnvelope<D, I>,
        signal_compatibility_digest: String,
        explanation: WorthQueryDeclarationSignalCompatibilityExplanation,
    ) -> Self {
        Self {
            class: WorthQueryDeclarationSignalCompatibilityClass::Compatible,
            primary_authority_family,
            execution_family,
            basis_families,
            aspect_contract,
            aspect_coverage,
            aspect_coverage_basis,
            aspect_fit,
            dependency_aspects,
            produced_aspects,
            future_projection,
            envelope,
            signal_compatibility_digest,
            explanation,
        }
    }

    pub fn class(&self) -> WorthQueryDeclarationSignalCompatibilityClass {
        self.class
    }

    pub fn execution_family(&self) -> WorthQueryDeclarationSignalExecutionFamily {
        self.execution_family
    }

    pub fn primary_authority_family(&self) -> WorthQueryDeclarationPrimaryAuthorityFamily {
        self.primary_authority_family
    }

    pub fn basis_families(&self) -> &[BasisFamily] {
        &self.basis_families
    }

    pub fn aspect_contract(&self) -> &WorthQueryDeclarationAspectContract {
        &self.aspect_contract
    }

    pub fn aspect_coverage(&self) -> &WorthQueryDeclarationAspectCoverage {
        &self.aspect_coverage
    }

    pub fn aspect_coverage_basis(&self) -> WorthQueryDeclarationAspectCoverageBasis {
        self.aspect_coverage_basis
    }

    pub fn aspect_fit(&self) -> WorthQueryDeclarationAspectFit {
        self.aspect_fit
    }

    pub fn dependency_aspects(&self) -> &WorthQueryDeclarationAspectContract {
        &self.dependency_aspects
    }

    pub fn produced_aspects(&self) -> &WorthQueryDeclarationAspectContract {
        &self.produced_aspects
    }

    pub fn future_projection(&self) -> &WorthQueryDeclarationFutureProjection {
        &self.future_projection
    }

    pub fn declaration_family_key(&self) -> &'static str {
        self.envelope.declaration_family_key()
    }

    pub fn handle_identity_digest(&self) -> &str {
        self.envelope.handle_identity_digest()
    }

    pub fn operating_context_identity_digest(&self) -> &str {
        self.envelope.operating_context_identity_digest()
    }

    pub fn declaration_digest(&self) -> &str {
        self.envelope.declaration_digest()
    }

    pub fn progression_digest(&self) -> Option<&str> {
        self.envelope.progression_digest()
    }

    pub fn route_plan_digest(&self) -> Option<&str> {
        self.envelope.route_plan_digest()
    }

    pub fn receipt_digest(&self) -> &worth_foundational::facade::CanonicalDerivedDigest {
        self.envelope.receipt_digest()
    }

    pub fn envelope_digest(&self) -> &worth_foundational::facade::CanonicalDerivedDigest {
        self.envelope.envelope_digest()
    }

    pub fn signal_compatibility_digest(&self) -> &str {
        &self.signal_compatibility_digest
    }

    pub fn envelope(&self) -> &WorthQueryDeclarationEnvelope<D, I> {
        &self.envelope
    }

    pub fn route_denial_cause(&self) -> Option<WorthQueryDeclarationRoutePlanDenialCause> {
        self.envelope.route_denial_cause()
    }

    pub fn receipt_denial_cause(&self) -> Option<WorthQueryDeclarationReceiptDenialCause> {
        self.envelope.receipt_denial_cause()
    }

    pub fn evidence_origin(&self) -> WorthQueryDeclarationEnvelopeEvidenceOrigin {
        self.envelope.evidence_origin()
    }

    pub fn explain(&self) -> &WorthQueryDeclarationSignalCompatibilityExplanation {
        &self.explanation
    }

    pub(crate) fn into_envelope(self) -> WorthQueryDeclarationEnvelope<D, I> {
        self.envelope
    }
}
