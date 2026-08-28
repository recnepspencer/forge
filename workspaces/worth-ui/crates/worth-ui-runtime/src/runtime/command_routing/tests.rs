use crate::capability::{
    CommandId, UiCommandContextConsumption, UiCommandKeyCode, UiCommandModifierSet,
    UiCommandRegistrationGeneration, UiCommandRegistrationOwner,
    UiCommandRegistrationOwnerIdentity, UiCommandRouteDeclaration, UiCommandRouteDestination,
    UiCommandRoutePriority, UiCommandRouteScope, UiCommandShortcutSequence,
    UiCommandShortcutStroke, UiCommandTextInputPolicy, UiIntent, UiIntentAcceptedInteractions,
    UiIntentId, UiIntentPayload, UiIntentPayloadFieldSet, UiIntentPayloadProjection,
    UiIntentPayloadProjectionViolation, UiIntentProductConsequenceFamilies,
    UiIntentProductConsequences, UiIntentProductOutcome, UiIntentSchema,
};

pub(super) struct FixturePayload;

impl UiIntentPayload for FixturePayload {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("command.fixture.payload", 1);
    const FIELDS: UiIntentPayloadFieldSet = UiIntentPayloadFieldSet::new(&[]);

    fn project(
        _fields: &mut UiIntentPayloadProjection<Self>,
    ) -> Result<Self, UiIntentPayloadProjectionViolation> {
        Ok(Self)
    }
}

pub(super) struct FixtureOutcome;

impl UiIntentProductOutcome for FixtureOutcome {
    const SCHEMA: UiIntentSchema = UiIntentSchema::stable("command.fixture.outcome", 1);
    const CONSEQUENCE_FAMILIES: UiIntentProductConsequenceFamilies =
        UiIntentProductConsequenceFamilies::NONE;

    fn into_consequences(self) -> UiIntentProductConsequences {
        UiIntentProductConsequences::none()
    }
}

pub(super) struct FixtureIntent;

impl UiIntent for FixtureIntent {
    type Payload = FixturePayload;
    type ProductOutcome = FixtureOutcome;

    const ID: UiIntentId = UiIntentId::stable("command.fixture.intent");
    const ACCEPTED_INTERACTIONS: UiIntentAcceptedInteractions =
        UiIntentAcceptedInteractions::new(&[]);
}

#[test]
fn active_portal_scope_precedes_application_scope_without_registration_order_authority() {
    let shortcut = single(UiCommandKeyCode::P);
    let application = candidate(
        "command.application",
        shortcut,
        UiCommandRouteScope::Application,
    );
    let portal = candidate(
        "command.portal",
        shortcut,
        UiCommandRouteScope::ActivePortal,
    );
    let mut state = state(vec![application, portal]);
    let context = context().with_portals(Box::new([scope_identity()]), 7);

    let super::UiCommandRoutingOutcome::Routed(receipt) =
        state.route_stroke(shortcut.strokes()[0], false, context, &generation(1))
    else {
        panic!("active portal route should win");
    };
    assert_eq!(receipt.command().as_str(), "command.portal");
}

#[test]
fn equal_lawful_candidates_are_ambiguous_instead_of_first_registered() {
    let shortcut = single(UiCommandKeyCode::K);
    let mut state = state(vec![
        candidate("command.first", shortcut, UiCommandRouteScope::Application),
        candidate("command.second", shortcut, UiCommandRouteScope::Application),
    ]);

    let super::UiCommandRoutingOutcome::Ambiguous(ambiguity) =
        state.route_stroke(shortcut.strokes()[0], false, context(), &generation(2))
    else {
        panic!("equal candidates must remain ambiguous");
    };
    assert_eq!(ambiguity.commands().len(), 2);
}

#[test]
fn equal_rank_single_and_sequence_prefix_reports_a_typed_prefix_conflict() {
    let first = stroke(UiCommandKeyCode::K);
    let mut state = state(vec![
        candidate(
            "command.open",
            UiCommandShortcutSequence::single(first),
            UiCommandRouteScope::Application,
        ),
        candidate(
            "command.chord",
            UiCommandShortcutSequence::two_stroke(first, stroke(UiCommandKeyCode::S)),
            UiCommandRouteScope::Application,
        ),
    ]);

    assert_eq!(
        state.route_stroke(first, false, context(), &generation(20)),
        super::UiCommandRoutingOutcome::Suppressed(
            super::UiCommandRoutingSuppression::PrefixConflict
        )
    );
    assert!(!state.inspect_for_certification().1);
}

#[test]
fn public_repeat_default_changes_live_routing_behavior() {
    let shortcut = single(UiCommandKeyCode::S);
    let plan = super::plan::UiCommandRoutingPlan::for_test(vec![candidate(
        "command.repeatable",
        shortcut,
        UiCommandRouteScope::Application,
    )]);
    let policy =
        crate::declaration::UiCommandRoutingPolicy::desktop().with_repeat_suppression(false);
    let mut state =
        super::UiCommandRoutingRuntimeState::with_plan_and_policy_for_test(plan, policy);

    assert!(matches!(
        state.route_stroke(shortcut.strokes()[0], true, context(), &generation(21)),
        super::UiCommandRoutingOutcome::Routed(_)
    ));
}

#[test]
fn primary_alias_matches_real_host_control_and_meta_by_platform() {
    for (platform, modifiers) in [
        (
            crate::capability::UiCommandShortcutPlatform::Windows,
            worth_ui_host_contract::UiHostKeyboardModifiers::new(false, true, false, false, false),
        ),
        (
            crate::capability::UiCommandShortcutPlatform::MacOs,
            worth_ui_host_contract::UiHostKeyboardModifiers::new(false, false, false, false, true),
        ),
    ] {
        let shortcut = single(UiCommandKeyCode::P);
        let candidate = candidate(
            "command.platform",
            shortcut,
            UiCommandRouteScope::Application,
        );
        let plan = super::plan::UiCommandRoutingPlan::for_test_platform(vec![candidate], platform);
        let mut state = super::UiCommandRoutingRuntimeState::with_plan_for_test(plan);
        let payload = worth_ui_host_contract::UiHostObservationPayload::Keyboard {
            logical_key: worth_ui_host_contract::UiHostKey::P,
            physical_key: Some(worth_ui_host_contract::UiHostKey::P),
            modifiers,
            transition: worth_ui_host_contract::UiHostKeyTransition::Pressed { repeat: false },
        };
        let (observed, repeat) = super::host_input::keyboard_stroke(&payload)
            .expect("host P press maps to a typed command stroke");
        assert!(matches!(
            state.route_input_stroke(observed, repeat, context(), &generation(22)),
            super::UiCommandRoutingOutcome::Routed(_)
        ));
    }
}

#[test]
fn repeat_ime_and_text_entry_suppression_are_typed_route_policy() {
    let shortcut = single(UiCommandKeyCode::S);
    let mut state = state(vec![candidate(
        "command.save",
        shortcut,
        UiCommandRouteScope::Application,
    )]);
    let generation = generation(4);

    assert_eq!(
        state.route_stroke(shortcut.strokes()[0], true, context(), &generation),
        super::UiCommandRoutingOutcome::Suppressed(
            super::UiCommandRoutingSuppression::RepeatSuppressed
        )
    );
    assert_eq!(
        state.route_stroke(
            shortcut.strokes()[0],
            false,
            context().with_text_input(true, false),
            &generation,
        ),
        super::UiCommandRoutingOutcome::Suppressed(
            super::UiCommandRoutingSuppression::ImeComposition
        )
    );
    assert_eq!(
        state.route_stroke(
            shortcut.strokes()[0],
            false,
            context().with_text_input(false, true),
            &generation,
        ),
        super::UiCommandRoutingOutcome::Suppressed(super::UiCommandRoutingSuppression::TextEntry)
    );
}

#[test]
fn unrelated_shortcut_neighborhoods_are_not_visited() {
    let mut candidates = (0..4096)
        .map(|index| {
            let key = if index == 2048 {
                UiCommandKeyCode::F35
            } else {
                UiCommandKeyCode::F34
            };
            candidate(
                &format!("command.scale.n{index}"),
                single(key),
                UiCommandRouteScope::Application,
            )
        })
        .collect::<Vec<_>>();
    candidates[2048] = candidate(
        "command.scale.target",
        single(UiCommandKeyCode::F35),
        UiCommandRouteScope::Application,
    );
    let mut state = state(candidates);

    let _ = state.route_stroke(
        stroke(UiCommandKeyCode::F35),
        false,
        context(),
        &generation(5),
    );
    let (_, _, invocations, visited) = state.inspect_for_certification();
    assert_eq!(invocations, 1);
    assert_eq!(visited, 1);
}

#[test]
fn owner_unload_removes_routes_and_related_prefix_occupancy() {
    let owner = UiCommandRegistrationOwner::new(
        UiCommandRegistrationOwnerIdentity::new(9),
        UiCommandRegistrationGeneration::new(2),
    );
    let first = stroke(UiCommandKeyCode::K);
    let route = route(UiCommandRouteScope::Application).with_registration_owner(owner);
    let owned = super::candidate::UiCommandRouteCandidate::new(
        command_id("command.extension"),
        Some(UiCommandShortcutSequence::two_stroke(
            first,
            stroke(UiCommandKeyCode::X),
        )),
        route,
    );
    let mut state = state(vec![owned]);
    let generation = generation(6);

    assert!(matches!(
        state.route_stroke(first, false, context_at(10), &generation),
        super::UiCommandRoutingOutcome::AwaitingPrefix(_)
    ));
    assert_eq!(state.unload_registration_owner(owner), 1);
    assert_eq!(state.inspect_for_certification(), (0, false, 1, 1));
}

#[test]
fn shutdown_releases_routes_and_cancels_prefix_occupancy() {
    let first = stroke(UiCommandKeyCode::K);
    let owned = candidate(
        "command.shutdown",
        UiCommandShortcutSequence::two_stroke(first, stroke(UiCommandKeyCode::X)),
        UiCommandRouteScope::Application,
    );
    let mut state = state(vec![owned]);
    let generation = generation(7);

    assert!(matches!(
        state.route_stroke(first, false, context_at(10), &generation),
        super::UiCommandRoutingOutcome::AwaitingPrefix(_)
    ));
    assert_eq!(state.shutdown(), 1);
    assert_eq!(state.inspect_for_certification(), (0, false, 1, 1));
}

pub(super) fn candidate(
    id: &str,
    shortcut: UiCommandShortcutSequence,
    scope: UiCommandRouteScope,
) -> super::candidate::UiCommandRouteCandidate {
    super::candidate::UiCommandRouteCandidate::new(command_id(id), Some(shortcut), route(scope))
}

fn route(scope: UiCommandRouteScope) -> UiCommandRouteDeclaration {
    let route =
        UiCommandRouteDeclaration::new(UiCommandRouteDestination::for_intent::<FixtureIntent>());
    let route = match scope {
        UiCommandRouteScope::FocusedControl => route.for_focused_control(scope_identity()),
        UiCommandRouteScope::ActivePortal => route.for_active_portal(scope_identity()),
        other => route.with_scope(other),
    };
    route
        .consuming(UiCommandContextConsumption::none())
        .with_priority(UiCommandRoutePriority::normal())
        .with_text_input_policy(UiCommandTextInputPolicy::SuppressDuringCompositionAndTextInput)
}

pub(super) fn state(
    candidates: Vec<super::candidate::UiCommandRouteCandidate>,
) -> super::UiCommandRoutingRuntimeState {
    super::UiCommandRoutingRuntimeState::with_plan_for_test(
        super::plan::UiCommandRoutingPlan::for_test(candidates),
    )
}

pub(super) fn single(key: UiCommandKeyCode) -> UiCommandShortcutSequence {
    UiCommandShortcutSequence::single(stroke(key))
}

pub(super) fn stroke(key: UiCommandKeyCode) -> UiCommandShortcutStroke {
    UiCommandShortcutStroke::logical(key, UiCommandModifierSet::none().with_primary())
}

pub(super) fn command_id(value: &str) -> CommandId {
    CommandId::new(value).expect("fixture command ID")
}

pub(super) fn context() -> super::UiCommandRoutingContext {
    super::UiCommandRoutingContext::new(
        worth_ui_host_contract::UiSemanticSurfaceIdentity::mint_unbound().unwrap(),
    )
}

pub(super) fn context_at(millis: u64) -> super::UiCommandRoutingContext {
    context().with_time_basis_for_test(millis)
}

pub(super) fn scope_identity() -> crate::capability::UiCommandRouteScopeIdentity {
    crate::capability::UiCommandRouteScopeIdentity::for_authored_semantic_name("fixture.scope")
}

pub(super) fn generation(
    seed: u64,
) -> crate::runtime::intent::WorthUiActiveApplicationGenerationIdentity {
    use worth_ui_dsl::{
        UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey, UiDslSourceProvenance,
        UiDslStructuralToken,
    };

    let app = crate::facade::WorthUi::app()
        .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(
            crate::facade::WorthUiRustAuthoredDeclarationFixture::named(format!(
                "command-routing-fixture-{seed}"
            ))
            .with_semantic_artifact_spec(
                UiDslSemanticArtifactSpec::new(
                    UiDslSemanticKey::new(format!("ui.command.fixture.{seed}")),
                    UiDslSemanticFamily::Control,
                    UiDslSourceProvenance::rust_authored("command/routing", 0),
                )
                .with_structural_token(UiDslStructuralToken::new("control:command-routing")),
            ),
        )
        .freeze()
        .map(crate::facade::entry::WorthUiCertificationApplicationTransition::activate_builder_host)
        .expect("command routing fixture prepares");
    let session =
        crate::lifecycle::WorthUiActiveApplicationSessionIdentity::from_host_session_value(seed);
    crate::runtime::intent::WorthUiActiveApplicationGenerationIdentity::current(
        session,
        app.generation_identity(),
    )
}
