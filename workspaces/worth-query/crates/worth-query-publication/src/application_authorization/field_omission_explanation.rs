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

pub(super) fn materialize_field_omission_explanation(
    identity: WorthQueryApplicationAuthorizationBoundaryIdentity,
    profile: FoundationalProfileSet,
) -> Result<FoundationalDiagnosticExplanationBundle, FoundationalDiagnosticMaterializationDenial> {
    let code = FoundationalDiagnosticCodeId::new("worth.query.disclosure.field-omission")
        .expect("static Query field-omission diagnostic code is valid");
    let scope = FoundationalDiagnosticScopeId::new("worth.query.application-authorization")
        .expect("static Query publication diagnostic scope is valid");
    let subject = foundational_diagnostic_boundary_artifact_subject(
        identity.artifact_id(),
        identity.locator().field(),
    );
    let outcome = FoundationalDiagnosticOutcomeKind::Partial;
    let row = FoundationalDiagnosticRow::Decision(FoundationalDiagnosticDecisionRow::new(
        code.clone(),
        scope,
        FoundationalDiagnosticSeverity::Advisory,
        subject.clone(),
        foundational_diagnostic_locator_boundary_artifact(identity.locator()),
        outcome,
        FoundationalDiagnosticSemanticLabelSet::new([code]),
        None,
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
