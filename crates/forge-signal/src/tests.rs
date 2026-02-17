//! Tests for the forge-signal reactive graph.
//!
//! DOMAIN: Verification of signal graph core, multi-aspect firewall,
//! lifecycle memory safety, and cycle detection.
//!
//! INVARIANTS:
//! - Each test is straight-line code (no loops/conditionals)
//! - Tests use local graph instances (no shared state)
//!
//! DEPENDENCIES: All forge-signal modules

use crate::context::EvaluationContext;
use crate::eval::{evaluate, mark_dirty};
use crate::graph::SignalGraph;
use crate::schema::{Aspect, AspectVersion, NodeState};

// =========================================================================
// Milestone 1C.1 — Signal Graph Core & State Transitions
// =========================================================================

#[test]
fn create_node_returns_valid_handle() {
    let mut graph = SignalGraph::new();
    let node = graph.create_node();
    assert!(graph.is_alive(node));
    assert_eq!(graph.active_node_count(), 1);
}

#[test]
fn new_node_starts_dirty() {
    let mut graph = SignalGraph::new();
    let node = graph.create_node();
    let state = graph.get_state(node).unwrap();
    assert_eq!(state, NodeState::Dirty);
}

#[test]
fn add_dependency_wires_both_directions() {
    let mut graph = SignalGraph::new();
    let upstream = graph.create_node();
    let downstream = graph.create_node();
    graph
        .add_dependency(downstream, upstream, Aspect::Topology)
        .unwrap();

    let deps = graph.get_entry(downstream).unwrap().get_dependencies();
    assert_eq!(deps.len(), 1);
    assert_eq!(deps[0].source(), upstream);

    let subs = graph.get_entry(upstream).unwrap().get_subscribers();
    assert_eq!(subs.len(), 1);
    assert_eq!(subs[0], downstream);
}

#[test]
fn dirty_direct_dependent() {
    let mut graph = SignalGraph::new();
    let source = graph.create_node();
    let dependent = graph.create_node();
    graph
        .add_dependency(dependent, source, Aspect::Geometry)
        .unwrap();

    let mut compute = |_id, _g: &SignalGraph| Ok(AspectVersion::new(1, 1));
    evaluate(&mut graph, source, &mut compute).unwrap();
    evaluate(&mut graph, dependent, &mut compute).unwrap();

    mark_dirty(&mut graph, source, Aspect::Geometry).unwrap();

    let state = graph.get_state(dependent).unwrap();
    assert_eq!(state, NodeState::Dirty);
}

#[test]
fn maybe_stale_transitive_dependent() {
    let mut graph = SignalGraph::new();
    let a = graph.create_node();
    let b = graph.create_node();
    let c = graph.create_node();

    graph.add_dependency(b, a, Aspect::Geometry).unwrap();
    graph.add_dependency(c, b, Aspect::Geometry).unwrap();

    let mut compute = |_id, _g: &SignalGraph| Ok(AspectVersion::new(1, 1));
    evaluate(&mut graph, a, &mut compute).unwrap();
    evaluate(&mut graph, b, &mut compute).unwrap();
    evaluate(&mut graph, c, &mut compute).unwrap();

    mark_dirty(&mut graph, a, Aspect::Geometry).unwrap();

    let state_b = graph.get_state(b).unwrap();
    let state_c = graph.get_state(c).unwrap();
    assert_eq!(state_b, NodeState::Dirty);
    assert_eq!(state_c, NodeState::MaybeStale);
}

#[test]
fn clean_version_skip_on_unchanged_upstream() {
    let mut graph = SignalGraph::new();
    let a = graph.create_node();
    let b = graph.create_node();
    let c = graph.create_node();

    graph.add_dependency(b, a, Aspect::Topology).unwrap();
    graph.add_dependency(c, b, Aspect::Topology).unwrap();

    let mut eval_count = 0u32;

    let mut compute_a = |_id, _g: &SignalGraph| Ok(AspectVersion::new(1, 0));
    evaluate(&mut graph, a, &mut compute_a).unwrap();

    let mut compute_b = |_id, _g: &SignalGraph| Ok(AspectVersion::new(1, 0));
    evaluate(&mut graph, b, &mut compute_b).unwrap();

    let mut compute_c = |_id, _g: &SignalGraph| Ok(AspectVersion::new(1, 0));
    evaluate(&mut graph, c, &mut compute_c).unwrap();

    mark_dirty(&mut graph, a, Aspect::Geometry).unwrap();

    let mut recompute = |_id, _g: &SignalGraph| {
        eval_count += 1;
        Ok(AspectVersion::new(1, 0))
    };
    evaluate(&mut graph, a, &mut recompute).unwrap();
    evaluate(&mut graph, b, &mut recompute).unwrap();
    evaluate(&mut graph, c, &mut recompute).unwrap();

    assert!(eval_count <= 2, "node c should have skipped recomputation (MaybeStale with unchanged topo)");
}

#[test]
fn chain_1000_minimal_recomputation() {
    let mut graph = SignalGraph::new();
    let mut chain: Vec<crate::handles::NodeId> = Vec::with_capacity(1000);

    let first = graph.create_node();
    chain.push(first);

    for i in 1..1000 {
        let node = graph.create_node();
        graph
            .add_dependency(node, chain[i - 1], Aspect::Geometry)
            .unwrap();
        chain.push(node);
    }

    let mut compute = |_id, _g: &SignalGraph| Ok(AspectVersion::new(0, 1));
    for node in &chain {
        evaluate(&mut graph, *node, &mut compute).unwrap();
    }

    mark_dirty(&mut graph, chain[0], Aspect::Geometry).unwrap();

    let state_first = graph.get_state(chain[0]).unwrap();
    assert_eq!(state_first, NodeState::Dirty);

    let state_second = graph.get_state(chain[1]).unwrap();
    assert_eq!(state_second, NodeState::Dirty);

    let state_last = graph.get_state(chain[999]).unwrap();
    assert_eq!(state_last, NodeState::MaybeStale);
}

#[test]
fn push_perf_10k_nodes() {
    let mut graph = SignalGraph::new();
    let mut chain: Vec<crate::handles::NodeId> = Vec::with_capacity(10_000);

    let first = graph.create_node();
    chain.push(first);

    for i in 1..10_000 {
        let node = graph.create_node();
        graph
            .add_dependency(node, chain[i - 1], Aspect::Geometry)
            .unwrap();
        chain.push(node);
    }

    let mut compute = |_id, _g: &SignalGraph| Ok(AspectVersion::new(0, 1));
    for node in &chain {
        evaluate(&mut graph, *node, &mut compute).unwrap();
    }

    let start = std::time::Instant::now();
    mark_dirty(&mut graph, chain[0], Aspect::Geometry).unwrap();
    let elapsed = start.elapsed();

    let max_push_ms: u128 = 500;
    assert!(
        elapsed.as_millis() < max_push_ms,
        "Push propagation took {}ms, expected < {}ms",
        elapsed.as_millis(),
        max_push_ms
    );
}

// =========================================================================
// Milestone 1C.2 — Multi-Aspect Topology Firewall
// =========================================================================

#[test]
fn kv60_geometry_change_skips_topo_dependents() {
    let mut graph = SignalGraph::new();
    let source = graph.create_node();
    let topo_sub = graph.create_node();

    graph
        .add_dependency(topo_sub, source, Aspect::Topology)
        .unwrap();

    let mut compute = |_id, _g: &SignalGraph| Ok(AspectVersion::new(1, 1));
    evaluate(&mut graph, source, &mut compute).unwrap();
    evaluate(&mut graph, topo_sub, &mut compute).unwrap();

    mark_dirty(&mut graph, source, Aspect::Geometry).unwrap();

    let state = graph.get_state(topo_sub).unwrap();
    assert_eq!(
        state,
        NodeState::MaybeStale,
        "Topo-only subscriber should be MaybeStale (not Dirty) on geometry change"
    );

    let mut same_version_compute = |_id, _g: &SignalGraph| Ok(AspectVersion::new(1, 2));
    evaluate(&mut graph, source, &mut same_version_compute).unwrap();

    let mut eval_ran = false;
    let mut topo_sub_compute = |_id, _g: &SignalGraph| {
        eval_ran = true;
        Ok(AspectVersion::new(1, 1))
    };
    evaluate(&mut graph, topo_sub, &mut topo_sub_compute).unwrap();

    assert!(
        !eval_ran,
        "Topo subscriber should NOT have re-evaluated when only geometry changed"
    );
}

#[test]
fn kv60_topology_change_triggers_topo_dependents() {
    let mut graph = SignalGraph::new();
    let source = graph.create_node();
    let topo_sub = graph.create_node();

    graph
        .add_dependency(topo_sub, source, Aspect::Topology)
        .unwrap();

    let mut compute = |_id, _g: &SignalGraph| Ok(AspectVersion::new(1, 1));
    evaluate(&mut graph, source, &mut compute).unwrap();
    evaluate(&mut graph, topo_sub, &mut compute).unwrap();

    mark_dirty(&mut graph, source, Aspect::Topology).unwrap();

    let state = graph.get_state(topo_sub).unwrap();
    assert_eq!(
        state,
        NodeState::Dirty,
        "Topo subscriber must be Dirty on topology change"
    );
}

#[test]
fn aspect_independent_versioning() {
    let ver = AspectVersion::new(5, 10);
    let bumped_topo = ver.bump_topology();
    let bumped_geom = ver.bump_geometry();

    assert_eq!(bumped_topo.topology(), 6);
    assert_eq!(bumped_topo.geometry(), 10);

    assert_eq!(bumped_geom.topology(), 5);
    assert_eq!(bumped_geom.geometry(), 11);
}

// =========================================================================
// Milestone 1C.3 — Graph Lifecycle & Arena Memory Safety
// =========================================================================

#[test]
fn kv61_add_delete_10k_flat_memory() {
    let mut graph = SignalGraph::with_gc_threshold(100);

    for _ in 0..10_000 {
        let node = graph.create_node();
        graph.unregister_node(node).unwrap();

        if graph.should_gc() {
            graph.run_gc_epoch();
        }
    }

    assert!(
        graph.arena_capacity() <= 200,
        "Arena capacity {} should stay bounded via slot reuse",
        graph.arena_capacity()
    );
    assert_eq!(graph.active_node_count(), 0);
}

#[test]
fn kv62_delete_mid_chain_no_panic() {
    let mut graph = SignalGraph::new();
    let param = graph.create_node();
    let middle = graph.create_node();
    let feature = graph.create_node();

    graph
        .add_dependency(middle, param, Aspect::Geometry)
        .unwrap();
    graph
        .add_dependency(feature, middle, Aspect::Geometry)
        .unwrap();

    let mut compute = |_id, _g: &SignalGraph| Ok(AspectVersion::new(1, 1));
    evaluate(&mut graph, param, &mut compute).unwrap();
    evaluate(&mut graph, middle, &mut compute).unwrap();
    evaluate(&mut graph, feature, &mut compute).unwrap();

    graph.unregister_node(middle).unwrap();

    mark_dirty(&mut graph, param, Aspect::Geometry).unwrap();

    let feature_state = graph.get_state(feature).unwrap();
    assert_eq!(
        feature_state,
        NodeState::Dirty,
        "Feature should be Dirty after middle node was deleted"
    );
}

#[test]
fn unregister_severs_subscriptions() {
    let mut graph = SignalGraph::new();
    let upstream = graph.create_node();
    let node = graph.create_node();
    let downstream = graph.create_node();

    graph
        .add_dependency(node, upstream, Aspect::Geometry)
        .unwrap();
    graph
        .add_dependency(downstream, node, Aspect::Geometry)
        .unwrap();

    graph.unregister_node(node).unwrap();

    let upstream_subs = graph.get_entry(upstream).unwrap().get_subscribers();
    assert!(
        upstream_subs.is_empty(),
        "Upstream should have no subscribers after middle node deleted"
    );

    let downstream_deps = graph.get_entry(downstream).unwrap().get_dependencies();
    assert!(
        downstream_deps.is_empty(),
        "Downstream should have no deps on deleted node"
    );
}

#[test]
fn gc_epoch_compacts_arena() {
    let mut graph = SignalGraph::with_gc_threshold(5);

    let mut nodes = Vec::new();
    for _ in 0..10 {
        nodes.push(graph.create_node());
    }

    for node in &nodes[..5] {
        graph.unregister_node(*node).unwrap();
    }

    assert_eq!(graph.tombstone_count(), 5);
    graph.run_gc_epoch();
    assert_eq!(graph.tombstone_count(), 0);
}

// =========================================================================
// Milestone 1C.4 — Cycle Detection & Deterministic Parallelism
// =========================================================================

#[test]
fn kv63_circular_reference_detected() {
    let mut graph = SignalGraph::new();
    let a = graph.create_node();
    let b = graph.create_node();

    graph.add_dependency(b, a, Aspect::Geometry).unwrap();
    graph.add_dependency(a, b, Aspect::Geometry).unwrap();

    let result = mark_dirty(&mut graph, a, Aspect::Geometry);
    assert!(
        result.is_err(),
        "Circular reference A↔B should produce an error"
    );

    let err_msg = format!("{}", result.unwrap_err());
    assert!(
        err_msg.contains("Circular reference"),
        "Error should mention circular reference: {}",
        err_msg
    );
}

#[test]
fn kv64_parallel_branches_deterministic() {
    let mut graph = SignalGraph::new();
    let root = graph.create_node();

    let mut branches: Vec<Vec<crate::handles::NodeId>> = Vec::new();
    for _ in 0..5 {
        let mut branch = Vec::new();
        let first = graph.create_node();
        graph
            .add_dependency(first, root, Aspect::Geometry)
            .unwrap();
        branch.push(first);

        for j in 1..10 {
            let node = graph.create_node();
            graph
                .add_dependency(node, branch[j - 1], Aspect::Geometry)
                .unwrap();
            branch.push(node);
        }
        branches.push(branch);
    }

    let mut compute_counter = 0u64;
    let mut compute = |_id, _g: &SignalGraph| {
        compute_counter += 1;
        Ok(AspectVersion::new(0, compute_counter))
    };

    evaluate(&mut graph, root, &mut compute).unwrap();
    for branch in &branches {
        for node in branch {
            evaluate(&mut graph, *node, &mut compute).unwrap();
        }
    }

    mark_dirty(&mut graph, root, Aspect::Geometry).unwrap();

    let mut recompute_counter = 0u64;
    let mut recompute = |_id, _g: &SignalGraph| {
        recompute_counter += 1;
        Ok(AspectVersion::new(0, 100 + recompute_counter))
    };

    evaluate(&mut graph, root, &mut recompute).unwrap();
    for branch in &branches {
        for node in branch {
            evaluate(&mut graph, *node, &mut recompute).unwrap();
        }
    }

    assert_eq!(
        recompute_counter, 51,
        "All 51 nodes (root + 5×10) should recompute after root dirty"
    );
}

#[test]
fn evaluation_context_tracks_deps() {
    let mut graph = SignalGraph::new();
    let upstream_a = graph.create_node();
    let upstream_b = graph.create_node();

    let mut compute = |_id, _g: &SignalGraph| Ok(AspectVersion::new(1, 1));
    evaluate(&mut graph, upstream_a, &mut compute).unwrap();
    evaluate(&mut graph, upstream_b, &mut compute).unwrap();

    let evaluating = graph.create_node();
    let mut ctx = EvaluationContext::new(evaluating);

    let ver_a = ctx.read(&graph, upstream_a, Aspect::Topology).unwrap();
    assert_eq!(ver_a.topology(), 1);

    let ver_b = ctx.read(&graph, upstream_b, Aspect::Geometry).unwrap();
    assert_eq!(ver_b.geometry(), 1);

    ctx.read(&graph, upstream_a, Aspect::Topology).unwrap();

    let deps = ctx.finalize();
    assert_eq!(deps.len(), 2, "Duplicate reads should not create duplicate deps");
    assert_eq!(deps[0].source(), upstream_a);
    assert_eq!(deps[0].aspect(), Aspect::Topology);
    assert_eq!(deps[1].source(), upstream_b);
    assert_eq!(deps[1].aspect(), Aspect::Geometry);
}
