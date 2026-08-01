use worth_ui::facade::intent::{
    UiIntentConcurrencyScope, UiIntentConfirmationContract, UiIntentConsequenceContract,
    UiIntentDeclaration, UiIntentDefinition, UiIntentMutabilitySource, UiIntentOperabilityContract,
    UiIntentPolicySource, UiIntentReadinessSource,
};
use worth_ui_certification::scenario::filesystem_application_lifecycle::FilesystemApplicationLifecycleScenario;
use worth_ui_dsl::{
    WorthUiIntentInteractionFamily, WorthUiIntentInteractionRoute, WorthUiRustAuthoredArtifactInput,
};

use crate::{
    intent::{
        execution::lifecycle::ScriptedProvider,
        operability::{OperabilityFacts, PrimaryIntent},
    },
    projection_presentation::{
        collection_query::collection_module,
        scalar_query_only::{
            component_descriptor, status_region_descriptor, text_token_descriptor, ACTIVE_COMPONENT,
        },
    },
};

const DECLARATION: &str = "phase4.ia09.intent";
const CONTROL: &str = "visual.identity.component.hit_only";

pub(super) fn build(
    registration: worth_ui_query_binding::UiCollectionProjectionRegistration,
    host: worth_ui_host_egui::WorthUiHostEgui,
    provider: ScriptedProvider,
) -> (worth_ui::facade::app::WorthUiApp, OperabilityFacts) {
    let facts = OperabilityFacts::new();
    let source = source_input(true, &facts);
    let app = FilesystemApplicationLifecycleScenario::new("phase-4-ia-09-ordering")
        .visual_identity_application_builder(host)
        .register_component(component_descriptor(ACTIVE_COMPONENT))
        .register_mosaic_region_kind(status_region_descriptor())
        .register_theme_token(text_token_descriptor())
        .register_collection_projection(registration)
        .expect("the IA-09 collection projection registers")
        .register_intent_boolean_fact(facts.mutability.clone(), true)
        .unwrap()
        .register_intent_boolean_fact(facts.readiness.clone(), true)
        .unwrap()
        .register_intent_boolean_fact(facts.policy.clone(), true)
        .unwrap()
        .register_intent_boolean_fact(facts.confirmation.clone(), false)
        .unwrap()
        .register_intent_definition(UiIntentDefinition::<PrimaryIntent>::application_effect())
        .unwrap()
        .register_intent_provider(provider)
        .unwrap()
        .with_rust_authored_input(source)
        .freeze()
        .expect("the combined ordering world freezes through production preparation");
    (app, facts)
}

pub(super) fn source_input(
    with_region: bool,
    facts: &OperabilityFacts,
) -> WorthUiRustAuthoredArtifactInput {
    let module = collection_module(with_region)
        .with_control_routes_and_authored_identity(
            CONTROL,
            "phase4-ia09-control",
            [WorthUiIntentInteractionRoute::product(
                WorthUiIntentInteractionFamily::Activate,
                DECLARATION,
            )],
        )
        .with_intent_declaration(
            UiIntentDeclaration::<PrimaryIntent>::activate(DECLARATION)
                .unwrap()
                .operability_from(
                    UiIntentOperabilityContract::new(
                        "phase4.ia09.operability",
                        UiIntentMutabilitySource::application_fact(&facts.mutability),
                        UiIntentReadinessSource::application_fact(&facts.readiness),
                        UiIntentPolicySource::application_fact(&facts.policy),
                    )
                    .unwrap(),
                )
                .confirmation(
                    UiIntentConfirmationContract::application_fact(
                        "phase4.ia09.confirmation",
                        &facts.confirmation,
                    )
                    .unwrap(),
                )
                .concurrency(UiIntentConcurrencyScope::TargetRouteSingleFlight)
                .consequences(UiIntentConsequenceContract::none())
                .into_dsl_spec(),
        );
    WorthUiRustAuthoredArtifactInput::from_modules([module])
}

pub(super) fn successor_candidate(
    session: &worth_ui::facade::app::WorthUiActiveApplicationSession,
    facts: &OperabilityFacts,
    run: usize,
) -> worth_ui::facade::source::WorthUiWatchedCandidateSubmission {
    use worth_ui::facade::source::{
        WorthUiSourceEventIngress, WorthUiSourceProvider, WorthUiWatcherEvent,
    };

    let provider = format!("phase-4-ia09-source-{run}");
    let source = WorthUiSourceProvider::rust_authored(&provider)
        .with_rust_authored_input(source_input(false, facts));
    WorthUiSourceEventIngress::new(source)
        .start()
        .ingest([WorthUiWatcherEvent::provider_revision(&provider)])
        .unwrap()
        .attempt_candidate_for_certification(session.capabilities())
        .expect("the IA-09 authored successor lowers through production")
}
