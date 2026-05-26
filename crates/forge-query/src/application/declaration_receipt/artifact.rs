use forge_foundational::facade::{
    CanonicalDerivedDigest, FoundationalBoundaryEvidenceCompletedReceiptArtifact,
    FoundationalBoundaryReceiptSurface, FoundationalMaterializedBoundaryArtifact,
};

use crate::application::{
    ForgeQueryDeclarationFoundationalEvidence, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationRoutePlan, ForgeQueryDeclarationRoutePlanDenialCause,
    ForgeQueryDomainEntryMarker,
};
use crate::target_binding::ForgeQueryDeclarationReceiptBindingTarget;

use super::explain::ForgeQueryDeclarationReceiptExplanation;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationReceiptClass {
    CoveredCrossing,
    DeferredCrossing,
    DeniedCrossing,
    FailedCrossing,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationReceiptKind {
    Relational,
    Bridge,
    Mixed,
    Deferred,
    Denied,
    Failed,
}

enum ForgeQueryDeclarationReceiptEvidenceOwner<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    Planned(ForgeQueryDeclarationRoutePlan<D, I>),
    Standalone(ForgeQueryDeclarationFoundationalEvidence<D, I>),
}

pub struct ForgeQueryDeclarationReceipt<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    class: ForgeQueryDeclarationReceiptClass,
    kind: ForgeQueryDeclarationReceiptKind,
    declaration_family_key: &'static str,
    handle_identity_digest: String,
    operating_context_identity_digest: String,
    declaration_digest: String,
    progression_digest: Option<String>,
    route_plan_digest: Option<String>,
    evidence_owner: ForgeQueryDeclarationReceiptEvidenceOwner<D, I>,
    route_denial_cause: Option<ForgeQueryDeclarationRoutePlanDenialCause>,
    explanation: ForgeQueryDeclarationReceiptExplanation,
    descriptive_receipt: Option<FoundationalBoundaryEvidenceCompletedReceiptArtifact>,
    boundary_receipt: FoundationalMaterializedBoundaryArtifact<FoundationalBoundaryReceiptSurface>,
    receipt_digest: CanonicalDerivedDigest,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryDeclarationReceipt<D, I>
{
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        class: ForgeQueryDeclarationReceiptClass,
        kind: ForgeQueryDeclarationReceiptKind,
        route_plan: Option<ForgeQueryDeclarationRoutePlan<D, I>>,
        foundational_evidence: Option<ForgeQueryDeclarationFoundationalEvidence<D, I>>,
        route_denial_cause: Option<ForgeQueryDeclarationRoutePlanDenialCause>,
        explanation: ForgeQueryDeclarationReceiptExplanation,
        descriptive_receipt: Option<FoundationalBoundaryEvidenceCompletedReceiptArtifact>,
        boundary_receipt: FoundationalMaterializedBoundaryArtifact<
            FoundationalBoundaryReceiptSurface,
        >,
        receipt_digest: CanonicalDerivedDigest,
    ) -> Self {
        let evidence_owner = match (route_plan, foundational_evidence) {
            (Some(plan), None) => ForgeQueryDeclarationReceiptEvidenceOwner::Planned(plan),
            (None, Some(evidence)) => {
                ForgeQueryDeclarationReceiptEvidenceOwner::Standalone(evidence)
            }
            _ => panic!("receipt artifacts must retain either a route plan or standalone evidence"),
        };
        let evidence = match &evidence_owner {
            ForgeQueryDeclarationReceiptEvidenceOwner::Planned(plan) => {
                plan.foundational_evidence()
            }
            ForgeQueryDeclarationReceiptEvidenceOwner::Standalone(evidence) => evidence,
        };
        let declaration_family_key = evidence.declaration_family_key();
        let handle_identity_digest = evidence.handle_identity_digest().to_string();
        let operating_context_identity_digest =
            evidence.operating_context_identity_digest().to_string();
        let declaration_digest = evidence.declaration_digest().to_string();
        let progression_digest = evidence.progression_digest().map(ToOwned::to_owned);
        let route_plan_digest = match &evidence_owner {
            ForgeQueryDeclarationReceiptEvidenceOwner::Planned(plan) => {
                Some(plan.route_plan_digest().to_string())
            }
            ForgeQueryDeclarationReceiptEvidenceOwner::Standalone(_) => None,
        };

        Self {
            class,
            kind,
            declaration_family_key,
            handle_identity_digest,
            operating_context_identity_digest,
            declaration_digest,
            progression_digest,
            route_plan_digest,
            evidence_owner,
            route_denial_cause,
            explanation,
            descriptive_receipt,
            boundary_receipt,
            receipt_digest,
        }
    }

    pub fn class(&self) -> ForgeQueryDeclarationReceiptClass {
        self.class
    }

    pub fn kind(&self) -> ForgeQueryDeclarationReceiptKind {
        self.kind
    }

    pub fn declaration_family_key(&self) -> &'static str {
        self.declaration_family_key
    }

    pub fn handle_identity_digest(&self) -> &str {
        &self.handle_identity_digest
    }

    pub fn operating_context_identity_digest(&self) -> &str {
        &self.operating_context_identity_digest
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

    pub fn binding_target(&self) -> ForgeQueryDeclarationReceiptBindingTarget {
        ForgeQueryDeclarationReceiptBindingTarget::for_receipt(self)
    }

    pub fn foundational_evidence(&self) -> &ForgeQueryDeclarationFoundationalEvidence<D, I> {
        match &self.evidence_owner {
            ForgeQueryDeclarationReceiptEvidenceOwner::Planned(plan) => {
                plan.foundational_evidence()
            }
            ForgeQueryDeclarationReceiptEvidenceOwner::Standalone(evidence) => evidence,
        }
    }

    pub fn route_plan(&self) -> Option<&ForgeQueryDeclarationRoutePlan<D, I>> {
        match &self.evidence_owner {
            ForgeQueryDeclarationReceiptEvidenceOwner::Planned(plan) => Some(plan),
            ForgeQueryDeclarationReceiptEvidenceOwner::Standalone(_) => None,
        }
    }

    pub fn route_denial_cause(&self) -> Option<ForgeQueryDeclarationRoutePlanDenialCause> {
        self.route_denial_cause
    }

    pub fn explain(&self) -> &ForgeQueryDeclarationReceiptExplanation {
        &self.explanation
    }

    pub fn descriptive_receipt(
        &self,
    ) -> Option<&FoundationalBoundaryEvidenceCompletedReceiptArtifact> {
        self.descriptive_receipt.as_ref()
    }

    pub fn boundary_receipt(
        &self,
    ) -> &FoundationalMaterializedBoundaryArtifact<FoundationalBoundaryReceiptSurface> {
        &self.boundary_receipt
    }
}
