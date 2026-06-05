use worth_spatial::facade::bindings::{
    attach_curve_to_edge, attach_pcurve_to_coedge, attach_surface_to_face, attach_vertex_geometry,
    CoedgeBindingSite, CoedgePCurveBindingSpec, EdgeBindingSite, EdgeCurveBindingSpec,
    FaceBindingSite, FaceSurfaceBindingSpec, SpatialBindingKind, VertexBindingSite,
    VertexGeometryBindingSpec, VertexGeometryProvenanceKind, VertexToleranceRegime,
};

use crate::facade::authoring::binding::{
    author_primitive_binding_declaration, AuthorPrimitiveBindingIntent,
};

use super::support::{
    admitted_binding_handle, canonical_geometry, declaration_digest_string,
    inspect_progressed_binding_entry, progress_binding_entry, shell_with_hole_contract,
};

#[test]
fn planar_binding_authority_roundtrip_preserves_binding_truth() {
    let contract = shell_with_hole_contract();
    let geometry = canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);

    let face_spec = FaceSurfaceBindingSpec::new(
        FaceBindingSite::new("face-1").with_persistent_name("surface-alpha"),
        contract,
        geometry.clone(),
    );
    let edge_spec = EdgeCurveBindingSpec::new(
        EdgeBindingSite::new("edge-1").with_persistent_name("curve-alpha"),
        contract,
        geometry.clone(),
    );
    let coedge_spec = CoedgePCurveBindingSpec::new(
        CoedgeBindingSite::new("coedge-1").with_persistent_name("pcurve-alpha"),
        contract,
        geometry.clone(),
    );
    let vertex_spec = VertexGeometryBindingSpec::new(
        VertexBindingSite::new("vertex-1").with_persistent_name("vertex-alpha"),
        contract,
        geometry.clone(),
        VertexGeometryProvenanceKind::CanonicalWitness,
        VertexToleranceRegime::ExactBits,
    );

    let face_kernel = author_primitive_binding_declaration(
        AuthorPrimitiveBindingIntent::attach_surface_to_face(face_spec.clone()),
    );
    let edge_kernel = author_primitive_binding_declaration(
        AuthorPrimitiveBindingIntent::attach_curve_to_edge(edge_spec.clone()),
    );
    let coedge_kernel = author_primitive_binding_declaration(
        AuthorPrimitiveBindingIntent::attach_pcurve_to_coedge(coedge_spec.clone()),
    );
    let vertex_kernel = author_primitive_binding_declaration(
        AuthorPrimitiveBindingIntent::attach_vertex_geometry(vertex_spec.clone()),
    );
    let handle = admitted_binding_handle("main");

    let face_progressed = progress_binding_entry(&face_kernel, &handle);
    let edge_progressed = progress_binding_entry(&edge_kernel, &handle);
    let coedge_progressed = progress_binding_entry(&coedge_kernel, &handle);
    let vertex_progressed = progress_binding_entry(&vertex_kernel, &handle);
    let face_inspection = inspect_progressed_binding_entry(&handle, face_progressed.clone());
    let edge_inspection = inspect_progressed_binding_entry(&handle, edge_progressed.clone());
    let coedge_inspection = inspect_progressed_binding_entry(&handle, coedge_progressed.clone());
    let vertex_inspection = inspect_progressed_binding_entry(&handle, vertex_progressed.clone());

    let face_direct = attach_surface_to_face(face_spec).expect("direct face binding");
    let edge_direct = attach_curve_to_edge(edge_spec).expect("direct edge binding");
    let coedge_direct = attach_pcurve_to_coedge(coedge_spec).expect("direct coedge binding");
    let vertex_direct = attach_vertex_geometry(vertex_spec).expect("direct vertex binding");

    let face_admitted = face_kernel.clone().admit().expect("kernel face binding");
    let edge_admitted = edge_kernel.clone().admit().expect("kernel edge binding");
    let coedge_admitted = coedge_kernel
        .clone()
        .admit()
        .expect("kernel coedge binding");
    let vertex_admitted = vertex_kernel
        .clone()
        .admit()
        .expect("kernel vertex binding");

    assert_eq!(face_kernel.binding_kind(), SpatialBindingKind::FaceSurface);
    assert_eq!(edge_kernel.binding_kind(), SpatialBindingKind::EdgeCurve);
    assert_eq!(
        coedge_kernel.binding_kind(),
        SpatialBindingKind::CoedgePCurve
    );
    assert_eq!(
        vertex_kernel.binding_kind(),
        SpatialBindingKind::VertexGeometry
    );
    assert_eq!(face_admitted.identity(), face_direct.identity().as_str());
    assert_eq!(edge_admitted.identity(), edge_direct.identity().as_str());
    assert_eq!(
        coedge_admitted.identity(),
        coedge_direct.identity().as_str()
    );
    assert_eq!(
        vertex_admitted.identity(),
        vertex_direct.identity().as_str()
    );
    assert!(face_admitted.completeness().is_complete());
    assert!(edge_admitted.completeness().is_complete());
    assert!(coedge_admitted.completeness().is_complete());
    assert!(vertex_admitted.completeness().is_complete());
    assert_eq!(
        declaration_digest_string(&face_progressed),
        face_inspection.declaration_digest()
    );
    assert_eq!(
        declaration_digest_string(&edge_progressed),
        edge_inspection.declaration_digest()
    );
    assert_eq!(
        declaration_digest_string(&coedge_progressed),
        coedge_inspection.declaration_digest()
    );
    assert_eq!(
        declaration_digest_string(&vertex_progressed),
        vertex_inspection.declaration_digest()
    );
    assert_eq!(
        Some(face_progressed.progression_digest()),
        face_inspection.progression_digest()
    );
    assert_eq!(
        Some(edge_progressed.progression_digest()),
        edge_inspection.progression_digest()
    );
    assert_eq!(
        Some(coedge_progressed.progression_digest()),
        coedge_inspection.progression_digest()
    );
    assert_eq!(
        Some(vertex_progressed.progression_digest()),
        vertex_inspection.progression_digest()
    );
    assert_eq!(
        face_progressed.declaration_family_key(),
        "PrimitiveBindingDeclarationFamily"
    );
    assert_eq!(
        edge_progressed.declaration_family_key(),
        "PrimitiveBindingDeclarationFamily"
    );
    assert_eq!(
        coedge_progressed.declaration_family_key(),
        "PrimitiveBindingDeclarationFamily"
    );
    assert_eq!(
        vertex_progressed.declaration_family_key(),
        "PrimitiveBindingDeclarationFamily"
    );
}
