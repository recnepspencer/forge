use worth_foundational::facade::{
    boundary_evidence, boundary_receipt_category_of, BoundaryArtifactField, BoundaryArtifactId,
    BoundaryArtifactLocator, BoundaryProfiledArtifact, FoundationalBoundaryArtifactCategory,
    FoundationalBoundaryCategoryConstructionDenial,
    FoundationalBoundaryEvidenceExecutedReceiptArtifact,
    FoundationalBoundaryEvidenceFreshnessPosture, FoundationalBoundaryEvidenceProvenanceArtifact,
    FoundationalBoundaryEvidenceProvenanceConstructionDenial,
    FoundationalBoundaryEvidenceReceiptBoundary, FoundationalBoundaryEvidenceSourceBasis,
    FoundationalBoundaryReceiptSurface, FoundationalDiagnosticExplanationBundle,
    FoundationalDiagnosticMaterializationDenial, FoundationalProfileProgressionDenial,
};
use worth_proof::TransitionOutcome;
use worth_query_execution::facade::primary_graph::{
    WorthQueryApplicationCommitOutcomeIdentity, WorthQueryApplicationCommitReceipt,
    WorthQueryApplicationDisclosureOutcomeIdentity, WorthQueryApprovedElevation,
    WorthQueryElevationClosureKind, WorthQueryMandatoryReview,
    WorthQueryOperationAuthorizationDenialIdentity, WorthQueryRequestedElevation,
    WorthQueryReviewedElevation,
};

use crate::domain_computation::WorthQueryApplicationCommitPublicationReceipt;

use super::explanation::{
    materialize_explanation, WorthQueryPublishedApplicationAuthorizationKind,
};

pub(super) mod profile;
#[cfg(test)]
mod tests;

pub use profile::{
    WorthQueryApplicationAuthorizationProfileStage,
    WorthQueryApplicationAuthorizationPublicationProfile,
};

#[derive(Clone, Copy)]
pub(super) struct WorthQueryApplicationAuthorizationBoundaryIdentity {
    locator: BoundaryArtifactLocator,
}

impl WorthQueryApplicationAuthorizationBoundaryIdentity {
    fn from_commit(identity: WorthQueryApplicationCommitOutcomeIdentity) -> Self {
        Self {
            locator: BoundaryArtifactLocator::new(
                BoundaryArtifactId::new(identity.get()),
                BoundaryArtifactField::Payload,
            ),
        }
    }

    pub(super) fn from_denial(identity: WorthQueryOperationAuthorizationDenialIdentity) -> Self {
        Self {
            locator: BoundaryArtifactLocator::new(
                BoundaryArtifactId::new(identity.get()),
                BoundaryArtifactField::Payload,
            ),
        }
    }

    pub(super) fn from_disclosure(
        identity: WorthQueryApplicationDisclosureOutcomeIdentity,
    ) -> Self {
        Self {
            locator: BoundaryArtifactLocator::new(
                BoundaryArtifactId::new(identity.get()),
                BoundaryArtifactField::Payload,
            ),
        }
    }

    pub(super) const fn artifact_id(self) -> BoundaryArtifactId {
        self.locator.artifact_id()
    }

    pub(super) const fn locator(self) -> BoundaryArtifactLocator {
        self.locator
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryApplicationAuthorizationPublicationDenial {
    OutcomeIdentityUnavailable,
    OutcomeNotPublishable,
    BoundaryCategory(FoundationalBoundaryCategoryConstructionDenial),
    ProfileAdmission(FoundationalProfileProgressionDenial),
    ProfileMaterialization(FoundationalProfileProgressionDenial),
    Diagnostic(FoundationalDiagnosticMaterializationDenial),
    Provenance(FoundationalBoundaryEvidenceProvenanceConstructionDenial),
}

/// Descriptive publication output. It is not an approved elevation and cannot
/// be substituted for Query's active-use authority.
///
/// ```compile_fail
/// use worth_query_execution::facade::primary_graph::WorthQueryApprovedElevation;
/// use worth_query_publication::facade::domain_computation::
///     WorthQueryPublishedApplicationAuthorization;
///
/// fn requires_query_authority(_: &WorthQueryApprovedElevation) {}
///
/// fn published_description_cannot_authorize(
///     published: &WorthQueryPublishedApplicationAuthorization,
/// ) {
///     requires_query_authority(published);
/// }
/// ```
#[derive(Debug)]
pub struct WorthQueryPublishedApplicationAuthorization {
    kind: WorthQueryPublishedApplicationAuthorizationKind,
    query_receipt: WorthQueryApplicationCommitPublicationReceipt,
    boundary: BoundaryProfiledArtifact<FoundationalBoundaryReceiptSurface>,
    explanation: FoundationalDiagnosticExplanationBundle,
    provenance: FoundationalBoundaryEvidenceProvenanceArtifact,
    publication_receipt: FoundationalBoundaryEvidenceExecutedReceiptArtifact,
}

impl WorthQueryPublishedApplicationAuthorization {
    pub const fn kind(&self) -> WorthQueryPublishedApplicationAuthorizationKind {
        self.kind
    }

    pub const fn query_receipt(&self) -> &WorthQueryApplicationCommitPublicationReceipt {
        &self.query_receipt
    }

    pub const fn boundary(&self) -> &BoundaryProfiledArtifact<FoundationalBoundaryReceiptSurface> {
        &self.boundary
    }

    pub fn boundary_category(&self) -> FoundationalBoundaryArtifactCategory {
        boundary_receipt_category_of(self.boundary.payload().payload())
    }

    pub const fn explanation(&self) -> &FoundationalDiagnosticExplanationBundle {
        &self.explanation
    }

    pub const fn provenance(&self) -> &FoundationalBoundaryEvidenceProvenanceArtifact {
        &self.provenance
    }

    pub const fn publication_receipt(
        &self,
    ) -> &FoundationalBoundaryEvidenceExecutedReceiptArtifact {
        &self.publication_receipt
    }
}

pub fn publish_requested_elevation(
    requested: &WorthQueryRequestedElevation,
    profile: WorthQueryApplicationAuthorizationPublicationProfile,
) -> Result<
    WorthQueryPublishedApplicationAuthorization,
    WorthQueryApplicationAuthorizationPublicationDenial,
> {
    publish_transition(
        WorthQueryPublishedApplicationAuthorizationKind::ElevationRequested,
        requested.commit_receipt(),
        profile,
    )
}

pub fn publish_approved_elevation(
    approved: &WorthQueryApprovedElevation,
    profile: WorthQueryApplicationAuthorizationPublicationProfile,
) -> Result<
    WorthQueryPublishedApplicationAuthorization,
    WorthQueryApplicationAuthorizationPublicationDenial,
> {
    publish_transition(
        WorthQueryPublishedApplicationAuthorizationKind::ElevationApproved,
        approved.approval_commit_receipt(),
        profile,
    )
}

pub fn publish_mandatory_review(
    review: &WorthQueryMandatoryReview,
    profile: WorthQueryApplicationAuthorizationPublicationProfile,
) -> Result<
    WorthQueryPublishedApplicationAuthorization,
    WorthQueryApplicationAuthorizationPublicationDenial,
> {
    let kind = match review.closure_kind() {
        WorthQueryElevationClosureKind::Revoked => {
            WorthQueryPublishedApplicationAuthorizationKind::RevokedReviewRequired
        }
        WorthQueryElevationClosureKind::Expired => {
            WorthQueryPublishedApplicationAuthorizationKind::ExpiredReviewRequired
        }
    };
    publish_transition(kind, review.close_commit_receipt(), profile)
}

pub fn publish_reviewed_elevation(
    reviewed: &WorthQueryReviewedElevation,
    profile: WorthQueryApplicationAuthorizationPublicationProfile,
) -> Result<
    WorthQueryPublishedApplicationAuthorization,
    WorthQueryApplicationAuthorizationPublicationDenial,
> {
    let kind = match reviewed.closure_kind() {
        WorthQueryElevationClosureKind::Revoked => {
            WorthQueryPublishedApplicationAuthorizationKind::RevokedElevationReviewed
        }
        WorthQueryElevationClosureKind::Expired => {
            WorthQueryPublishedApplicationAuthorizationKind::ExpiredElevationReviewed
        }
    };
    publish_transition(kind, reviewed.review_commit_receipt(), profile)
}

fn publish_transition(
    kind: WorthQueryPublishedApplicationAuthorizationKind,
    commit: &WorthQueryApplicationCommitReceipt,
    profile: WorthQueryApplicationAuthorizationPublicationProfile,
) -> Result<
    WorthQueryPublishedApplicationAuthorization,
    WorthQueryApplicationAuthorizationPublicationDenial,
> {
    let outcome_identity = commit
        .outcome_identity()
        .ok_or(WorthQueryApplicationAuthorizationPublicationDenial::OutcomeIdentityUnavailable)?;
    let identity =
        WorthQueryApplicationAuthorizationBoundaryIdentity::from_commit(outcome_identity);
    let lowered = lower_boundary_material(kind, identity, commit.emitted_effect_count(), profile)?;
    Ok(WorthQueryPublishedApplicationAuthorization {
        kind,
        query_receipt: WorthQueryApplicationCommitPublicationReceipt::from_terminal(commit.clone()),
        boundary: lowered.boundary,
        explanation: lowered.explanation,
        provenance: lowered.provenance,
        publication_receipt: lowered.publication_receipt,
    })
}

struct WorthQueryLoweredApplicationAuthorizationPublication {
    boundary: BoundaryProfiledArtifact<FoundationalBoundaryReceiptSurface>,
    explanation: FoundationalDiagnosticExplanationBundle,
    provenance: FoundationalBoundaryEvidenceProvenanceArtifact,
    publication_receipt: FoundationalBoundaryEvidenceExecutedReceiptArtifact,
}

fn lower_boundary_material(
    kind: WorthQueryPublishedApplicationAuthorizationKind,
    identity: WorthQueryApplicationAuthorizationBoundaryIdentity,
    attested_effect_count: usize,
    profile: WorthQueryApplicationAuthorizationPublicationProfile,
) -> Result<
    WorthQueryLoweredApplicationAuthorizationPublication,
    WorthQueryApplicationAuthorizationPublicationDenial,
> {
    let boundary = profile::profile_boundary(kind, attested_effect_count, profile)?;
    let explanation = materialize_explanation(kind, identity, profile.materialized())
        .map_err(WorthQueryApplicationAuthorizationPublicationDenial::Diagnostic)?;
    let provenance = current_provenance(identity)?;
    let publication_receipt = boundary_evidence()
        .receipt()
        .publication(
            FoundationalBoundaryEvidenceReceiptBoundary::boundary_artifact(identity.locator()),
        )
        .with_provenance(provenance.clone());
    Ok(WorthQueryLoweredApplicationAuthorizationPublication {
        boundary,
        explanation,
        provenance,
        publication_receipt,
    })
}

pub(super) fn current_provenance(
    identity: WorthQueryApplicationAuthorizationBoundaryIdentity,
) -> Result<
    FoundationalBoundaryEvidenceProvenanceArtifact,
    WorthQueryApplicationAuthorizationPublicationDenial,
> {
    match boundary_evidence()
        .provenance()
        .current(FoundationalBoundaryEvidenceSourceBasis::boundary_artifact(
            identity.locator(),
        ))
        .with_freshness(FoundationalBoundaryEvidenceFreshnessPosture::FreshRetained)
    {
        TransitionOutcome::Success(provenance) => Ok(provenance),
        TransitionOutcome::Denied(denial) => {
            Err(WorthQueryApplicationAuthorizationPublicationDenial::Provenance(denial))
        }
        _ => unreachable!("Foundational provenance construction has no nonterminal outcome"),
    }
}
