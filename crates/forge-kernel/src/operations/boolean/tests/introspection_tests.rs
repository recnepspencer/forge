//! KV-53: Boolean Result Introspection Tests.
//!
//! Verifies that Boolean operations return structured metadata.

use crate::operations::boolean::schema::BooleanOp;
use crate::operations::boolean::classify_schema::FaceClassification;
use crate::operations::boolean::test_helpers::run_boolean;

#[test]
fn introspection_data_populated() {
    // Intersect two overlapping cubes
    // Overlapping region is a smaller cube.
    // Target: Cube at (0,0,0) size 1
    // Tool: Cube at (0.5, 0.5, 0.5) size 1
    let result = run_boolean(
        [0.0, 0.0, 0.0], 1.0,
        [0.5, 0.5, 0.5], 1.0,
        BooleanOp::Intersection,
    );

    let intro = result.introspection();

    println!("Introspection: {:?}", intro);

    // Verify duration is non-zero
    assert!(intro.duration_micros > 0, "Duration should be measured");

    // Verify split count
    // With overlapping cubes, faces must split.
    assert!(intro.split_count > 0, "Split count should be > 0 for overlapping cubes");

    // Verify classification counts
    // Intersection should have Inside faces.
    let target_inside = intro.target_classification.get(&FaceClassification::Inside).copied().unwrap_or(0);
    let tool_inside = intro.tool_classification.get(&FaceClassification::Inside).copied().unwrap_or(0);

    assert!(target_inside > 0, "Should have target faces classified as Inside");
    assert!(tool_inside > 0, "Should have tool faces classified as Inside");

    // Outside faces should also exist due to the non-overlapping parts
    let target_outside = intro.target_classification.get(&FaceClassification::Outside).copied().unwrap_or(0);
    assert!(target_outside > 0, "Should have target faces classified as Outside");
}

#[test]
fn introspection_disjoint_fast_path() {
    // Disjoint cubes
    let result = run_boolean(
        [0.0, 0.0, 0.0], 1.0,
        [5.0, 0.0, 0.0], 1.0,
        BooleanOp::Union,
    );

    let intro = result.introspection();
    println!("Introspection Disjoint: {:?}", intro);

    // Split count should be 0
    assert_eq!(intro.split_count, 0, "Disjoint solids should have 0 splits");

    // Maps should be empty (skipped classification)
    assert!(intro.target_classification.is_empty(), "Fast path skips classification");
    assert!(intro.tool_classification.is_empty(), "Fast path skips classification");

    // Duration should still be recorded
    assert!(intro.duration_micros > 0, "Duration should be recorded even for fast path");
}
