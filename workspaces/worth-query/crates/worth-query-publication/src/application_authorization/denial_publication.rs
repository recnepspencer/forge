use worth_foundational::facade::{
    boundary_artifact_category_of, boundary_evidence, BoundaryProfiledArtifact,
    FoundationalBoundaryArtifactCategory, FoundationalBoundaryArtifactSurface,
    FoundationalBoundaryEvidenceCompletedReceiptArtifact,
    FoundationalBoundaryEvidenceExecutedReceiptArtifact,
    FoundationalBoundaryEvidenceProvenanceArtifact, FoundationalBoundaryEvidenceReceiptBoundary,
    FoundationalDiagnosticExplanationBundle,
};
use worth_query_execution::facade::primary_graph::{
    WorthQueryApplicationAuthorizationExplanationCause, WorthQueryOperationAuthorizationDenial,
};

use super::boundary_evidence::{
    current_provenance, profile, WorthQueryApplicationAuthorizationBoundaryIdentity,
    WorthQueryApplicationAuthorizationPublicationDenial,
    WorthQueryApplicationAuthorizationPublicationProfile,
};
use super::denial_explanation::materialize_denial_explanation;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryApplicationAuthorizationDenialArtifact {
    cause: WorthQueryApplicationAuthorizationExplanationCause,
    denial: WorthQueryOperationAuthorizationDenial,
}

impl WorthQueryApplicationAuthorizationDenialArtifact {
    pub const fn cause(&self) -> WorthQueryApplicationAuthorizationExplanationCause {
        self.cause
    }

    pub const fn denial(&self) -> &WorthQueryOperationAuthorizationDenial {
        &self.denial
    }
}

/// Descriptive denial publication. It carries no Query execution authority.
///
/// ```compile_fail
/// use worth_query_execution::facade::primary_graph::WorthQueryApprovedElevation;
/// use worth_query_publication::facade::domain_computation::
///     WorthQueryPublishedApplicationAuthorizationDenial;
///
/// fn requires_query_authority(_: &WorthQueryApprovedElevation) {}
///
/// fn denial_description_cannot_authorize(
///     published: &WorthQueryPublishedApplicationAuthorizationDenial,
/// ) {
///     requires_query_authority(published);
/// }
/// ```
#[derive(Debug)]
pub struct WorthQueryPublishedApplicationAuthorizationDenial {
    boundary: BoundaryProfiledArtifact<
        FoundationalBoundaryArtifactSurface<WorthQueryApplicationAuthorizationDenialArtifact>,
    >,
    explanation: FoundationalDiagnosticExplanationBundle,
    provenance: FoundationalBoundaryEvidenceProvenanceArtifact,
    denied_closeout_receipt: FoundationalBoundaryEvidenceCompletedReceiptArtifact,
    publication_receipt: FoundationalBoundaryEvidenceExecutedReceiptArtifact,
}

impl WorthQueryPublishedApplicationAuthorizationDenial {
    pub fn artifact(&self) -> &WorthQueryApplicationAuthorizationDenialArtifact {
        self.boundary.payload().payload().payload()
    }

    pub const fn boundary(
        &self,
    ) -> &BoundaryProfiledArtifact<
        FoundationalBoundaryArtifactSurface<WorthQueryApplicationAuthorizationDenialArtifact>,
    > {
        &self.boundary
    }

    pub fn boundary_category(&self) -> FoundationalBoundaryArtifactCategory {
        boundary_artifact_category_of(self.boundary.payload().payload())
    }

    pub const fn explanation(&self) -> &FoundationalDiagnosticExplanationBundle {
        &self.explanation
    }

    pub const fn provenance(&self) -> &FoundationalBoundaryEvidenceProvenanceArtifact {
        &self.provenance
    }

    pub const fn denied_closeout_receipt(
        &self,
    ) -> &FoundationalBoundaryEvidenceCompletedReceiptArtifact {
        &self.denied_closeout_receipt
    }

    pub const fn publication_receipt(
        &self,
    ) -> &FoundationalBoundaryEvidenceExecutedReceiptArtifact {
        &self.publication_receipt
    }
}

pub fn publish_application_authorization_denial(
    denial: &WorthQueryOperationAuthorizationDenial,
    profile: WorthQueryApplicationAuthorizationPublicationProfile,
) -> Result<
    WorthQueryPublishedApplicationAuthorizationDenial,
    WorthQueryApplicationAuthorizationPublicationDenial,
> {
    let denial_identity = denial
        .identity()
        .ok_or(WorthQueryApplicationAuthorizationPublicationDenial::OutcomeIdentityUnavailable)?;
    let cause = denial
        .explanation_cause()
        .ok_or(WorthQueryApplicationAuthorizationPublicationDenial::OutcomeNotPublishable)?;
    let identity = WorthQueryApplicationAuthorizationBoundaryIdentity::from_denial(denial_identity);
    let artifact = WorthQueryApplicationAuthorizationDenialArtifact {
        cause,
        denial: denial.clone(),
    };
    let boundary = profile::profile_boundary_artifact(
        FoundationalBoundaryArtifactSurface::new(artifact, 0),
        profile,
    )?;
    let explanation = materialize_denial_explanation(cause, identity, profile.materialized())
        .map_err(WorthQueryApplicationAuthorizationPublicationDenial::Diagnostic)?;
    let provenance = current_provenance(identity)?;
    let receipt_boundary =
        FoundationalBoundaryEvidenceReceiptBoundary::boundary_artifact(identity.locator());
    let denied_closeout_receipt = boundary_evidence()
        .receipt()
        .denied_closeout(receipt_boundary.clone())
        .with_provenance(provenance.clone());
    let publication_receipt = boundary_evidence()
        .receipt()
        .publication(receipt_boundary)
        .with_provenance(provenance.clone());
    Ok(WorthQueryPublishedApplicationAuthorizationDenial {
        boundary,
        explanation,
        provenance,
        denied_closeout_receipt,
        publication_receipt,
    })
}
