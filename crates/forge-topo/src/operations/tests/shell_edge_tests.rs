//! Shell and Edge entity lifecycle tests.
//!
//! DOMAIN: Verifies the integration between production Euler operators and
//! Shell/Edge arena storage. These tests check both entity counts AND
//! referential integrity — every test walks at least one IDs←→arena pointer
//! pair to catch wiring bugs that count-only assertions would miss.

use crate::b_rep::ShellKind;
use crate::boundary_editing::join_faces::JoinFaces;
use crate::entity_lifecycle::kill_edge_vertex::KillEdgeVertex;
use crate::entity_lifecycle::make_edge_face::MakeEdgeFace;
use crate::entity_lifecycle::make_vertex_face::MakeVertexFace;
use crate::entity_lifecycle::split_edge::SplitEdge;
use crate::transactions::TopologyState;

/// MVF produces exactly 1 Shell and 1 Edge, both properly referenced.
///
/// INVARIANTS TESTED:
/// - shell_count == 1 after MVF
/// - edge_count == 1 after MVF
/// - face.shell() is a live ShellId
/// - halfedge.edge() is a live EdgeId
/// - EdgeData.half_edge() round-trips back to the same halfedge
#[test]
fn mvf_shell_and_edge_are_live_and_wired() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let out = draft
        .execute(MakeVertexFace {
            shell_kind: ShellKind::Sheet,
        })
        .unwrap()
        .into_value();

    assert_eq!(draft.arena().shell_count(), 1);
    assert_eq!(draft.arena().edge_count(), 1);

    let shell_id = draft.arena().get_face(out.face).unwrap().shell();
    assert!(
        draft.arena().get_shell(shell_id).is_ok(),
        "face.shell must be a live ShellId"
    );

    let edge_id = draft.arena().get_half_edge(out.half_edge).unwrap().edge();
    assert!(
        draft.arena().get_edge(edge_id).is_ok(),
        "halfedge.edge must be a live EdgeId"
    );

    let rep = draft.arena().get_edge(edge_id).unwrap().half_edge();
    assert_eq!(
        rep, out.half_edge,
        "EdgeData.half_edge must round-trip to the seed halfedge"
    );
}

/// MEF creates a new face that inherits the Shell from its parent — not a new Shell.
///
/// INVARIANTS TESTED:
/// - shell_count == 1 throughout the sequence
/// - both faces reference the same ShellId
#[test]
fn mef_inherits_shell_does_not_create_new_one() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let mvf = draft
        .execute(MakeVertexFace {
            shell_kind: ShellKind::Sheet,
        })
        .unwrap()
        .into_value();
    let se = draft
        .execute(SplitEdge {
            edge: mvf.half_edge,
        })
        .unwrap()
        .into_value();

    let original_shell = draft.arena().get_face(mvf.face).unwrap().shell();

    let mef = draft
        .execute(MakeEdgeFace {
            vertex_a: mvf.vertex,
            vertex_b: se.new_vertex,
            face: mvf.face,
        })
        .unwrap()
        .into_value();

    assert_eq!(
        draft.arena().shell_count(),
        1,
        "MEF must not create a new Shell"
    );

    let shell_a = draft.arena().get_face(mvf.face).unwrap().shell();
    let shell_b = draft.arena().get_face(mef.new_face).unwrap().shell();
    assert_eq!(
        shell_a, original_shell,
        "original face must still reference the original Shell"
    );
    assert_eq!(
        shell_b, original_shell,
        "new MEF face must inherit the same Shell"
    );
}

/// Both twin halfedges produced by MEF share the same EdgeId.
///
/// INVARIANTS TESTED:
/// - halfedge_ab.edge() == halfedge_ba.edge()
/// - EdgeData.half_edge() is one of the two twins
#[test]
fn twin_halfedges_share_edge_id() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let mvf = draft
        .execute(MakeVertexFace {
            shell_kind: ShellKind::Sheet,
        })
        .unwrap()
        .into_value();
    let se = draft
        .execute(SplitEdge {
            edge: mvf.half_edge,
        })
        .unwrap()
        .into_value();
    let mef = draft
        .execute(MakeEdgeFace {
            vertex_a: mvf.vertex,
            vertex_b: se.new_vertex,
            face: mvf.face,
        })
        .unwrap()
        .into_value();

    let ab_data = draft.arena().get_half_edge(mef.half_edge_ab).unwrap();
    let ba_data = draft.arena().get_half_edge(mef.half_edge_ba).unwrap();
    assert_eq!(
        ab_data.edge(),
        ba_data.edge(),
        "twin halfedges must share the same EdgeId"
    );

    let rep = draft.arena().get_edge(ab_data.edge()).unwrap().half_edge();
    assert!(
        rep == mef.half_edge_ab || rep == mef.half_edge_ba,
        "EdgeData.half_edge must be one of the two twins"
    );
}

/// MEF + JoinFaces is an exact inverse: net edge delta is zero.
///
/// INVARIANTS TESTED:
/// - edge_count returns to baseline after round-trip
/// - shell_count is unchanged
/// - the EdgeId created by MEF is dead (arena.get_edge returns Err) after JoinFaces
#[test]
fn mef_then_join_faces_is_exact_inverse() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let mvf = draft
        .execute(MakeVertexFace {
            shell_kind: ShellKind::Sheet,
        })
        .unwrap()
        .into_value();
    let se = draft
        .execute(SplitEdge {
            edge: mvf.half_edge,
        })
        .unwrap()
        .into_value();

    let edges_before = draft.arena().edge_count();
    let shells_before = draft.arena().shell_count();

    let mef = draft
        .execute(MakeEdgeFace {
            vertex_a: mvf.vertex,
            vertex_b: se.new_vertex,
            face: mvf.face,
        })
        .unwrap()
        .into_value();

    let mef_edge_id = draft
        .arena()
        .get_half_edge(mef.half_edge_ab)
        .unwrap()
        .edge();
    assert_eq!(
        draft.arena().edge_count(),
        edges_before + 1,
        "MEF must add 1 Edge"
    );

    draft
        .execute(JoinFaces {
            edge: mef.half_edge_ab,
        })
        .unwrap()
        .into_value();

    assert_eq!(
        draft.arena().edge_count(),
        edges_before,
        "JoinFaces must remove 1 Edge (net zero)"
    );
    assert_eq!(
        draft.arena().shell_count(),
        shells_before,
        "Shell count must be unchanged"
    );
    assert!(
        draft.arena().get_edge(mef_edge_id).is_err(),
        "MEF EdgeId must be dead after JoinFaces"
    );
}

/// SplitEdge + KillEdgeVertex is an exact inverse: net edge delta is zero.
///
/// INVARIANTS TESTED:
/// - edge_count returns to baseline after round-trip
/// - the EdgeId created by SplitEdge is dead (arena.get_edge returns Err) after KEV
#[test]
fn split_edge_then_kev_is_exact_inverse() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    // Build a real (two-vertex) edge to split
    let mvf = draft
        .execute(MakeVertexFace {
            shell_kind: ShellKind::Sheet,
        })
        .unwrap()
        .into_value();
    let se1 = draft
        .execute(SplitEdge {
            edge: mvf.half_edge,
        })
        .unwrap()
        .into_value();

    let edges_before = draft.arena().edge_count();

    let se2 = draft
        .execute(SplitEdge { edge: se1.he_am })
        .unwrap()
        .into_value();

    let split_edge_id = draft.arena().get_half_edge(se2.he_am).unwrap().edge();
    assert_eq!(
        draft.arena().edge_count(),
        edges_before + 1,
        "SplitEdge must add 1 Edge"
    );

    draft
        .execute(KillEdgeVertex { edge: se2.he_am })
        .unwrap()
        .into_value();

    assert_eq!(
        draft.arena().edge_count(),
        edges_before,
        "KEV must remove 1 Edge (net zero)"
    );
    assert!(
        draft.arena().get_edge(split_edge_id).is_err(),
        "SplitEdge EdgeId must be dead after KEV"
    );
}
