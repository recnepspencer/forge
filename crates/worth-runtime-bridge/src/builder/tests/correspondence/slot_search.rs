use super::*;

#[test]
fn automatic_allocation_searches_for_a_free_slot_the_node_actually_admits() {
    let mut graph = SignalGraph::new();
    let admitted_aspect = Aspect::new(5);
    let admitted_mask = worth_signal::facade::AspectMask::from_aspect(admitted_aspect);
    let node = graph
        .node()
        .reads_aspects(admitted_mask)
        .projection_contract(worth_signal::facade::adapters::NodeProjectionContract {
            consumes: admitted_mask,
            consumes_partitions: None,
        })
        .build();
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
        panic!("allocator should find the later admitted Signal slot");
    };
    assert_eq!(
        correspondence.targets().next().unwrap().aspect(),
        admitted_aspect
    );
    assert_eq!(
        correspondence
            .admission_counters()
            .allocation_keys_examined(),
        6
    );
}
