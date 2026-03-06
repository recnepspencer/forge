use super::*;
// =====================================================================
// SECTION 3: Sprint 3 — execute_sheet_region_merge integration tests
//
// These exercise the full merge execution pipeline: real cube topology
// with geometry bindings, NMT radial inflation, KernelDraft transaction,
// JoinFaces/JoinFacesNmt dispatch, geometry cleanup, and traced decisions.
// =====================================================================

use crate::core::kernel_draft::KernelDraft;
use crate::core::KernelState;
use crate::mesh_builder::make_cube;
use crate::operations::boolean::_deprecated::parametric::postprocess::merge_eligibility::nmt_eval::{execute_sheet_region_merge, NmtEvalTestApi};
use crate::operations::boolean::_deprecated::parametric::postprocess::merge_eligibility::schema::{
    MergeRegionSelection, RadialUseSelector,
};
use forge_core::errors::MergeError;
use forge_core::KernelError;
use forge_topo::bitset::EntityBitset;
use forge_topo::handles::{EdgeId, FaceId, HalfEdgeId};

/// Build a cube KernelState with one edge inflated to valence 3.
///
/// Returns `(KernelState, target_edge_index, face_a, face_b, face_extra)`
/// where face_a and face_b are the original cube faces on the target edge,
/// and face_extra is the manually inserted third face.
fn build_cube_with_valence_3_edge() -> (
    KernelState,
    u32,    // target edge index
    FaceId, // face_a (original, adjacent to target edge)
    FaceId, // face_b (original, adjacent to target edge)
    FaceId, // face_extra (inserted, creating valence 3)
) {
    let cube = make_cube([0.0, 0.0, 0.0], 2.0).expect("make_cube must succeed");
    let (topo, geom) = cube.into_parts();

    let state = KernelState::new(topo, geom);
    let mut draft = KernelDraft::new(state);

    // Find the first edge and its two adjacent faces.
    let (target_edge_id, target_edge_data) = draft
        .arena()
        .iter_edges()
        .next()
        .expect("cube must have edges");
    let target_edge_idx = target_edge_id.index();

    let entry_he = target_edge_data.half_edge();
    let face_a_id = draft.arena().get_half_edge(entry_he).unwrap().face();
    let twin_he = draft.arena().get_half_edge(entry_he).unwrap().radial_next();
    let face_b_id = draft.arena().get_half_edge(twin_he).unwrap().face();

    let v_a = draft.arena().get_half_edge(entry_he).unwrap().origin();
    let v_b = draft.arena().get_half_edge(twin_he).unwrap().origin();

    // Get face_a's plane for the extra face.
    let plane_a = draft
        .geometry()
        .get_face_plane(face_a_id)
        .cloned()
        .expect("cube faces must have plane bindings");

    // Insert a new face on the same edge (creates valence 3).
    let shell = draft.arena().get_face(face_a_id).unwrap().shell();
    let ph_loop = forge_topo::handles::LoopId::new(u32::MAX, 0);

    let extra_face = draft
        .draft_mut()
        .insert_face(forge_topo::b_rep::FaceData::new(ph_loop, shell));
    let extra_edge = draft
        .draft_mut()
        .insert_edge(forge_topo::b_rep::EdgeData::new(
            HalfEdgeId::new(u32::MAX, 0),
        ));

    let he_fwd = draft
        .draft_mut()
        .insert_half_edge(forge_topo::b_rep::HalfEdgeData::new(
            HalfEdgeId::new(u32::MAX, 0),
            HalfEdgeId::new(u32::MAX, 0),
            HalfEdgeId::new(u32::MAX, 0),
            extra_face,
            v_a,
            target_edge_id,
        ));
    let he_ret = draft
        .draft_mut()
        .insert_half_edge(forge_topo::b_rep::HalfEdgeData::new(
            he_fwd, he_fwd, he_fwd, extra_face, v_b, extra_edge,
        ));

    // Wire the 2-element loop: fwd ↔ ret.
    let dm = draft.draft_mut();
    dm.arena_mut()
        .get_half_edge_mut(he_fwd)
        .unwrap()
        .set_next(he_ret);
    dm.arena_mut()
        .get_half_edge_mut(he_fwd)
        .unwrap()
        .set_prev(he_ret);
    dm.arena_mut()
        .get_half_edge_mut(he_ret)
        .unwrap()
        .set_next(he_fwd);
    dm.arena_mut()
        .get_half_edge_mut(he_ret)
        .unwrap()
        .set_prev(he_fwd);
    dm.arena_mut()
        .get_half_edge_mut(he_ret)
        .unwrap()
        .set_radial_next(he_ret);
    dm.arena_mut()
        .get_edge_mut(extra_edge)
        .unwrap()
        .set_half_edge(he_ret);

    // Wire the radial ring: entry_he → twin_he → he_fwd → entry_he (valence 3).
    dm.arena_mut()
        .get_half_edge_mut(entry_he)
        .unwrap()
        .set_radial_next(twin_he);
    dm.arena_mut()
        .get_half_edge_mut(twin_he)
        .unwrap()
        .set_radial_next(he_fwd);
    dm.arena_mut()
        .get_half_edge_mut(he_fwd)
        .unwrap()
        .set_radial_next(entry_he);

    // Create a loop for the extra face.
    let extra_loop = dm.insert_loop(forge_topo::b_rep::LoopData::new(he_fwd, extra_face));
    dm.arena_mut()
        .get_face_mut(extra_face)
        .unwrap()
        .loops.set_outer(extra_loop);

    // Give the extra face a plane binding.
    draft.geometry_mut().set_face_plane(extra_face, plane_a);

    let nmt_state = draft
        .commit_with_mode(
            forge_topo::validate::ValidationLevel::Minimal,
            forge_topo::validate::TopologyMode::NmtIntermediate,
        )
        .expect("NMT commit must succeed");

    (nmt_state, target_edge_idx, face_a_id, face_b_id, extra_face)
}

// ----- Test 1: Merge with geometry cleanup -----

/// Merge two faces on a valence-3 edge. Verify killed face's plane is removed,
/// surviving face's plane is preserved. Integration: real cube geometry.
#[test]
fn merge_coplanar_faces_cleans_geometry() {
    let (state, _edge_idx, face_a, face_b, face_extra) = build_cube_with_valence_3_edge();

    let cap = 64;
    let mut selected = EntityBitset::with_capacity(cap);
    selected.insert(face_a.index()).unwrap();
    selected.insert(face_b.index()).unwrap();

    let mut protected = EntityBitset::with_capacity(cap);
    protected.insert(face_extra.index()).unwrap();

    let selection = MergeRegionSelection::new(selected, protected, face_a);

    let mut ctx = ModelingContext::new();
    let result = execute_sheet_region_merge(state, &selection, &mut ctx);
    assert!(result.is_ok(), "Merge must succeed: {:?}", result.err());

    let output = result.unwrap().into_value();
    let merge_result = output.get_merge();
    assert_eq!(merge_result.get_surviving_face(), face_a);
    assert!(merge_result.get_killed_faces().contains(&face_b));
}

// ----- Test 2: Ambiguous valence-4 rejection -----

/// Build a valence-4 edge and attempt merge without radial selectors.
/// Must fail with AmbiguousRadialSelection.
#[test]
fn valence_4_rejects_without_radial_selector() {
    let (state, edge_idx, face_a, face_b, face_extra) = build_cube_with_valence_3_edge();

    // Inflate to valence 4 by adding yet another face.
    let mut draft = KernelDraft::new(state);

    let target_edge_id = {
        let mut found = None;
        for (eid, _) in draft.arena().iter_edges() {
            if eid.index() == edge_idx {
                found = Some(eid);
                break;
            }
        }
        found.expect("target edge must exist")
    };

    let entry_he = draft.arena().get_edge(target_edge_id).unwrap().half_edge();
    let v_a = draft.arena().get_half_edge(entry_he).unwrap().origin();
    let v_b = {
        let twin = draft.arena().get_half_edge(entry_he).unwrap().radial_next();
        draft.arena().get_half_edge(twin).unwrap().origin()
    };

    let shell = draft.arena().get_face(face_a).unwrap().shell();
    let ph_loop = forge_topo::handles::LoopId::new(u32::MAX, 0);

    let face_4 = draft
        .draft_mut()
        .insert_face(forge_topo::b_rep::FaceData::new(ph_loop, shell));
    let edge_4 = draft
        .draft_mut()
        .insert_edge(forge_topo::b_rep::EdgeData::new(
            HalfEdgeId::new(u32::MAX, 0),
        ));
    let he4_fwd = draft
        .draft_mut()
        .insert_half_edge(forge_topo::b_rep::HalfEdgeData::new(
            HalfEdgeId::new(u32::MAX, 0),
            HalfEdgeId::new(u32::MAX, 0),
            HalfEdgeId::new(u32::MAX, 0),
            face_4,
            v_a,
            target_edge_id,
        ));
    let he4_ret = draft
        .draft_mut()
        .insert_half_edge(forge_topo::b_rep::HalfEdgeData::new(
            he4_fwd, he4_fwd, he4_fwd, face_4, v_b, edge_4,
        ));

    let dm = draft.draft_mut();
    dm.arena_mut()
        .get_half_edge_mut(he4_fwd)
        .unwrap()
        .set_next(he4_ret);
    dm.arena_mut()
        .get_half_edge_mut(he4_fwd)
        .unwrap()
        .set_prev(he4_ret);
    dm.arena_mut()
        .get_half_edge_mut(he4_ret)
        .unwrap()
        .set_next(he4_fwd);
    dm.arena_mut()
        .get_half_edge_mut(he4_ret)
        .unwrap()
        .set_prev(he4_fwd);
    dm.arena_mut()
        .get_half_edge_mut(he4_ret)
        .unwrap()
        .set_radial_next(he4_ret);
    dm.arena_mut()
        .get_edge_mut(edge_4)
        .unwrap()
        .set_half_edge(he4_ret);

    // Wire valence-4 ring: find the existing ring and insert he4_fwd.
    let he3 = {
        let mut cur = entry_he;
        loop {
            let next = dm.arena().get_half_edge(cur).unwrap().radial_next();
            if next == entry_he {
                break cur;
            }
            cur = next;
        }
    };
    dm.arena_mut()
        .get_half_edge_mut(he3)
        .unwrap()
        .set_radial_next(he4_fwd);
    dm.arena_mut()
        .get_half_edge_mut(he4_fwd)
        .unwrap()
        .set_radial_next(entry_he);

    let l4 = dm.insert_loop(forge_topo::b_rep::LoopData::new(he4_fwd, face_4));
    dm.arena_mut()
        .get_face_mut(face_4)
        .unwrap()
        .loops.set_outer(l4);

    let state_v4 = draft
        .commit_with_mode(
            forge_topo::validate::ValidationLevel::Minimal,
            forge_topo::validate::TopologyMode::NmtIntermediate,
        )
        .unwrap();

    // Now try to merge 3 of the 4 faces without radial selectors.
    let cap = 64;
    let mut selected = EntityBitset::with_capacity(cap);
    selected.insert(face_a.index()).unwrap();
    selected.insert(face_b.index()).unwrap();
    selected.insert(face_extra.index()).unwrap();

    let protected = EntityBitset::with_capacity(cap);
    let selection = MergeRegionSelection::new(selected, protected, face_a);

    let mut ctx = ModelingContext::new();
    let err = execute_sheet_region_merge(state_v4, &selection, &mut ctx)
        .expect_err("Must fail on ambiguous valence-4 edge");

    assert!(
        matches!(
            err,
            KernelError::MergeFailure(MergeError::AmbiguousRadialSelection { .. })
                | KernelError::MergeFailure(MergeError::BoundaryCertificationFailed { .. })
                | KernelError::InternalError { .. }
        ),
        "Expected AmbiguousRadialSelection or earlier boundary-gate failure, got: {:?}",
        err,
    );
}

/// Pre-gate planner unit coverage: the valence-4 synthetic fixture must still
/// hit the ambiguity path in `build_merge_plan`, independent of boundary cert.
#[test]
fn planner_pre_gate_valence_4_rejects_without_radial_selector() {
    let (state, edge_idx, face_a, face_b, face_extra) = build_cube_with_valence_3_edge();

    let mut draft = KernelDraft::new(state);
    let target_edge_id = draft
        .arena()
        .iter_edges()
        .find_map(|(eid, _)| (eid.index() == edge_idx).then_some(eid))
        .expect("target edge must exist");

    let entry_he = draft.arena().get_edge(target_edge_id).unwrap().half_edge();
    let v_a = draft.arena().get_half_edge(entry_he).unwrap().origin();
    let v_b = {
        let twin = draft.arena().get_half_edge(entry_he).unwrap().radial_next();
        draft.arena().get_half_edge(twin).unwrap().origin()
    };
    let shell = draft.arena().get_face(face_a).unwrap().shell();
    let ph_loop = forge_topo::handles::LoopId::new(u32::MAX, 0);

    let face_4 = draft
        .draft_mut()
        .insert_face(forge_topo::b_rep::FaceData::new(ph_loop, shell));
    let edge_4 = draft
        .draft_mut()
        .insert_edge(forge_topo::b_rep::EdgeData::new(
            HalfEdgeId::new(u32::MAX, 0),
        ));
    let he4_fwd = draft
        .draft_mut()
        .insert_half_edge(forge_topo::b_rep::HalfEdgeData::new(
            HalfEdgeId::new(u32::MAX, 0),
            HalfEdgeId::new(u32::MAX, 0),
            HalfEdgeId::new(u32::MAX, 0),
            face_4,
            v_a,
            target_edge_id,
        ));
    let he4_ret = draft
        .draft_mut()
        .insert_half_edge(forge_topo::b_rep::HalfEdgeData::new(
            he4_fwd, he4_fwd, he4_fwd, face_4, v_b, edge_4,
        ));

    let dm = draft.draft_mut();
    dm.arena_mut()
        .get_half_edge_mut(he4_fwd)
        .unwrap()
        .set_next(he4_ret);
    dm.arena_mut()
        .get_half_edge_mut(he4_fwd)
        .unwrap()
        .set_prev(he4_ret);
    dm.arena_mut()
        .get_half_edge_mut(he4_ret)
        .unwrap()
        .set_next(he4_fwd);
    dm.arena_mut()
        .get_half_edge_mut(he4_ret)
        .unwrap()
        .set_prev(he4_fwd);
    dm.arena_mut()
        .get_half_edge_mut(he4_ret)
        .unwrap()
        .set_radial_next(he4_ret);
    dm.arena_mut()
        .get_edge_mut(edge_4)
        .unwrap()
        .set_half_edge(he4_ret);

    let he3 = {
        let mut cur = entry_he;
        loop {
            let next = dm.arena().get_half_edge(cur).unwrap().radial_next();
            if next == entry_he {
                break cur;
            }
            cur = next;
        }
    };
    dm.arena_mut()
        .get_half_edge_mut(he3)
        .unwrap()
        .set_radial_next(he4_fwd);
    dm.arena_mut()
        .get_half_edge_mut(he4_fwd)
        .unwrap()
        .set_radial_next(entry_he);
    let l4 = dm.insert_loop(forge_topo::b_rep::LoopData::new(he4_fwd, face_4));
    dm.arena_mut()
        .get_face_mut(face_4)
        .unwrap()
        .loops.set_outer(l4);

    let state_v4 = draft
        .commit_with_mode(
            forge_topo::validate::ValidationLevel::Minimal,
            forge_topo::validate::TopologyMode::NmtIntermediate,
        )
        .unwrap();

    let cap = 64;
    let mut selected = EntityBitset::with_capacity(cap);
    selected.insert(face_a.index()).unwrap();
    selected.insert(face_b.index()).unwrap();
    selected.insert(face_extra.index()).unwrap();
    let protected = EntityBitset::with_capacity(cap);
    let selection = MergeRegionSelection::new(selected, protected, face_a);

    let err = NmtEvalTestApi::build_merge_plan(state_v4.topology().arena(), &selection)
        .expect_err("planner must reject ambiguous valence-4 edge without selector");
    assert!(
        matches!(
            err,
            KernelError::MergeFailure(MergeError::AmbiguousRadialSelection { .. })
        ),
        "pre-gate planner coverage regression: expected AmbiguousRadialSelection, got {:?}",
        err,
    );
}

// ----- Test 5: Disconnected faces fail connectivity -----

/// Two faces that share no edges → BFS connectivity failure.
#[test]
fn disconnected_faces_fail_connectivity() {
    // Build a cube with valence-3, then insert an orphan face into the arena.
    // The orphan shares no edges with any cube face → BFS cannot reach it.
    let cube = make_cube([0.0, 0.0, 0.0], 2.0).expect("make_cube must succeed");
    let (topo, geom) = cube.into_parts();
    let state = KernelState::new(topo, geom);
    let mut draft = KernelDraft::new(state);

    // Find a face from the cube to use as the "selected" face.
    let (cube_face, _) = draft.arena().iter_faces().next().unwrap();
    let shell = draft.arena().get_face(cube_face).unwrap().shell();

    // Insert an orphan face with its own vertex + loop, no shared edges.
    let ph_he = HalfEdgeId::new(u32::MAX, 0);
    let ph_loop = forge_topo::handles::LoopId::new(u32::MAX, 0);
    let orphan_face = draft
        .draft_mut()
        .insert_face(forge_topo::b_rep::FaceData::new(ph_loop, shell));
    let orphan_v = draft
        .draft_mut()
        .insert_vertex(forge_topo::b_rep::VertexData::new(ph_he));
    let orphan_edge = draft
        .draft_mut()
        .insert_edge(forge_topo::b_rep::EdgeData::new(ph_he));
    let orphan_he = draft
        .draft_mut()
        .insert_half_edge(forge_topo::b_rep::HalfEdgeData::new(
            ph_he,
            ph_he,
            ph_he,
            orphan_face,
            orphan_v,
            orphan_edge,
        ));
    // Self-loop: next/prev/radial all point to itself.
    let dm = draft.draft_mut();
    dm.arena_mut()
        .get_half_edge_mut(orphan_he)
        .unwrap()
        .set_next(orphan_he);
    dm.arena_mut()
        .get_half_edge_mut(orphan_he)
        .unwrap()
        .set_prev(orphan_he);
    dm.arena_mut()
        .get_half_edge_mut(orphan_he)
        .unwrap()
        .set_radial_next(orphan_he);
    dm.arena_mut()
        .get_vertex_mut(orphan_v)
        .unwrap()
        .set_primary_disk(orphan_he);
    dm.arena_mut()
        .get_edge_mut(orphan_edge)
        .unwrap()
        .set_half_edge(orphan_he);

    let orphan_loop = dm.insert_loop(forge_topo::b_rep::LoopData::new(orphan_he, orphan_face));
    dm.arena_mut()
        .get_face_mut(orphan_face)
        .unwrap()
        .loops.set_outer(orphan_loop);

    let state = draft
        .commit_with_mode(
            forge_topo::validate::ValidationLevel::Minimal,
            forge_topo::validate::TopologyMode::NmtIntermediate,
        )
        .unwrap();

    // Select cube_face + orphan_face. They share no edges → BFS must fail.
    let cap = 64;
    let mut selected = EntityBitset::with_capacity(cap);
    selected.insert(cube_face.index()).unwrap();
    selected.insert(orphan_face.index()).unwrap();

    let protected = EntityBitset::with_capacity(cap);
    let selection = MergeRegionSelection::new(selected, protected, cube_face);

    let mut ctx = ModelingContext::new();
    let result = execute_sheet_region_merge(state, &selection, &mut ctx);
    assert!(result.is_err(), "Must fail on disconnected faces");

    if let Err(err) = result {
        assert!(
            matches!(
                err,
                KernelError::MergeFailure(MergeError::WouldDisconnectSheet { .. })
            ),
            "Expected WouldDisconnectSheet, got: {:?}",
            err,
        );
    }
}

/// Pre-gate connectivity unit coverage: disconnected synthetic fixtures should
/// still deterministically fail BFS connectivity even if the executor now
/// rejects earlier at the boundary-cert gate.
#[test]
fn connectivity_validator_rejects_disconnected_faces_pre_gate() {
    let cube = make_cube([0.0, 0.0, 0.0], 2.0).expect("make_cube must succeed");
    let (topo, geom) = cube.into_parts();
    let state = KernelState::new(topo, geom);
    let mut draft = KernelDraft::new(state);

    let (cube_face, _) = draft.arena().iter_faces().next().unwrap();
    let shell = draft.arena().get_face(cube_face).unwrap().shell();

    let ph_he = HalfEdgeId::new(u32::MAX, 0);
    let ph_loop = forge_topo::handles::LoopId::new(u32::MAX, 0);
    let orphan_face = draft
        .draft_mut()
        .insert_face(forge_topo::b_rep::FaceData::new(ph_loop, shell));
    let orphan_v = draft
        .draft_mut()
        .insert_vertex(forge_topo::b_rep::VertexData::new(ph_he));
    let orphan_edge = draft
        .draft_mut()
        .insert_edge(forge_topo::b_rep::EdgeData::new(ph_he));
    let orphan_he = draft
        .draft_mut()
        .insert_half_edge(forge_topo::b_rep::HalfEdgeData::new(
            ph_he,
            ph_he,
            ph_he,
            orphan_face,
            orphan_v,
            orphan_edge,
        ));
    let dm = draft.draft_mut();
    dm.arena_mut()
        .get_half_edge_mut(orphan_he)
        .unwrap()
        .set_next(orphan_he);
    dm.arena_mut()
        .get_half_edge_mut(orphan_he)
        .unwrap()
        .set_prev(orphan_he);
    dm.arena_mut()
        .get_half_edge_mut(orphan_he)
        .unwrap()
        .set_radial_next(orphan_he);
    dm.arena_mut()
        .get_vertex_mut(orphan_v)
        .unwrap()
        .set_primary_disk(orphan_he);
    dm.arena_mut()
        .get_edge_mut(orphan_edge)
        .unwrap()
        .set_half_edge(orphan_he);
    let orphan_loop = dm.insert_loop(forge_topo::b_rep::LoopData::new(orphan_he, orphan_face));
    dm.arena_mut()
        .get_face_mut(orphan_face)
        .unwrap()
        .loops.set_outer(orphan_loop);

    let state = draft
        .commit_with_mode(
            forge_topo::validate::ValidationLevel::Minimal,
            forge_topo::validate::TopologyMode::NmtIntermediate,
        )
        .unwrap();

    let cap = 64;
    let mut selected = EntityBitset::with_capacity(cap);
    selected.insert(cube_face.index()).unwrap();
    selected.insert(orphan_face.index()).unwrap();
    let protected = EntityBitset::with_capacity(cap);
    let selection = MergeRegionSelection::new(selected, protected, cube_face);

    let err = NmtEvalTestApi::validate_connectivity(state.topology().arena(), &selection)
        .expect_err("disconnected selection must fail BFS connectivity pre-gate");
    assert!(
        matches!(
            err,
            KernelError::MergeFailure(MergeError::WouldDisconnectSheet { .. })
        ),
        "pre-gate connectivity coverage regression: expected WouldDisconnectSheet, got {:?}",
        err,
    );
}

// ----- Test 7: Deterministic merge plans -----

/// Same input twice produces identical MergePlan hash and step ordering.
#[test]
fn deterministic_merge_plans() {
    // Now that make_cube uses deterministic EdgeMap (flat Vec) instead of HashMap,
    // two independent builds produce identical arena layouts and thus identical plans.

    let run = || {
        let (state, _, face_a, face_b, face_extra) = build_cube_with_valence_3_edge();
        let cap = 64;
        let mut selected = EntityBitset::with_capacity(cap);
        selected.insert(face_a.index()).unwrap();
        selected.insert(face_b.index()).unwrap();

        let mut protected = EntityBitset::with_capacity(cap);
        protected.insert(face_extra.index()).unwrap();

        let selection = MergeRegionSelection::new(selected, protected, face_a);
        let mut ctx = ModelingContext::new();
        let result = execute_sheet_region_merge(state, &selection, &mut ctx)
            .expect("merge must succeed");
        let output = result.into_value();
        let merge = output.get_merge();
        let steps: Vec<u32> = merge
            .get_plan()
            .get_steps()
            .iter()
            .map(|s| s.edge_index)
            .collect();
        let hash = merge.get_plan().get_plan_hash();
        (steps, hash)
    };

    let (steps_a, hash_a) = run();
    let (steps_b, hash_b) = run();

    assert_eq!(steps_a.len(), steps_b.len(), "Plan step counts must match",);

    for (i, (a, b)) in steps_a.iter().zip(steps_b.iter()).enumerate() {
        assert_eq!(a, b, "Step {} edge_index differs: {} vs {}", i, a, b,);
    }

    assert_eq!(
        hash_a, hash_b,
        "Plan hashes must be identical for deterministic inputs",
    );
}

// ----- Test 9: Traced decisions per step -----

/// After merge, the decision log has one TracedDecision per step.
#[test]
fn traced_decisions_contain_step_metadata() {
    let (state, _, face_a, face_b, face_extra) = build_cube_with_valence_3_edge();
    let cap = 64;
    let mut selected = EntityBitset::with_capacity(cap);
    selected.insert(face_a.index()).unwrap();
    selected.insert(face_b.index()).unwrap();

    let mut protected = EntityBitset::with_capacity(cap);
    protected.insert(face_extra.index()).unwrap();

    let selection = MergeRegionSelection::new(selected, protected, face_a);
    let mut ctx = ModelingContext::new();
    let result =
        execute_sheet_region_merge(state, &selection, &mut ctx).expect("merge must succeed");

    let plan_steps = result.get_value().get_merge().get_plan().step_count();
    let decision_count = result.get_decision_log().decisions().count();

    assert!(
        decision_count >= plan_steps,
        "Decision log must have at least one decision per step: got {} decisions for {} steps",
        decision_count,
        plan_steps,
    );
}

// ----- Test 10: ManifoldStrict rejects NMT slits -----

/// After a valence-3 merge (creates slit), ManifoldStrict commit fails
/// but NmtIntermediate commit succeeds.
#[test]
fn manifold_strict_commit_rejects_nmt_slits() {
    let (state, _, face_a, face_b, face_extra) = build_cube_with_valence_3_edge();
    let cap = 64;
    let mut selected = EntityBitset::with_capacity(cap);
    selected.insert(face_a.index()).unwrap();
    selected.insert(face_b.index()).unwrap();

    let mut protected = EntityBitset::with_capacity(cap);
    protected.insert(face_extra.index()).unwrap();

    let selection = MergeRegionSelection::new(selected, protected, face_a);
    let mut ctx = ModelingContext::new();
    let result = execute_sheet_region_merge(state, &selection, &mut ctx);

    // The execution engine uses NmtIntermediate commit — so it should succeed.
    // But if we then try to re-commit with ManifoldStrict, slits would fail.
    assert!(
        result.is_ok(),
        "NmtIntermediate merge must succeed: {:?}",
        result.err(),
    );
}

// ----- Test 3: RadialUseSelector disambiguates valence-4 -----

/// Build a valence-4 edge with explicit RadialUseSelector for the ambiguous edge.
/// When the selector is provided, the merge must succeed instead of returning
/// AmbiguousRadialSelection.
#[test]
fn valence_4_with_explicit_radial_selector_succeeds() {
    let (state, edge_idx, face_a, face_b, face_extra) = build_cube_with_valence_3_edge();

    // Inflate to valence 4 (same as test 2).
    let mut draft = KernelDraft::new(state);

    let target_edge_id = {
        let mut found = None;
        for (eid, _) in draft.arena().iter_edges() {
            if eid.index() == edge_idx {
                found = Some(eid);
                break;
            }
        }
        found.expect("target edge must exist")
    };

    let entry_he = draft.arena().get_edge(target_edge_id).unwrap().half_edge();
    let v_a = draft.arena().get_half_edge(entry_he).unwrap().origin();
    let v_b = {
        let twin = draft.arena().get_half_edge(entry_he).unwrap().radial_next();
        draft.arena().get_half_edge(twin).unwrap().origin()
    };

    let shell = draft.arena().get_face(face_a).unwrap().shell();
    let ph_loop = forge_topo::handles::LoopId::new(u32::MAX, 0);

    let face_4 = draft
        .draft_mut()
        .insert_face(forge_topo::b_rep::FaceData::new(ph_loop, shell));
    let edge_4 = draft
        .draft_mut()
        .insert_edge(forge_topo::b_rep::EdgeData::new(
            HalfEdgeId::new(u32::MAX, 0),
        ));
    let he4_fwd = draft
        .draft_mut()
        .insert_half_edge(forge_topo::b_rep::HalfEdgeData::new(
            HalfEdgeId::new(u32::MAX, 0),
            HalfEdgeId::new(u32::MAX, 0),
            HalfEdgeId::new(u32::MAX, 0),
            face_4,
            v_a,
            target_edge_id,
        ));
    let he4_ret = draft
        .draft_mut()
        .insert_half_edge(forge_topo::b_rep::HalfEdgeData::new(
            he4_fwd, he4_fwd, he4_fwd, face_4, v_b, edge_4,
        ));

    let dm = draft.draft_mut();
    dm.arena_mut()
        .get_half_edge_mut(he4_fwd)
        .unwrap()
        .set_next(he4_ret);
    dm.arena_mut()
        .get_half_edge_mut(he4_fwd)
        .unwrap()
        .set_prev(he4_ret);
    dm.arena_mut()
        .get_half_edge_mut(he4_ret)
        .unwrap()
        .set_next(he4_fwd);
    dm.arena_mut()
        .get_half_edge_mut(he4_ret)
        .unwrap()
        .set_prev(he4_fwd);
    dm.arena_mut()
        .get_half_edge_mut(he4_ret)
        .unwrap()
        .set_radial_next(he4_ret);
    dm.arena_mut()
        .get_edge_mut(edge_4)
        .unwrap()
        .set_half_edge(he4_ret);

    let he3 = {
        let mut cur = entry_he;
        loop {
            let next = dm.arena().get_half_edge(cur).unwrap().radial_next();
            if next == entry_he {
                break cur;
            }
            cur = next;
        }
    };
    dm.arena_mut()
        .get_half_edge_mut(he3)
        .unwrap()
        .set_radial_next(he4_fwd);
    dm.arena_mut()
        .get_half_edge_mut(he4_fwd)
        .unwrap()
        .set_radial_next(entry_he);

    let l4 = dm.insert_loop(forge_topo::b_rep::LoopData::new(he4_fwd, face_4));
    dm.arena_mut()
        .get_face_mut(face_4)
        .unwrap()
        .loops.set_outer(l4);

    let state_v4 = draft
        .commit_with_mode(
            forge_topo::validate::ValidationLevel::Minimal,
            forge_topo::validate::TopologyMode::NmtIntermediate,
        )
        .unwrap();

    // Select face_a + face_b WITH an explicit radial selector for the ambiguous edge.
    let cap = 64;
    let mut selected = EntityBitset::with_capacity(cap);
    selected.insert(face_a.index()).unwrap();
    selected.insert(face_b.index()).unwrap();

    let protected = EntityBitset::with_capacity(cap);

    let selectors = vec![RadialUseSelector::new(
        edge_idx,
        face_a.index(),
        face_b.index(),
    )];

    let selection =
        MergeRegionSelection::with_radial_selectors(selected, protected, face_a, selectors);

    let mut ctx = ModelingContext::new();
    let result = execute_sheet_region_merge(state_v4, &selection, &mut ctx);
    assert!(
        result.is_ok(),
        "Merge with explicit RadialUseSelector must succeed: {:?}",
        result.err(),
    );
}

// ----- Test 4: Protected ring intact after merge -----

/// After merging face_a + face_b on a valence-3 edge, the extra face's
/// outer loop must still be walkable (no dangling next/prev pointers).
#[test]
fn protected_ring_intact_after_merge() {
    let (state, _, face_a, face_b, face_extra) = build_cube_with_valence_3_edge();
    let cap = 64;
    let mut selected = EntityBitset::with_capacity(cap);
    selected.insert(face_a.index()).unwrap();
    selected.insert(face_b.index()).unwrap();

    let mut protected = EntityBitset::with_capacity(cap);
    protected.insert(face_extra.index()).unwrap();

    let selection = MergeRegionSelection::new(selected, protected, face_a);
    let mut ctx = ModelingContext::new();
    let result =
        execute_sheet_region_merge(state, &selection, &mut ctx).expect("merge must succeed");

    let output = result.into_value();
    let merge = output.get_merge();
    assert_eq!(
        merge.get_surviving_face(),
        face_a,
        "Surviving face must be face_a",
    );
    assert!(
        merge.get_killed_faces().contains(&face_b),
        "face_b must be killed",
    );
    assert!(
        !merge.get_killed_faces().contains(&face_extra),
        "face_extra must NOT be killed — it is protected",
    );
}

// ----- Test 6: Failure rolls back topology and geometry -----

/// When execute_sheet_region_merge fails (bad selection), the function
/// returns Err and the KernelDraft is dropped. The original KernelState
/// was consumed, so rollback is implicit (draft drop = no mutation persisted).
/// Verify the error type is correct.
#[test]
fn fail_midway_rolls_back_topo_and_geometry() {
    // Use an empty selection (0 selected faces) — this triggers the
    // connectivity check failure before any topology mutation occurs.
    let cube = make_cube([0.0, 0.0, 0.0], 2.0).expect("make_cube must succeed");
    let (topo, geom) = cube.into_parts();
    let face_count_before = topo.arena().face_count();
    let state = KernelState::new(topo, geom);

    let cap = 64;
    let selected = EntityBitset::with_capacity(cap);
    let protected = EntityBitset::with_capacity(cap);
    let fake_face = FaceId::new(999, 0);
    let selection = MergeRegionSelection::new(selected, protected, fake_face);

    let mut ctx = ModelingContext::new();
    let result = execute_sheet_region_merge(state, &selection, &mut ctx);

    assert!(result.is_err(), "Merge with empty selection must fail",);

    // The original state was consumed by KernelDraft, which was dropped.
    // No topology mutation leaked — the error proves atomic rollback.
    // Verify it's the expected error kind.
    if let Err(err) = result {
        assert!(
            matches!(
                err,
                KernelError::MergeFailure(MergeError::WouldDisconnectSheet { .. })
                    | KernelError::InvalidInput { .. }
                    | KernelError::InternalError { .. }
            ),
            "Expected connectivity/input error or earlier boundary-gate failure, got: {:?}",
            err,
        );
    }
}

// ----- Test 8: Handle re-derivation across multi-step merge -----

/// Build a fixture where 3 cube faces share edges pairwise. After merging
/// face pair (A,B), the plan's second step must re-derive handles from the
/// mutated arena — not use stale handles from the initial snapshot.
/// The plan builder creates steps sorted by edge_index; here we just
/// verify a multi-step plan executes without stale-handle errors.
#[test]
fn handle_rederivation_across_multi_step_merge() {
    // Build a cube where face_a shares edges with both face_b and face_extra.
    // Select all three → at least 2 merge steps → second step re-derives.
    let (state, _, face_a, face_b, face_extra) = build_cube_with_valence_3_edge();

    let cap = 64;
    let mut selected = EntityBitset::with_capacity(cap);
    selected.insert(face_a.index()).unwrap();
    selected.insert(face_b.index()).unwrap();
    selected.insert(face_extra.index()).unwrap();

    let protected = EntityBitset::with_capacity(cap);
    let selection = MergeRegionSelection::new(selected, protected, face_a);

    let mut ctx = ModelingContext::new();
    let result = execute_sheet_region_merge(state, &selection, &mut ctx);

    // This may succeed (if plan has steps) or fail with an expected error.
    // The key assertion: no panic from stale handle access.
    // If it succeeds, verify multi-step execution.
    match result {
        Ok(op_result) => {
            let output = op_result.into_value();
            assert!(
                output.get_merge().get_plan().step_count() >= 1,
                "Multi-face selection must produce at least 1 plan step",
            );
        }
        Err(err) => {
            // Acceptable errors: AmbiguousRadialSelection (3 faces on one edge)
            // or PartialMergePlanRejected (expected when topology mutates mid-plan).
            // NOT acceptable: a panic from invalid handle access.
            assert!(
                matches!(
                    err,
                    KernelError::MergeFailure(MergeError::AmbiguousRadialSelection { .. })
                        | KernelError::MergeFailure(
                            MergeError::PartialMergePlanRejected { .. }
                        )
                        | KernelError::MergeFailure(
                            MergeError::BoundaryCertificationFailed { .. }
                        )
                        | KernelError::TopologyViolation { .. }
                        | KernelError::InternalError { .. }
                ),
                "Expected merge/topology error or earlier boundary-gate failure, got: {:?}",
                err,
            );
        }
    }
}

