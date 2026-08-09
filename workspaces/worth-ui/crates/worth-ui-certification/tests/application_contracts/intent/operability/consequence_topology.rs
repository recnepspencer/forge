use worth_ui::facade::intent::{
    UiIntentConcurrencyScope, UiIntentConfirmationContract, UiIntentConsequenceContract,
    UiIntentDeclaration, UiIntentDefinition, UiIntentMutabilitySource, UiIntentOperabilityContract,
    UiIntentPolicySource, UiIntentReadinessSource,
};
use worth_ui::facade::rebind::UiChangeProfile;
use worth_ui_certification::scenario::filesystem_application_lifecycle::FilesystemApplicationLifecycleScenario;
use worth_ui_dsl::WorthUiIntentInteractionFamily;
use worth_ui_host_headless::WorthUiHeadlessRecorder;

use super::facts::OperabilityFacts;
use super::intent_types::ConsequenceIntent;
use super::topology::{module, PRIMARY_DECLARATION};

const CONTRACT: &str = "phase3.operability.contract";
const CONFIRMATION_POLICY: &str = "phase3.operability.confirmation-policy";

pub(in crate::intent) fn build_consequence_with_provider<P>(
    provider: P,
    view: worth_ui_query_binding::WorthUiInstalledLiveQueryView,
    consequences: UiIntentConsequenceContract,
    host: WorthUiHeadlessRecorder,
) -> (worth_ui::facade::app::WorthUiApp, OperabilityFacts)
where
    P: worth_ui::facade::intent::UiIntentExecutionProvider<ConsequenceIntent>,
{
    build_consequence_with_provider_and_profile(
        provider,
        view,
        consequences,
        host,
        UiChangeProfile::platform_pulse(),
    )
}

pub(in crate::intent) fn build_consequence_with_provider_and_profile<P>(
    provider: P,
    view: worth_ui_query_binding::WorthUiInstalledLiveQueryView,
    consequences: UiIntentConsequenceContract,
    host: WorthUiHeadlessRecorder,
    profile: UiChangeProfile,
) -> (worth_ui::facade::app::WorthUiApp, OperabilityFacts)
where
    P: worth_ui::facade::intent::UiIntentExecutionProvider<ConsequenceIntent>,
{
    let facts = OperabilityFacts::new();
    let input = module(
        PRIMARY_DECLARATION,
        PRIMARY_DECLARATION,
        WorthUiIntentInteractionFamily::Activate,
        [consequence_declaration(
            &facts,
            consequences,
            UiIntentConcurrencyScope::TargetRouteSingleFlight,
        )],
    );
    let app = FilesystemApplicationLifecycleScenario::new("phase-4-consequence-world")
        .semantic_text_action_application_builder_with_change_profile(host, profile)
        .register_intent_boolean_fact(facts.mutability.clone(), true)
        .unwrap()
        .register_intent_boolean_fact(facts.readiness.clone(), true)
        .unwrap()
        .register_intent_boolean_fact(facts.policy.clone(), true)
        .unwrap()
        .register_intent_boolean_fact(facts.confirmation.clone(), false)
        .unwrap()
        .register_query_view(view)
        .unwrap()
        .register_intent_definition(UiIntentDefinition::<ConsequenceIntent>::application_effect())
        .unwrap()
        .register_intent_provider(provider)
        .unwrap()
        .with_rust_authored_input(input)
        .freeze()
        .expect("consequence world compiles through production preparation");
    (app, facts)
}

pub(in crate::intent) fn consequence_replacement_input(
    facts: &OperabilityFacts,
    consequences: UiIntentConsequenceContract,
) -> worth_ui_dsl::WorthUiRustAuthoredArtifactInput {
    module(
        PRIMARY_DECLARATION,
        PRIMARY_DECLARATION,
        WorthUiIntentInteractionFamily::Activate,
        [consequence_declaration(
            facts,
            consequences,
            UiIntentConcurrencyScope::ApplicationSingleFlight,
        )],
    )
}

fn consequence_declaration(
    facts: &OperabilityFacts,
    consequences: UiIntentConsequenceContract,
    concurrency: UiIntentConcurrencyScope,
) -> worth_ui_dsl::WorthUiIntentDeclarationSpec {
    UiIntentDeclaration::<ConsequenceIntent>::activate(PRIMARY_DECLARATION)
        .unwrap()
        .operability_from(
            UiIntentOperabilityContract::new(
                CONTRACT,
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
        .concurrency(concurrency)
        .consequences(consequences)
        .into_dsl_spec()
}
