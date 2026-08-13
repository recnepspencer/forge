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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryPublishedApplicationAuthorizationDenialCause {
    MissingCapability,
    ExplicitPolicyDenial,
    ScopeMismatch,
    PurposeMismatch,
    Conflict,
    SeparationOfDuty,
    ElevationRequired,
    ElevationDenied,
    ElevationExpired,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryApplicationAuthorizationDenialArtifact {
    cause: WorthQueryPublishedApplicationAuthorizationDenialCause,
    contributing_cause_count: usize,
}

impl WorthQueryApplicationAuthorizationDenialArtifact {
    pub const fn cause(&self) -> WorthQueryPublishedApplicationAuthorizationDenialCause {
        self.cause
    }

    pub const fn contributing_cause_count(&self) -> usize {
        self.contributing_cause_count
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
    let execution_cause = denial
        .explanation_cause()
        .ok_or(WorthQueryApplicationAuthorizationPublicationDenial::OutcomeNotPublishable)?;
    let cause = publish_denial_cause(execution_cause);
    let cause_count = denial.causes().len();
    let identity = WorthQueryApplicationAuthorizationBoundaryIdentity::from_closed_publication(
        cause.label(),
        &[cause_count.to_string()],
        profile,
    );
    let artifact = WorthQueryApplicationAuthorizationDenialArtifact {
        cause,
        contributing_cause_count: cause_count,
    };
    let boundary = profile::profile_boundary_artifact(
        FoundationalBoundaryArtifactSurface::new(artifact, 0),
        profile,
    )?;
    let explanation =
        materialize_denial_explanation(execution_cause, identity, profile.materialized())
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

impl WorthQueryPublishedApplicationAuthorizationDenialCause {
    const fn label(self) -> &'static str {
        match self {
            Self::MissingCapability => "missing-capability",
            Self::ExplicitPolicyDenial => "explicit-policy-denial",
            Self::ScopeMismatch => "scope-mismatch",
            Self::PurposeMismatch => "purpose-mismatch",
            Self::Conflict => "conflict",
            Self::SeparationOfDuty => "separation-of-duty",
            Self::ElevationRequired => "elevation-required",
            Self::ElevationDenied => "elevation-denied",
            Self::ElevationExpired => "elevation-expired",
        }
    }
}

const fn publish_denial_cause(
    cause: WorthQueryApplicationAuthorizationExplanationCause,
) -> WorthQueryPublishedApplicationAuthorizationDenialCause {
    use WorthQueryApplicationAuthorizationExplanationCause as Execution;
    match cause {
        Execution::MissingCapability => {
            WorthQueryPublishedApplicationAuthorizationDenialCause::MissingCapability
        }
        Execution::ExplicitPolicyDenial => {
            WorthQueryPublishedApplicationAuthorizationDenialCause::ExplicitPolicyDenial
        }
        Execution::ScopeMismatch => {
            WorthQueryPublishedApplicationAuthorizationDenialCause::ScopeMismatch
        }
        Execution::PurposeMismatch => {
            WorthQueryPublishedApplicationAuthorizationDenialCause::PurposeMismatch
        }
        Execution::Conflict => WorthQueryPublishedApplicationAuthorizationDenialCause::Conflict,
        Execution::SeparationOfDuty => {
            WorthQueryPublishedApplicationAuthorizationDenialCause::SeparationOfDuty
        }
        Execution::ElevationRequired => {
            WorthQueryPublishedApplicationAuthorizationDenialCause::ElevationRequired
        }
        Execution::ElevationDenied => {
            WorthQueryPublishedApplicationAuthorizationDenialCause::ElevationDenied
        }
        Execution::ElevationExpired => {
            WorthQueryPublishedApplicationAuthorizationDenialCause::ElevationExpired
        }
    }
}
