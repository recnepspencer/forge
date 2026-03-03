//! Determinism verification — same inputs must produce identical results.
//!
//! DOMAIN: Uses the production `compute_arena_topology_hash` to verify
//! that every shape factory and Euler operator chain is deterministic.

use crate::integration_tests::harness::determinism::{
    assert_deterministic, assert_deterministic_n,
};
use crate::integration_tests::harness::shapes;

// ── Primitive determinism ────────────────────────────────────────────────────

#[test]
fn cube_is_deterministic() {
    assert_deterministic(|| shapes::unit_cube());
}

#[test]
fn tetrahedron_is_deterministic() {
    assert_deterministic(|| shapes::tetrahedron());
}

#[test]
fn dodecahedron_is_deterministic() {
    assert_deterministic(|| shapes::dodecahedron([0.0; 3], 1.0));
}

#[test]
fn prism_is_deterministic() {
    assert_deterministic(|| shapes::prism([0.0; 3], 5, 1.0, 2.0));
}

#[test]
fn block_is_deterministic() {
    assert_deterministic(|| shapes::block([0.0; 3], [1.0, 2.0, 3.0]));
}

#[test]
fn wedge_is_deterministic() {
    assert_deterministic(|| shapes::wedge([0.0; 3], [1.0, 1.0, 1.0]));
}

// ── Stress: 5-run determinism ────────────────────────────────────────────────

/// Run 5 times to catch intermittent nondeterminism (HashMap iteration order, etc.)
#[test]
fn cube_5_run_determinism() {
    assert_deterministic_n(|| shapes::unit_cube(), 5);
}

#[test]
fn dodecahedron_5_run_determinism() {
    assert_deterministic_n(|| shapes::dodecahedron([0.0; 3], 1.0), 5);
}

// ── Euler operator chain determinism ─────────────────────────────────────────

/// Build a cube, split an edge, MEF — must be deterministic.
#[test]
fn split_then_mef_chain_is_deterministic() {
    use forge_core::OperationResult;
    use crate::integration_tests::harness::shapes::{
        collect_face_loop, first_halfedge_of_face,
    };
    use forge_topo::entity_lifecycle::split_edge::SplitEdge;
    use forge_topo::entity_lifecycle::make_edge_face::MakeEdgeFace;

    assert_deterministic(|| {
        let env_res = shapes::unit_cube()?;
        let faces = env_res.get_value().faces().to_vec();
        let (mut draft, geom): (forge_topo::transactions::MutableDraft, _) = env_res.into_value().into_draft();

        let face = faces[0];
        let start_he = first_halfedge_of_face(draft.arena(), face)?;

        let se = draft.execute(SplitEdge {
            edge: start_he,
            parameter: 0.5,
        })?.into_value();

        let loop_hes = collect_face_loop(draft.arena(), start_he)?;
        let opposite_v = draft.arena().get_half_edge(loop_hes[3])?.origin();

        draft.execute(MakeEdgeFace {
            face,
            vertex_a: se.new_vertex,
            vertex_b: opposite_v,
        })?;

        let topo = draft.commit()?;
        Ok(OperationResult::new(crate::engine::facade::SolidEnvelope::new(topo, geom)))
    });
}
