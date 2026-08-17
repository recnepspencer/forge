use worth_ui::facade::intent::{
    UiIntentConcurrencyScope, UiIntentConfirmationContract, UiIntentConsequenceContract,
    UiIntentDeclaration, UiIntentDefinition, UiIntentExecutionProvider, UiIntentMutabilitySource,
    UiIntentOperabilityContract, UiIntentPayloadSource, UiIntentPolicySource,
    UiIntentReadinessSource, UiIntentUnsigned64,
};
use worth_ui_certification::scenario::filesystem_application_lifecycle::FilesystemApplicationLifecycleScenario;
use worth_ui_dsl::{
    WorthUiIntentInteractionFamily, WorthUiIntentInteractionRoute,
    WorthUiRustAuthoredArtifactInput, WorthUiRustAuthoredArtifactInputModule,
};
use worth_ui_host_headless::{UiHeadlessRecorderCapacity, WorthUiHeadlessRecorder};
use worth_ui_runtime::facade::measurement_exchange::UiViewportExtentObservation;

use super::super::types::{ConfirmationIntent, REVISION_FIELD};
use super::{
    ConfirmationFacts, CONFIRMATION_POLICY, DECLARATION, HIT_ONLY, NEITHER, OPERABILITY,
    PAINT_AND_HIT, PAINT_AND_HIT_TOKEN, PAINT_ONLY, PAINT_ONLY_TOKEN, SURFACE,
};

pub(super) fn build<P>(facts: &ConfirmationFacts, provider: P) -> worth_ui::facade::app::WorthUiApp
where
    P: UiIntentExecutionProvider<ConfirmationIntent>,
{
    let host = WorthUiHeadlessRecorder::with_viewport_extent(
        UiHeadlessRecorderCapacity::production_default(),
        UiViewportExtentObservation {
            width: 160.0,
            height: 96.0,
        },
    );
    FilesystemApplicationLifecycleScenario::new("phase-3-confirmation-world")
        .visual_identity_application_builder(host)
        .register_intent_boolean_fact(facts.mutability.clone(), true)
        .unwrap()
        .register_intent_boolean_fact(facts.readiness.clone(), true)
        .unwrap()
        .register_intent_boolean_fact(facts.policy.clone(), true)
        .unwrap()
        .register_intent_boolean_fact(facts.confirmation.clone(), true)
        .unwrap()
        .register_intent_unsigned64_fact(facts.revision.clone(), 1)
        .unwrap()
        .register_intent_definition(UiIntentDefinition::<ConfirmationIntent>::application_effect())
        .unwrap()
        .register_intent_provider(provider)
        .unwrap()
        .with_rust_authored_input(authored_input(facts, false))
        .freeze()
        .expect("confirmation world freezes through production preparation")
}

pub(super) fn replacement_input(facts: &ConfirmationFacts) -> WorthUiRustAuthoredArtifactInput {
    authored_input(facts, true)
}

fn authored_input(
    facts: &ConfirmationFacts,
    replacement: bool,
) -> WorthUiRustAuthoredArtifactInput {
    let declaration = UiIntentDeclaration::<ConfirmationIntent>::activate(DECLARATION)
        .unwrap()
        .bind_payload(
            REVISION_FIELD,
            UiIntentPayloadSource::<UiIntentUnsigned64>::application_fact(&facts.revision),
        )
        .operability_from(
            UiIntentOperabilityContract::new(
                OPERABILITY,
                UiIntentMutabilitySource::application_fact(&facts.mutability),
                UiIntentReadinessSource::application_fact(&facts.readiness),
                UiIntentPolicySource::application_fact(&facts.policy),
            )
            .unwrap(),
        )
        .confirmation(
            UiIntentConfirmationContract::application_fact(
                CONFIRMATION_POLICY,
                &facts.confirmation,
            )
            .unwrap(),
        )
        .concurrency(UiIntentConcurrencyScope::TargetRouteSingleFlight)
        .consequences(UiIntentConsequenceContract::none())
        .into_dsl_spec();
    let confirmation_route = if replacement {
        WorthUiIntentInteractionRoute::product(
            WorthUiIntentInteractionFamily::Activate,
            DECLARATION,
        )
    } else {
        WorthUiIntentInteractionRoute::confirmation(DECLARATION)
    };
    let module = WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
        .with_component(PAINT_ONLY)
        .with_control_routes(
            HIT_ONLY,
            [WorthUiIntentInteractionRoute::product(
                WorthUiIntentInteractionFamily::Activate,
                DECLARATION,
            )],
        )
        .with_control_routes(PAINT_AND_HIT, [confirmation_route])
        .with_component(NEITHER)
        .with_surface(SURFACE)
        .with_token(PAINT_ONLY_TOKEN, "theme.visual_identity.red")
        .with_token(PAINT_AND_HIT_TOKEN, "theme.visual_identity.purple")
        .with_intent_declaration(declaration);
    WorthUiRustAuthoredArtifactInput::from_modules([module])
}
