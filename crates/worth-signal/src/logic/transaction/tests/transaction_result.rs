use super::runtime_world::build_runtime;
use crate::tests::support::ASPECT_A;

#[test]
fn committed_transaction_result_retains_touched_node_count_after_patch_commit() {
    let mut graph = crate::data::graph::SignalGraph::new();
    let source = graph.node().build();
    let mut runtime = build_runtime(graph);

    let mut ctx = ();
    let mut tx = runtime.begin(&mut ctx);
    tx.mark_dirty(source, ASPECT_A).unwrap();

    let result = tx.commit().unwrap();

    assert_eq!(
        result.touched_nodes, 1,
        "commit result must retain touched-node accounting after the patch buffer is cleared"
    );
}
