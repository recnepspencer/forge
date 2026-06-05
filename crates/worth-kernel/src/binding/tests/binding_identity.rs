use worth_spatial::facade::bindings::SpatialAdmittedPrimitiveBinding;

use crate::facade::authoring::binding::{
    author_primitive_binding_declaration, AuthorPrimitiveBindingIntent,
};

use super::support::{
    admitted_binding_handle, canonical_geometry, declaration_digest_string,
    inspect_progressed_binding_entry, orthotope_contract, progress_binding_entry,
};
use worth_spatial::facade::bindings::{
    FaceBindingSite, FaceSurfaceBindingSpec, VertexBindingSite, VertexGeometryBindingSpec,
    VertexGeometryProvenanceKind, VertexToleranceRegime,
};

#[test]
fn binding_identity_diverges_from_topology_and_naming_when_geometry_changes() {
    let contract = orthotope_contract();
    let same_site = FaceBindingSite::new("face-1").with_persistent_name("surface-alpha");
    let first_geometry = canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
    let second_geometry = canonical_geometry([[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]]);

    let equivalent = author_primitive_binding_declaration(
        AuthorPrimitiveBindingIntent::attach_surface_to_face(FaceSurfaceBindingSpec::new(
            FaceBindingSite::new("face-1").with_persistent_name("surface-beta"),
            contract,
            canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
        )),
    );
    let first =
        author_primitive_binding_declaration(AuthorPrimitiveBindingIntent::attach_surface_to_face(
            FaceSurfaceBindingSpec::new(same_site.clone(), contract, first_geometry),
        ));
    let second =
        author_primitive_binding_declaration(AuthorPrimitiveBindingIntent::attach_surface_to_face(
            FaceSurfaceBindingSpec::new(same_site, contract, second_geometry),
        ));
    let handle = admitted_binding_handle("identity");

    let first_admitted = first.clone().admit().expect("first binding");
    let second_admitted = second.clone().admit().expect("second binding");
    let equivalent_admitted = equivalent.clone().admit().expect("equivalent binding");
    let first_progressed = progress_binding_entry(&first, &handle);
    let second_progressed = progress_binding_entry(&second, &handle);
    let equivalent_progressed = progress_binding_entry(&equivalent, &handle);
    let first_inspection = inspect_progressed_binding_entry(&handle, first_progressed.clone());
    let second_inspection = inspect_progressed_binding_entry(&handle, second_progressed.clone());
    let equivalent_inspection =
        inspect_progressed_binding_entry(&handle, equivalent_progressed.clone());

    assert_ne!(first_admitted.identity(), second_admitted.identity());
    assert_eq!(first_admitted.identity(), equivalent_admitted.identity());
    assert_ne!(
        declaration_digest_string(&first_progressed),
        declaration_digest_string(&second_progressed)
    );
    assert_eq!(
        declaration_digest_string(&first_progressed),
        declaration_digest_string(&equivalent_progressed)
    );
    assert_ne!(
        first_progressed.progression_digest(),
        second_progressed.progression_digest()
    );
    assert_eq!(
        first_progressed.progression_digest(),
        equivalent_progressed.progression_digest()
    );
    assert_ne!(
        first_inspection.inspection_digest(),
        second_inspection.inspection_digest()
    );
    assert_eq!(
        first_inspection.inspection_digest(),
        equivalent_inspection.inspection_digest()
    );

    let SpatialAdmittedPrimitiveBinding::FaceSurface(first_face) = first_admitted else {
        panic!("expected face binding");
    };
    let SpatialAdmittedPrimitiveBinding::FaceSurface(second_face) = second_admitted else {
        panic!("expected face binding");
    };
    assert_eq!(
        first_face.site().topology_face_identity(),
        second_face.site().topology_face_identity()
    );
    assert_eq!(
        first_face.site().persistent_name(),
        second_face.site().persistent_name()
    );
    assert_ne!(
        first_face.geometry_identity().scaffold_geometry_digest(),
        second_face.geometry_identity().scaffold_geometry_digest()
    );
}

#[test]
fn binding_identity_is_stable_under_equivalent_authoring_order_variation() {
    let contract = orthotope_contract();
    let geometry = canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
    let canonical = author_primitive_binding_declaration(
        AuthorPrimitiveBindingIntent::attach_vertex_geometry(VertexGeometryBindingSpec::new(
            VertexBindingSite::new("vertex-1").with_persistent_name("vertex-alpha"),
            contract,
            geometry.clone(),
            VertexGeometryProvenanceKind::CanonicalWitness,
            VertexToleranceRegime::ExactBits,
        )),
    );
    let equivalent = author_primitive_binding_declaration(
        AuthorPrimitiveBindingIntent::attach_vertex_geometry(VertexGeometryBindingSpec::new(
            VertexBindingSite::new("vertex-1").with_persistent_name("vertex-beta"),
            contract,
            geometry,
            VertexGeometryProvenanceKind::CanonicalWitness,
            VertexToleranceRegime::ExactBits,
        )),
    );
    let changed = author_primitive_binding_declaration(
        AuthorPrimitiveBindingIntent::attach_vertex_geometry(VertexGeometryBindingSpec::new(
            VertexBindingSite::new("vertex-1").with_persistent_name("vertex-alpha"),
            contract,
            canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
            VertexGeometryProvenanceKind::RealizedVertex,
            VertexToleranceRegime::AdmittedTolerance,
        )),
    );
    let left_handle = admitted_binding_handle("vertex-identity-order");
    let right_handle = admitted_binding_handle("vertex-identity-order");

    let canonical_admitted = canonical.clone().admit().expect("canonical binding");
    let equivalent_admitted = equivalent.clone().admit().expect("equivalent binding");
    let changed_admitted = changed.clone().admit().expect("changed binding");
    let left_canonical_progressed = progress_binding_entry(&canonical, &left_handle);
    let left_equivalent_progressed = progress_binding_entry(&equivalent, &left_handle);
    let right_equivalent_progressed = progress_binding_entry(&equivalent, &right_handle);
    let right_canonical_progressed = progress_binding_entry(&canonical, &right_handle);
    let changed_progressed = progress_binding_entry(&changed, &left_handle);
    let left_canonical_inspection =
        inspect_progressed_binding_entry(&left_handle, left_canonical_progressed.clone());
    let left_equivalent_inspection =
        inspect_progressed_binding_entry(&left_handle, left_equivalent_progressed.clone());
    let right_equivalent_inspection =
        inspect_progressed_binding_entry(&right_handle, right_equivalent_progressed.clone());
    let right_canonical_inspection =
        inspect_progressed_binding_entry(&right_handle, right_canonical_progressed.clone());
    let changed_inspection =
        inspect_progressed_binding_entry(&left_handle, changed_progressed.clone());

    assert_eq!(
        canonical_admitted.identity(),
        equivalent_admitted.identity()
    );
    assert_ne!(canonical_admitted.identity(), changed_admitted.identity());
    assert_eq!(
        declaration_digest_string(&left_canonical_progressed),
        declaration_digest_string(&left_equivalent_progressed)
    );
    assert_ne!(
        declaration_digest_string(&left_canonical_progressed),
        declaration_digest_string(&changed_progressed)
    );
    assert_eq!(
        declaration_digest_string(&left_canonical_progressed),
        declaration_digest_string(&right_canonical_progressed)
    );
    assert_eq!(
        declaration_digest_string(&left_equivalent_progressed),
        declaration_digest_string(&right_equivalent_progressed)
    );
    assert_eq!(
        left_canonical_progressed.progression_digest(),
        left_equivalent_progressed.progression_digest()
    );
    assert_ne!(
        left_canonical_progressed.progression_digest(),
        changed_progressed.progression_digest()
    );
    assert_eq!(
        left_canonical_progressed.progression_digest(),
        right_canonical_progressed.progression_digest()
    );
    assert_eq!(
        left_equivalent_progressed.progression_digest(),
        right_equivalent_progressed.progression_digest()
    );
    assert_eq!(
        left_canonical_inspection.inspection_digest(),
        left_equivalent_inspection.inspection_digest()
    );
    assert_ne!(
        left_canonical_inspection.inspection_digest(),
        changed_inspection.inspection_digest()
    );
    assert_eq!(
        left_canonical_inspection.inspection_digest(),
        right_canonical_inspection.inspection_digest()
    );
    assert_eq!(
        left_equivalent_inspection.inspection_digest(),
        right_equivalent_inspection.inspection_digest()
    );
}
