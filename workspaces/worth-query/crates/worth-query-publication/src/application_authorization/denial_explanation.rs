use worth_foundational::facade::{
    foundational_diagnostic_boundary_artifact_subject,
    foundational_diagnostic_locator_boundary_artifact, FoundationalDiagnosticCodeId,
    FoundationalDiagnosticCounterSnapshot, FoundationalDiagnosticDecisionRow,
    FoundationalDiagnosticDeliveryClass, FoundationalDiagnosticDenialClass,
    FoundationalDiagnosticExplanationBundle, FoundationalDiagnosticExplanationInput,
    FoundationalDiagnosticLocalityClaim, FoundationalDiagnosticMaterializationDenial,
    FoundationalDiagnosticOutcomeKind, FoundationalDiagnosticPartiality, FoundationalDiagnosticRow,
    FoundationalDiagnosticScopeId, FoundationalDiagnosticSemanticLabelSet,
    FoundationalDiagnosticSeverity, FoundationalDiagnosticSurfaceAvailability,
    FoundationalDiagnosticWidenedFalloutPosture, FoundationalProfileSet,
};
use worth_query_execution::facade::primary_graph::WorthQueryApplicationAuthorizationExplanationCause;

use super::boundary_evidence::WorthQueryApplicationAuthorizationBoundaryIdentity;

pub(super) fn materialize_denial_explanation(
    cause: WorthQueryApplicationAuthorizationExplanationCause,
    identity: WorthQueryApplicationAuthorizationBoundaryIdentity,
    profile: FoundationalProfileSet,
) -> Result<FoundationalDiagnosticExplanationBundle, FoundationalDiagnosticMaterializationDenial> {
    let code = FoundationalDiagnosticCodeId::new(diagnostic_code(cause))
        .expect("static Query denial diagnostic code is valid");
    let scope = FoundationalDiagnosticScopeId::new("worth.query.application-authorization")
        .expect("static Query denial diagnostic scope is valid");
    let subject = foundational_diagnostic_boundary_artifact_subject(
        identity.artifact_id(),
        identity.locator().field(),
    );
    let outcome = diagnostic_outcome(cause);
    let row = FoundationalDiagnosticRow::Decision(FoundationalDiagnosticDecisionRow::new(
        code.clone(),
        scope,
        FoundationalDiagnosticSeverity::Denial,
        subject.clone(),
        foundational_diagnostic_locator_boundary_artifact(identity.locator()),
        outcome,
        FoundationalDiagnosticSemanticLabelSet::new([code]),
        Some(FoundationalDiagnosticDenialClass::PolicyDenied),
        FoundationalDiagnosticLocalityClaim::ExactSubject,
        FoundationalDiagnosticWidenedFalloutPosture::NotWidened,
    ));
    worth_foundational::facade::materialize_diagnostic_explanation_bundle(
        FoundationalDiagnosticExplanationInput::new(
            subject,
            outcome,
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

const fn diagnostic_code(
    cause: WorthQueryApplicationAuthorizationExplanationCause,
) -> &'static str {
    use WorthQueryApplicationAuthorizationExplanationCause as Cause;
    match cause {
        Cause::MissingCapability => "worth.query.authorization.missing-capability",
        Cause::ExplicitPolicyDenial => "worth.query.authorization.explicit-policy-denial",
        Cause::ScopeMismatch => "worth.query.authorization.scope-mismatch",
        Cause::PurposeMismatch => "worth.query.authorization.purpose-mismatch",
        Cause::Conflict => "worth.query.authorization.conflict",
        Cause::SeparationOfDuty => "worth.query.authorization.separation-of-duty",
        Cause::ElevationRequired => "worth.query.authorization.elevation-required",
        Cause::ElevationDenied => "worth.query.authorization.elevation-denied",
        Cause::ElevationExpired => "worth.query.authorization.elevation-expired",
    }
}

const fn diagnostic_outcome(
    cause: WorthQueryApplicationAuthorizationExplanationCause,
) -> FoundationalDiagnosticOutcomeKind {
    use WorthQueryApplicationAuthorizationExplanationCause as Cause;
    match cause {
        Cause::ScopeMismatch | Cause::PurposeMismatch => {
            FoundationalDiagnosticOutcomeKind::Mismatch
        }
        Cause::Conflict | Cause::SeparationOfDuty => FoundationalDiagnosticOutcomeKind::Violation,
        Cause::MissingCapability
        | Cause::ExplicitPolicyDenial
        | Cause::ElevationRequired
        | Cause::ElevationDenied
        | Cause::ElevationExpired => FoundationalDiagnosticOutcomeKind::Denied,
    }
}
