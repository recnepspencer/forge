use forge_query::facade::{
    ForgeQueryRuntimeAsyncResultState, ForgeQueryRuntimeAsyncResultStateKind,
};
use worth_ui::facade::{
    RuntimeOutcomeAffordance, RuntimeOutcomeDenialPosture, RuntimeOutcomeFamily,
    RuntimeOutcomePresentation, RuntimeOutcomeProjectionDescriptor, RuntimeOutcomeProjectionId,
    RuntimeOutcomeSourceReference, RuntimeOutcomeTone, WorthUi,
};

fn main() {
    let state = ForgeQueryRuntimeAsyncResultState::new(
        ForgeQueryRuntimeAsyncResultStateKind::Denied,
        "causality.digest",
        "basis.digest",
        "generation.digest",
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
