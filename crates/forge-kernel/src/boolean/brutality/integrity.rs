use super::super::test_helpers::{build_cube, run_boolean};
use super::super::schema::{BooleanInput, BooleanOp};
use super::super::assemble::execute_boolean;

// ══════════════════════════════════════════════════════════════
// §4  TOPOLOGY INTEGRITY TORTURE
// ══════════════════════════════════════════════════════════════

/// 4.1 — Edge Split Storm
///
/// Build a cube and verify Euler characteristic.
#[test]
fn edge_split_storm() {
    let (topo, _geom) = build_cube([0.0, 0.0, 0.0], 1.0);
    let arena = topo.arena();

    let v = arena.vertex_count() as isize;
    let e = (arena.half_edge_count() / 2) as isize;
    let f = arena.face_count() as isize;
    let euler = v - e + f;

    assert_eq!(euler, 2, "Cube should satisfy Euler: V-E+F=2, got V={v} E={e} F={f} Euler={euler}");
}

/// 4.2 — Boolean round-trip validity
#[test]
fn boolean_round_trip_validity() {
    let result = run_boolean(
        [0.0, 0.0, 0.0], 2.0,
        [1.5, 0.0, 0.0], 2.0,
        BooleanOp::Union,
    );

    let arena = result.topology().arena();
    let v = arena.vertex_count() as isize;
    let e = (arena.half_edge_count() / 2) as isize;
    let f = arena.face_count() as isize;

    assert_eq!(v - e + f, 2, "Boolean result violates Euler: V={v} E={e} F={f}");

    for (_he_id, he) in arena.iter_half_edges() {
        let twin = arena.get_half_edge(he.twin);
        assert!(twin.is_ok(), "Orphan halfedge");
    }
}

/// 4.3 — Multiple boolean operations preserve Euler
///
/// Chain: A ∪ B, then result ∩ C.
#[test]
fn chained_booleans_preserve_euler() {
    let result_ab = run_boolean(
        [0.0, 0.0, 0.0], 1.5,
        [1.0, 0.0, 0.0], 1.5,
        BooleanOp::Union,
    );

    let (topo_ab, geom_ab) = result_ab.into_parts();
    let (topo_c, geom_c) = build_cube([0.5, 0.5, 0.0], 1.0);

    let input = BooleanInput::new(topo_ab, geom_ab, topo_c, geom_c, BooleanOp::Intersection);
    let result = execute_boolean(input);

    match result {
        Ok(r) => {
            let arena = r.topology().arena();
            let v = arena.vertex_count() as isize;
            let e = (arena.half_edge_count() / 2) as isize;
            let f = arena.face_count() as isize;
            let euler = v - e + f;
            assert_eq!(euler, 2, "Chained boolean Euler violation: V={v} E={e} F={f}");
        }
        Err(e) => {
            panic!("Chained boolean must not fail: {e:?}");
        }
    }
}

/// 4.4 — Random Edge Split Storm
///
/// Build a cube, split edges, verify Euler holds after each split.
#[test]
fn random_edge_split_storm() {
    use forge_topo::euler::split_edge::SplitEdge;
    use forge_topo::operator::EulerOperator;
    use forge_topo::lineage::OpSignature;

    let (topo, _geom) = build_cube([0.0, 0.0, 0.0], 1.0);
    let mut draft = topo.begin_mutation();
    let sig = OpSignature::new("split_storm");

    let mut split_count = 0usize;
    let half_edges: Vec<_> = topo.arena().iter_half_edges()
        .map(|(id, _)| id)
        .collect();

    for &he_id in half_edges.iter().take(24) {
        let op = SplitEdge { edge: he_id };
        let result = op.execute(&mut draft, &sig);
        if result.is_ok() {
            split_count += 1;

            let arena = draft.arena();
            let v = arena.vertex_count() as isize;
            let e = (arena.half_edge_count() / 2) as isize;
            let f = arena.face_count() as isize;
            let euler = v - e + f;
            assert_eq!(
                euler, 2,
                "Euler violation after {split_count} splits: V={v} E={e} F={f}"
            );
        }
    }

    assert!(split_count > 0, "Should have split at least one edge");
}

/// 4.5 — Operation Permutation Hash Equality
///
/// Union(A,B) then Union(result,C) vs Union(B,C) then Union(A,result)
/// should produce the same vertex/edge/face counts for associative operations.
#[test]
fn operation_permutation_counts() {
    let result_ab = run_boolean(
        [0.0, 0.0, 0.0], 1.0,
        [0.5, 0.0, 0.0], 1.0,
        BooleanOp::Union,
    );
    let (topo_ab, geom_ab) = result_ab.into_parts();
    let (topo_c, geom_c) = build_cube([1.0, 0.0, 0.0], 1.0);

    let input_abc = BooleanInput::new(topo_ab, geom_ab, topo_c, geom_c, BooleanOp::Union);
    let result_abc = execute_boolean(input_abc).expect("(A∪B)∪C must not fail");

    let result_bc = run_boolean(
        [0.5, 0.0, 0.0], 1.0,
        [1.0, 0.0, 0.0], 1.0,
        BooleanOp::Union,
    );
    let (topo_bc, geom_bc) = result_bc.into_parts();
    let (topo_a2, geom_a2) = build_cube([0.0, 0.0, 0.0], 1.0);

    let input_a_bc = BooleanInput::new(topo_a2, geom_a2, topo_bc, geom_bc, BooleanOp::Union);
    let result_a_bc = execute_boolean(input_a_bc).expect("A∪(B∪C) must not fail");

    assert_eq!(
        result_abc.topology().arena().face_count(),
        result_a_bc.topology().arena().face_count(),
        "(A∪B)∪C vs A∪(B∪C) face count mismatch"
    );
    assert_eq!(
        result_abc.topology().arena().vertex_count(),
        result_a_bc.topology().arena().vertex_count(),
        "(A∪B)∪C vs A∪(B∪C) vertex count mismatch"
    );
}
