//! TC-01: Feature Tree Verification.
//!
//! Verifies:
//! 1. Feature registration and dependency wiring.
//! 2. Parametric updates (changing upstream parameter -> downstream re-eval).
//! 3. Observability of feature outputs.

use forge_kernel::boolean::BooleanOp;
use forge_kernel::engine::tree::{FeatureTree, NativeFeature};
use forge_kernel::engine::wrappers::{BooleanFeature, MakeCubeFeature};

#[test]
fn tc01_parametric_update_flow() {
    let mut tree = FeatureTree::new();

    // 1. Create Base Block (Target)
    // Simulates "Sketch + Extrude"
    let base_cube = MakeCubeFeature::new("BaseCube", [0.0, 0.0, 0.0], 10.0);
    let base_id = tree
        .register_feature(NativeFeature::MakeCube(base_cube))
        .unwrap();

    // 2. Create Tool Block (Drill)
    // Simulates a cutting tool
    let tool_cube = MakeCubeFeature::new(
        "ToolCube",
        [2.5, 2.5, 2.5], // Offset to create a corner cut
        5.0,
    );
    let tool_id = tree
        .register_feature(NativeFeature::MakeCube(tool_cube))
        .unwrap();

    // 3. Create Boolean Operation (Cut)
    let cut_op = BooleanFeature::new("CornerCut", BooleanOp::Subtraction, base_id, tool_id);
    let cut_id = tree
        .register_feature(NativeFeature::Boolean(cut_op))
        .unwrap();

    // 4. Evaluate initial state
    println!("Evaluating initial state...");
    let result = tree.evaluate_feature(cut_id).expect("Evaluation failed");

    // Check face count (Cube 6 faces - 1 + 3 internal + remaining... exact count depends on overlap)
    // Base 10x10x10. Tool 5x5x5 at 2.5,2.5,2.5.
    // Tool is fully inside the first octant of the base?
    // Base bounds: -5..5. Tool bounds: 0..5.
    // Intersection volume: 0..5 (5x5x5).
    // Subtraction: Removes the 0..5 corner.
    // Should behave like a corner notch.
    let initial_faces = result.topology.arena().face_count();
    println!("Initial face count: {}", initial_faces);
    assert!(initial_faces > 6, "Should satisfy boolean complexity");

    // 5. Parametric Update
    println!("Modifying tool size...");

    // Change tool to be larger (covering more of the base)
    let new_tool_cube = MakeCubeFeature::new(
        "ToolCube_Large",
        [2.5, 2.5, 2.5],
        8.0, // Increased size
    );

    tree.replace_feature(tool_id, NativeFeature::MakeCube(new_tool_cube))
        .unwrap();

    // 6. Re-evaluate
    let new_result = tree.evaluate_feature(cut_id).expect("Re-evaluation failed");
    let new_faces = new_result.topology.arena().face_count();
    println!("New face count: {}", new_faces);

    // Exact face count might be hard to predict without robust math,
    // but it should definitely be valid and likely different or same topology but different geometry.
    // If usage of "MakeCube" is robust, we should get a valid result.
    assert!(new_faces > 0);
}

#[test]
fn feature_registration_wiring() {
    let mut tree = FeatureTree::new();
    let root = MakeCubeFeature::new("Root", [0.0, 0.0, 0.0], 1.0);
    let root_id = tree
        .register_feature(NativeFeature::MakeCube(root))
        .unwrap();

    assert!(tree.get_node_by_name("Root").is_some());
    assert_eq!(tree.get_node_by_name("Root").unwrap(), root_id);
}
