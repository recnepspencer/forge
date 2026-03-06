//! Tests for MakeFaceVertex (MFV): disjoint face+vertex in existing shell.
//!
//! DOMAIN: MFV adds a face seed to an existing shell, without creating
//! a new solid. Tests verify entity counts, self-loop wiring, and lineage.

use crate::b_rep::ShellKind;
use crate::entity_lifecycle::make_face_vertex::MakeFaceVertex;
use crate::entity_lifecycle::make_vertex_face::MakeVertexFace;
use crate::provenance::OpSignature;
use crate::transactions::TopologyState;

/// MFV creates exactly 1 face, 1 vertex, 1 halfedge, 1 loop, 1 edge
/// without creating any new shell or solid.
#[test]
fn mfv_creates_face_vertex_in_shell() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let mvf = draft
        .execute(MakeVertexFace {
            shell_kind: ShellKind::Sheet,
        })
        .unwrap()
        .into_value();

    let v_before = draft.arena().vertex_count();
    let f_before = draft.arena().face_count();
    let he_before = draft.arena().half_edge_count();
    let l_before = draft.arena().loop_count();
    let e_before = draft.arena().edge_count();
    let s_before = draft.arena().shell_count();
    let so_before = draft.arena().body_count();

    let mfv = draft
        .execute(MakeFaceVertex { shell: mvf.shell })
        .unwrap()
        .into_value();

    assert_eq!(draft.arena().vertex_count(), v_before + 1, "ΔV = +1");
    assert_eq!(draft.arena().face_count(), f_before + 1, "ΔF = +1");
    assert_eq!(draft.arena().half_edge_count(), he_before + 1, "ΔHE = +1");
    assert_eq!(draft.arena().loop_count(), l_before + 1, "ΔL = +1");
    assert_eq!(draft.arena().edge_count(), e_before + 1, "ΔE = +1");
    assert_eq!(draft.arena().shell_count(), s_before, "ΔS = 0");
    assert_eq!(draft.arena().body_count(), so_before, "ΔSo = 0");

    let he = draft.arena().get_half_edge(mfv.half_edge).unwrap();
    assert_eq!(he.radial_next(), mfv.half_edge, "seed must be self-radial");
    assert_eq!(he.next(), mfv.half_edge, "seed must be self-next");
    assert_eq!(he.prev(), mfv.half_edge, "seed must be self-prev");
    assert_eq!(he.origin(), mfv.vertex);
    assert_eq!(he.face(), mfv.face);
}

// TODO(Phase 3): Re-enable once LineageStore lookup is wired.
// /// MFV stamps lineage derived from the OpSignature on all created entities.
// #[test]
// fn mfv_stamps_lineage() { ... }
