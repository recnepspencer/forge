use super::super::super::evidence_identities::typed_identity_drift;
use super::super::super::validation_evidence::validation_evidence_identity_label;
use super::super::context::QuerySubscriptionDiagnosticSelectionContext;
use super::super::stage::QuerySubscriptionDiagnosticStage;
use super::failure::{
    QuerySubscriptionDiagnosticBundleError, QuerySubscriptionDiagnosticBundleErrorKind,
    QuerySubscriptionDiagnosticFailure,
};

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
                "diagnostic bundle assembly may only use a selection-denied context for family-selection failures",
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
                "diagnostic bundle assembly requires the selection-denied context and failure to bind the same canonical source digest",
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
                "diagnostic bundle assembly may not attach declaration, lowering, or support artifacts after family-selection denial",
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
