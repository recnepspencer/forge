use worth_foundational::facade::CanonicalDerivedDigest;

use crate::application::{
    WorthQueryDeclarationAspectContract, WorthQueryDeclarationAspectPublication,
    WorthQueryDeclarationFoundationalEvidence, WorthQueryDeclarationInput,
    WorthQueryDeclarationReceipt, WorthQueryDeclarationReceiptDeferred,
    WorthQueryDeclarationReceiptDenialCause, WorthQueryDeclarationReceiptDenied,
    WorthQueryDeclarationReceiptFailed, WorthQueryDeclarationRoutePlan,
    WorthQueryDeclarationRoutePlanDenialCause, WorthQueryDomainEntryMarker,
};
use crate::target_binding::WorthQueryDeclarationEnvelopeBindingTarget;

use super::{
    class::{WorthQueryDeclarationEnvelopeClass, WorthQueryDeclarationEnvelopeEvidenceOrigin},
    explain::WorthQueryDeclarationEnvelopeExplanation,
};

enum WorthQueryDeclarationEnvelopeOwner<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    Issued(WorthQueryDeclarationReceipt<D, I>),
    Deferred(WorthQueryDeclarationReceiptDeferred<D, I>),
    Denied(WorthQueryDeclarationReceiptDenied<D, I>),
    Failed(WorthQueryDeclarationReceiptFailed<D, I>),
}

pub struct WorthQueryDeclarationEnvelope<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    class: WorthQueryDeclarationEnvelopeClass,
    owner: WorthQueryDeclarationEnvelopeOwner<D, I>,
    evidence_origin: WorthQueryDeclarationEnvelopeEvidenceOrigin,
    declaration_family_key: &'static str,
    declaration_digest: String,
    progression_digest: Option<String>,
    route_plan_digest: Option<String>,
    receipt_digest: CanonicalDerivedDigest,
    published_aspect_contract: WorthQueryDeclarationAspectContract,
    published_aspect_publication: WorthQueryDeclarationAspectPublication,
    envelope_digest: CanonicalDerivedDigest,
    explanation: WorthQueryDeclarationEnvelopeExplanation,
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryDeclarationEnvelope<D, I>
{
    pub(crate) fn from_issued(
        receipt: WorthQueryDeclarationReceipt<D, I>,
        envelope_digest: CanonicalDerivedDigest,
        explanation: WorthQueryDeclarationEnvelopeExplanation,
    ) -> Self {
        Self::new(
            WorthQueryDeclarationEnvelopeClass::CoveredCrossing,
            WorthQueryDeclarationEnvelopeOwner::Issued(receipt),
            envelope_digest,
            explanation,
        )
    }

    pub(crate) fn from_deferred(
        receipt: WorthQueryDeclarationReceiptDeferred<D, I>,
        envelope_digest: CanonicalDerivedDigest,
        explanation: WorthQueryDeclarationEnvelopeExplanation,
    ) -> Self {
        Self::new(
            WorthQueryDeclarationEnvelopeClass::DeferredCrossing,
            WorthQueryDeclarationEnvelopeOwner::Deferred(receipt),
            envelope_digest,
            explanation,
        )
    }

    pub(crate) fn from_denied(
        receipt: WorthQueryDeclarationReceiptDenied<D, I>,
        envelope_digest: CanonicalDerivedDigest,
        explanation: WorthQueryDeclarationEnvelopeExplanation,
    ) -> Self {
        Self::new(
            WorthQueryDeclarationEnvelopeClass::DeniedCrossing,
            WorthQueryDeclarationEnvelopeOwner::Denied(receipt),
            envelope_digest,
            explanation,
        )
    }

    pub(crate) fn from_failed(
        receipt: WorthQueryDeclarationReceiptFailed<D, I>,
        envelope_digest: CanonicalDerivedDigest,
        explanation: WorthQueryDeclarationEnvelopeExplanation,
    ) -> Self {
        Self::new(
            WorthQueryDeclarationEnvelopeClass::FailedCrossing,
            WorthQueryDeclarationEnvelopeOwner::Failed(receipt),
            envelope_digest,
            explanation,
        )
    }

    fn new(
        class: WorthQueryDeclarationEnvelopeClass,
        owner: WorthQueryDeclarationEnvelopeOwner<D, I>,
        envelope_digest: CanonicalDerivedDigest,
        explanation: WorthQueryDeclarationEnvelopeExplanation,
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
                WorthQueryDeclarationEnvelopeEvidenceOrigin::from_foundational_class(
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

    pub fn class(&self) -> WorthQueryDeclarationEnvelopeClass {
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

    pub fn aspect_contract(&self) -> &WorthQueryDeclarationAspectContract {
        &self.published_aspect_contract
    }

    pub fn aspect_publication(&self) -> &WorthQueryDeclarationAspectPublication {
        &self.published_aspect_publication
    }

    pub fn binding_target(&self) -> WorthQueryDeclarationEnvelopeBindingTarget {
        WorthQueryDeclarationEnvelopeBindingTarget::for_envelope(self)
    }

    pub fn foundational_evidence(&self) -> &WorthQueryDeclarationFoundationalEvidence<D, I> {
        self.receipt().foundational_evidence()
    }

    pub fn receipt(&self) -> &WorthQueryDeclarationReceipt<D, I> {
        self.owner.receipt()
    }

    pub fn route_plan(&self) -> Option<&WorthQueryDeclarationRoutePlan<D, I>> {
        self.receipt().route_plan()
    }

    pub fn route_denial_cause(&self) -> Option<WorthQueryDeclarationRoutePlanDenialCause> {
        match &self.owner {
            WorthQueryDeclarationEnvelopeOwner::Denied(denial) => denial.route_cause(),
            _ => self.receipt().route_denial_cause(),
        }
    }

    pub fn receipt_denial_cause(&self) -> Option<WorthQueryDeclarationReceiptDenialCause> {
        match &self.owner {
            WorthQueryDeclarationEnvelopeOwner::Denied(denial) => denial.receipt_cause(),
            _ => None,
        }
    }

    pub fn evidence_origin(&self) -> WorthQueryDeclarationEnvelopeEvidenceOrigin {
        self.evidence_origin
    }

    pub fn explain(&self) -> &WorthQueryDeclarationEnvelopeExplanation {
        &self.explanation
    }
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryDeclarationEnvelopeOwner<D, I>
{
    fn receipt(&self) -> &WorthQueryDeclarationReceipt<D, I> {
        match self {
            Self::Issued(receipt) => receipt,
            Self::Deferred(receipt) => receipt.receipt(),
            Self::Denied(receipt) => receipt.receipt(),
            Self::Failed(receipt) => receipt.receipt(),
        }
    }
}
