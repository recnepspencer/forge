use forge_spec::facade::{
    MakeEdgeFaceMutation, MakeVertexFaceMutation, SpecState, SplitEdgeMutation,
};

use crate::projection::facade::{
    validate_projected_acyclic_containment, validate_projected_bidirectional_links,
    validate_projected_hierarchy, validate_projected_inner_outer_loop_consistency,
    validate_projected_no_dangling_refs, validate_projected_no_orphan_half_edges,
    validate_projected_topology_baseline, ProjectedEdgeData, ProjectedEdgeId, ProjectedFaceId,
    ProjectedHalfEdgeData, ProjectedHalfEdgeId, ProjectedLumpId, ProjectedRegionData,
    ProjectedRegionId, ProjectedTopology, ProjectedVertexData, ProjectedVertexId,
    ProjectionBuilder,
};

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
    append_orphan_half_edge(&mut projection);

    let error = validate_projected_no_orphan_half_edges(&projection).unwrap_err();
    assert!(format!("{error}").contains("projected_no_orphan_half_edges"));
}

#[test]
fn projected_reference_integrity_rejects_inner_loop_that_is_also_outer_loop() {
    let mut projection = build_two_face_projection();
    let borrowed_outer = projection.faces[0].outer_loop;
    projection.faces[1].inner_loops.push(borrowed_outer);

    let error = validate_projected_inner_outer_loop_consistency(&projection).unwrap_err();
    assert!(format!("{error}").contains("projected_inner_outer_loop_consistency"));
}

#[test]
fn projected_reference_integrity_rejects_loop_face_mismatch() {
    let mut projection = build_two_face_projection();
    let foreign_face = crate::projection::data::ProjectedFaceId::new(1);
    let outer = projection.faces[0].outer_loop;
    projection.loops[outer.index()].face = foreign_face;

    let error = validate_projected_topology_baseline(&projection).unwrap_err();
    assert!(format!("{error}").contains("projected_hierarchy"));
}

#[test]
fn projected_reference_integrity_rejects_duplicate_shell_claim() {
    let mut projection = build_two_face_projection();
    let duplicate_region = ProjectedRegionId::new(projection.region_count() as u32);
    projection.regions.push(ProjectedRegionData {
        spec_id: forge_spec::facade::SpecNodeId::new(9_996),
        lump: ProjectedLumpId::new(0),
        shells: vec![projection.faces[0].shell],
    });
    projection.lumps[0].regions.push(duplicate_region);

    let error = validate_projected_acyclic_containment(&projection).unwrap_err();
    assert!(format!("{error}").contains("projected_acyclic_containment"));
}

#[test]
fn projected_reference_integrity_rejects_missing_parent_child_membership() {
    let mut projection = build_two_face_projection();
    let shell = projection.faces[0].shell;
    projection.regions[projection.shells[shell.index()].region.index()]
        .shells
        .clear();

    let error = validate_projected_hierarchy(&projection).unwrap_err();
    assert!(format!("{error}").contains("projected_hierarchy"));
}

#[test]
fn projected_reference_integrity_rejects_nonreciprocal_edge_representative() {
    let mut projection = build_two_face_projection();
    let wrong_half_edge = projection
        .half_edges
        .iter()
        .position(|half_edge| half_edge.edge != ProjectedEdgeId::new(0))
        .expect("fixture should contain a second edge");
    projection.edges[0].half_edge = ProjectedHalfEdgeId::new(wrong_half_edge as u32);

    let error = validate_projected_bidirectional_links(&projection).unwrap_err();
    assert!(format!("{error}").contains("projected_bidirectional_links"));
}

#[test]
fn projected_reference_integrity_rejects_missing_halfedge_vertex_reference() {
    let mut projection = build_two_face_projection();
    projection.half_edges[0].origin = ProjectedVertexId::new(projection.vertex_count() as u32);

    let error = validate_projected_no_dangling_refs(&projection).unwrap_err();
    assert!(format!("{error}").contains("projected_no_dangling_refs"));
}

#[test]
fn projected_reference_integrity_rejects_missing_halfedge_next_reference() {
    let mut projection = build_two_face_projection();
    projection.half_edges[0].next = ProjectedHalfEdgeId::new(projection.half_edge_count() as u32);

    let error = validate_projected_no_dangling_refs(&projection).unwrap_err();
    assert!(format!("{error}").contains("projected_no_dangling_refs"));
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

fn append_orphan_half_edge(projection: &mut ProjectedTopology) {
    let orphan_half_edge = ProjectedHalfEdgeId::new(projection.half_edge_count() as u32);
    let orphan_edge = ProjectedEdgeId::new(projection.edge_count() as u32);
    let orphan_vertex = ProjectedVertexId::new(projection.vertex_count() as u32);

    projection.vertices.push(ProjectedVertexData {
        spec_id: forge_spec::facade::SpecNodeId::new(9_999),
        primary_half_edge: Some(orphan_half_edge),
        geometry_binding: None,
    });
    projection.edges.push(ProjectedEdgeData {
        spec_id: forge_spec::facade::SpecNodeId::new(9_998),
        half_edge: orphan_half_edge,
        curve_binding: None,
    });
    projection.half_edges.push(ProjectedHalfEdgeData {
        spec_id: forge_spec::facade::SpecNodeId::new(9_997),
        radial_next: orphan_half_edge,
        next: orphan_half_edge,
        prev: orphan_half_edge,
        face: ProjectedFaceId::new(0),
        origin: orphan_vertex,
        edge: orphan_edge,
        coedge_binding: None,
    });
}
