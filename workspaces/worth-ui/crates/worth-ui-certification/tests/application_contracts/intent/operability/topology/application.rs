use worth_ui::facade::intent::{
    UiIntentDefinition, UiIntentExecutionProvider, UiIntentRuntimeServiceDestination,
};
use worth_ui_certification::scenario::filesystem_application_lifecycle::FilesystemApplicationLifecycleScenario;
use worth_ui_dsl::WorthUiRustAuthoredArtifactInput;
use worth_ui_runtime::facade::host::{UiHeadlessRecorderCapacity, WorthUiHeadlessRecorder};
use worth_ui_runtime::facade::measurement_exchange::UiViewportExtentObservation;

use super::super::facts::OperabilityFacts;
use super::super::intent_types::{
    PrimaryIntent, ProjectionIntent, SecondaryIntent, UnsupportedIntent,
};

pub(super) struct OperabilityApplicationInput<P> {
    source: WorthUiRustAuthoredArtifactInput,
    primary_provider: P,
    unsupported: bool,
    projection: Option<worth_ui_query_binding::UiScalarProjectionRegistration>,
}

impl<P> OperabilityApplicationInput<P> {
    pub(super) fn new(source: WorthUiRustAuthoredArtifactInput, primary_provider: P) -> Self {
        Self {
            source,
            primary_provider,
            unsupported: false,
            projection: None,
        }
    }

    pub(super) fn with_unsupported_definition(mut self) -> Self {
        self.unsupported = true;
        self
    }

    pub(super) fn with_projection(
        mut self,
        projection: worth_ui_query_binding::UiScalarProjectionRegistration,
    ) -> Self {
        self.projection = Some(projection);
        self
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
        .unwrap()
        .register_intent_definition(UiIntentDefinition::<ProjectionIntent>::application_effect())
        .unwrap()
        .register_intent_provider(
            worth_ui_certification::WorthUiCertificationBeforeEffectProvider::<ProjectionIntent>::new(),
        )
        .unwrap();
    let builder = if input.unsupported {
        builder
            .register_unsupported_intent_definition(
                UiIntentDefinition::<UnsupportedIntent>::runtime_service(
                    UiIntentRuntimeServiceDestination::InvokeCommand,
                ),
            )
            .unwrap()
    } else {
        builder
    };
    let builder = match input.projection {
        Some(registration) => builder
            .register_scalar_projection(registration)
            .expect("operability projection registers"),
        None => builder,
    };
    builder
        .with_rust_authored_input(input.source)
        .freeze()
        .expect("operability world compiles through production preparation")
}
