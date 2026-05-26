use crate::application::{
    ForgeQueryDeclarationEnvelope, ForgeQueryDeclarationEnvelopeEvidenceOrigin,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationReceiptDenialCause,
    ForgeQueryDeclarationRoutePlanDenialCause, ForgeQueryDomainEntryMarker,
};

use super::{
    contract::{
        ForgeQueryDeclarationRelationalAuthorityFamily, ForgeQueryDeclarationRelationalTruthClaim,
    },
    explain::ForgeQueryDeclarationRelationalRoutingExplanation,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationRelationalRoutingClass {
    ExclusiveRelationalTruth,
    MixedAuthorityRelationalTruth,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationRelationalBinding {
    Runtime(&'static str),
    History(&'static str),
    GroupedTruth(&'static str),
    CommitStrategies(&'static str),
    BridgeSource(&'static str),
}

impl ForgeQueryDeclarationRelationalBinding {
    pub fn surface(&self) -> &'static str {
        match self {
            Self::Runtime(surface)
            | Self::History(surface)
            | Self::GroupedTruth(surface)
            | Self::CommitStrategies(surface)
            | Self::BridgeSource(surface) => surface,
        }
    }
}

pub struct ForgeQueryDeclarationRelationalRouting<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    class: ForgeQueryDeclarationRelationalRoutingClass,
    truth_claim: ForgeQueryDeclarationRelationalTruthClaim,
    authority_family: ForgeQueryDeclarationRelationalAuthorityFamily,
    binding: ForgeQueryDeclarationRelationalBinding,
    envelope: ForgeQueryDeclarationEnvelope<D, I>,
    relational_routing_digest: String,
    explanation: ForgeQueryDeclarationRelationalRoutingExplanation,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryDeclarationRelationalRouting<D, I>
{
    pub(crate) fn new(
        class: ForgeQueryDeclarationRelationalRoutingClass,
        truth_claim: ForgeQueryDeclarationRelationalTruthClaim,
        authority_family: ForgeQueryDeclarationRelationalAuthorityFamily,
        binding: ForgeQueryDeclarationRelationalBinding,
        envelope: ForgeQueryDeclarationEnvelope<D, I>,
        relational_routing_digest: String,
        explanation: ForgeQueryDeclarationRelationalRoutingExplanation,
    ) -> Self {
        Self {
            class,
            truth_claim,
            authority_family,
            binding,
            envelope,
            relational_routing_digest,
            explanation,
        }
    }

    pub fn class(&self) -> ForgeQueryDeclarationRelationalRoutingClass {
        self.class
    }

    pub fn truth_claim(&self) -> ForgeQueryDeclarationRelationalTruthClaim {
        self.truth_claim
    }

    pub fn authority_family(&self) -> ForgeQueryDeclarationRelationalAuthorityFamily {
        self.authority_family
    }

    pub fn binding(&self) -> &ForgeQueryDeclarationRelationalBinding {
        &self.binding
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

    pub fn relational_routing_digest(&self) -> &str {
        &self.relational_routing_digest
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

    pub fn explain(&self) -> &ForgeQueryDeclarationRelationalRoutingExplanation {
        &self.explanation
    }

    pub(crate) fn into_envelope(self) -> ForgeQueryDeclarationEnvelope<D, I> {
        self.envelope
    }
}
