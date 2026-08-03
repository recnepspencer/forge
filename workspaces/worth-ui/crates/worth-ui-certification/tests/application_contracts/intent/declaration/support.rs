use worth_ui::facade::app::{WorthUi, WorthUiApp, WorthUiApplicationPreparationDenial};
use worth_ui::facade::declaration::{
    ComponentChildPolicy, ComponentDescriptor, ComponentId, ComponentPropSchema,
    ComponentStateOwnership,
};
use worth_ui::facade::intent::{
    UiIntent, UiIntentAcceptedInteractions, UiIntentDeclaration, UiIntentId, UiIntentPayload,
    UiIntentProductOutcome, UiIntentSchema, UiSemanticInteractionFamily,
};
use worth_ui_certification::scenario::filesystem_application_lifecycle::FilesystemApplicationLifecycleScenario;
use worth_ui_dsl::{
    WorthUiIntentDeclarationSpec, WorthUiIntentInteractionFamily, WorthUiIntentInteractionRoute,
    WorthUiRustAuthoredArtifactInput, WorthUiRustAuthoredArtifactInputModule,
};

use super::super::super::filesystem_mounted_world::{
    launch_file_intent_world, launch_rust_intent_world,
};
use super::super::interaction_world::InteractionWorld;

pub(super) const DECLARATION: &str = "platform.pulse.advance.route";
pub(super) const DEFINITION: &str = "platform.pulse.advance";
const PAINT_ONLY: &str = "visual.identity.component.paint_only";
const HIT_ONLY: &str = "visual.identity.component.hit_only";
const PAINT_AND_HIT: &str = "visual.identity.component.paint_and_hit";
const NEITHER: &str = "visual.identity.component.neither";
const SURFACE: &str = "visual.identity.surface.main";
const PAINT_ONLY_TOKEN: &str = "theme.visual_identity.paint_only";
const PAINT_AND_HIT_TOKEN: &str = "theme.visual_identity.paint_and_hit";

pub(super) struct AdvancePayload;

impl UiIntentPayload for AdvancePayload {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("platform.pulse.advance_payload", 1);
    const FIELDS: worth_ui::facade::intent::UiIntentPayloadFieldSet =
        worth_ui::facade::intent::UiIntentPayloadFieldSet::EMPTY;

    fn project(
        _fields: &mut worth_ui::facade::intent::UiIntentPayloadProjection<Self>,
    ) -> Result<Self, worth_ui::facade::intent::UiIntentPayloadProjectionViolation> {
        Ok(Self)
    }
}

pub(super) struct AdvanceOutcome;

impl UiIntentProductOutcome for AdvanceOutcome {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("platform.pulse.advance_outcome", 1);
}

pub(super) struct AdvanceStatus;

impl UiIntent for AdvanceStatus {
    type Payload = AdvancePayload;
    type ProductOutcome = AdvanceOutcome;

    const ID: UiIntentId = UiIntentId::stable(DEFINITION);
    const ACCEPTED_INTERACTIONS: UiIntentAcceptedInteractions =
        UiIntentAcceptedInteractions::new(&[UiSemanticInteractionFamily::Activate]);
}

pub(crate) fn file_world() -> InteractionWorld {
    InteractionWorld::from_session(launch_file_intent_world::<AdvanceStatus>(
        &routed_file_source(),
    ))
}

pub(super) fn rust_world() -> InteractionWorld {
    InteractionWorld::from_session(launch_rust_intent_world::<AdvanceStatus>(
        routed_rust_input(),
    ))
}

pub(super) fn confirmation_file_world() -> InteractionWorld {
    let source = FilesystemApplicationLifecycleScenario::visual_identity_source_text();
    let source = attach_file_confirmation_route(source, HIT_ONLY);
    let source = format!(
        "{source}intent {DECLARATION} {{ definition {DEFINITION}; interaction activate; }}\n"
    );
    InteractionWorld::from_session(launch_file_intent_world::<AdvanceStatus>(&source))
}

pub(super) fn confirmation_rust_world() -> InteractionWorld {
    let declaration = UiIntentDeclaration::<AdvanceStatus>::activate(DECLARATION)
        .expect("typed declaration accepts activation")
        .into_dsl_spec();
    let module = WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
        .with_component(PAINT_ONLY)
        .with_control_routes(
            HIT_ONLY,
            [WorthUiIntentInteractionRoute::confirmation(DECLARATION)],
        )
        .with_component(PAINT_AND_HIT)
        .with_component(NEITHER)
        .with_surface(SURFACE)
        .with_token(PAINT_ONLY_TOKEN, "theme.visual_identity.red")
        .with_token(PAINT_AND_HIT_TOKEN, "theme.visual_identity.purple")
        .with_intent_declaration(declaration);
    let input = WorthUiRustAuthoredArtifactInput::from_modules([module]);
    InteractionWorld::from_session(launch_rust_intent_world::<AdvanceStatus>(input))
}

pub(super) fn routed_file_source() -> String {
    let source = FilesystemApplicationLifecycleScenario::visual_identity_source_text();
    let source = attach_file_route(source, HIT_ONLY);
    let source = attach_file_route(source, PAINT_AND_HIT);
    format!("{source}intent {DECLARATION} {{ definition {DEFINITION}; interaction activate; }}\n")
}

fn attach_file_route(source: String, component: &str) -> String {
    let empty = format!("component {component} {{}}");
    let routed = format!("component {component} {{ interaction activate routes {DECLARATION}; }}");
    let changed = source.replace(&empty, &routed);
    assert_ne!(
        source, changed,
        "canonical visual source must contain {component}"
    );
    changed
}

fn attach_file_confirmation_route(source: String, component: &str) -> String {
    let empty = format!("component {component} {{}}");
    let routed =
        format!("component {component} {{ interaction activate confirms {DECLARATION}; }}");
    let changed = source.replace(&empty, &routed);
    assert_ne!(
        source, changed,
        "canonical visual source must contain {component}"
    );
    changed
}

pub(super) fn routed_rust_input() -> WorthUiRustAuthoredArtifactInput {
    let route = || {
        WorthUiIntentInteractionRoute::product(
            WorthUiIntentInteractionFamily::Activate,
            DECLARATION,
        )
    };
    let declaration = UiIntentDeclaration::<AdvanceStatus>::activate(DECLARATION)
        .expect("typed declaration accepts activation")
        .into_dsl_spec();
    let module = WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
        .with_component(PAINT_ONLY)
        .with_control_routes(HIT_ONLY, [route()])
        .with_control_routes(PAINT_AND_HIT, [route()])
        .with_component(NEITHER)
        .with_surface(SURFACE)
        .with_token(PAINT_ONLY_TOKEN, "theme.visual_identity.red")
        .with_token(PAINT_AND_HIT_TOKEN, "theme.visual_identity.purple")
        .with_intent_declaration(declaration);
    WorthUiRustAuthoredArtifactInput::from_modules([module])
}

pub(super) fn freeze_module(
    module: WorthUiRustAuthoredArtifactInputModule,
) -> Result<WorthUiApp, WorthUiApplicationPreparationDenial> {
    WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .register_component(ComponentDescriptor::new(
            ComponentId::new("routed.control").expect("fixture component identity is valid"),
            ComponentPropSchema::named("routed.control.props"),
            ComponentChildPolicy::no_children(),
            ComponentStateOwnership::runtime_owned(),
        ))
        .register_intent_definition(
            worth_ui::facade::intent::UiIntentDefinition::<AdvanceStatus>::application_effect(),
        )
        .expect("test definition should register")
        .with_rust_authored_input(WorthUiRustAuthoredArtifactInput::from_modules([module]))
        .freeze()
}

pub(super) fn declaration(
    identity: &str,
    definition: &str,
    family: WorthUiIntentInteractionFamily,
) -> WorthUiIntentDeclarationSpec {
    WorthUiIntentDeclarationSpec::new(identity, definition, family)
}

pub(super) fn component_with_routes(
    routes: impl IntoIterator<Item = WorthUiIntentInteractionRoute>,
) -> WorthUiRustAuthoredArtifactInputModule {
    WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
        .with_control_routes("routed.control", routes)
}
