use super::*;

#[test]
fn runtime_world_port_counts_direct_currentness_lookups_and_shares_its_ledger() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let runtime = runtime(
        exact_mapping(),
        vec![registration(
            dependency("query:one"),
            vec![target(&graph, node)],
        )],
    );
    let TransitionOutcome::Success(correspondence) =
        runtime.install_semantic_correspondence(dependency("query:one"), &graph)
    else {
        panic!("installed correspondence");
    };
    let port = runtime.runtime_world_correspondence_port();
    let admitted = port
        .admit_installed_basis(&correspondence)
        .expect("the issuing Bridge runtime admits its current basis");
    assert_eq!(port.inspection_counters().binding_index_lookups(), 1);
    assert_eq!(
        port.inspection_counters()
            .authoritative_registration_inspections(),
        0
    );

    let clone = port.clone();
    clone
        .compare_current_exact(&admitted)
        .expect("a cloned port shares the currentness ledger");
    assert_eq!(port.inspection_counters().binding_index_lookups(), 2);

    let fresh = runtime.runtime_world_correspondence_port();
    fresh
        .compare_current_exact(&admitted)
        .expect("a fresh port still performs one direct lookup");
    assert_eq!(fresh.inspection_counters().binding_index_lookups(), 1);
    assert_eq!(port.inspection_counters().binding_index_lookups(), 2);

    let foreign = runtime.fork_managed_request_lane();
    let foreign_port = foreign.runtime_world_correspondence_port();
    assert!(matches!(
        foreign_port.compare_current_exact(&admitted),
        Err(crate::facade::RuntimeWorldCorrespondenceAdmissionDenial::ForeignBridgeRuntime { .. })
    ));
    assert_eq!(
        foreign_port.inspection_counters().binding_index_lookups(),
        0,
        "foreign runtime denial precedes the direct binding lookup"
    );
    assert_eq!(
        foreign_port
            .inspection_counters()
            .authoritative_registration_inspections(),
        0
    );
}
