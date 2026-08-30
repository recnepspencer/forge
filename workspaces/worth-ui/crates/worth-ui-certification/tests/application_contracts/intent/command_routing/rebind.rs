use super::*;

#[test]
fn application_rebind_cancels_live_prefix_before_successor_routing() {
    let command = CommandDescriptor::new(
        CommandId::new("platform.pulse.chord").expect("valid command identity"),
        "Run platform pulse chord",
    )
    .with_default_shortcut(UiCommandShortcutSequence::two_stroke(
        UiCommandShortcutStroke::logical(
            UiCommandKeyCode::K,
            UiCommandModifierSet::none().with_primary().with_shift(),
        ),
        UiCommandShortcutStroke::logical(
            UiCommandKeyCode::X,
            UiCommandModifierSet::none().with_primary().with_shift(),
        ),
    ))
    .with_intent_destination::<AdvanceStatus>();
    let session =
        launch_rust_command_intent_world::<AdvanceStatus>(routed_command_input(), command);
    let mut world = InteractionWorld::from_session(session);

    assert_eq!(
        command_route_at(&mut world, 1, 10, UiHostKey::K),
        ObservedCommandRoute::AwaitingPrefix
    );
    rebind_command_world(&mut world);
    world.publish_successor();

    assert_ne!(
        command_route_at(&mut world, 2, 11, UiHostKey::X),
        ObservedCommandRoute::Routed
    );
    assert_eq!(
        command_route_at(&mut world, 3, 12, UiHostKey::K),
        ObservedCommandRoute::AwaitingPrefix
    );
    assert_eq!(
        command_route_at(&mut world, 4, 13, UiHostKey::X),
        ObservedCommandRoute::Routed
    );
    assert_eq!(world.session.shutdown().command_routes_released(), 1);
}

#[derive(Debug, Eq, PartialEq)]
enum ObservedCommandRoute {
    AwaitingPrefix,
    Routed,
    Other,
}

fn command_route_at(
    world: &mut InteractionWorld,
    sequence: u64,
    tick: u64,
    key: UiHostKey,
) -> ObservedCommandRoute {
    match world.payload_at(sequence, tick, key_payload(key)) {
        UiHostInteractionIngressOutcome::Applied(receipt) => match receipt.command_routes() {
            [UiCommandRoutingOutcome::AwaitingPrefix(_)] => ObservedCommandRoute::AwaitingPrefix,
            [UiCommandRoutingOutcome::Routed(_)] => ObservedCommandRoute::Routed,
            _ => ObservedCommandRoute::Other,
        },
        other => panic!("command keyboard observation must apply: {other:?}"),
    }
}

fn rebind_command_world(world: &mut InteractionWorld) {
    use worth_ui::facade::{
        observation::UiChangeClassificationOutcome,
        rebind::{UiRebindExecutionPolicy, UiRebindExecutionRequest, UiRebindOutcome},
        source::{WorthUiSourceIngressExt, WorthUiSourceProvider, WorthUiWatcherEvent},
    };
    const PROVIDER: &str = "phase-6-command-prefix-replacement";
    let provider = WorthUiSourceProvider::rust_authored(PROVIDER)
        .with_rust_authored_input(routed_command_replacement_input());
    let mut ingress = world.session.source_event_ingress(provider).start();
    let settled = ingress
        .ingest([WorthUiWatcherEvent::provider_revision(PROVIDER)])
        .expect("command replacement source settles");
    let submission = settled
        .attempt_candidate_for_certification(world.session.capabilities())
        .expect("command replacement source lowers");
    let mut turn = world.session.begin_observation_turn().unwrap();
    turn.admit_source(submission).unwrap();
    let admitted = turn.seal().unwrap();
    let changed = match world.session.classify_observations(admitted).unwrap() {
        UiChangeClassificationOutcome::Changed(changed) => changed,
        _ => panic!("replacement token is executable meaning"),
    };
    let lifecycle = world
        .session
        .resolve_affected_scope(changed)
        .unwrap()
        .resolve_identity_lifecycle()
        .unwrap();
    let plan = world
        .session
        .compile_rebind_plan(lifecycle, UiRebindExecutionPolicy::ordinary())
        .unwrap();
    let prepared = world
        .session
        .prepare_rebind(plan, UiRebindExecutionRequest::new(30))
        .expect("command replacement prepares");
    assert!(matches!(
        prepared.execute(30),
        UiRebindOutcome::Published(_)
    ));
}
