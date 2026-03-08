use crate::facade::*;
use crate::tests::support::*;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Domain {
    Cache,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Impact {
    One,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Ev {
    Tick,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Tier {
    Feature,
}

fn build_runtime(graph: SignalGraph) -> SignalRuntime<Domain, Impact, Ev, (), Tier> {
    SignalRuntime::builder(graph)
        .with_domains::<Domain>()
        .with_impacts::<Impact>()
        .with_events::<Ev>()
        .with_tiers::<Tier>()
        .checkpoint_barrier(CheckpointBarrier::PerOperation)
        .build()
}

#[test]
fn rollback_heavy_workload_leaves_runtime_consistent() {
    let mut graph = SignalGraph::new();
    let root = graph.node().build();
    let mut runtime = build_runtime(graph);

    let mut ctx = ();
    for _ in 0..100 {
        let mut tx = runtime.begin();
        tx.mark_dirty(root, ASPECT_B).unwrap();
        tx.emit_event(Ev::Tick);
        tx.flush_events(CheckpointBarrier::PerOperation).unwrap();
        assert_eq!(tx.rollback(&mut ctx).unwrap(), TransactionOutcome::RolledBack);
    }

    assert_eq!(runtime.telemetry().transaction_rollback_count, 100);
}

#[test]
#[ignore = "long-running stress test for CI/nightly profiles"]
fn stress_100k_nodes_transaction_commit() {
    let mut graph = SignalGraph::new();
    let mut nodes = Vec::with_capacity(100_000);
    for _ in 0..100_000 {
        nodes.push(graph.node().build());
    }

    let mut runtime = build_runtime(graph);
    let mut ctx = ();
    let mut tx = runtime.begin();

    for node in nodes.iter().step_by(97) {
        tx.mark_dirty(*node, ASPECT_B).unwrap();
    }
    assert_eq!(tx.commit(&mut ctx).unwrap(), TransactionOutcome::Committed);
    assert!(runtime.telemetry().staged_node_patch_count > 0);
}
