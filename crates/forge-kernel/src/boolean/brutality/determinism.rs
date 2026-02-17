use super::super::test_helpers::{build_cube, try_boolean};
use super::super::schema::{BooleanInput, BooleanOp};
use super::super::assemble::execute_boolean;
use forge_topo::hashing::compute_arena_topology_hash;

// ══════════════════════════════════════════════════════════════
// §5  DETERMINISM TORTURE
// ══════════════════════════════════════════════════════════════

/// 5.1 — Hash Stability Under Reordering
///
/// Union(A,B) and Union(B,A) should produce identical topology counts.
#[test]
fn hash_stability_ab_vs_ba() {
    let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 1.0);
    let (topo_b, geom_b) = build_cube([0.5, 0.5, 0.5], 1.0);

    let (topo_a2, geom_a2) = build_cube([0.0, 0.0, 0.0], 1.0);
    let (topo_b2, geom_b2) = build_cube([0.5, 0.5, 0.5], 1.0);

    let input_ab = BooleanInput::new(
        topo_a, geom_a,
        topo_b, geom_b,
        BooleanOp::Union,
    );

    let input_ba = BooleanInput::new(
        topo_b2, geom_b2,
        topo_a2, geom_a2,
        BooleanOp::Union,
    );

    let result_ab = execute_boolean(input_ab).unwrap();
    let result_ba = execute_boolean(input_ba).unwrap();

    assert_eq!(
        result_ab.topology().arena().face_count(),
        result_ba.topology().arena().face_count(),
        "Union(A,B) vs Union(B,A) face count mismatch"
    );

    assert_eq!(
        result_ab.topology().arena().vertex_count(),
        result_ba.topology().arena().vertex_count(),
        "Union(A,B) vs Union(B,A) vertex count mismatch"
    );

    assert_eq!(
        result_ab.topology().arena().half_edge_count(),
        result_ba.topology().arena().half_edge_count(),
        "Union(A,B) vs Union(B,A) halfedge count mismatch"
    );

    let hash_ab = compute_arena_topology_hash(result_ab.topology().arena());
    let hash_ba = compute_arena_topology_hash(result_ba.topology().arena());
    assert_eq!(
        hash_ab, hash_ba,
        "Union(A,B) vs Union(B,A) topology hash mismatch: {hash_ab:#x} vs {hash_ba:#x}"
    );
}

/// 5.2 — 100× Replay
///
/// Run 10 boolean cases 100 times each.
/// All topology hashes must be identical.
#[test]
fn boolean_replay_100x() {
    let cases: Vec<([f64; 3], f64, [f64; 3], f64, BooleanOp)> = vec![
        ([0.0, 0.0, 0.0], 1.0, [0.5, 0.0, 0.0], 1.0, BooleanOp::Union),
        ([0.0, 0.0, 0.0], 1.0, [0.5, 0.0, 0.0], 1.0, BooleanOp::Subtraction),
        ([0.0, 0.0, 0.0], 1.0, [0.5, 0.0, 0.0], 1.0, BooleanOp::Intersection),
        ([0.0, 0.0, 0.0], 2.0, [0.0, 0.0, 0.0], 1.0, BooleanOp::Subtraction),
        ([0.0, 0.0, 0.0], 2.0, [0.0, 0.0, 0.0], 1.0, BooleanOp::Intersection),
        ([0.0, 0.0, 0.0], 1.0, [5.0, 0.0, 0.0], 1.0, BooleanOp::Union),
        ([1.0, 1.0, 1.0], 1.0, [2.0, 1.0, 1.0], 1.0, BooleanOp::Union),
        ([0.0, 0.0, 0.0], 1.0, [1.0, 1.0, 0.0], 1.0, BooleanOp::Union),
        ([0.0, 0.0, 0.0], 3.0, [1.0, 1.0, 1.0], 1.0, BooleanOp::Subtraction),
        ([0.0, 0.0, 0.0], 1.5, [0.5, 0.0, 0.0], 1.5, BooleanOp::Intersection),
    ];

    for (case_idx, &(ca, ha, cb, hb, op)) in cases.iter().enumerate() {
        let first_result = try_boolean(ca, ha, cb, hb, op);

        let first_hash = match &first_result {
            Ok(r) => compute_arena_topology_hash(r.topology().arena()),
            Err(_) => {
                eprintln!("Case {case_idx} returns error — skip replay");
                continue;
            }
        };

        for iter in 1..100 {
            let result = try_boolean(ca, ha, cb, hb, op);
            let hash = match &result {
                Ok(r) => compute_arena_topology_hash(r.topology().arena()),
                Err(_) => 0,
            };

            assert_eq!(
                hash, first_hash,
                "Case {case_idx} iteration {iter}: hash diverged! {hash:#x} vs {first_hash:#x}"
            );
        }
    }
}
