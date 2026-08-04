use super::runtime_world::build_runtime;
use crate::facade::EvaluationRequestMode;
use crate::logic::transaction::TransactionOutcome;
use crate::tests::support::{evaluate, version_ab, GraphDependencyBatchExt, ASPECT_A};

#[test]
fn rollback_removes_dynamic_dependency_capture_ghost_subscribers() {
    let mut graph = crate::data::graph::SignalGraph::new();
    let source_a = graph.node().build();
    let source_b = graph.node().build();
    let target = graph.node().build();
    graph.append_dependency(target, source_a, ASPECT_A).unwrap();

    let mut runtime = build_runtime(graph);
    let mut ctx = ();

    evaluate(runtime.graph_mut(), source_a, &mut |_id, _graph| {
        Ok(version_ab(1, 0))
    })
    .unwrap();
    evaluate(runtime.graph_mut(), source_b, &mut |_id, _graph| {
        Ok(version_ab(2, 0))
    })
    .unwrap();
    evaluate(runtime.graph_mut(), target, &mut |_id, _graph| {
        Ok(version_ab(10, 0))
    })
    .unwrap();

    assert!(runtime.graph().subscribers_of(source_b).unwrap().is_empty());

    let mut tx = runtime.begin(&mut ctx);
    tx.evaluate_with_plan(
        target,
        &|view| {
            let _ = view.read_aspect_version(source_a, ASPECT_A)?;
            let _ = view.read_aspect_version(source_b, ASPECT_A)?;
            Ok(view.finish(version_ab(11, 0)))
        },
        crate::logic::evaluation::EvaluationRequestMode::ForceOnDemand,
    )
    .unwrap();
    assert_eq!(
        tx.staged_graph().subscribers_of(source_b).unwrap(),
        &[target],
        "transactional graph should see the newly captured dependency before rollback"
    );

    assert_eq!(
        tx.rollback().unwrap().outcome,
        TransactionOutcome::RolledBack
    );
    assert!(
        runtime.graph().subscribers_of(source_b).unwrap().is_empty(),
        "rollback must clear subscriber edges introduced by abandoned dynamic dependency capture"
    );
    let dependencies = runtime.graph().dependencies_of(target).unwrap();
    assert_eq!(dependencies.len(), 1);
    assert_eq!(dependencies[0].source(), source_a);
}

#[test]
fn rollback_restores_original_source_subscriber_membership_after_rewire() {
    let mut graph = crate::data::graph::SignalGraph::new();
    let source_a = graph.node().build();
    let source_b = graph.node().build();
    let target = graph.node().build();
    graph.append_dependency(target, source_a, ASPECT_A).unwrap();

    let mut runtime = build_runtime(graph);
    let mut ctx = ();

    evaluate(runtime.graph_mut(), source_a, &mut |_id, _graph| {
        Ok(version_ab(1, 0))
    })
    .unwrap();
    evaluate(runtime.graph_mut(), source_b, &mut |_id, _graph| {
        Ok(version_ab(2, 0))
    })
    .unwrap();
    evaluate(runtime.graph_mut(), target, &mut |_id, _graph| {
        Ok(version_ab(10, 0))
    })
    .unwrap();

    assert_eq!(runtime.graph().subscribers_of(source_a).unwrap(), &[target]);
    assert!(runtime.graph().subscribers_of(source_b).unwrap().is_empty());

    let mut tx = runtime.begin(&mut ctx);
    tx.evaluate_with_plan(
        target,
        &|view| {
            let _ = view.read_aspect_version(source_b, ASPECT_A)?;
            Ok(view.finish(version_ab(11, 0)))
        },
        EvaluationRequestMode::ForceOnDemand,
    )
    .unwrap();

    assert!(tx
        .staged_graph()
        .subscribers_of(source_a)
        .unwrap()
        .is_empty());
    assert_eq!(
        tx.staged_graph().subscribers_of(source_b).unwrap(),
        &[target]
    );

    assert_eq!(
        tx.rollback().unwrap().outcome,
        TransactionOutcome::RolledBack
    );

    assert_eq!(runtime.graph().subscribers_of(source_a).unwrap(), &[target]);
    assert!(runtime.graph().subscribers_of(source_b).unwrap().is_empty());
    runtime
        .graph()
        .assert_bidirectional_consistency()
        .expect("rollback should restore bidirectional dependency/subscriber topology");
    assert_eq!(
        runtime
            .telemetry()
            .transaction
            .rollback_packet_graph_patch_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .transaction
            .rollback_packet_subscriber_repair_count,
        1
    );
}
