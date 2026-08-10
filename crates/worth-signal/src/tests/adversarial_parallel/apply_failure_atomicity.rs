use std::num::NonZeroUsize;

use crate::data::comparator::{
    DefaultComparatorPolicyResolver, DefaultComparatorResolver, VersionComparatorPolicy,
};
use crate::facade::{
    mark_dirty, EvaluationRequestMode, NodeEvaluationResult, NodeId, ParallelExecutionPolicy,
    SignalError, SignalGraph, StageExecutor,
};
use crate::logic::prepared::{PreparedDependencyCapture, PreparedEvaluation};
use crate::tests::support::{version_ab, ASPECT_A};

use super::executor_policy::aggressive_parallel_runtime_policy;

#[test]
fn full_parallel_apply_failure_does_not_leak_partial_semantic_state() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(aggressive_parallel_runtime_policy());
    let stable = graph.node().build();
    let unstable = graph.node().build();
    let requested = [stable, unstable];

    let bootstrap = graph
        .build_evaluation_plan(&requested, EvaluationRequestMode::ForceOnDemand)
        .unwrap();
    graph
        .execute_prepared_plan(&bootstrap, &(), &|ctx| {
            Ok(ctx.finish(NodeEvaluationResult::from_version(version_ab(1, 0))))
        })
        .unwrap();

    let stable_baseline = graph.get_entry(stable).unwrap().get_aspect_version();
    let unstable_baseline = graph.get_entry(unstable).unwrap().get_aspect_version();
    let stable_fact_before = graph.explanation_fact(stable).cloned();
    let unstable_fact_before = graph.explanation_fact(unstable).cloned();
    let replay_len_before = graph.replay_events().len();

    mark_dirty(&mut graph, stable, ASPECT_A).unwrap();
    mark_dirty(&mut graph, unstable, ASPECT_A).unwrap();
    let plan = graph
        .build_evaluation_plan(&requested, EvaluationRequestMode::Default)
        .unwrap();
    let err = {
        let mut comparator = DefaultComparatorResolver;
        let mut resolver = DefaultComparatorPolicyResolver {
            fallback: VersionComparatorPolicy::Exact,
            custom: &mut comparator,
        };
        crate::logic::planner::execute_prepared_plan_with_precompute(
            &mut graph,
            &plan,
            &move |node, _view| {
                let mut prepared = PreparedEvaluation::from_result(
                    NodeEvaluationResult::from_version(version_ab(2, 0)),
                );
                if node == unstable {
                    let mut capture = PreparedDependencyCapture::new();
                    capture.record(NodeId::new(999_999, 0), ASPECT_A, None);
                    prepared = prepared.with_dependencies(capture);
                }
                Ok(prepared)
            },
            &mut resolver,
            crate::logic::planner::TemporalLoweringContext::graph_only(),
            StageExecutor::full_parallel(1).with_parallel_policy(
                ParallelExecutionPolicy::new(NonZeroUsize::new(1).unwrap())
                    .with_worker_count(2)
                    .with_chunk_size(1)
                    .with_apply_group_min_width(1)
                    .with_max_concurrent_apply_groups(2),
            ),
        )
        .unwrap_err()
    };
    assert!(
        matches!(err, SignalError::StaleHandle { .. }),
        "apply failure should surface stale dependency-capture error, got: {err}"
    );

    assert_eq!(
        graph.get_entry(stable).unwrap().get_aspect_version(),
        stable_baseline,
        "stable node state must not commit when the parallel stage fails"
    );
    assert_eq!(
        graph.get_entry(unstable).unwrap().get_aspect_version(),
        unstable_baseline,
        "failing node state must be rewound"
    );
    assert_eq!(graph.explanation_fact(stable), stable_fact_before.as_ref());
    assert_eq!(
        graph.explanation_fact(unstable),
        unstable_fact_before.as_ref()
    );
    assert_eq!(
        graph.replay_events().len(),
        replay_len_before,
        "failed planner stage must not leak task-applied replay events"
    );
}
