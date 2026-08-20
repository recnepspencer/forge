use worth_ui_host_contract::{
    UiMountedFrameIdentity, UiMountedInstanceIdentity, UiMountedPaintCommandIdentity,
    UiMountedPresentationWorkView, UiMountedRgba8,
};

use super::work_producer::UiMountedPresentationState;

#[path = "work_producer_tests/delta_source.rs"]
mod delta_source;

#[path = "work_producer_tests/damage_bounds.rs"]
mod damage_bounds;

#[path = "work_producer_tests/rect_node.rs"]
mod rect_node;

#[path = "work_producer_tests/text_node.rs"]
mod text_node;

#[path = "work_producer_tests/producer_slope.rs"]
mod producer_slope;

#[path = "work_producer_tests/batch_b_preplan_slope.rs"]
mod batch_b_preplan_slope;

#[path = "work_producer_tests/precise_damage.rs"]
mod precise_damage;

#[path = "work_producer_tests/effect_expectations.rs"]
mod effect_expectations;

#[path = "work_producer_tests/world.rs"]
mod world;

use world::{rect_spec, rect_spec_with_clip, MountedPresentationWorld};

#[test]
fn equal_layer_total_order_follows_authored_node_order_not_command_identity() {
    let world = MountedPresentationWorld::new();
    let frame = UiMountedFrameIdentity::mint_unbound().unwrap();
    let projection = world.projection(
        frame,
        [
            rect_spec(world.second_instance, 0.0),
            rect_spec(world.first_instance, 0.0),
        ],
    );
    let state = UiMountedPresentationState::from_projection(&projection, world.requirement, None);
    let lease = super::UiMountedPresentationLeaseGate::default()
        .claim()
        .unwrap();

    let work = state.issue_initial(&lease, &projection);
    let UiMountedPresentationWorkView::Initial(initial) = work.view() else {
        panic!("first presentation must issue initial work");
    };
    let second = UiMountedPaintCommandIdentity::filled_rect(&projection.filled_rects().rows()[0]);
    let first = UiMountedPaintCommandIdentity::filled_rect(&projection.filled_rects().rows()[1]);
    assert!(
        first.mounted_instance().diagnostic_value() < second.mounted_instance().diagnostic_value(),
        "fixture identity allocation must oppose authored order"
    );
    assert_eq!(
        initial.order(),
        &[
            worth_ui_host_contract::UiMountedPaintOrderIdentity::for_command(second),
            worth_ui_host_contract::UiMountedPaintOrderIdentity::for_command(first),
        ]
    );
    println!(
        "WORTH_UI_LEDGER_COUNTERS={{\"P1-ORDER-01\":{}}}",
        initial.order().len()
    );
}

#[test]
fn equal_layer_successor_reorder_remains_authored_when_identity_order_opposes_it() {
    let world = MountedPresentationWorld::new();
    let predecessor = world.projection(
        UiMountedFrameIdentity::mint_unbound().unwrap(),
        [
            rect_spec(world.first_instance, 0.0),
            rect_spec(world.second_instance, 0.0),
        ],
    );
    let successor = world.projection(
        UiMountedFrameIdentity::mint_unbound().unwrap(),
        [
            rect_spec(world.second_instance, 0.0),
            rect_spec(world.first_instance, 0.0),
        ],
    );
    let predecessor_state =
        UiMountedPresentationState::from_projection(&predecessor, world.requirement, None);
    let successor_state = UiMountedPresentationState::from_projection(
        &successor,
        world.requirement,
        Some(predecessor.frame()),
    );
    let lease = super::UiMountedPresentationLeaseGate::default()
        .claim()
        .unwrap();
    let work = predecessor_state
        .issue_successor(
            &successor_state,
            &[world.first_instance, world.second_instance],
            &[],
            false,
            Some(predecessor.frame()),
            &lease,
        )
        .unwrap();
    let UiMountedPresentationWorkView::Delta(delta) = work.view() else {
        panic!("authored equal-layer reorder must issue delta work");
    };
    let expected = successor
        .retained_paint_order()
        .iter()
        .copied()
        .collect::<Vec<_>>();
    let mut observed = predecessor.retained_paint_order().to_vec();
    for edit in delta.order() {
        if let Some(position) = observed.iter().position(|entry| *entry == edit.identity()) {
            observed.remove(position);
        }
        if !edit.is_removal() {
            let position = edit
                .predecessor()
                .and_then(|predecessor| observed.iter().position(|entry| *entry == predecessor))
                .map_or(0, |position| position + 1);
            observed.insert(position, edit.identity());
        }
    }
    assert_eq!(observed, expected);
    assert!(
        expected[0].command().mounted_instance().diagnostic_value()
            > expected[1].command().mounted_instance().diagnostic_value(),
        "successor authored order must oppose identity ordering"
    );
    println!("WORTH_UI_LEDGER_COUNTERS={{\"P3-TOTAL-ORDER-01\":2}}");
}

#[test]
fn unchanged_successor_carries_zero_command_order_and_damage_work() {
    let world = MountedPresentationWorld::new();
    let predecessor_frame = UiMountedFrameIdentity::mint_unbound().unwrap();
    let successor_frame = UiMountedFrameIdentity::mint_unbound().unwrap();
    let predecessor = world.projection(predecessor_frame, [rect_spec(world.first_instance, 0.0)]);
    let successor = world.projection(successor_frame, [rect_spec(world.first_instance, 0.0)]);
    let predecessor_state =
        UiMountedPresentationState::from_projection(&predecessor, world.requirement, None);
    let successor_state = UiMountedPresentationState::from_projection(
        &successor,
        world.requirement,
        Some(predecessor.frame()),
    );
    let lease = super::UiMountedPresentationLeaseGate::default()
        .claim()
        .unwrap();

    let work = predecessor_state
        .issue_successor(
            &successor_state,
            &[],
            &[],
            false,
            Some(predecessor.frame()),
            &lease,
        )
        .unwrap();
    let UiMountedPresentationWorkView::Unchanged(unchanged) = work.view() else {
        panic!("frame-only affinity progression must be unchanged work");
    };
    assert_eq!(unchanged.affinity().predecessor(), Some(predecessor_frame));
    assert_eq!(unchanged.affinity().successor(), successor_frame);
    assert_eq!(
        unchanged.affinity().baseline(),
        world.requirement.baseline()
    );
    println!("WORTH_UI_LEDGER_COUNTERS={{\"P1-PRODUCER-COST-01\":0}}");
}

#[test]
fn one_replacement_carries_one_change_and_exact_predecessor_successor_damage() {
    let world = MountedPresentationWorld::new();
    let predecessor = world.projection(
        UiMountedFrameIdentity::mint_unbound().unwrap(),
        [rect_spec(world.first_instance, 0.0)],
    );
    let mut changed = rect_spec(world.first_instance, 0.0);
    changed.color = UiMountedRgba8::new(242, 204, 96, 255);
    changed.x = 12.0;
    changed.clip_x = 12.0;
    let successor = world.projection(UiMountedFrameIdentity::mint_unbound().unwrap(), [changed]);
    let predecessor_state =
        UiMountedPresentationState::from_projection(&predecessor, world.requirement, None);
    let successor_state = UiMountedPresentationState::from_projection(
        &successor,
        world.requirement,
        Some(predecessor.frame()),
    );
    let lease = super::UiMountedPresentationLeaseGate::default()
        .claim()
        .unwrap();

    let initial_work = predecessor_state.issue_initial(&lease, &predecessor);
    let UiMountedPresentationWorkView::Initial(initial) = initial_work.view() else {
        panic!("predecessor projection must issue initial work");
    };
    assert_eq!(initial.affinity().predecessor(), None);
    assert_eq!(initial.affinity().successor(), predecessor.frame());
    assert_eq!(initial.affinity().baseline(), world.requirement.baseline());

    let work = predecessor_state
        .issue_successor(
            &successor_state,
            &[world.first_instance],
            &[],
            false,
            Some(predecessor.frame()),
            &lease,
        )
        .unwrap();
    let UiMountedPresentationWorkView::Delta(delta) = work.view() else {
        panic!("changed retained command must produce delta work");
    };
    assert_eq!(delta.affinity().predecessor(), Some(predecessor.frame()));
    assert_eq!(delta.affinity().successor(), successor.frame());
    assert_eq!(delta.affinity().baseline(), world.requirement.baseline());
    assert_eq!(delta.changes().len(), 1);
    assert_eq!(delta.damage().len(), 2);
    assert!(delta
        .damage()
        .iter()
        .any(|damage| damage.bounds().x() == 0.0));
    assert!(delta
        .damage()
        .iter()
        .any(|damage| damage.bounds().x() == 12.0));
    assert!(delta.order_integrity().admits(&[
        worth_ui_host_contract::UiMountedPaintOrderIdentity::for_command(
            delta.changes()[0].identity(),
        ),
    ]));
    let unchanged_projection =
        world.projection(UiMountedFrameIdentity::mint_unbound().unwrap(), [changed]);
    let unchanged_state = UiMountedPresentationState::from_projection(
        &unchanged_projection,
        world.requirement,
        Some(successor.frame()),
    );
    let unchanged_work = successor_state
        .issue_successor(
            &unchanged_state,
            &[],
            &[],
            false,
            Some(successor.frame()),
            &lease,
        )
        .unwrap();
    let UiMountedPresentationWorkView::Unchanged(unchanged) = unchanged_work.view() else {
        panic!("frame-only progression must issue unchanged work");
    };
    assert_eq!(unchanged.affinity().predecessor(), Some(successor.frame()));
    assert_eq!(
        unchanged.affinity().successor(),
        unchanged_projection.frame()
    );
    assert_eq!(
        unchanged.affinity().baseline(),
        world.requirement.baseline()
    );
    println!("WORTH_UI_LEDGER_COUNTERS={{\"P1-AFFINITY-01\":3}}");
}

#[test]
fn removal_and_insert_carry_exact_identities_vacated_damage_and_total_order() {
    let world = MountedPresentationWorld::new();
    let predecessor = world.projection(
        UiMountedFrameIdentity::mint_unbound().unwrap(),
        [
            rect_spec(world.first_instance, 0.0),
            rect_spec(world.second_instance, 40.0),
        ],
    );
    let third = UiMountedInstanceIdentity::mint_unbound().unwrap();
    let successor = world.projection(
        UiMountedFrameIdentity::mint_unbound().unwrap(),
        [
            rect_spec(world.second_instance, 40.0),
            rect_spec(third, 80.0),
        ],
    );
    let predecessor_state =
        UiMountedPresentationState::from_projection(&predecessor, world.requirement, None);
    let successor_state = UiMountedPresentationState::from_projection(
        &successor,
        world.requirement,
        Some(predecessor.frame()),
    );
    let lease = super::UiMountedPresentationLeaseGate::default()
        .claim()
        .unwrap();

    let work = predecessor_state
        .issue_successor(
            &successor_state,
            &[world.first_instance, third],
            &[],
            false,
            Some(predecessor.frame()),
            &lease,
        )
        .unwrap();
    let UiMountedPresentationWorkView::Delta(delta) = work.view() else {
        panic!("membership change must produce delta work");
    };
    assert_eq!(delta.changes().len(), 2);
    let removed = UiMountedPaintCommandIdentity::filled_rect(&predecessor.filled_rects().rows()[0]);
    let retained = UiMountedPaintCommandIdentity::filled_rect(&successor.filled_rects().rows()[0]);
    let inserted = UiMountedPaintCommandIdentity::filled_rect(&successor.filled_rects().rows()[1]);
    assert!(delta
        .changes()
        .iter()
        .any(|change| matches!(change, worth_ui_host_contract::UiMountedPaintCommandChange::Remove(identity) if *identity == removed)));
    assert!(delta
        .changes()
        .iter()
        .any(|change| matches!(change, worth_ui_host_contract::UiMountedPaintCommandChange::Insert(command) if command.identity() == inserted)));
    assert!(delta
        .damage()
        .iter()
        .any(|damage| damage.bounds().x() == 0.0));
    assert!(delta
        .damage()
        .iter()
        .any(|damage| damage.bounds().x() == 80.0));
    let removed_order = worth_ui_host_contract::UiMountedPaintOrderIdentity::for_command(removed);
    let retained_order = worth_ui_host_contract::UiMountedPaintOrderIdentity::for_command(retained);
    let inserted_order = worth_ui_host_contract::UiMountedPaintOrderIdentity::for_command(inserted);
    assert_eq!(
        delta.order(),
        &[
            worth_ui_host_contract::UiMountedPaintOrderEdit::remove(removed_order),
            worth_ui_host_contract::UiMountedPaintOrderEdit::place_after(
                inserted_order,
                Some(retained_order),
            ),
        ]
    );
    println!(
        "WORTH_UI_LEDGER_COUNTERS={{\"P1-PRODUCER-01\":{}}}",
        delta.changes().len()
    );
}

trait CommandChangeIdentity {
    fn identity(&self) -> worth_ui_host_contract::UiMountedPaintCommandIdentity;
}

impl CommandChangeIdentity for worth_ui_host_contract::UiMountedPaintCommandChange {
    fn identity(&self) -> worth_ui_host_contract::UiMountedPaintCommandIdentity {
        match self {
            Self::Insert(command)
            | Self::Replace {
                successor: command, ..
            } => command.identity(),
            Self::Remove(identity) => *identity,
        }
    }
}
