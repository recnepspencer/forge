use worth_ui::facade::intent::{
    UiIntent, UiIntentConcurrencyScope, UiIntentConfirmationContract, UiIntentConsequenceContract,
    UiIntentDeclaration, UiIntentDefinition, UiIntentMutabilitySource, UiIntentOperabilityContract,
    UiIntentPayloadSource, UiIntentPolicySource, UiIntentReadinessSource,
};
use worth_ui_certification::scenario::filesystem_application_lifecycle::FilesystemApplicationLifecycleScenario;
use worth_ui_dsl::{
    WorthUiIntentInteractionFamily, WorthUiIntentInteractionRoute,
    WorthUiRustAuthoredArtifactInput, WorthUiRustAuthoredArtifactInputModule,
};
use worth_ui_host_headless::{UiHeadlessRecorderCapacity, WorthUiHeadlessRecorder};
use worth_ui_runtime::facade::measurement_exchange::UiViewportExtentObservation;

use super::facts::OperabilityFacts;
use super::intent_types::{
    EditIntent, PrimaryIntent, ProjectionIntent, SecondaryIntent, UnsupportedIntent, EDIT_FIELD,
};

mod application;

use application::{prepare as prepare_application, OperabilityApplicationInput};

pub(super) const PRIMARY_DECLARATION: &str = "phase3.operability.primary";
const PEER_DECLARATION: &str = "phase3.operability.peer";
const CONTRACT: &str = "phase3.operability.contract";
const CONFIRMATION_POLICY: &str = "phase3.operability.confirmation-policy";
const PAINT_ONLY: &str = "visual.identity.component.paint_only";
const HIT_ONLY: &str = "visual.identity.component.hit_only";
const PAINT_AND_HIT: &str = "visual.identity.component.paint_and_hit";
const NEITHER: &str = "visual.identity.component.neither";
const SURFACE: &str = "visual.identity.surface.main";
const PAINT_ONLY_TOKEN: &str = "theme.visual_identity.paint_only";
const PAINT_AND_HIT_TOKEN: &str = "theme.visual_identity.paint_and_hit";

#[derive(Clone, Copy)]
pub(in crate::intent) enum OccupancyLayout {
    TargetRoute,
    Declaration,
    Definition,
    Application,
}

pub(in crate::intent) fn build_scoped(
    layout: OccupancyLayout,
) -> (worth_ui::facade::app::WorthUiApp, OperabilityFacts) {
    build_scoped_with_provider(
        layout,
        worth_ui_certification::WorthUiCertificationBeforeEffectProvider::<PrimaryIntent>::new(),
    )
}

pub(in crate::intent) fn build_scoped_with_provider<P>(
    layout: OccupancyLayout,
    provider: P,
) -> (worth_ui::facade::app::WorthUiApp, OperabilityFacts)
where
    P: worth_ui::facade::intent::UiIntentExecutionProvider<PrimaryIntent>,
{
    let facts = OperabilityFacts::new();
    let input = scoped_input(layout, &facts);
    let app = prepare_application(OperabilityApplicationInput::new(input, provider), &facts);
    (app, facts)
}

pub(in crate::intent) fn build_scoped_with_provider_observation(
    layout: OccupancyLayout,
) -> (
    worth_ui::facade::app::WorthUiApp,
    OperabilityFacts,
    worth_ui_certification::WorthUiCertificationProviderObservation,
) {
    let (provider, observation) = worth_ui_certification::WorthUiCertificationBeforeEffectProvider::<
        PrimaryIntent,
    >::with_observation();
    let (app, facts) = build_scoped_with_provider(layout, provider);
    (app, facts, observation)
}

pub(in crate::intent) fn replacement_input(
    facts: &OperabilityFacts,
) -> WorthUiRustAuthoredArtifactInput {
    shared_declaration_input(UiIntentConcurrencyScope::DeclarationSingleFlight, facts)
}

pub(in crate::intent) fn build_route_scale(
    route_count: usize,
) -> worth_ui::facade::app::WorthUiApp {
    let facts = OperabilityFacts::new();
    let module = (0..route_count).fold(
        WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
            .with_surface(SURFACE)
            .with_intent_declaration(declaration::<PrimaryIntent>(
                PRIMARY_DECLARATION,
                UiIntentConcurrencyScope::TargetRouteSingleFlight,
                &facts,
            )),
        |module, index| {
            let identity = format!("phase3.scale.control.{index}");
            module.with_control_routes_and_authored_identity(
                HIT_ONLY,
                identity,
                [WorthUiIntentInteractionRoute::product(
                    WorthUiIntentInteractionFamily::Activate,
                    PRIMARY_DECLARATION,
                )],
            )
        },
    );
    let input = WorthUiRustAuthoredArtifactInput::from_modules([module]);
    let scenario = FilesystemApplicationLifecycleScenario::new("phase-3-route-scale");
    let host = WorthUiHeadlessRecorder::with_viewport_extent(
        UiHeadlessRecorderCapacity::production_default(),
        UiViewportExtentObservation {
            width: 160.0,
            height: 96.0,
        },
    );
    scenario
        .visual_identity_application_builder(host)
        .register_intent_boolean_fact(facts.mutability, true)
        .unwrap()
        .register_intent_boolean_fact(facts.readiness, true)
        .unwrap()
        .register_intent_boolean_fact(facts.policy, true)
        .unwrap()
        .register_intent_boolean_fact(facts.confirmation, false)
        .unwrap()
        .register_intent_definition(UiIntentDefinition::<PrimaryIntent>::application_effect())
        .unwrap()
        .register_intent_provider(
            worth_ui_certification::WorthUiCertificationBeforeEffectProvider::<PrimaryIntent>::new(
            ),
        )
        .unwrap()
        .with_rust_authored_input(input)
        .freeze()
        .expect("route-scale application compiles through production preparation")
}

pub(super) fn build_unsupported() -> (worth_ui::facade::app::WorthUiApp, OperabilityFacts) {
    let facts = OperabilityFacts::new();
    let input = single_input::<UnsupportedIntent>(
        PRIMARY_DECLARATION,
        UiIntentConcurrencyScope::TargetRouteSingleFlight,
        &facts,
    );
    let app = prepare_application(
        OperabilityApplicationInput::new(
            input,
            worth_ui_certification::WorthUiCertificationBeforeEffectProvider::<PrimaryIntent>::new(
            ),
        )
        .with_unsupported_definition(),
        &facts,
    );
    (app, facts)
}

pub(super) fn build_projection(
    registration: worth_ui_query_binding::UiScalarProjectionRegistration,
) -> (worth_ui::facade::app::WorthUiApp, OperabilityFacts) {
    let facts = OperabilityFacts::new();
    let projection = registration.view().identity().clone();
    let input = module(
        PRIMARY_DECLARATION,
        PRIMARY_DECLARATION,
        WorthUiIntentInteractionFamily::Activate,
        [projection_declaration(&projection, &facts)],
    );
    let app = prepare_application(
        OperabilityApplicationInput::new(
            input,
            worth_ui_certification::WorthUiCertificationBeforeEffectProvider::<PrimaryIntent>::new(
            ),
        )
        .with_projection(registration),
        &facts,
    );
    (app, facts)
}

pub(super) fn build_edit(
    host: worth_ui_host_egui::WorthUiHostEgui,
) -> (worth_ui::facade::app::WorthUiApp, OperabilityFacts) {
    let facts = OperabilityFacts::new();
    let input = module(
        PRIMARY_DECLARATION,
        PRIMARY_DECLARATION,
        WorthUiIntentInteractionFamily::EditCommit,
        [edit_declaration(&facts)],
    );
    let app = FilesystemApplicationLifecycleScenario::new("phase-3-operability-edit-world")
        .visual_identity_application_builder(host)
        .register_intent_boolean_fact(facts.policy.clone(), true)
        .unwrap()
        .register_intent_boolean_fact(facts.confirmation.clone(), false)
        .unwrap()
        .register_intent_definition(UiIntentDefinition::<EditIntent>::application_effect())
        .unwrap()
        .register_intent_provider(
            worth_ui_certification::WorthUiCertificationBeforeEffectProvider::<EditIntent>::new(),
        )
        .unwrap()
        .with_rust_authored_input(input)
        .freeze()
        .expect("committed-draft operability world prepares");
    (app, facts)
}

fn projection_declaration(
    projection: &worth_ui_query_binding::WorthUiQueryViewIdentity,
    facts: &OperabilityFacts,
) -> worth_ui_dsl::WorthUiIntentDeclarationSpec {
    UiIntentDeclaration::<ProjectionIntent>::activate(PRIMARY_DECLARATION)
        .unwrap()
        .operability_from(
            UiIntentOperabilityContract::new(
                CONTRACT,
                UiIntentMutabilitySource::readonly_projection(projection),
                UiIntentReadinessSource::projection(projection),
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
        .into_dsl_spec()
}

fn scoped_input(
    layout: OccupancyLayout,
    facts: &OperabilityFacts,
) -> WorthUiRustAuthoredArtifactInput {
    match layout {
        OccupancyLayout::TargetRoute => {
            shared_declaration_input(UiIntentConcurrencyScope::TargetRouteSingleFlight, facts)
        }
        OccupancyLayout::Declaration => {
            shared_declaration_input(UiIntentConcurrencyScope::DeclarationSingleFlight, facts)
        }
        OccupancyLayout::Definition => two_declaration_input::<PrimaryIntent, PrimaryIntent>(
            UiIntentConcurrencyScope::DefinitionSingleFlight,
            facts,
        ),
        OccupancyLayout::Application => two_declaration_input::<PrimaryIntent, SecondaryIntent>(
            UiIntentConcurrencyScope::ApplicationSingleFlight,
            facts,
        ),
    }
}

fn edit_declaration(facts: &OperabilityFacts) -> worth_ui_dsl::WorthUiIntentDeclarationSpec {
    UiIntentDeclaration::<EditIntent>::edit_commit(PRIMARY_DECLARATION)
        .unwrap()
        .bind_payload(EDIT_FIELD, UiIntentPayloadSource::committed_draft())
        .operability_from(
            UiIntentOperabilityContract::new(
                CONTRACT,
                UiIntentMutabilitySource::committed_draft(),
                UiIntentReadinessSource::committed_draft(),
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
        .into_dsl_spec()
}

fn shared_declaration_input(
    scope: UiIntentConcurrencyScope,
    facts: &OperabilityFacts,
) -> WorthUiRustAuthoredArtifactInput {
    module(
        PRIMARY_DECLARATION,
        PRIMARY_DECLARATION,
        WorthUiIntentInteractionFamily::Activate,
        [declaration::<PrimaryIntent>(
            PRIMARY_DECLARATION,
            scope,
            facts,
        )],
    )
}

fn two_declaration_input<I: UiIntent, J: UiIntent>(
    scope: UiIntentConcurrencyScope,
    facts: &OperabilityFacts,
) -> WorthUiRustAuthoredArtifactInput {
    module(
        PRIMARY_DECLARATION,
        PEER_DECLARATION,
        WorthUiIntentInteractionFamily::Activate,
        [
            declaration::<I>(PRIMARY_DECLARATION, scope, facts),
            declaration::<J>(PEER_DECLARATION, scope, facts),
        ],
    )
}

fn single_input<I: UiIntent>(
    identity: &str,
    scope: UiIntentConcurrencyScope,
    facts: &OperabilityFacts,
) -> WorthUiRustAuthoredArtifactInput {
    module(
        identity,
        identity,
        WorthUiIntentInteractionFamily::Activate,
        [declaration::<I>(identity, scope, facts)],
    )
}

fn declaration<I: UiIntent>(
    identity: &str,
    scope: UiIntentConcurrencyScope,
    facts: &OperabilityFacts,
) -> worth_ui_dsl::WorthUiIntentDeclarationSpec {
    UiIntentDeclaration::<I>::activate(identity)
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
        .concurrency(scope)
        .consequences(UiIntentConsequenceContract::none())
        .into_dsl_spec()
}

pub(super) fn module(
    primary: &str,
    peer: &str,
    family: WorthUiIntentInteractionFamily,
    declarations: impl IntoIterator<Item = worth_ui_dsl::WorthUiIntentDeclarationSpec>,
) -> WorthUiRustAuthoredArtifactInput {
    let module = WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
        .with_component(PAINT_ONLY)
        .with_control_routes(
            HIT_ONLY,
            [WorthUiIntentInteractionRoute::product(family, primary)],
        )
        .with_control_routes(
            PAINT_AND_HIT,
            [WorthUiIntentInteractionRoute::product(family, peer)],
        )
        .with_component(NEITHER)
        .with_surface(SURFACE)
        .with_token(PAINT_ONLY_TOKEN, "theme.visual_identity.red")
        .with_token(PAINT_AND_HIT_TOKEN, "theme.visual_identity.purple");
    WorthUiRustAuthoredArtifactInput::from_modules([declarations
        .into_iter()
        .fold(module, |module, declaration| {
            module.with_intent_declaration(declaration)
        })])
}
