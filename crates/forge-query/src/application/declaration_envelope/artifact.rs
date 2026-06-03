use forge_foundational::facade::CanonicalDerivedDigest;

use crate::application::{
    ForgeQueryDeclarationAspectContract, ForgeQueryDeclarationAspectPublication,
    ForgeQueryDeclarationFoundationalEvidence, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationReceipt, ForgeQueryDeclarationReceiptDeferred,
    ForgeQueryDeclarationReceiptDenialCause, ForgeQueryDeclarationReceiptDenied,
    ForgeQueryDeclarationReceiptFailed, ForgeQueryDeclarationRoutePlan,
    ForgeQueryDeclarationRoutePlanDenialCause, ForgeQueryDomainEntryMarker,
};
use crate::target_binding::ForgeQueryDeclarationEnvelopeBindingTarget;

use super::{
    class::{ForgeQueryDeclarationEnvelopeClass, ForgeQueryDeclarationEnvelopeEvidenceOrigin},
    explain::ForgeQueryDeclarationEnvelopeExplanation,
};

enum ForgeQueryDeclarationEnvelopeOwner<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    Issued(ForgeQueryDeclarationReceipt<D, I>),
    Deferred(ForgeQueryDeclarationReceiptDeferred<D, I>),
    Denied(ForgeQueryDeclarationReceiptDenied<D, I>),
    Failed(ForgeQueryDeclarationReceiptFailed<D, I>),
}

pub struct ForgeQueryDeclarationEnvelope<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    class: ForgeQueryDeclarationEnvelopeClass,
    owner: ForgeQueryDeclarationEnvelopeOwner<D, I>,
    evidence_origin: ForgeQueryDeclarationEnvelopeEvidenceOrigin,
    declaration_family_key: &'static str,
    declaration_digest: String,
    progression_digest: Option<String>,
    route_plan_digest: Option<String>,
    receipt_digest: CanonicalDerivedDigest,
    published_aspect_contract: ForgeQueryDeclarationAspectContract,
    published_aspect_publication: ForgeQueryDeclarationAspectPublication,
    envelope_digest: CanonicalDerivedDigest,
    explanation: ForgeQueryDeclarationEnvelopeExplanation,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryDeclarationEnvelope<D, I>
{
    pub(crate) fn from_issued(
        receipt: ForgeQueryDeclarationReceipt<D, I>,
        envelope_digest: CanonicalDerivedDigest,
        explanation: ForgeQueryDeclarationEnvelopeExplanation,
    ) -> Self {
        Self::new(
            ForgeQueryDeclarationEnvelopeClass::CoveredCrossing,
            ForgeQueryDeclarationEnvelopeOwner::Issued(receipt),
            envelope_digest,
            explanation,
        )
    }

    pub(crate) fn from_deferred(
        receipt: ForgeQueryDeclarationReceiptDeferred<D, I>,
        envelope_digest: CanonicalDerivedDigest,
        explanation: ForgeQueryDeclarationEnvelopeExplanation,
    ) -> Self {
        Self::new(
            ForgeQueryDeclarationEnvelopeClass::DeferredCrossing,
            ForgeQueryDeclarationEnvelopeOwner::Deferred(receipt),
            envelope_digest,
            explanation,
        )
    }

    pub(crate) fn from_denied(
        receipt: ForgeQueryDeclarationReceiptDenied<D, I>,
        envelope_digest: CanonicalDerivedDigest,
        explanation: ForgeQueryDeclarationEnvelopeExplanation,
    ) -> Self {
        Self::new(
            ForgeQueryDeclarationEnvelopeClass::DeniedCrossing,
            ForgeQueryDeclarationEnvelopeOwner::Denied(receipt),
            envelope_digest,
            explanation,
        )
    }

    pub(crate) fn from_failed(
        receipt: ForgeQueryDeclarationReceiptFailed<D, I>,
        envelope_digest: CanonicalDerivedDigest,
        explanation: ForgeQueryDeclarationEnvelopeExplanation,
    ) -> Self {
        Self::new(
            ForgeQueryDeclarationEnvelopeClass::FailedCrossing,
            ForgeQueryDeclarationEnvelopeOwner::Failed(receipt),
            envelope_digest,
            explanation,
        )
    }

    fn new(
        class: ForgeQueryDeclarationEnvelopeClass,
        owner: ForgeQueryDeclarationEnvelopeOwner<D, I>,
        envelope_digest: CanonicalDerivedDigest,
        explanation: ForgeQueryDeclarationEnvelopeExplanation,
    ) -> Self {
        let (
            evidence_origin,
            declaration_family_key,
            declaration_digest,
            progression_digest,
            route_plan_digest,
            receipt_digest,
            published_aspect_contract,
            published_aspect_publication,
        ) = {
            let receipt = owner.receipt();
            let evidence = receipt.foundational_evidence();
            (
                ForgeQueryDeclarationEnvelopeEvidenceOrigin::from_foundational_class(
                    evidence.class(),
                ),
                receipt.declaration_family_key(),
                receipt.declaration_digest().to_string(),
                receipt.progression_digest().map(ToOwned::to_owned),
                receipt.route_plan_digest().map(ToOwned::to_owned),
                receipt.receipt_digest().clone(),
                receipt.aspect_contract().clone(),
                receipt.aspect_publication().clone(),
            )
        };
        Self {
            class,
            owner,
            evidence_origin,
            declaration_family_key,
            declaration_digest,
            progression_digest,
            route_plan_digest,
            receipt_digest,
            published_aspect_contract,
            published_aspect_publication,
            envelope_digest,
            explanation,
        }
    }

    pub fn class(&self) -> ForgeQueryDeclarationEnvelopeClass {
        self.class
    }

    pub fn declaration_family_key(&self) -> &'static str {
        self.declaration_family_key
    }

    pub fn handle_identity_digest(&self) -> &str {
        self.receipt().handle_identity_digest()
    }

    pub fn operating_context_identity_digest(&self) -> &str {
        self.receipt().operating_context_identity_digest()
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn progression_digest(&self) -> Option<&str> {
        self.progression_digest.as_deref()
    }

    pub fn route_plan_digest(&self) -> Option<&str> {
        self.route_plan_digest.as_deref()
    }

    pub fn receipt_digest(&self) -> &CanonicalDerivedDigest {
        &self.receipt_digest
    }

    pub fn envelope_digest(&self) -> &CanonicalDerivedDigest {
        &self.envelope_digest
    }

    pub fn aspect_contract(&self) -> &ForgeQueryDeclarationAspectContract {
        &self.published_aspect_contract
    }

    pub fn aspect_publication(&self) -> &ForgeQueryDeclarationAspectPublication {
        &self.published_aspect_publication
    }

    pub fn binding_target(&self) -> ForgeQueryDeclarationEnvelopeBindingTarget {
        ForgeQueryDeclarationEnvelopeBindingTarget::for_envelope(self)
    }

    pub fn foundational_evidence(&self) -> &ForgeQueryDeclarationFoundationalEvidence<D, I> {
        self.receipt().foundational_evidence()
    }

    pub fn receipt(&self) -> &ForgeQueryDeclarationReceipt<D, I> {
        self.owner.receipt()
    }

    pub fn route_plan(&self) -> Option<&ForgeQueryDeclarationRoutePlan<D, I>> {
        self.receipt().route_plan()
    }

    pub fn route_denial_cause(&self) -> Option<ForgeQueryDeclarationRoutePlanDenialCause> {
        match &self.owner {
            ForgeQueryDeclarationEnvelopeOwner::Denied(denial) => denial.route_cause(),
            _ => self.receipt().route_denial_cause(),
        }
    }

    pub fn receipt_denial_cause(&self) -> Option<ForgeQueryDeclarationReceiptDenialCause> {
        match &self.owner {
            ForgeQueryDeclarationEnvelopeOwner::Denied(denial) => denial.receipt_cause(),
            _ => None,
        }
    }

    pub fn evidence_origin(&self) -> ForgeQueryDeclarationEnvelopeEvidenceOrigin {
        self.evidence_origin
    }

    pub fn explain(&self) -> &ForgeQueryDeclarationEnvelopeExplanation {
        &self.explanation
    }
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryDeclarationEnvelopeOwner<D, I>
{
    fn receipt(&self) -> &ForgeQueryDeclarationReceipt<D, I> {
        match self {
            Self::Issued(receipt) => receipt,
            Self::Deferred(receipt) => receipt.receipt(),
            Self::Denied(receipt) => receipt.receipt(),
            Self::Failed(receipt) => receipt.receipt(),
        }
    }
}
