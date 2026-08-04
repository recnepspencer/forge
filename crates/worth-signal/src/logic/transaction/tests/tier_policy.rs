use super::runtime_world::{build_runtime, Tier};
use crate::data::comparator::VersionComparatorPolicy;
use crate::data::tier::{DependencyMode, DirtyPropagation, EvaluationTrigger, TierPolicy};
use crate::facade::{mark_dirty, SignalRuntimePolicy};
use crate::logic::transaction::SignalRuntime;
use crate::tests::support::{evaluate, version_ab, GraphDependencyBatchExt, ASPECT_B};

#[test]
fn tier_comparator_inheritance_uses_tier_default() {
    let mut graph = crate::data::graph::SignalGraph::new();
    let a = graph.node().build();
    let b = graph.node().build();
    let c = graph.node().build();
    graph.append_dependency(b, a, ASPECT_B).unwrap();
    graph.append_dependency(c, b, ASPECT_B).unwrap();
    let mut runtime = build_runtime(graph);
    runtime.set_node_tier(c, Tier::A);
    runtime.set_tier_policy(
        TierPolicy::new(
            Tier::A,
            DependencyMode::AutoDiscovered,
            DirtyPropagation::Immediate,
            EvaluationTrigger::LazyPull,
        )
        .with_default_comparator(VersionComparatorPolicy::Tolerance { epsilon: 2 }),
    );
    let mut compute_a_10 = |_id: crate::data::handle::NodeId,
                            _g: &crate::data::graph::SignalGraph| {
        Ok(version_ab(0, 10))
    };
    let mut compute_a_12 = |_id: crate::data::handle::NodeId,
                            _g: &crate::data::graph::SignalGraph| {
        Ok(version_ab(0, 12))
    };
    let mut compute_b = |_id: crate::data::handle::NodeId, _g: &crate::data::graph::SignalGraph| {
        Ok(version_ab(0, 100))
    };
    let mut compute_c = |_id: crate::data::handle::NodeId, _g: &crate::data::graph::SignalGraph| {
        Ok(version_ab(0, 1_000))
    };

    evaluate(runtime.graph_mut(), a, &mut compute_a_10).unwrap();
    evaluate(runtime.graph_mut(), b, &mut compute_b).unwrap();
    evaluate(runtime.graph_mut(), c, &mut compute_c).unwrap();
    mark_dirty(runtime.graph_mut(), a, ASPECT_B).unwrap();
    evaluate(runtime.graph_mut(), a, &mut compute_a_12).unwrap();
    evaluate(runtime.graph_mut(), b, &mut compute_b).unwrap();
    evaluate(runtime.graph_mut(), c, &mut compute_c).unwrap();

    assert!(
        runtime.graph().telemetry().evaluation.skipped_by_comparator >= 1,
        "tier tolerance comparator should skip small delta"
    );
}

#[test]
fn node_tier_metadata_is_generation_safe_on_slot_reuse() {
    let mut graph = crate::data::graph::SignalGraph::new();
    let first = graph.node().build();
    graph.unregister_node(first).unwrap();
    let reused = graph.node().build();
    let mut runtime = build_runtime(graph);
    runtime.set_node_tier(first, Tier::A);

    assert!(
        runtime.config().node_meta().tier_for_node(reused).is_none(),
        "reused slot must not inherit stale tier metadata from prior generation"
    );
}

#[test]
fn unregister_clears_tier_metadata_tombstones() {
    let mut runtime = build_runtime(crate::data::graph::SignalGraph::new());
    let node = runtime.graph_mut().node().build();
    runtime.set_node_tier(node, Tier::A);

    assert_eq!(runtime.config().node_meta().occupied_slot_count(), 1);

    runtime.graph_mut().unregister_node(node).unwrap();

    assert_eq!(
        runtime.config().node_meta().occupied_slot_count(),
        0,
        "unregister should clear retained tier metadata for dead slots"
    );
}

#[test]
fn runtime_builder_applies_seeded_tier_policy() {
    let policy = TierPolicy::new(
        Tier::A,
        DependencyMode::AutoDiscovered,
        DirtyPropagation::Immediate,
        EvaluationTrigger::LazyPull,
    );
    let runtime = SignalRuntime::builder(crate::data::graph::SignalGraph::new())
        .with_kernel_defaults()
        .with_tiers::<Tier>()
        .tier_policy(policy.clone())
        .build();

    assert_eq!(runtime.config().tier_policies().get(Tier::A), Some(&policy));
}

#[test]
fn runtime_builder_named_policy_helpers_apply_stock_postures() {
    let development = SignalRuntime::builder(crate::data::graph::SignalGraph::new())
        .with_kernel_defaults()
        .development_policy()
        .build();
    let operational = SignalRuntime::builder(crate::data::graph::SignalGraph::new())
        .with_kernel_defaults()
        .operational_policy()
        .build();
    let forensic = SignalRuntime::builder(crate::data::graph::SignalGraph::new())
        .with_kernel_defaults()
        .forensic_policy()
        .build();

    assert_eq!(
        development.runtime_policy(),
        SignalRuntimePolicy::development()
    );
    assert_eq!(
        operational.runtime_policy(),
        SignalRuntimePolicy::operational()
    );
    assert_eq!(forensic.runtime_policy(), SignalRuntimePolicy::forensic());
}
