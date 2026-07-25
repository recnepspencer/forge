use worth_ui::facade::mounted::{UiMountWorkClass, UiMountedFrameOutcome, UiPresentationDeadline};

use super::mounted_application_lifecycle::in_flight_presentation_world::{
    mounted_session, prepared,
};
use super::mounted_host_protocol::scripted_host::ScriptedPresentationHost;

#[test]
fn one_instance_delta_names_its_order_batch_without_rescanning_semantic_truth() {
    const INITIAL_INSTANCES: usize = 128;
    let host = ScriptedPresentationHost::default();
    let (mut session, _) = mounted_session(host.clone(), "mounted-delta-cost", 1);
    let identity = session.inspect_mounted_identity();
    let first = identity.mounted_instances()[0].clone();
    let surface = first.basis().semantic_surface_identity();
    let node = session
        .mounted_graph_node(first.graph_node_identity())
        .unwrap();
    for _ in 1..INITIAL_INSTANCES {
        session.mount_instance(node, surface).unwrap();
    }

    let initial = prepared(&mut session);
    let initial_cost = initial.cost_report();
    assert_eq!(initial_cost.work_class(), UiMountWorkClass::InitialMount);
    assert_eq!(
        initial_cost.initial_mounted_instances(),
        INITIAL_INSTANCES as u64
    );
    host.push_presented();
    assert!(matches!(
        session.present_prepared_mounted_frame(initial, UiPresentationDeadline::at_tick(10), 0),
        UiMountedFrameOutcome::Published(_)
    ));

    let comparison = prepared(&mut session);
    let comparison_cost = comparison.cost_report();
    assert_eq!(
        comparison_cost.work_class(),
        UiMountWorkClass::ComparisonRequired
    );
    assert_eq!(comparison_cost.changed_mounted_instances(), 0);
    assert!(
        comparison_cost.named().considered() >= INITIAL_INSTANCES as u64,
        "absence of an exact reuse witness must expose comparison work"
    );
    drop(comparison);

    session.mount_instance(node, surface).unwrap();
    let delta = prepared(&mut session);
    let delta_cost = delta.cost_report();
    assert_eq!(delta_cost.work_class(), UiMountWorkClass::SemanticDelta);
    assert_eq!(delta_cost.initial_mounted_instances(), 0);
    assert_eq!(delta_cost.changed_mounted_instances(), 1);
    assert_eq!(delta_cost.surface_instance_pairs(), 1);
    assert_eq!(delta_cost.changed_binding_generations(), 0);
    assert_eq!(delta_cost.named().considered(), 1);
    assert!(
        delta_cost.index_entries_touched() < INITIAL_INSTANCES as u64,
        "persistent index work must follow the local delta, not graph size"
    );
    assert_eq!(
        delta_cost.replaced_batch_rows(),
        INITIAL_INSTANCES as u64 + 3,
        "ordinary layer and paint rows plus the honestly replaced semantic-order batch"
    );
}
