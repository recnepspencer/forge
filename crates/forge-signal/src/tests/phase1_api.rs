use crate::easy::ReactiveGraph;
use crate::facade::*;
use crate::tests::support::*;

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
    let _ = Impact::One;
    let _ = Ev::Tick;
    let _ = Tier::Feature;
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

    assert_eq!(outcome.outcome, TransactionOutcome::Committed);
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
        .reads_aspects([ASPECT_A, ASPECT_B])
        .produces_aspects([ASPECT_B])
        .requires_context(ContextRequirement::DomainContext)
        .on_demand()
        .tolerance(2)
        .build();

    let config = graph.get_entry(node).unwrap().get_eval_config().clone();
    assert_eq!(
        config.contract.reads,
        AspectMask::from([ASPECT_A, ASPECT_B])
    );
    assert_eq!(config.contract.produces, AspectMask::from([ASPECT_B]));
    assert_eq!(
        config.contract.required_context,
        ContextRequirement::DomainContext
    );
    assert_eq!(config.condition, EvaluationCondition::OnDemand);
    assert_eq!(
        config.comparator,
        Some(VersionComparatorPolicy::Tolerance { epsilon: 2 })
    );
}

#[test]
fn graph_node_builder_accepts_explicit_node_contract() {
    let mut graph = SignalGraph::new();
    let contract = NodeContract::reads([ASPECT_A])
        .with_produces([ASPECT_B])
        .with_required_context(ContextRequirement::RelationalSnapshot);
    let node = graph.node().with_contract(contract.clone()).build();

    let stored = graph.get_contract(node).unwrap().clone();
    assert_eq!(stored, contract);
}

#[test]
fn define_computation_applies_contract_comparator_and_tier_to_created_nodes() {
    let graph = SignalGraph::new();
    let mut runtime = SignalRuntime::builder(graph).with_tiers::<Tier>().build();
    let contract = NodeContract::reads([ASPECT_A])
        .with_produces([ASPECT_B])
        .with_required_context(ContextRequirement::DomainContext);
    let computation = runtime
        .define_computation(ComputationSpec {
            family: "geometry".into(),
            contract: contract.clone(),
            tier: Tier::Feature,
            comparator: VersionComparatorPolicy::OutputIdentity,
            evaluator: |_node: NodeId, view: &ExecutionReadView<'_>| {
                Ok::<PreparedEvaluation, SignalError>(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0)),
                ))
            },
        })
        .unwrap();

    let node = computation.keyed("bulkhead").node(&mut runtime);
    let stored = runtime.graph().get_entry(node).unwrap().get_eval_config().clone();

    assert_eq!(runtime.graph().get_contract(node).unwrap(), &contract);
    assert_eq!(
        stored.comparator,
        Some(VersionComparatorPolicy::OutputIdentity)
    );
    assert_eq!(
        runtime.config().node_meta().tier_for_node(node),
        Some(Tier::Feature)
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

#[test]
fn easy_mode_computed_chains_observe_staged_upstream_values_in_the_same_pass() {
    let mut graph = ReactiveGraph::new();
    let source = graph.input(2_i32);
    let doubled = graph.computed(move |context| context.get(source) * 2);
    let chained = graph.computed(move |context| context.get(doubled) + 1);

    assert_eq!(graph.get(chained), 5);

    graph.set(source, 7);

    assert_eq!(
        graph.get(chained),
        15,
        "downstream computed nodes should see freshly staged upstream values, not the pre-plan cache"
    );
}

#[test]
fn easy_mode_failed_batch_restores_input_values() {
    let mut graph = ReactiveGraph::new();
    let price = graph.input(100_i32);
    let tax = graph.input(5_i32);

    let err = graph.try_batch(|reactive| {
        reactive.try_set(price, 200)?;
        reactive.try_set(tax, 9)?;
        Err(SignalError::invalid_input("force easy-mode rollback"))
    });
    assert!(err.is_err());

    assert_eq!(graph.get(price), 100);
    assert_eq!(graph.get(tax), 5);
}

#[test]
fn easy_mode_failed_batch_restores_downstream_invalidation_state() {
    let mut graph = ReactiveGraph::new();
    let source = graph.input(2_i32);
    let doubled = graph.computed(move |context| context.get(source) * 2);

    assert_eq!(graph.get(doubled), 4);

    let err = graph.try_batch(|reactive| {
        reactive.try_set(source, 9)?;
        reactive.try_get(doubled)?;
        Err(SignalError::invalid_input(
            "force rollback after dirty propagation",
        ))
    });
    assert!(err.is_err());

    assert_eq!(graph.get(source), 2);
    assert_eq!(graph.get(doubled), 4);
}
