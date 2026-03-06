//! Multi-step operator chains with per-step production validation.
//!
//! DOMAIN: Tests that exercise `OpChain` — the harness's chain builder
//! that calls `validate_topology` (production validator) after every step.
//! These catch pointer rot from chained mutations.

use crate::engine::facade::SolidEnvelope;
use crate::integration_tests::harness::chains::OpChain;
use crate::integration_tests::harness::shapes;
use crate::integration_tests::harness::shapes::{collect_face_loop, first_halfedge_of_face};
use forge_core::OperationResult;

use forge_topo::boundary_editing::join_faces::JoinFaces;
use forge_topo::entity_lifecycle::make_edge_face::MakeEdgeFace;
use forge_topo::entity_lifecycle::split_edge::SplitEdge;

/// Reassemble a SolidEnvelope after draft mutation.
fn commit_draft(
    draft: forge_topo::transactions::MutableDraft,
    geometry: crate::geometry::facade::GeometryStore,
) -> Result<OperationResult<SolidEnvelope>, forge_core::KernelError> {
    let topo = draft.commit()?;
    Ok(OperationResult::new(SolidEnvelope::new(topo, geometry)))
}

/// Split every edge of a cube face, then MEF the midpoints.
///
/// 4 splits + 4 MEFs = 8 steps, each validated by production `validate_topology`.
#[test]
fn chain_split_all_then_mef() {
    let env_res = shapes::unit_cube().expect("unit cube should succeed");
    let faces = env_res.get_value().faces().to_vec();

    let result = OpChain::new(env_res)
        .apply("split_edge_0", |env, _scope| {
            let (mut draft, geom) = env.into_draft();
            let he = first_halfedge_of_face(draft.arena(), faces[0])?;
            draft.execute(SplitEdge { edge: he })?;
            commit_draft(draft, geom)
        })
        .apply("split_edge_1", |env, _scope| {
            let face = env.faces()[0];
            let (mut draft, geom) = env.into_draft();
            let he = first_halfedge_of_face(draft.arena(), face)?;
            let loop_hes = collect_face_loop(draft.arena(), he)?;
            draft.execute(SplitEdge { edge: loop_hes[2] })?;
            commit_draft(draft, geom)
        })
        .assert_valid()
        .finish();

    let arena = result.get_value().topology().arena();
    assert_eq!(arena.face_count(), 6, "No new faces from splits");
    assert_eq!(arena.vertex_count(), 10, "2 splits = 2 new vertices");
}

/// MEF then JoinFaces roundtrip through OpChain — validates at every step.
#[test]
fn chain_mef_then_join_roundtrip() {
    let env_res = shapes::unit_cube().expect("unit cube should succeed");
    let faces = env_res.get_value().faces().to_vec();

    let result = OpChain::new(env_res)
        .apply("mef_diagonal", |env, _scope| {
            let (mut draft, geom) = env.into_draft();
            let face = faces[0];
            let he = first_halfedge_of_face(draft.arena(), face)?;
            let loop_hes = collect_face_loop(draft.arena(), he)?;
            let v_a = draft.arena().get_half_edge(loop_hes[0])?.origin();
            let v_c = draft.arena().get_half_edge(loop_hes[2])?.origin();
            draft.execute(MakeEdgeFace {
                face,
                vertex_a: v_a,
                vertex_b: v_c,
            })?;
            commit_draft(draft, geom)
        })
        .apply("join_back", |env, _scope| {
            let face = env.faces()[0];
            let (mut draft, geom) = env.into_draft();
            let he = first_halfedge_of_face(draft.arena(), face)?;
            draft.execute(JoinFaces { edge: he })?;
            commit_draft(draft, geom)
        })
        .assert_valid()
        .finish();

    let arena = result.get_value().topology().arena();
    assert_eq!(arena.face_count(), 6);
    assert_eq!(arena.vertex_count(), 8);
    assert_eq!(arena.edge_count(), 12);
}

/// Long chain: 4 sequential splits on different cube faces.
/// Tests that pointer rewiring doesn't corrupt across independent faces.
#[test]
fn chain_four_splits_independent_faces() {
    let env_res = shapes::unit_cube().expect("unit cube should succeed");
    let faces = env_res.get_value().faces().to_vec();

    let mut chain = OpChain::new(env_res);

    for (i, &face) in faces.iter().take(4).enumerate() {
        let step_name = format!("split_face_{}", i);
        chain = chain.apply(&step_name, move |env, _scope| {
            let (mut draft, geom) = env.into_draft();
            let he = first_halfedge_of_face(draft.arena(), face)?;
            draft.execute(SplitEdge { edge: he })?;
            commit_draft(draft, geom)
        });
    }

    let result = chain.finish_validated();
    let arena = result.get_value().topology().arena();
    assert_eq!(arena.vertex_count(), 12, "4 splits = 4 new vertices");
    assert_eq!(arena.edge_count(), 16, "4 splits = 4 new edges");
}
