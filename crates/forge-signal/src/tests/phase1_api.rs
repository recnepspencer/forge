use crate::easy::ReactiveGraph;
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

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Ev {
    Tick,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Tier {
    Feature,
}

#[test]
fn runtime_builder_uses_expected_defaults() {
    let graph = SignalGraph::new();
    let runtime = SignalRuntime::builder(graph).build();

    assert_eq!(
        runtime.checkpoint().policy().barrier_for(()),
        CheckpointBarrier::PerOperation
    );
    assert_eq!(
        *runtime.config().fallback_comparator(),
        VersionComparatorPolicy::Exact
    );
}

#[test]
fn runtime_builder_supports_typed_runtime_configuration() {
    let graph = SignalGraph::new();
    let runtime = SignalRuntime::builder(graph)
        .with_domains::<Domain>()
        .with_impacts::<Impact>()
        .with_events::<Ev>()
        .with_tiers::<Tier>()
        .checkpoint_barrier(CheckpointBarrier::PerOperation)
        .fallback_comparator(VersionComparatorPolicy::Exact)
        .build();

    assert_eq!(
        runtime.checkpoint().policy().barrier_for(Domain::Cache),
        CheckpointBarrier::PerOperation
    );
}

#[test]
fn transaction_helper_commits_on_success() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let dependent = graph.node().build();
    graph.add_dependency(dependent, source, ASPECT_A).unwrap();

    let mut runtime = SignalRuntime::builder(graph).build();
    let outcome = runtime
        .transaction(&mut (), |transaction| {
            transaction.mark_dirty(source, ASPECT_A)?;
            Ok(())
        })
        .unwrap();

    assert_eq!(outcome, TransactionOutcome::Committed);
    assert_eq!(
        runtime.graph().get_state(dependent).unwrap(),
        NodeState::Dirty
    );
}

#[test]
fn transaction_helper_rolls_back_on_error() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let dependent = graph.node().build();
    graph.add_dependency(dependent, source, ASPECT_A).unwrap();
    let before = graph.get_state(dependent).unwrap();

    let mut runtime = SignalRuntime::builder(graph).build();
    let err = runtime
        .transaction(&mut (), |transaction| {
            transaction.mark_dirty(source, ASPECT_A)?;
            Err(SignalError::internal("fail the transaction"))
        })
        .unwrap_err();

    assert!(format!("{err}").contains("fail the transaction"));
    assert_eq!(runtime.graph().get_state(dependent).unwrap(), before);
}

#[test]
fn graph_node_builder_sets_accessible_configuration() {
    let mut graph = SignalGraph::new();
    let node = graph
        .node()
        .depends_on_aspects([ASPECT_A, ASPECT_B])
        .on_demand()
        .tolerance(2)
        .build();

    let config = graph.get_entry(node).unwrap().get_eval_config().clone();
    assert_eq!(
        config.depends_on_aspects,
        Some(AspectMask::from([ASPECT_A, ASPECT_B]))
    );
    assert_eq!(config.condition, EvaluationCondition::OnDemand);
    assert_eq!(
        config.comparator,
        Some(VersionComparatorPolicy::Tolerance { epsilon: 2 })
    );
}

#[test]
fn easy_mode_supports_input_computed_get_set_and_batch() {
    let mut graph = ReactiveGraph::new();
    let price = graph.input(100.0_f64);
    let tax = graph.input(0.08_f64);
    let total = graph.computed(move |context| context.get(price) * (1.0 + context.get(tax)));

    assert_eq!(graph.get(total), 108.0);

    graph.set(price, 200.0);
    assert_eq!(graph.get(total), 216.0);

    graph.batch(|reactive| {
        reactive.set(price, 300.0);
        reactive.set(tax, 0.10);
    });
    assert_eq!(graph.get(total), 330.0);
}
