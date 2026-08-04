use crate::facade::{
    EvaluationContext, EvaluationOutput, NodeContract, NodeEvaluationResult, Recipe, SignalError,
    SignalGraph, SignalRuntime, VersionComparatorPolicy,
};
use crate::tests::support::{define_keyed_computation, version_ab, ASPECT_A, ASPECT_B};

#[test]
fn keyed_node_lookup_reuses_same_runtime_entry() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let family = define_keyed_computation(&mut runtime, "fighter-projection", ());

    let node_a = family.keyed("left-wing").node(&mut runtime);
    let node_b = family.keyed("left-wing").node(&mut runtime);
    let node_c = family.keyed("right-wing").node(&mut runtime);

    assert_eq!(node_a, node_b);
    assert_ne!(node_a, node_c);
}

#[test]
fn defined_computation_keyed_lookup_reuses_same_runtime_entry() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let volumes = runtime
        .define(Recipe {
            family: "fighter-projection".into(),
            contract: NodeContract::reads([ASPECT_A]).with_produces([ASPECT_B]),
            tier: (),
            comparator: VersionComparatorPolicy::Exact,
            evaluator: |_ctx: &mut EvaluationContext<'_, ()>| {
                Ok::<EvaluationOutput, SignalError>(EvaluationOutput::from_result(
                    NodeEvaluationResult::from_version(version_ab(1, 0)),
                ))
            },
        })
        .unwrap();

    let node_a = volumes.keyed("left-wing").node(&mut runtime);
    let node_b = volumes.keyed("left-wing").node(&mut runtime);
    let node_c = volumes.keyed("right-wing").node(&mut runtime);

    assert_eq!(node_a, node_b);
    assert_ne!(node_a, node_c);
}
