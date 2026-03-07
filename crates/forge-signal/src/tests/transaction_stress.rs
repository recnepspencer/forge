use crate::facade::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Domain {
    Cache,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Impact {
    One,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Ev {
    Tick,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Tier {
    Feature,
}

#[test]
fn rollback_heavy_workload_leaves_runtime_consistent() {
    let mut graph = SignalGraph::new();
    let root = graph.create_node();

    let mut runtime: SignalTransactionRuntime<Domain, Impact, Ev, (), Tier> =
        SignalTransactionRuntime::with_policy(graph, CheckpointPolicy::new(CheckpointBarrier::PerOperation));

    let mut ctx = ();
    for _ in 0..100 {
        let mut tx = runtime.begin();
        tx.mark_dirty(root, Aspect::Geometry).unwrap();
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
        nodes.push(graph.create_node());
    }

    let mut runtime: SignalTransactionRuntime<Domain, Impact, Ev, (), Tier> =
        SignalTransactionRuntime::with_policy(graph, CheckpointPolicy::new(CheckpointBarrier::PerOperation));
    let mut ctx = ();
    let mut tx = runtime.begin();

    for node in nodes.iter().step_by(97) {
        tx.mark_dirty(*node, Aspect::Geometry).unwrap();
    }
    assert_eq!(tx.commit(&mut ctx).unwrap(), TransactionOutcome::Committed);
    assert!(runtime.telemetry().staged_node_patch_count > 0);
}
