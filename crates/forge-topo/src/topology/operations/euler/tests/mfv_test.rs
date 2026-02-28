//! Tests for MakeFaceVertex (MFV): disjoint face+vertex in existing shell.
//!
//! DOMAIN: MFV adds a face seed to an existing shell, without creating
//! a new solid. Tests verify entity counts, self-loop wiring, and lineage.

use crate::euler::make_face_vertex::MakeFaceVertex;
use crate::euler::make_vertex_face::MakeVertexFace;
use crate::operator::apply_op;
use crate::state::TopologyState;

/// MFV creates exactly 1 face, 1 vertex, 1 halfedge, 1 loop, 1 edge
/// without creating any new shell or solid.
#[test]
fn mfv_creates_face_vertex_in_shell() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();

    let v_before = draft.arena().vertex_count();
    let f_before = draft.arena().face_count();
    let he_before = draft.arena().half_edge_count();
    let l_before = draft.arena().loop_count();
    let e_before = draft.arena().edge_count();
    let s_before = draft.arena().shell_count();
    let so_before = draft.arena().body_count();

    let mfv = apply_op(
        &mut draft,
        MakeFaceVertex {
            shell: mvf.shell,
        },
    )
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

/// MFV stamps lineage derived from the OpSignature on all created entities.
#[test]
fn mfv_stamps_lineage() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();

    let mfv = apply_op(
        &mut draft,
        MakeFaceVertex {
            shell: mvf.shell,
        },
    )
    .unwrap()
    .into_value();

    let v_lineage = draft
        .arena()
        .get_vertex(mfv.vertex)
        .unwrap()
        .lineage()
        .expect("vertex must have lineage");
    let f_lineage = draft
        .arena()
        .get_face(mfv.face)
        .unwrap()
        .lineage()
        .expect("face must have lineage");
    let he_lineage = draft
        .arena()
        .get_half_edge(mfv.half_edge)
        .unwrap()
        .lineage()
        .expect("halfedge must have lineage");
    let e_lineage = draft
        .arena()
        .get_edge(mfv.edge)
        .unwrap()
        .lineage()
        .expect("edge must have lineage");

    assert_eq!(v_lineage.get_creation_op().get_name(), "make_face_vertex");
    assert_eq!(f_lineage.get_creation_op().get_name(), "make_face_vertex");
    assert_eq!(he_lineage.get_creation_op().get_name(), "make_face_vertex");
    assert_eq!(e_lineage.get_creation_op().get_name(), "make_face_vertex");
}
