use worth_query::facade::foundation::worth_ui_query_binding_evidence_identity;
use worth_query::facade::runtime::{
    WorthQueryRuntimeAsyncResultState, WorthQueryRuntimeAsyncResultStateKind,
};
use worth_ui::facade::{
    RuntimeOutcomeAffordance, RuntimeOutcomeDenialPosture, RuntimeOutcomeFamily,
    RuntimeOutcomePresentation, RuntimeOutcomeProjectionDescriptor, RuntimeOutcomeProjectionId,
    RuntimeOutcomeSourceReference, RuntimeOutcomeTone, WorthUi,
};

fn main() {
    let state = WorthQueryRuntimeAsyncResultState::new(
        WorthQueryRuntimeAsyncResultStateKind::Denied,
        &worth_ui_query_binding_evidence_identity(
            "runtime-outcome-fixture",
            &["causality.digest".to_string()],
        ),
        &worth_ui_query_binding_evidence_identity(
            "runtime-outcome-fixture",
            &["basis.digest".to_string()],
        ),
        &worth_ui_query_binding_evidence_identity(
            "runtime-outcome-fixture",
            &["generation.digest".to_string()],
        ),
    );

    let _app = WorthUi::app()
        .register_runtime_outcome_projection(
            RuntimeOutcomeProjectionDescriptor::new(
                RuntimeOutcomeProjectionId::new("workspace.outcome.denied").unwrap(),
                RuntimeOutcomeFamily::denied(),
                RuntimeOutcomeSourceReference::from_query_async_result_state(&state),
            )
            .with_presentation(
                RuntimeOutcomePresentation::new()
                    .with_label("Needs permission")
                    .with_tone(RuntimeOutcomeTone::blocking())
                    .with_affordance(RuntimeOutcomeAffordance::recoverable_action()),
            )
            .with_denial_posture(RuntimeOutcomeDenialPosture::structured_status()),
        )
        .freeze();
}
