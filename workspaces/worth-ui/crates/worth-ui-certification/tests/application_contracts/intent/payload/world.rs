use std::sync::Arc;

use worth_ui::facade::intent::{
    UiIntent, UiIntentApplicationFact, UiIntentBoolean, UiIntentDeclaration, UiIntentDefinition,
    UiIntentText, UiIntentUnsigned64,
};
use worth_ui_dsl::{
    WorthUiIntentInteractionFamily, WorthUiIntentInteractionRoute,
    WorthUiRustAuthoredArtifactInput, WorthUiRustAuthoredArtifactInputModule,
};
use worth_ui_query_binding::{
    UiCollectionProjectionRegistration, UiProjectionInputSlot, UiScalarProjectionRegistration,
    WorthUiQueryBindingPlan,
};
use worth_ui_runtime::facade::host::{UiHeadlessRecorderCapacity, WorthUiHeadlessRecorder};
use worth_ui_runtime::facade::measurement_exchange::UiViewportExtentObservation;
use worth_ui_runtime::facade::mounted::UiHostSurfacePresentationMode;

use super::super::super::filesystem_mounted_world::{
    component_graph_nodes, launch_mounted_components,
};
use super::super::interaction_world::InteractionWorld;
use worth_ui_certification::scenario::filesystem_application_lifecycle::FilesystemApplicationLifecycleScenario;

pub(super) const DECLARATION: &str = "phase3.payload.route";
const PAINT_ONLY: &str = "visual.identity.component.paint_only";
const HIT_ONLY: &str = "visual.identity.component.hit_only";
const PAINT_AND_HIT: &str = "visual.identity.component.paint_and_hit";
const NEITHER: &str = "visual.identity.component.neither";
const SURFACE: &str = "visual.identity.surface.main";
const PAINT_ONLY_TOKEN: &str = "theme.visual_identity.paint_only";
const PAINT_AND_HIT_TOKEN: &str = "theme.visual_identity.paint_and_hit";

pub(super) struct PayloadWorld {
    pub(super) interaction: InteractionWorld,
    pub(super) projection_slot: Option<UiProjectionInputSlot>,
}

#[derive(Clone)]
pub(super) enum PayloadProjectionRegistration {
    None,
    Scalar(UiScalarProjectionRegistration),
    Collection(UiCollectionProjectionRegistration),
}

#[derive(Clone, Default)]
pub(super) struct PayloadApplicationFacts {
    text: Option<(UiIntentApplicationFact<UiIntentText>, Arc<str>)>,
    boolean: Option<(UiIntentApplicationFact<UiIntentBoolean>, bool)>,
    unsigned64: Option<(UiIntentApplicationFact<UiIntentUnsigned64>, u64)>,
}

impl PayloadApplicationFacts {
    pub(super) fn standard(
        text: UiIntentApplicationFact<UiIntentText>,
        boolean: UiIntentApplicationFact<UiIntentBoolean>,
        unsigned64: UiIntentApplicationFact<UiIntentUnsigned64>,
    ) -> Self {
        Self {
            text: Some((text, Arc::from("application-current"))),
            boolean: Some((boolean, true)),
            unsigned64: Some((unsigned64, 42)),
        }
    }
}

pub(super) fn launch<I: UiIntent>(
    input: WorthUiRustAuthoredArtifactInput,
    projection: PayloadProjectionRegistration,
    facts: PayloadApplicationFacts,
) -> PayloadWorld {
    let host = WorthUiHeadlessRecorder::with_viewport_extent(
        UiHeadlessRecorderCapacity::production_default(),
        UiViewportExtentObservation {
            width: 160.0,
            height: 96.0,
        },
    );
    let scenario = FilesystemApplicationLifecycleScenario::new("phase-3-payload-world");
    let projection_slot = projection_slot(&projection);
    let builder = scenario
        .visual_identity_application_builder(host)
        .register_intent_definition(UiIntentDefinition::<I>::application_effect())
        .expect("typed payload definition registers");
    let builder = register_projection(builder, projection);
    let builder = register_facts(builder, facts);
    let application = builder
        .with_rust_authored_input(input)
        .freeze()
        .expect("payload world compiles through production application preparation");
    let component_nodes = component_graph_nodes(&application);
    let session = launch_mounted_components(
        application,
        component_nodes,
        UiHostSurfacePresentationMode::RecordOnly,
    );
    PayloadWorld {
        interaction: InteractionWorld::from_session(session),
        projection_slot,
    }
}

pub(super) fn routed_input<I: UiIntent>(
    declaration: UiIntentDeclaration<I>,
    family: WorthUiIntentInteractionFamily,
) -> WorthUiRustAuthoredArtifactInput {
    let module = WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
        .with_component(PAINT_ONLY)
        .with_control_routes(
            HIT_ONLY,
            [WorthUiIntentInteractionRoute::product(family, DECLARATION)],
        )
        .with_component(PAINT_AND_HIT)
        .with_component(NEITHER)
        .with_surface(SURFACE)
        .with_token(PAINT_ONLY_TOKEN, "theme.visual_identity.red")
        .with_token(PAINT_AND_HIT_TOKEN, "theme.visual_identity.purple")
        .with_intent_declaration(declaration.into_dsl_spec());
    WorthUiRustAuthoredArtifactInput::from_modules([module])
}

fn projection_slot(projection: &PayloadProjectionRegistration) -> Option<UiProjectionInputSlot> {
    let plan = match projection {
        PayloadProjectionRegistration::None => return None,
        PayloadProjectionRegistration::Scalar(registration) => WorthUiQueryBindingPlan::default()
            .register_scalar_projection(registration.clone())
            .expect("scenario scalar projection produces one installed plan"),
        PayloadProjectionRegistration::Collection(registration) => {
            WorthUiQueryBindingPlan::default()
                .register_collection_projection(registration.clone())
                .expect("scenario collection projection produces one installed plan")
        }
    };
    let identity = match projection {
        PayloadProjectionRegistration::None => unreachable!(),
        PayloadProjectionRegistration::Scalar(registration) => registration.view().identity(),
        PayloadProjectionRegistration::Collection(registration) => registration.view().identity(),
    };
    plan.projection_input_slot(identity)
}

fn register_projection(
    builder: worth_ui::facade::app::WorthUiApplicationBuilder,
    projection: PayloadProjectionRegistration,
) -> worth_ui::facade::app::WorthUiApplicationBuilder {
    match projection {
        PayloadProjectionRegistration::None => builder,
        PayloadProjectionRegistration::Scalar(registration) => builder
            .register_scalar_projection(registration)
            .expect("scenario scalar projection registers"),
        PayloadProjectionRegistration::Collection(registration) => builder
            .register_collection_projection(registration)
            .expect("scenario collection projection registers"),
    }
}

fn register_facts(
    mut builder: worth_ui::facade::app::WorthUiApplicationBuilder,
    facts: PayloadApplicationFacts,
) -> worth_ui::facade::app::WorthUiApplicationBuilder {
    if let Some((fact, initial)) = facts.text {
        builder = builder
            .register_intent_text_fact(fact, initial)
            .expect("scenario text fact registers");
    }
    if let Some((fact, initial)) = facts.boolean {
        builder = builder
            .register_intent_boolean_fact(fact, initial)
            .expect("scenario boolean fact registers");
    }
    if let Some((fact, initial)) = facts.unsigned64 {
        builder = builder
            .register_intent_unsigned64_fact(fact, initial)
            .expect("scenario unsigned fact registers");
    }
    builder
}
