use super::super::super::admission::QuerySubscriptionAdmissionArtifact;
use super::super::super::bridge_lowering::BridgeSubscriptionLoweringPlan;
use super::super::super::certification::SubscriptionLifecycleCertificationBundle;
use super::super::super::declaration::QuerySubscriptionDeclarationArtifact;
use super::super::super::evidence_identities::typed_identity_drift;
use super::super::super::support::QuerySubscriptionSupportReport;
use super::super::super::validation_evidence::validation_evidence_identity_label;
use super::super::bundle::{
    QuerySubscriptionDiagnosticBundleError, QuerySubscriptionDiagnosticBundleErrorKind,
    QuerySubscriptionDiagnosticFailure,
};
use super::super::context::QuerySubscriptionDiagnosticSelectionContext;
use super::super::stage::QuerySubscriptionDiagnosticStage;

pub(super) fn validate_admitted_sources(
    selection: &super::super::super::selection::QuerySubscriptionFamilySelection,
    declaration: &QuerySubscriptionDeclarationArtifact,
    lowering: &BridgeSubscriptionLoweringPlan,
    admission: &QuerySubscriptionAdmissionArtifact,
    support: &QuerySubscriptionSupportReport,
    lifecycle: &SubscriptionLifecycleCertificationBundle,
) -> Result<(), QuerySubscriptionDiagnosticBundleError> {
    if selection.family() != declaration.family() {
        return Err(QuerySubscriptionDiagnosticBundleError::new(
            QuerySubscriptionDiagnosticBundleErrorKind::DeclarationSourceMismatch,
            "admitted diagnostic trace requires declaration and family selection to preserve the same query subscription family",
            &[
                format!("selection_family:{}", selection.family().as_str()),
                format!("declaration_family:{}", declaration.family().as_str()),
            ],
        ));
    }
    if typed_identity_drift(
        declaration.declaration_identity(),
        lowering.query_declaration_identity(),
    ) {
        return Err(QuerySubscriptionDiagnosticBundleError::new(
            QuerySubscriptionDiagnosticBundleErrorKind::BridgeLoweringSourceMismatch,
            "admitted diagnostic trace requires bridge lowering to preserve declaration identity",
            &[
                format!(
                    "declaration:{}",
                    declaration.declaration_projection().label()
                ),
                format!(
                    "lowering:{}",
                    lowering.query_declaration_projection().label()
                ),
            ],
        ));
    }
    if typed_identity_drift(
        declaration.declaration_identity(),
        admission.query_declaration_identity(),
    ) {
        return Err(QuerySubscriptionDiagnosticBundleError::new(
            QuerySubscriptionDiagnosticBundleErrorKind::AdmissionSourceMismatch,
            "admitted diagnostic trace requires admission to preserve declaration identity",
            &[
                format!(
                    "declaration:{}",
                    declaration.declaration_projection().label()
                ),
                format!(
                    "admission:{}",
                    admission.query_declaration_projection().label()
                ),
            ],
        ));
    }
    if typed_identity_drift(
        declaration.declaration_identity(),
        support.support_subject().declaration_identity(),
    ) {
        return Err(QuerySubscriptionDiagnosticBundleError::new(
            QuerySubscriptionDiagnosticBundleErrorKind::SupportSourceMismatch,
            "admitted diagnostic trace requires support reporting to preserve declaration identity",
            &[
                format!(
                    "declaration:{}",
                    declaration.declaration_projection().label()
                ),
                format!(
                    "support_declaration:{}",
                    support.support_subject().declaration_projection().label()
                ),
            ],
        ));
    }
    if typed_identity_drift(
        declaration.declaration_identity(),
        lifecycle.subscription_declaration_identity(),
    ) {
        return Err(QuerySubscriptionDiagnosticBundleError::new(
            QuerySubscriptionDiagnosticBundleErrorKind::LifecycleSourceMismatch,
            "admitted diagnostic trace requires lifecycle certification to preserve declaration identity",
            &[
                format!("declaration:{}", declaration.declaration_projection().label()),
                format!(
                    "lifecycle_declaration:{}",
                    lifecycle.subscription_declaration_projection().label()
                ),
            ],
        ));
    }
    Ok(())
}

pub(super) fn validate_denied_selection_context(
    selection_context: &QuerySubscriptionDiagnosticSelectionContext,
    failure_stage: &QuerySubscriptionDiagnosticStage,
    failure: &QuerySubscriptionDiagnosticFailure,
    carries_later_artifacts: bool,
) -> Result<(), QuerySubscriptionDiagnosticBundleError> {
    if selection_context.is_selection_denied() {
        if !failure_is_selection_stage(*failure_stage) {
            return Err(QuerySubscriptionDiagnosticBundleError::new(
                QuerySubscriptionDiagnosticBundleErrorKind::SelectionContextMismatch,
                "diagnostic trace assembly may only use a selection-denied context for family-selection failures",
                &[
                    format!("selection_context:{}", selection_context.context_projection().label()),
                    format!("failure_stage:{}", failure_stage.as_str()),
                ],
            ));
        }
        if typed_identity_drift(
            &selection_context.source_identity(),
            failure.source_identity(),
        ) {
            return Err(QuerySubscriptionDiagnosticBundleError::new(
                QuerySubscriptionDiagnosticBundleErrorKind::SelectionContextMismatch,
                "diagnostic trace assembly requires the selection-denied context and failure to bind the same canonical source digest",
                &[
                    format!(
                        "selection_source:{}",
                        validation_evidence_identity_label(&selection_context.source_identity())
                    ),
                    format!(
                        "failure_source:{}",
                        validation_evidence_identity_label(failure.source_identity())
                    ),
                ],
            ));
        }
        if carries_later_artifacts {
            return Err(QuerySubscriptionDiagnosticBundleError::new(
                QuerySubscriptionDiagnosticBundleErrorKind::SelectionContextMismatch,
                "diagnostic trace assembly may not attach declaration, lowering, admission, or support artifacts after family-selection denial",
                &[
                    format!("selection_context:{}", selection_context.context_projection().label()),
                    format!("failure_stage:{}", failure_stage.as_str()),
                ],
            ));
        }
    }
    Ok(())
}

pub(super) fn failure_is_selection_stage(stage: QuerySubscriptionDiagnosticStage) -> bool {
    matches!(
        stage,
        QuerySubscriptionDiagnosticStage::FamilySelection
            | QuerySubscriptionDiagnosticStage::ViewMismatch
            | QuerySubscriptionDiagnosticStage::RelationshipProofDrift
    )
}
