use crate::application::{
    ForgeQueryDeclarationEnvelope, ForgeQueryDeclarationEnvelopeEvidenceOrigin,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationPrimaryAuthorityFamily,
    ForgeQueryDeclarationReceiptDenialCause, ForgeQueryDeclarationRoutePlanDenialCause,
    ForgeQueryDomainEntryMarker,
};
use crate::basis_lifecycle::BasisFamily;

use super::{
    contract::ForgeQueryDeclarationSignalExecutionFamily,
    explain::ForgeQueryDeclarationSignalCompatibilityExplanation,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationSignalCompatibilityClass {
    Compatible,
    Deferred,
    Denied,
    Failed,
}

pub struct ForgeQueryDeclarationSignalCompatibility<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    class: ForgeQueryDeclarationSignalCompatibilityClass,
    primary_authority_family: ForgeQueryDeclarationPrimaryAuthorityFamily,
    execution_family: ForgeQueryDeclarationSignalExecutionFamily,
    basis_families: Vec<BasisFamily>,
    envelope: ForgeQueryDeclarationEnvelope<D, I>,
    signal_compatibility_digest: String,
    explanation: ForgeQueryDeclarationSignalCompatibilityExplanation,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryDeclarationSignalCompatibility<D, I>
{
    pub(crate) fn new(
        primary_authority_family: ForgeQueryDeclarationPrimaryAuthorityFamily,
        execution_family: ForgeQueryDeclarationSignalExecutionFamily,
        basis_families: Vec<BasisFamily>,
        envelope: ForgeQueryDeclarationEnvelope<D, I>,
        signal_compatibility_digest: String,
        explanation: ForgeQueryDeclarationSignalCompatibilityExplanation,
    ) -> Self {
        Self {
            class: ForgeQueryDeclarationSignalCompatibilityClass::Compatible,
            primary_authority_family,
            execution_family,
            basis_families,
            envelope,
            signal_compatibility_digest,
            explanation,
        }
    }

    pub fn class(&self) -> ForgeQueryDeclarationSignalCompatibilityClass {
        self.class
    }

    pub fn execution_family(&self) -> ForgeQueryDeclarationSignalExecutionFamily {
        self.execution_family
    }

    pub fn primary_authority_family(&self) -> ForgeQueryDeclarationPrimaryAuthorityFamily {
        self.primary_authority_family
    }

    pub fn basis_families(&self) -> &[BasisFamily] {
        &self.basis_families
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

    pub fn receipt_digest(&self) -> &forge_foundational::facade::CanonicalDerivedDigest {
        self.envelope.receipt_digest()
    }

    pub fn envelope_digest(&self) -> &forge_foundational::facade::CanonicalDerivedDigest {
        self.envelope.envelope_digest()
    }

    pub fn signal_compatibility_digest(&self) -> &str {
        &self.signal_compatibility_digest
    }

    pub fn envelope(&self) -> &ForgeQueryDeclarationEnvelope<D, I> {
        &self.envelope
    }

    pub fn route_denial_cause(&self) -> Option<ForgeQueryDeclarationRoutePlanDenialCause> {
        self.envelope.route_denial_cause()
    }

    pub fn receipt_denial_cause(&self) -> Option<ForgeQueryDeclarationReceiptDenialCause> {
        self.envelope.receipt_denial_cause()
    }

    pub fn evidence_origin(&self) -> ForgeQueryDeclarationEnvelopeEvidenceOrigin {
        self.envelope.evidence_origin()
    }

    pub fn explain(&self) -> &ForgeQueryDeclarationSignalCompatibilityExplanation {
        &self.explanation
    }

    pub(crate) fn into_envelope(self) -> ForgeQueryDeclarationEnvelope<D, I> {
        self.envelope
    }
}
