use super::*;

#[test]
fn installed_correspondence_fans_out_into_real_signal_slots() {
    let mut graph = SignalGraph::new();
    let first = graph.node().build();
    let second = graph.node().build();
    let installed_runtime = runtime(
        exact_mapping(),
        vec![registration(
            dependency("query:one"),
            vec![target(&graph, first), target(&graph, second)],
        )],
    );
    let TransitionOutcome::Success(correspondence) =
        installed_runtime.install_semantic_correspondence(dependency("query:one"), &graph)
    else {
        panic!("exact installed correspondence");
    };
    assert_eq!(correspondence.target_count(), 2);
    assert_admission_width(&correspondence);
    let targets = correspondence.targets().collect::<Vec<_>>();
    assert_eq!(
        targets[0].signal_graph_instance_id(),
        graph.installed_graph_capability().graph_instance_id()
    );
    assert_eq!(
        targets[0].partition(),
        &worth_signal::facade::PartitionToken::new("bridge-main")
    );
    let before = targets
        .iter()
        .map(|target| {
            graph
                .node_aspect_version(target.node())
                .unwrap()
                .get(target.aspect())
        })
        .collect::<Vec<_>>();
    let TransitionOutcome::Success(receipt) = installed_runtime
        .deliver_installed_correspondence_envelope(
            &correspondence,
            &mut graph,
            &field_change_envelope(),
        )
    else {
        panic!("matching authoritative change should invalidate Signal targets");
    };
    assert_delivery_width(&receipt, 2);
    for (target, before) in correspondence.targets().zip(before) {
        assert_eq!(
            graph
                .node_aspect_version(target.node())
                .unwrap()
                .get(target.aspect()),
            before + 1
        );
    }
}

#[test]
fn correspondence_cost_exposes_truth_width_target_width_and_unrelated_items() {
    let mut graph = SignalGraph::new();
    let targets = (0..3)
        .map(|_| {
            let node = graph.node().build();
            target(&graph, node)
        })
        .collect::<Vec<_>>();
    let installed_runtime = runtime(
        exact_mapping(),
        vec![registration(dependency("query:one"), targets)],
    );
    let TransitionOutcome::Success(correspondence) =
        installed_runtime.install_semantic_correspondence(dependency("query:one"), &graph)
    else {
        panic!("exact installed correspondence");
    };
    let TransitionOutcome::Success(receipt) = installed_runtime
        .deliver_installed_correspondence_envelope(
            &correspondence,
            &mut graph,
            &field_change_envelope_with_width(1, 5),
        )
    else {
        panic!("multi-change authoritative envelope should deliver");
    };
    assert_eq!(receipt.correspondence_lookups(), 6);
    assert_delivery_width(&receipt, 3);
}

fn assert_admission_width(correspondence: &crate::facade::BridgeInstalledSemanticCorrespondence) {
    let admission = correspondence.admission_counters();
    assert_eq!(admission.query_dependency_lookups(), 1);
    assert_eq!(admission.registered_targets_materialized(), 2);
    assert_eq!(admission.source_profile_cache_reads(), 1);
    assert_eq!(admission.allocation_registry_lock_attempts(), 1);
    assert_eq!(admission.mapping_lookups(), 2);
    assert_eq!(admission.allocation_owner_lookups(), 2);
    assert_eq!(admission.exact_matches(), 2);
    assert_eq!(admission.widened_matches(), 0);
    assert_eq!(admission.signal_node_admissions(), 2);
    assert_eq!(admission.targets_admitted(), 2);
    assert_eq!(admission.authoritative_records_committed(), 2);
    assert_eq!(admission.failed_admissions(), 0);
}

fn assert_delivery_width(
    receipt: &crate::facade::BridgeCorrespondenceDeliveryReceipt,
    target_width: usize,
) {
    let delivery = receipt.counters();
    assert_eq!(delivery.semantic_match_checks(), 1);
    assert_eq!(receipt.truth_targets_admitted(), 1);
    assert_eq!(delivery.allocation_source_set_checks(), target_width);
    assert_eq!(delivery.signal_basis_target_checks(), target_width);
    assert_eq!(receipt.signal_capability_admissions(), target_width);
    assert_eq!(delivery.source_widening_target_checks(), 0);
    assert_eq!(receipt.signal_seeds_emitted(), target_width);
    assert_eq!(receipt.node_fan_out(), target_width);
    assert_eq!(receipt.slots_touched(), target_width);
}
