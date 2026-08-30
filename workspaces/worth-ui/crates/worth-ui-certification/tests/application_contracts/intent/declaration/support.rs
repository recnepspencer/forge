use worth_ui::facade::app::{WorthUi, WorthUiApp, WorthUiApplicationPreparationDenial};
use worth_ui::facade::declaration::{
    ComponentChildPolicy, ComponentDescriptor, ComponentId, ComponentPropSchema,
    ComponentStateOwnership,
};
use worth_ui::facade::intent::{
    UiIntent, UiIntentAcceptedInteractions, UiIntentConcurrencyScope, UiIntentConfirmationContract,
    UiIntentConsequenceContract, UiIntentDeclaration, UiIntentId, UiIntentMutabilitySource,
    UiIntentOperabilityContract, UiIntentPayload, UiIntentPolicySource, UiIntentProductOutcome,
    UiIntentReadinessSource, UiIntentSchema, UiSemanticInteractionFamily,
};
use worth_ui_certification::scenario::filesystem_application_lifecycle::FilesystemApplicationLifecycleScenario;
use worth_ui_dsl::{
    WorthUiIntentDeclarationSpec, WorthUiIntentInteractionFamily, WorthUiIntentInteractionRoute,
    WorthUiRustAuthoredArtifactInput, WorthUiRustAuthoredArtifactInputModule,
};

use super::super::super::filesystem_mounted_world::{
    intent_world_operability_fact, launch_file_intent_world, launch_rust_intent_world,
    INTENT_WORLD_OPERABILITY_FACT,
};
use super::super::interaction_world::InteractionWorld;

pub(super) const DECLARATION: &str = "platform.pulse.advance.route";
pub(super) const DEFINITION: &str = "platform.pulse.advance";
const OPERABILITY: &str = "platform.pulse.advance.operability";
const CONFIRMATION: &str = "platform.pulse.advance.confirmation";
const PAINT_ONLY: &str = "visual.identity.component.paint_only";
const HIT_ONLY: &str = "visual.identity.component.hit_only";
const PAINT_AND_HIT: &str = "visual.identity.component.paint_and_hit";
const NEITHER: &str = "visual.identity.component.neither";
const SURFACE: &str = "visual.identity.surface.main";
const PAINT_ONLY_TOKEN: &str = "theme.visual_identity.paint_only";
const PAINT_AND_HIT_TOKEN: &str = "theme.visual_identity.paint_and_hit";

pub(in crate::intent) struct AdvancePayload;

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

pub(in crate::intent) struct AdvanceOutcome;

impl UiIntentProductOutcome for AdvanceOutcome {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("platform.pulse.advance_outcome", 1);
    const CONSEQUENCE_FAMILIES: worth_ui::facade::intent::UiIntentProductConsequenceFamilies =
        worth_ui::facade::intent::UiIntentProductConsequenceFamilies::NONE;

    fn into_consequences(self) -> worth_ui::facade::intent::UiIntentProductConsequences {
        worth_ui::facade::intent::UiIntentProductConsequences::none()
    }
}

pub(in crate::intent) struct AdvanceStatus;

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
        "{source}{}\n",
        file_declaration(DECLARATION, DEFINITION, "activate")
    );
    InteractionWorld::from_session(launch_file_intent_world::<AdvanceStatus>(&source))
}

pub(super) fn confirmation_rust_world() -> InteractionWorld {
    let declaration = UiIntentDeclaration::<AdvanceStatus>::activate(DECLARATION)
        .expect("typed declaration accepts activation");
    let declaration = bind_operability(declaration);
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
    format!(
        "{source}{}\n",
        file_declaration(DECLARATION, DEFINITION, "activate")
    )
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
        .expect("typed declaration accepts activation");
    let declaration = bind_operability(declaration);
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

pub(in crate::intent) fn routed_command_input() -> WorthUiRustAuthoredArtifactInput {
    routed_command_input_with_relocated_route(false)
}

pub(in crate::intent) fn routed_command_replacement_input() -> WorthUiRustAuthoredArtifactInput {
    routed_command_input_with_relocated_route(true)
}

fn routed_command_input_with_relocated_route(
    replacement: bool,
) -> WorthUiRustAuthoredArtifactInput {
    let declaration = UiIntentDeclaration::<AdvanceStatus>::activate(DECLARATION)
        .expect("typed declaration accepts command activation");
    let route = WorthUiIntentInteractionRoute::product(
        WorthUiIntentInteractionFamily::Activate,
        DECLARATION,
    );
    let module =
        WorthUiRustAuthoredArtifactInputModule::new("app/main.wui").with_component(PAINT_ONLY);
    let module = if replacement {
        module
            .with_component(HIT_ONLY)
            .with_control_routes(PAINT_AND_HIT, [route])
    } else {
        module
            .with_control_routes(HIT_ONLY, [route])
            .with_component(PAINT_AND_HIT)
    };
    let module = module
        .with_component(NEITHER)
        .with_surface(SURFACE)
        .with_token(PAINT_ONLY_TOKEN, "theme.visual_identity.red")
        .with_token(PAINT_AND_HIT_TOKEN, "theme.visual_identity.purple")
        .with_intent_declaration(bind_operability(declaration));
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
        .register_intent_provider(
            worth_ui_certification::WorthUiCertificationBeforeEffectProvider::<AdvanceStatus>::new(
            ),
        )
        .expect("test provider should register")
        .register_intent_boolean_fact(intent_world_operability_fact(), true)
        .expect("test operability fact should register")
        .with_rust_authored_input(WorthUiRustAuthoredArtifactInput::from_modules([module]))
        .freeze()
        .map(
            worth_ui_runtime::facade::entry::WorthUiCertificationApplicationTransition::activate_headless,
        )
}

pub(super) fn declaration(
    identity: &str,
    definition: &str,
    family: WorthUiIntentInteractionFamily,
) -> WorthUiIntentDeclarationSpec {
    WorthUiIntentDeclarationSpec::new(
        identity,
        definition,
        family,
        worth_ui_dsl::WorthUiIntentOperabilityContractSpec::new(
            OPERABILITY,
            worth_ui_dsl::WorthUiIntentMutabilitySourceSpec::application_boolean(
                INTENT_WORLD_OPERABILITY_FACT,
            ),
            worth_ui_dsl::WorthUiIntentReadinessSourceSpec::application_boolean(
                INTENT_WORLD_OPERABILITY_FACT,
            ),
            worth_ui_dsl::WorthUiIntentPolicySourceSpec::application_boolean(
                INTENT_WORLD_OPERABILITY_FACT,
            ),
        ),
        worth_ui_dsl::WorthUiIntentConfirmationContractSpec::not_required(CONFIRMATION),
        worth_ui_dsl::WorthUiIntentConcurrencyScope::TargetRouteSingleFlight,
        worth_ui_dsl::WorthUiIntentConsequenceContractSpec::none(),
    )
}

pub(super) fn component_with_routes(
    routes: impl IntoIterator<Item = WorthUiIntentInteractionRoute>,
) -> WorthUiRustAuthoredArtifactInputModule {
    WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
        .with_control_routes("routed.control", routes)
}

fn bind_operability<I: UiIntent>(
    declaration: UiIntentDeclaration<I>,
) -> WorthUiIntentDeclarationSpec {
    let fact = intent_world_operability_fact();
    declaration
        .operability_from(
            UiIntentOperabilityContract::new(
                OPERABILITY,
                UiIntentMutabilitySource::application_fact(&fact),
                UiIntentReadinessSource::application_fact(&fact),
                UiIntentPolicySource::application_fact(&fact),
            )
            .expect("test operability identity is valid"),
        )
        .confirmation(
            UiIntentConfirmationContract::not_required(CONFIRMATION)
                .expect("test confirmation identity is valid"),
        )
        .concurrency(UiIntentConcurrencyScope::TargetRouteSingleFlight)
        .consequences(UiIntentConsequenceContract::none())
        .into_dsl_spec()
}

fn file_declaration(identity: &str, definition: &str, interaction: &str) -> String {
    format!(
        "intent {identity} {{ definition {definition}; interaction {interaction}; \
         operability {OPERABILITY} mutability-application-boolean {INTENT_WORLD_OPERABILITY_FACT} \
         readiness-application-boolean {INTENT_WORLD_OPERABILITY_FACT} \
         policy-application-boolean {INTENT_WORLD_OPERABILITY_FACT}; \
         confirmation {CONFIRMATION} not-required; \
         concurrency target-route-single-flight; consequences none; }}"
    )
}
