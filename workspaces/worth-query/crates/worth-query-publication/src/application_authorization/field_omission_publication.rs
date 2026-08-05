use worth_foundational::facade::{
    boundary_artifact_category_of, boundary_evidence, BoundaryProfiledArtifact,
    FoundationalBoundaryArtifactCategory, FoundationalBoundaryArtifactSurface,
    FoundationalBoundaryEvidenceExecutedReceiptArtifact,
    FoundationalBoundaryEvidenceProvenanceArtifact, FoundationalBoundaryEvidenceReceiptBoundary,
    FoundationalDiagnosticExplanationBundle,
};
use worth_query_execution::facade::primary_graph::WorthQueryApplicationDisclosureReceipt;

use super::boundary_evidence::{
    current_provenance, profile, WorthQueryApplicationAuthorizationBoundaryIdentity,
    WorthQueryApplicationAuthorizationPublicationDenial,
    WorthQueryApplicationAuthorizationPublicationProfile,
};
use super::field_omission_explanation::materialize_field_omission_explanation;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryApplicationFieldOmissionArtifact {
    disclosure: WorthQueryApplicationDisclosureReceipt,
}

impl WorthQueryApplicationFieldOmissionArtifact {
    pub const fn disclosure(&self) -> &WorthQueryApplicationDisclosureReceipt {
        &self.disclosure
    }
}

#[derive(Debug)]
pub struct WorthQueryPublishedApplicationFieldOmission {
    boundary: BoundaryProfiledArtifact<
        FoundationalBoundaryArtifactSurface<WorthQueryApplicationFieldOmissionArtifact>,
    >,
    explanation: FoundationalDiagnosticExplanationBundle,
    provenance: FoundationalBoundaryEvidenceProvenanceArtifact,
    publication_receipt: FoundationalBoundaryEvidenceExecutedReceiptArtifact,
}

impl WorthQueryPublishedApplicationFieldOmission {
    pub fn artifact(&self) -> &WorthQueryApplicationFieldOmissionArtifact {
        self.boundary.payload().payload().payload()
    }

    pub const fn boundary(
        &self,
    ) -> &BoundaryProfiledArtifact<
        FoundationalBoundaryArtifactSurface<WorthQueryApplicationFieldOmissionArtifact>,
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

    pub const fn publication_receipt(
        &self,
    ) -> &FoundationalBoundaryEvidenceExecutedReceiptArtifact {
        &self.publication_receipt
    }
}

pub fn publish_application_field_omission(
    disclosure: &WorthQueryApplicationDisclosureReceipt,
    profile: WorthQueryApplicationAuthorizationPublicationProfile,
) -> Result<
    WorthQueryPublishedApplicationFieldOmission,
    WorthQueryApplicationAuthorizationPublicationDenial,
> {
    if !disclosure.has_omissions() {
        return Err(WorthQueryApplicationAuthorizationPublicationDenial::OutcomeNotPublishable);
    }
    let disclosure_identity = disclosure
        .outcome_identity()
        .ok_or(WorthQueryApplicationAuthorizationPublicationDenial::OutcomeIdentityUnavailable)?;
    let identity =
        WorthQueryApplicationAuthorizationBoundaryIdentity::from_disclosure(disclosure_identity);
    let artifact = WorthQueryApplicationFieldOmissionArtifact {
        disclosure: disclosure.clone(),
    };
    let boundary = profile::profile_boundary_artifact(
        FoundationalBoundaryArtifactSurface::new(artifact, 0),
        profile,
    )?;
    let explanation = materialize_field_omission_explanation(identity, profile.materialized())
        .map_err(WorthQueryApplicationAuthorizationPublicationDenial::Diagnostic)?;
    let provenance = current_provenance(identity)?;
    let publication_receipt = boundary_evidence()
        .receipt()
        .publication(
            FoundationalBoundaryEvidenceReceiptBoundary::boundary_artifact(identity.locator()),
        )
        .with_provenance(provenance.clone());
    Ok(WorthQueryPublishedApplicationFieldOmission {
        boundary,
        explanation,
        provenance,
        publication_receipt,
    })
}
