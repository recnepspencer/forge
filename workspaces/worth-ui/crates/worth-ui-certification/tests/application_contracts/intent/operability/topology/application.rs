use worth_ui::facade::intent::{UiIntentDefinition, UiIntentExecutionProvider};
use worth_ui_certification::scenario::filesystem_application_lifecycle::FilesystemApplicationLifecycleScenario;
use worth_ui_dsl::WorthUiRustAuthoredArtifactInput;
use worth_ui_host_headless::{UiHeadlessRecorderCapacity, WorthUiHeadlessRecorder};
use worth_ui_runtime::facade::measurement_exchange::UiViewportExtentObservation;

use super::super::facts::OperabilityFacts;
use super::super::intent_types::{PrimaryIntent, SecondaryIntent};

pub(super) struct OperabilityApplicationInput<P> {
    source: WorthUiRustAuthoredArtifactInput,
    primary_provider: P,
}

impl<P> OperabilityApplicationInput<P> {
    pub(super) fn new(source: WorthUiRustAuthoredArtifactInput, primary_provider: P) -> Self {
        Self {
            source,
            primary_provider,
        }
    }
}

pub(super) fn prepare<P>(
    input: OperabilityApplicationInput<P>,
    facts: &OperabilityFacts,
) -> worth_ui::facade::app::WorthUiApp
where
    P: UiIntentExecutionProvider<PrimaryIntent>,
{
    let scenario = FilesystemApplicationLifecycleScenario::new("phase-3-operability-world");
    let host = WorthUiHeadlessRecorder::with_viewport_extent(
        UiHeadlessRecorderCapacity::production_default(),
        UiViewportExtentObservation {
            width: 160.0,
            height: 96.0,
        },
    );
    let builder = scenario
        .visual_identity_application_builder(host)
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
        .register_intent_provider(input.primary_provider)
        .unwrap()
        .register_intent_definition(UiIntentDefinition::<SecondaryIntent>::application_effect())
        .unwrap()
        .register_intent_provider(
            worth_ui_certification::WorthUiCertificationBeforeEffectProvider::<SecondaryIntent>::new(),
        )
        .unwrap();
    builder
        .with_rust_authored_input(input.source)
        .freeze()
        .expect("operability world compiles through production preparation")
}
