use forge_spec::facade::{MakeEdgeFaceMutation, MakeVertexFaceMutation, SpecState, SplitEdgeMutation};

use crate::projection::facade::{ProjectionBuilder, validate_projected_topology_baseline};

#[test]
fn projected_reference_integrity_accepts_reachable_loop_owned_halfedges() {
    let mut draft = SpecState::empty().into_draft();
    let seed = draft.execute(MakeVertexFaceMutation).unwrap();
    draft
        .execute(SplitEdgeMutation {
            half_edge: seed.value.half_edge,
        })
        .unwrap();
    let state = draft.commit().unwrap();
    let projection = ProjectionBuilder::build(&state).unwrap();

    assert!(validate_projected_topology_baseline(&projection).is_ok());
}

#[test]
fn projected_reference_integrity_rejects_orphan_halfedge() {
    let mut draft = SpecState::empty().into_draft();
    let seed = draft.execute(MakeVertexFaceMutation).unwrap();
    draft
        .execute(SplitEdgeMutation {
            half_edge: seed.value.half_edge,
        })
        .unwrap();
    let state = draft.commit().unwrap();
    let mut projection = ProjectionBuilder::build(&state).unwrap();

    projection.loops.clear();

    let error = validate_projected_topology_baseline(&projection).unwrap_err();
    assert!(
        format!("{error}").contains("projected_face_has_at_least_one_loop")
            || format!("{error}").contains("projected_single_owner_per_loop")
            || format!("{error}").contains("projected_no_orphan_half_edges")
    );
}

#[test]
fn projected_reference_integrity_rejects_inner_loop_that_is_also_outer_loop() {
    let mut projection = build_two_face_projection();
    let borrowed_outer = projection.faces[0].outer_loop;
    projection.faces[1].inner_loops.push(borrowed_outer);

    let error = validate_projected_topology_baseline(&projection).unwrap_err();
    assert!(format!("{error}").contains("projected_inner_outer_loop_consistency"));
}

#[test]
fn projected_reference_integrity_rejects_loop_face_mismatch() {
    let mut projection = build_two_face_projection();
    let foreign_face = crate::projection::data::ProjectedFaceId::new(1);
    let outer = projection.faces[0].outer_loop;
    projection.loops[outer.index()].face = foreign_face;

    let error = validate_projected_topology_baseline(&projection).unwrap_err();
    assert!(format!("{error}").contains("projected_inner_outer_loop_consistency"));
}

fn build_two_face_projection() -> crate::projection::data::ProjectedTopology {
    let mut draft = SpecState::empty().into_draft();
    let seed = draft.execute(MakeVertexFaceMutation).unwrap().value;
    let split = draft
        .execute(SplitEdgeMutation {
            half_edge: seed.half_edge,
        })
        .unwrap()
        .value;
    draft
        .execute(MakeEdgeFaceMutation {
            face: seed.face,
            vertex_a: seed.vertex,
            vertex_b: split.new_vertex,
        })
        .unwrap();
    ProjectionBuilder::build(&draft.commit().unwrap()).unwrap()
}
