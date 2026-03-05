//! Tests for KillFaceVertex (KFV): inverse of MakeFaceVertex.
//!
//! DOMAIN: KFV removes a disjoint face seed from a shell.
//! Tests cover: entity count deltas, rejection of non-isolated faces,
//! and the MFV→KFV roundtrip.

use crate::entity_lifecycle::kill_face_vertex::KillFaceVertex;
use crate::entity_lifecycle::make_face_vertex::MakeFaceVertex;
use crate::entity_lifecycle::make_vertex_face::MakeVertexFace;
use crate::transactions::TopologyState;
use crate::b_rep::ShellKind;

/// KFV destroys exactly 1 face, 1 vertex, 1 halfedge, 1 loop, 1 edge.
#[test]
fn kfv_destroys_face_vertex() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let mvf = draft.execute(MakeVertexFace { shell_kind: ShellKind::Sheet }).unwrap().into_value();

    let mfv = draft.execute(
        MakeFaceVertex {
            shell: mvf.shell,
        },
    )
    .unwrap()
    .into_value();

    let v_before = draft.arena().vertex_count();
    let f_before = draft.arena().face_count();
    let he_before = draft.arena().half_edge_count();
    let l_before = draft.arena().loop_count();
    let e_before = draft.arena().edge_count();

    draft.execute(KillFaceVertex { face: mfv.face })
        .unwrap()
        .into_value();

    assert_eq!(draft.arena().vertex_count(), v_before - 1, "ΔV = -1");
    assert_eq!(draft.arena().face_count(), f_before - 1, "ΔF = -1");
    assert_eq!(draft.arena().half_edge_count(), he_before - 1, "ΔHE = -1");
    assert_eq!(draft.arena().loop_count(), l_before - 1, "ΔL = -1");
    assert_eq!(draft.arena().edge_count(), e_before - 1, "ΔE = -1");
}

/// MFV→KFV roundtrip preserves the original entity counts.
#[test]
fn mfv_kfv_roundtrip() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let mvf = draft.execute(MakeVertexFace { shell_kind: ShellKind::Sheet }).unwrap().into_value();

    let v_before = draft.arena().vertex_count();
    let f_before = draft.arena().face_count();
    let he_before = draft.arena().half_edge_count();
    let l_before = draft.arena().loop_count();
    let e_before = draft.arena().edge_count();

    let mfv = draft.execute(
        MakeFaceVertex {
            shell: mvf.shell,
        },
    )
    .unwrap()
    .into_value();

    draft.execute(KillFaceVertex { face: mfv.face })
        .unwrap()
        .into_value();

    assert_eq!(draft.arena().vertex_count(), v_before, "V restored");
    assert_eq!(draft.arena().face_count(), f_before, "F restored");
    assert_eq!(draft.arena().half_edge_count(), he_before, "HE restored");
    assert_eq!(draft.arena().loop_count(), l_before, "L restored");
    assert_eq!(draft.arena().edge_count(), e_before, "E restored");
}

/// KFV rejects a face that is not an isolated self-loop seed.
#[test]
fn kfv_rejects_non_isolated_face() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let mvf = draft.execute(MakeVertexFace { shell_kind: ShellKind::Sheet }).unwrap().into_value();

    // Split the seed edge to make it non-isolated
    draft.execute(
        crate::entity_lifecycle::split_edge::SplitEdge {
            edge: mvf.half_edge,
        },
    )
    .unwrap();

    let result = draft.execute(KillFaceVertex { face: mvf.face });
    assert!(
        result.is_err(),
        "KFV must reject a face with multiple halfedges"
    );
}
