use super::*;

fn surface() -> worth_ui_host_contract::UiSemanticSurfaceIdentity {
    worth_ui_host_contract::UiSemanticSurfaceIdentity::mint_unbound().expect("surface identity")
}

fn incarnation(value: u64) -> UiScrollOwnerIncarnation {
    UiScrollOwnerIncarnation::new(value).expect("nonzero incarnation")
}

fn bounds(inline: i64, block: i64) -> UiScrollBounds {
    UiScrollBounds::new(inline, block).expect("non-negative bounds")
}

fn register(
    state: &mut UiScrollRuntimeState,
    owner: UiScrollOwnerIdentity,
    incarnation: UiScrollOwnerIncarnation,
    axes: UiScrollAxes,
    bounds: UiScrollBounds,
    offset: UiScrollOffset,
) {
    state
        .register(UiScrollOwnerRegistration::new(
            owner,
            incarnation,
            axes,
            bounds,
            offset,
        ))
        .expect("registration");
}

fn host_cause() -> UiScrollDeltaCause {
    UiScrollDeltaCause::Host {
        source: worth_ui_host_contract::UiHostScrollDeltaSource::PointerWheel,
        phase: worth_ui_host_contract::UiHostScrollDeltaPhase::Updated,
        precision: worth_ui_host_contract::UiHostScrollDeltaPrecision::Pixel,
    }
}

#[test]
fn nested_route_consumes_at_inner_bound_then_bubbles_exact_remainder() {
    let surface = surface();
    let inner =
        UiScrollOwnerIdentity::region(surface, crate::graph::UiGraphNodeIdentity::new(1), 11);
    let outer = UiScrollOwnerIdentity::viewport(surface);
    let mut state = UiScrollRuntimeState::new_session_restore_candidate();
    register(
        &mut state,
        inner,
        incarnation(1),
        UiScrollAxes::Block,
        bounds(0, 100),
        UiScrollOffset::new(0, 90).unwrap(),
    );
    register(
        &mut state,
        outer,
        incarnation(2),
        UiScrollAxes::Block,
        bounds(0, 500),
        UiScrollOffset::origin(),
    );

    let receipt = state
        .route(
            UiScrollDeltaRequest::new(
                vec![
                    UiScrollChainEntry::new(inner, incarnation(1)),
                    UiScrollChainEntry::new(outer, incarnation(2)),
                ],
                UiScrollDelta::new(0, 35),
                host_cause(),
            )
            .unwrap(),
        )
        .unwrap();

    assert_eq!(receipt.owners_visited(), 2);
    let inspection = state
        .last_owner()
        .expect("the owner retains the latest routed scroll cause");
    assert_eq!(inspection.owner(), inner);
    assert_eq!(inspection.owners_visited(), 2);
    assert_eq!(inspection.owners_changed(), 2);
    assert_eq!(receipt.remainder(), UiScrollDelta::new(0, 0));
    assert_eq!(
        state
            .offset(inner, incarnation(1))
            .unwrap()
            .block_subpixels(),
        100
    );
    assert_eq!(
        state
            .offset(outer, incarnation(2))
            .unwrap()
            .block_subpixels(),
        25
    );
}

#[test]
fn disabled_remainder_bubbling_stops_at_the_inner_owner() {
    let surface = surface();
    let inner =
        UiScrollOwnerIdentity::region(surface, crate::graph::UiGraphNodeIdentity::new(2), 12);
    let outer = UiScrollOwnerIdentity::viewport(surface);
    let mut state = UiScrollRuntimeState::new_session_restore_candidate_with_policy(
        crate::declaration::UiScrollPolicy::nested_region().with_remainder_bubbling(false),
    );
    register(
        &mut state,
        inner,
        incarnation(1),
        UiScrollAxes::Block,
        bounds(0, 100),
        UiScrollOffset::new(0, 90).unwrap(),
    );
    register(
        &mut state,
        outer,
        incarnation(2),
        UiScrollAxes::Block,
        bounds(0, 500),
        UiScrollOffset::origin(),
    );

    let receipt = state
        .route(
            UiScrollDeltaRequest::new(
                vec![
                    UiScrollChainEntry::new(inner, incarnation(1)),
                    UiScrollChainEntry::new(outer, incarnation(2)),
                ],
                UiScrollDelta::new(0, 35),
                host_cause(),
            )
            .unwrap(),
        )
        .unwrap();

    assert_eq!(receipt.owners_visited(), 1);
    assert_eq!(receipt.remainder(), UiScrollDelta::new(0, 25));
    assert_eq!(
        state.offset(outer, incarnation(2)).unwrap(),
        UiScrollOffset::origin()
    );
}

#[test]
fn declared_axes_pass_unaccepted_delta_without_loss() {
    let surface = surface();
    let inner =
        UiScrollOwnerIdentity::region(surface, crate::graph::UiGraphNodeIdentity::new(4), 44);
    let outer = UiScrollOwnerIdentity::surface(surface);
    let mut state = UiScrollRuntimeState::new_session_restore_candidate();
    register(
        &mut state,
        inner,
        incarnation(1),
        UiScrollAxes::Block,
        bounds(0, 100),
        UiScrollOffset::origin(),
    );
    register(
        &mut state,
        outer,
        incarnation(2),
        UiScrollAxes::Inline,
        bounds(100, 0),
        UiScrollOffset::origin(),
    );
    let receipt = state
        .route(
            UiScrollDeltaRequest::new(
                vec![
                    UiScrollChainEntry::new(inner, incarnation(1)),
                    UiScrollChainEntry::new(outer, incarnation(2)),
                ],
                UiScrollDelta::new(20, 30),
                host_cause(),
            )
            .unwrap(),
        )
        .unwrap();
    assert_eq!(receipt.remainder(), UiScrollDelta::new(0, 0));
    assert_eq!(
        state
            .offset(inner, incarnation(1))
            .unwrap()
            .block_subpixels(),
        30
    );
    assert_eq!(
        state
            .offset(outer, incarnation(2))
            .unwrap()
            .inline_subpixels(),
        20
    );
}

#[test]
fn stale_or_cross_surface_chain_is_rejected_before_any_offset_changes() {
    let first_surface = surface();
    let second_surface = surface();
    let inner = UiScrollOwnerIdentity::viewport(first_surface);
    let foreign = UiScrollOwnerIdentity::viewport(second_surface);
    let mut state = UiScrollRuntimeState::new_session_restore_candidate();
    register(
        &mut state,
        inner,
        incarnation(1),
        UiScrollAxes::Both,
        bounds(100, 100),
        UiScrollOffset::origin(),
    );
    register(
        &mut state,
        foreign,
        incarnation(2),
        UiScrollAxes::Both,
        bounds(100, 100),
        UiScrollOffset::origin(),
    );
    let request = UiScrollDeltaRequest::new(
        vec![
            UiScrollChainEntry::new(inner, incarnation(1)),
            UiScrollChainEntry::new(foreign, incarnation(2)),
        ],
        UiScrollDelta::new(10, 10),
        host_cause(),
    )
    .unwrap();
    assert_eq!(
        state.route(request),
        Err(UiScrollRouteDenial::CrossSurfaceChain)
    );
    assert_eq!(
        state.offset(inner, incarnation(1)).unwrap(),
        UiScrollOffset::origin()
    );
    assert_eq!(state.counters().rejected_requests(), 1);
}

#[test]
fn duplicate_owner_is_a_typed_cycle_denial() {
    let owner = UiScrollOwnerIdentity::viewport(surface());
    assert_eq!(
        UiScrollDeltaRequest::new(
            vec![
                UiScrollChainEntry::new(owner, incarnation(1)),
                UiScrollChainEntry::new(owner, incarnation(1)),
            ],
            UiScrollDelta::new(0, 1),
            host_cause(),
        ),
        Err(UiScrollRouteDenial::OwnershipCycle)
    );
}

#[test]
fn eight_owner_high_precision_chain_cost_is_bounded_by_visited_depth() {
    let surface = surface();
    let mut state = UiScrollRuntimeState::new_session_restore_candidate();
    let mut chain = Vec::new();
    for index in 1..=8 {
        let owner = UiScrollOwnerIdentity::region(
            surface,
            crate::graph::UiGraphNodeIdentity::new(index),
            index,
        );
        let incarnation = incarnation(index);
        register(
            &mut state,
            owner,
            incarnation,
            UiScrollAxes::Block,
            bounds(0, 1),
            UiScrollOffset::origin(),
        );
        chain.push(UiScrollChainEntry::new(owner, incarnation));
    }
    let receipt = state
        .route(UiScrollDeltaRequest::new(chain, UiScrollDelta::new(0, 8), host_cause()).unwrap())
        .unwrap();
    assert_eq!(receipt.owners_visited(), 8);
    assert_eq!(state.counters().owners_visited(), 8);
    assert_eq!(state.counters().owners_changed(), 8);
    assert_eq!(state.counters().admitted_requests(), 1);
}

#[test]
fn extent_reconciliation_clamps_current_offset_but_cannot_supply_new_offset() {
    let owner = UiScrollOwnerIdentity::viewport(surface());
    let mut state = UiScrollRuntimeState::new_session_restore_candidate();
    register(
        &mut state,
        owner,
        incarnation(1),
        UiScrollAxes::Block,
        bounds(0, 200),
        UiScrollOffset::new(0, 150).unwrap(),
    );
    let reconciled = state
        .reconcile_bounds(owner, incarnation(1), bounds(0, 80))
        .unwrap();
    assert_eq!(reconciled, UiScrollOffset::new(0, 80).unwrap());
}

#[test]
fn programmatic_reveal_uses_current_viewport_and_scroll_owner_bounds() {
    let owner = UiScrollOwnerIdentity::viewport(surface());
    let incarnation = incarnation(1);
    let mut state = UiScrollRuntimeState::new_session_restore_candidate();
    register(
        &mut state,
        owner,
        incarnation,
        UiScrollAxes::Block,
        bounds(0, 500),
        UiScrollOffset::new(0, 40).unwrap(),
    );
    let receipt = state
        .reveal(
            UiScrollProgrammaticRevealRequest::new(
                vec![UiScrollChainEntry::new(owner, incarnation)],
                UiScrollRevealTarget::new(
                    UiScrollRevealInterval::new(0, 10).unwrap(),
                    UiScrollRevealInterval::new(180, 220).unwrap(),
                ),
                UiScrollViewportExtent::new(100, 100).unwrap(),
                UiScrollRevealAlignment::Nearest,
            )
            .unwrap(),
        )
        .unwrap();
    assert_eq!(receipt.cause(), UiScrollDeltaCause::ProgrammaticReveal);
    assert_eq!(
        state.offset(owner, incarnation).unwrap(),
        UiScrollOffset::new(0, 120).unwrap()
    );
}

#[test]
fn reveal_respects_declared_axis_and_clamps_alignment_to_bounds() {
    let owner = UiScrollOwnerIdentity::viewport(surface());
    let incarnation = incarnation(2);
    let mut state = UiScrollRuntimeState::new_session_restore_candidate();
    register(
        &mut state,
        owner,
        incarnation,
        UiScrollAxes::Inline,
        bounds(75, 0),
        UiScrollOffset::origin(),
    );
    state
        .reveal(
            UiScrollProgrammaticRevealRequest::new(
                vec![UiScrollChainEntry::new(owner, incarnation)],
                UiScrollRevealTarget::new(
                    UiScrollRevealInterval::new(120, 180).unwrap(),
                    UiScrollRevealInterval::new(120, 180).unwrap(),
                ),
                UiScrollViewportExtent::new(40, 40).unwrap(),
                UiScrollRevealAlignment::End,
            )
            .unwrap(),
        )
        .unwrap();
    assert_eq!(
        state.offset(owner, incarnation).unwrap(),
        UiScrollOffset::new(75, 0).unwrap()
    );
}
