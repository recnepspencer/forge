use crate::engine::tree::{FeatureTree, NativeFeature};
use crate::engine::wrappers::BooleanFeature;
use crate::primitives::MakePrimitiveFeature;
use crate::operations::boolean::BooleanOp;

// ══════════════════════════════════════════════════════════════
// §8  FEATURE TREE TESTS
// ══════════════════════════════════════════════════════════════

/// 8.1 — Topology Firewall Test
///
/// Evaluate feature. Change dimension (does NOT alter topology).
/// Verify the result is still valid.
#[test]
fn topology_firewall_dimension_change() {
    let mut tree = FeatureTree::new();

    let base = MakePrimitiveFeature::cube("Base", [0.0, 0.0, 0.0], 2.0);
    let base_id = tree
        .register_feature(NativeFeature::MakePrimitive(base))
        .unwrap();

    let result1 = tree.evaluate_feature(base_id).expect("Initial eval failed");
    let fc1 = result1.topology.arena().face_count();
    assert_eq!(fc1, 6, "Cube should have 6 faces");

    let new_base = MakePrimitiveFeature::cube("Base", [0.0, 0.0, 0.0], 3.0);
    tree.replace_feature(base_id, NativeFeature::MakePrimitive(new_base))
        .unwrap();

    let result2 = tree.evaluate_feature(base_id).expect("Re-eval failed");
    let fc2 = result2.topology.arena().face_count();
    assert_eq!(fc2, 6, "Resized cube should still have 6 faces");
}

/// 8.2 — Topology Change Propagation
///
/// Feature tree with boolean. Change a dimension that adds complexity.
#[test]
fn topology_change_propagation() {
    let mut tree = FeatureTree::new();

    let base = MakePrimitiveFeature::cube("Base", [0.0, 0.0, 0.0], 5.0);
    let base_id = tree
        .register_feature(NativeFeature::MakePrimitive(base))
        .unwrap();

    let tool = MakePrimitiveFeature::cube("Tool", [2.0, 2.0, 2.0], 2.0);
    let tool_id = tree
        .register_feature(NativeFeature::MakePrimitive(tool))
        .unwrap();

    let cut = BooleanFeature::new("Cut", BooleanOp::Subtraction, base_id, tool_id);
    let cut_id = tree.register_feature(NativeFeature::Boolean(cut)).unwrap();

    let result = tree.evaluate_feature(cut_id).expect("Boolean eval failed");
    let initial_faces = result.topology.arena().face_count();
    assert!(initial_faces > 6, "Subtraction should add faces");

    let big_tool = MakePrimitiveFeature::cube("Tool", [0.0, 0.0, 0.0], 3.0);
    tree.replace_feature(tool_id, NativeFeature::MakePrimitive(big_tool))
        .unwrap();

    let result2 = tree
        .evaluate_feature(cut_id)
        .expect("Re-eval after tool change failed");
    let new_faces = result2.topology.arena().face_count();
    assert!(new_faces > 0, "Result should have faces after tool change");
}

/// 8.3 — Trace Summary Retention
///
/// After evaluating a feature, the signal graph node should retain
/// a TraceSummary with a valid state hash matching the topology.
#[test]
fn trace_summary_retained_after_evaluation() {
    let mut tree = FeatureTree::new();

    let base = MakePrimitiveFeature::cube("Base", [0.0, 0.0, 0.0], 1.0);
    let base_id = tree
        .register_feature(NativeFeature::MakePrimitive(base))
        .unwrap();

    let result = tree.evaluate_feature(base_id).expect("Eval failed");

    let expected_hash = forge_topo::hashing::compute_arena_topology_hash(result.topology.arena());

    let entry = tree
        .get_graph()
        .get_entry(base_id)
        .expect("Node should exist");
    let trace = entry
        .get_trace_summary()
        .expect("TraceSummary should be set after evaluation");

    assert_eq!(
        trace.get_state_hash(),
        expected_hash,
        "State hash should match topology hash"
    );
}
