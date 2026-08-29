use crate::capability::{
    UiCommandContextConsumption, UiCommandKeyCode, UiCommandRouteDeclaration,
    UiCommandRouteDestination, UiCommandRouteScope, UiCommandShortcutSequence,
};

use super::tests::{
    candidate, command_id, context, generation, scope_identity, single, state, stroke,
    FixtureIntent,
};

#[test]
fn focused_control_index_selects_only_the_exact_bound_control() {
    let shortcut = single(UiCommandKeyCode::P);
    let first_scope =
        crate::capability::UiCommandRouteScopeIdentity::for_authored_component("control.first");
    let second_scope =
        crate::capability::UiCommandRouteScopeIdentity::for_authored_component("control.second");
    let destination = UiCommandRouteDestination::for_intent::<FixtureIntent>();
    let first = super::candidate::UiCommandRouteCandidate::new(
        command_id("command.first.control"),
        Some(shortcut),
        UiCommandRouteDeclaration::new(destination).for_focused_control(first_scope),
    );
    let second = super::candidate::UiCommandRouteCandidate::new(
        command_id("command.second.control"),
        Some(shortcut),
        UiCommandRouteDeclaration::new(destination).for_focused_control(second_scope),
    );
    let focused = worth_ui_host_contract::UiMountedInstanceIdentity::mint_unbound().unwrap();
    let context = context().with_focus(Some(focused), None, Some(second_scope), 4);
    let mut state = state(vec![first, second]);

    let super::UiCommandRoutingOutcome::Routed(receipt) =
        state.route_stroke(shortcut.strokes()[0], false, context, &generation(23))
    else {
        panic!("the exact focused-control index should resolve one route");
    };
    assert_eq!(receipt.command().as_str(), "command.second.control");
    assert_eq!(state.inspect_for_certification().3, 1);
}

#[test]
fn route_receipt_revalidates_the_exact_consumed_focus_snapshot() {
    let shortcut = single(UiCommandKeyCode::F);
    let scope = scope_identity();
    let participant = worth_ui_host_contract::UiMountedInstanceIdentity::mint_unbound().unwrap();
    let route =
        UiCommandRouteDeclaration::new(UiCommandRouteDestination::for_intent::<FixtureIntent>())
            .for_focused_control(scope)
            .consuming(UiCommandContextConsumption::none().with_focus());
    let candidate = super::candidate::UiCommandRouteCandidate::new(
        command_id("command.focus.currentness"),
        Some(shortcut),
        route,
    );
    let mut state = state(vec![candidate]);
    let routed_context = context().with_focus(Some(participant), None, Some(scope), 9);
    let super::UiCommandRoutingOutcome::Routed(receipt) = state.route_stroke(
        shortcut.strokes()[0],
        false,
        routed_context.clone(),
        &generation(26),
    ) else {
        panic!("the exact focused route should resolve");
    };
    assert!(receipt.consumed_context_is_current(&routed_context));
    assert!(!receipt.consumed_context_is_current(&context().with_focus(
        Some(participant),
        None,
        Some(scope),
        10
    )));
}

#[test]
fn two_stroke_prefix_is_bounded_cancellable_and_routes_only_an_exact_second_stroke() {
    let first = stroke(UiCommandKeyCode::K);
    let second = stroke(UiCommandKeyCode::S);
    let shortcut = UiCommandShortcutSequence::two_stroke(first, second);
    let mut state = state(vec![candidate(
        "command.save",
        shortcut,
        UiCommandRouteScope::Application,
    )]);
    let generation = generation(3);
    let context = context();

    let super::UiCommandRoutingOutcome::AwaitingPrefix(prefix) = state.route_stroke(
        first,
        false,
        context.clone().with_time_basis_for_test(10),
        &generation,
    ) else {
        panic!("first stroke should occupy one bounded prefix");
    };
    assert_eq!(prefix.candidate_count(), 1);
    assert_eq!(prefix.occupancy_revision(), 1);
    assert!(state.inspect_for_certification().1);

    let super::UiCommandRoutingOutcome::Routed(receipt) = state.route_stroke(
        second,
        false,
        context.with_time_basis_for_test(20),
        &generation,
    ) else {
        panic!("exact second stroke should route");
    };
    assert_eq!(receipt.command().as_str(), "command.save");
    assert!(!state.inspect_for_certification().1);
}

#[test]
fn two_stroke_prefix_denies_context_drift_and_expiry() {
    let first = stroke(UiCommandKeyCode::K);
    let second = stroke(UiCommandKeyCode::S);
    let shortcut = UiCommandShortcutSequence::two_stroke(first, second);
    let generation = generation(24);

    let mut drifted = state(vec![candidate(
        "command.context.bound",
        shortcut,
        UiCommandRouteScope::Application,
    )]);
    let drift_context = context();
    assert!(matches!(
        drifted.route_stroke(
            first,
            false,
            drift_context.clone().with_time_basis_for_test(10),
            &generation,
        ),
        super::UiCommandRoutingOutcome::AwaitingPrefix(_)
    ));
    // The drifted prefix is discarded, and the stroke the caller just pressed is
    // resolved on its own merits rather than being swallowed with it.
    assert_eq!(
        drifted.route_stroke(
            second,
            false,
            drift_context
                .with_time_basis_for_test(20)
                .with_portals(Box::new([scope_identity()]), 1),
            &generation,
        ),
        super::UiCommandRoutingOutcome::Unmatched
    );

    let mut expired = state(vec![candidate(
        "command.expiring",
        shortcut,
        UiCommandRouteScope::Application,
    )]);
    let expiry_context = context();
    assert!(matches!(
        expired.route_stroke(
            first,
            false,
            expiry_context.clone().with_time_basis_for_test(10),
            &generation,
        ),
        super::UiCommandRoutingOutcome::AwaitingPrefix(_)
    ));
    assert_eq!(
        expired.route_stroke(
            second,
            false,
            expiry_context.with_time_basis_for_test(1_011),
            &generation,
        ),
        super::UiCommandRoutingOutcome::Unmatched
    );
}

#[test]
fn an_expired_prefix_does_not_swallow_the_next_single_stroke_command() {
    let first = stroke(UiCommandKeyCode::K);
    let second = stroke(UiCommandKeyCode::S);
    let generation = generation(26);
    let mut state = state(vec![
        candidate(
            "command.two.stroke",
            UiCommandShortcutSequence::two_stroke(first, second),
            UiCommandRouteScope::Application,
        ),
        candidate(
            "command.single.stroke",
            single(UiCommandKeyCode::S),
            UiCommandRouteScope::Application,
        ),
    ]);
    let base = context();

    assert!(matches!(
        state.route_stroke(
            first,
            false,
            base.clone().with_time_basis_for_test(10),
            &generation,
        ),
        super::UiCommandRoutingOutcome::AwaitingPrefix(_)
    ));

    let super::UiCommandRoutingOutcome::Routed(receipt) = state.route_stroke(
        second,
        false,
        base.with_time_basis_for_test(1_011),
        &generation,
    ) else {
        panic!("an expired prefix must release the stroke to its own single-stroke route");
    };
    assert_eq!(receipt.command().as_str(), "command.single.stroke");
    assert_eq!(state.resource_counts().1, 0, "the prefix is released");
}

#[test]
fn two_stroke_prefix_requires_a_monotonic_deadline_basis() {
    let first = stroke(UiCommandKeyCode::K);
    let mut state = state(vec![candidate(
        "command.no.clock",
        UiCommandShortcutSequence::two_stroke(first, stroke(UiCommandKeyCode::S)),
        UiCommandRouteScope::Application,
    )]);
    assert_eq!(
        state.route_stroke(first, false, context(), &generation(25)),
        super::UiCommandRoutingOutcome::Suppressed(
            super::UiCommandRoutingSuppression::PrefixBasisUnavailable
        )
    );
}
