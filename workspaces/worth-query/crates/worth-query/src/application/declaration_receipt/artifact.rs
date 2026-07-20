use worth_foundational::facade::{
    CanonicalDerivedDigest, FoundationalBoundaryEvidenceCompletedReceiptArtifact,
    FoundationalBoundaryReceiptSurface, FoundationalMaterializedBoundaryArtifact,
};

use crate::application::{
    WorthQueryDeclarationAspectContract, WorthQueryDeclarationAspectCoverage,
    WorthQueryDeclarationAspectPublication, WorthQueryDeclarationFoundationalEvidence,
    WorthQueryDeclarationInput, WorthQueryDeclarationRoutePlan,
    WorthQueryDeclarationRoutePlanDenialCause, WorthQueryDomainEntryMarker,
};
use crate::target_binding::WorthQueryDeclarationReceiptBindingTarget;

use super::explain::WorthQueryDeclarationReceiptExplanation;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDeclarationReceiptClass {
    CoveredCrossing,
    DeferredCrossing,
    DeniedCrossing,
    FailedCrossing,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDeclarationReceiptKind {
    Relational,
    Bridge,
    Mixed,
    Deferred,
    Denied,
    Failed,
}

enum WorthQueryDeclarationReceiptEvidenceOwner<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    Planned(WorthQueryDeclarationRoutePlan<D, I>),
    Standalone(WorthQueryDeclarationFoundationalEvidence<D, I>),
}

pub struct WorthQueryDeclarationReceipt<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    class: WorthQueryDeclarationReceiptClass,
    kind: WorthQueryDeclarationReceiptKind,
    declaration_family_key: &'static str,
    declaration_digest: String,
    progression_digest: Option<String>,
    route_plan_digest: Option<String>,
    evidence_owner: WorthQueryDeclarationReceiptEvidenceOwner<D, I>,
    route_denial_cause: Option<WorthQueryDeclarationRoutePlanDenialCause>,
    explanation: WorthQueryDeclarationReceiptExplanation,
    crossing_aspect_contract: WorthQueryDeclarationAspectContract,
    crossing_aspect_coverage: WorthQueryDeclarationAspectCoverage,
    crossing_aspect_publication: WorthQueryDeclarationAspectPublication,
    descriptive_receipt: Option<FoundationalBoundaryEvidenceCompletedReceiptArtifact>,
    boundary_receipt: FoundationalMaterializedBoundaryArtifact<FoundationalBoundaryReceiptSurface>,
    receipt_digest: CanonicalDerivedDigest,
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryDeclarationReceipt<D, I>
{
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        class: WorthQueryDeclarationReceiptClass,
        kind: WorthQueryDeclarationReceiptKind,
        route_plan: Option<WorthQueryDeclarationRoutePlan<D, I>>,
        foundational_evidence: Option<WorthQueryDeclarationFoundationalEvidence<D, I>>,
        route_denial_cause: Option<WorthQueryDeclarationRoutePlanDenialCause>,
        explanation: WorthQueryDeclarationReceiptExplanation,
        crossing_aspect_contract: WorthQueryDeclarationAspectContract,
        crossing_aspect_coverage: WorthQueryDeclarationAspectCoverage,
        crossing_aspect_publication: WorthQueryDeclarationAspectPublication,
        descriptive_receipt: Option<FoundationalBoundaryEvidenceCompletedReceiptArtifact>,
        boundary_receipt: FoundationalMaterializedBoundaryArtifact<
            FoundationalBoundaryReceiptSurface,
        >,
        receipt_digest: CanonicalDerivedDigest,
    ) -> Self {
        let evidence_owner = match (route_plan, foundational_evidence) {
            (Some(plan), None) => WorthQueryDeclarationReceiptEvidenceOwner::Planned(plan),
            (None, Some(evidence)) => {
                WorthQueryDeclarationReceiptEvidenceOwner::Standalone(evidence)
            }
            _ => panic!("receipt artifacts must retain either a route plan or standalone evidence"),
        };
        let evidence = match &evidence_owner {
            WorthQueryDeclarationReceiptEvidenceOwner::Planned(plan) => {
                plan.foundational_evidence()
            }
            WorthQueryDeclarationReceiptEvidenceOwner::Standalone(evidence) => evidence,
        };
        let declaration_family_key = evidence.declaration_family_key();
        let declaration_digest = evidence.declaration_digest().to_string();
        let progression_digest = evidence.progression_digest().map(ToOwned::to_owned);
        let route_plan_digest = match &evidence_owner {
            WorthQueryDeclarationReceiptEvidenceOwner::Planned(plan) => {
                Some(plan.route_plan_digest().to_string())
            }
            WorthQueryDeclarationReceiptEvidenceOwner::Standalone(_) => None,
        };

        Self {
            class,
            kind,
            declaration_family_key,
            declaration_digest,
            progression_digest,
            route_plan_digest,
            evidence_owner,
            route_denial_cause,
            explanation,
            crossing_aspect_contract,
            crossing_aspect_coverage,
            crossing_aspect_publication,
            descriptive_receipt,
            boundary_receipt,
            receipt_digest,
        }
    }

    pub fn class(&self) -> WorthQueryDeclarationReceiptClass {
        self.class
    }

    pub fn kind(&self) -> WorthQueryDeclarationReceiptKind {
        self.kind
    }

    pub fn declaration_family_key(&self) -> &'static str {
        self.declaration_family_key
    }

    pub fn handle_identity_digest(&self) -> &str {
        self.foundational_evidence().handle_identity_digest()
    }

    pub fn operating_context_identity_digest(&self) -> &str {
        self.foundational_evidence()
            .operating_context_identity_digest()
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

    pub fn binding_target(&self) -> WorthQueryDeclarationReceiptBindingTarget {
        WorthQueryDeclarationReceiptBindingTarget::for_receipt(self)
    }

    pub fn foundational_evidence(&self) -> &WorthQueryDeclarationFoundationalEvidence<D, I> {
        match &self.evidence_owner {
            WorthQueryDeclarationReceiptEvidenceOwner::Planned(plan) => {
                plan.foundational_evidence()
            }
            WorthQueryDeclarationReceiptEvidenceOwner::Standalone(evidence) => evidence,
        }
    }

    pub fn route_plan(&self) -> Option<&WorthQueryDeclarationRoutePlan<D, I>> {
        match &self.evidence_owner {
            WorthQueryDeclarationReceiptEvidenceOwner::Planned(plan) => Some(plan),
            WorthQueryDeclarationReceiptEvidenceOwner::Standalone(_) => None,
        }
    }

    pub fn route_denial_cause(&self) -> Option<WorthQueryDeclarationRoutePlanDenialCause> {
        self.route_denial_cause
    }

    pub fn explain(&self) -> &WorthQueryDeclarationReceiptExplanation {
        &self.explanation
    }

    pub fn aspect_contract(&self) -> &WorthQueryDeclarationAspectContract {
        &self.crossing_aspect_contract
    }

    pub fn aspect_coverage(&self) -> &WorthQueryDeclarationAspectCoverage {
        &self.crossing_aspect_coverage
    }

    pub fn aspect_publication(&self) -> &WorthQueryDeclarationAspectPublication {
        &self.crossing_aspect_publication
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
