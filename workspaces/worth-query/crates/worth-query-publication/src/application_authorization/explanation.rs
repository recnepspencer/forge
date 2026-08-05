use worth_foundational::facade::{
    foundational_diagnostic_boundary_artifact_subject,
    foundational_diagnostic_locator_boundary_artifact, FoundationalDiagnosticCodeId,
    FoundationalDiagnosticCounterSnapshot, FoundationalDiagnosticDecisionRow,
    FoundationalDiagnosticDeliveryClass, FoundationalDiagnosticExplanationBundle,
    FoundationalDiagnosticExplanationInput, FoundationalDiagnosticLocalityClaim,
    FoundationalDiagnosticMaterializationDenial, FoundationalDiagnosticOutcomeKind,
    FoundationalDiagnosticPartiality, FoundationalDiagnosticRow, FoundationalDiagnosticScopeId,
    FoundationalDiagnosticSemanticLabelSet, FoundationalDiagnosticSeverity,
    FoundationalDiagnosticSurfaceAvailability, FoundationalDiagnosticWidenedFalloutPosture,
    FoundationalProfileSet,
};

use super::boundary_evidence::WorthQueryApplicationAuthorizationBoundaryIdentity;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryPublishedApplicationAuthorizationKind {
    ElevationRequested,
    ElevationApproved,
    RevokedReviewRequired,
    ExpiredReviewRequired,
    RevokedElevationReviewed,
    ExpiredElevationReviewed,
}

impl WorthQueryPublishedApplicationAuthorizationKind {
    pub(super) const fn diagnostic_code(self) -> &'static str {
        match self {
            Self::ElevationRequested => "worth.query.elevation.requested",
            Self::ElevationApproved => "worth.query.elevation.approved",
            Self::RevokedReviewRequired => "worth.query.elevation.revoked.review-required",
            Self::ExpiredReviewRequired => "worth.query.elevation.expired.review-required",
            Self::RevokedElevationReviewed => "worth.query.elevation.revoked.reviewed",
            Self::ExpiredElevationReviewed => "worth.query.elevation.expired.reviewed",
        }
    }

    pub(super) const fn completed_boundary(self) -> &'static str {
        match self {
            Self::ElevationRequested => "query elevation request committed",
            Self::ElevationApproved => "query elevation approval committed",
            Self::RevokedReviewRequired => "query elevation revocation committed",
            Self::ExpiredReviewRequired => "query elevation expiry committed",
            Self::RevokedElevationReviewed => "query revoked elevation review committed",
            Self::ExpiredElevationReviewed => "query expired elevation review committed",
        }
    }
}

pub(super) fn materialize_explanation(
    kind: WorthQueryPublishedApplicationAuthorizationKind,
    identity: WorthQueryApplicationAuthorizationBoundaryIdentity,
    profile: FoundationalProfileSet,
) -> Result<FoundationalDiagnosticExplanationBundle, FoundationalDiagnosticMaterializationDenial> {
    let code = FoundationalDiagnosticCodeId::new(kind.diagnostic_code())
        .expect("static Query publication diagnostic code is valid");
    let scope = FoundationalDiagnosticScopeId::new("worth.query.application-authorization")
        .expect("static Query publication diagnostic scope is valid");
    let subject = foundational_diagnostic_boundary_artifact_subject(
        identity.artifact_id(),
        identity.locator().field(),
    );
    let row = FoundationalDiagnosticRow::Decision(FoundationalDiagnosticDecisionRow::new(
        code.clone(),
        scope,
        FoundationalDiagnosticSeverity::Info,
        subject.clone(),
        foundational_diagnostic_locator_boundary_artifact(identity.locator()),
        FoundationalDiagnosticOutcomeKind::Accepted,
        FoundationalDiagnosticSemanticLabelSet::new([code]),
        None,
        FoundationalDiagnosticLocalityClaim::ExactSubject,
        FoundationalDiagnosticWidenedFalloutPosture::NotWidened,
    ));
    worth_foundational::facade::materialize_diagnostic_explanation_bundle(
        FoundationalDiagnosticExplanationInput::new(
            subject,
            FoundationalDiagnosticOutcomeKind::Accepted,
            vec![row],
            Vec::new(),
            Vec::new(),
            FoundationalDiagnosticSurfaceAvailability::retained_hot(),
            FoundationalDiagnosticPartiality::Complete,
            FoundationalDiagnosticCounterSnapshot::new(1, 0, 0, 0, 0, 0),
            Vec::new(),
        ),
        profile,
        FoundationalDiagnosticDeliveryClass::MustBeHot,
    )
}
