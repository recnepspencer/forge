use worth_primitives::{
    PrimitiveConstructionFamilyContractRegistry, PrimitiveGeometryIdentityBundle,
    PrimitiveSupportPlaneIdentity, PrimitiveVertexIdentity, PrimitiveWitnessDescriptor,
};
use worth_spatial::facade::bindings::{
    attach_curve_to_edge, attach_pcurve_to_coedge, attach_surface_to_face, attach_vertex_geometry,
    AdmittedPartialBindingPosture, CoedgeBindingSite, CoedgePCurveBindingSpec, EdgeBindingSite,
    EdgeCurveBindingSpec, FaceBindingSite, FaceSurfaceBindingSpec, SpatialBindingAuthorityError,
    SpatialBindingCompleteness, SpatialBindingIllegalityReason, SpatialBindingIncompleteness,
    SpatialBindingKind, SpatialBindingUnsupportedReason, VertexBindingSite,
    VertexGeometryBindingSpec, VertexGeometryProvenanceKind, VertexToleranceRegime,
};

#[test]
fn spatial_public_facade_exports_band_one_binding_authority_surface() {
    let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::ShellWithHole {
            outer_loop_edge_count: 4,
            hole_loop_edge_counts: vec![3],
        },
    );
    let geometry = PrimitiveGeometryIdentityBundle::new(
        vec![PrimitiveSupportPlaneIdentity::new(
            "0".to_string(),
            "0".to_string(),
            "1".to_string(),
            "0".to_string(),
        )],
        vec![
            PrimitiveVertexIdentity::from_position([0.0, 0.0, 0.0]),
            PrimitiveVertexIdentity::from_position([1.0, 0.0, 0.0]),
        ],
    );

    let face = attach_surface_to_face(FaceSurfaceBindingSpec::new(
        FaceBindingSite::new("face-1").with_persistent_name("surface-alpha"),
        contract,
        geometry.clone(),
    ))
    .expect("face");
    let edge = attach_curve_to_edge(EdgeCurveBindingSpec::new(
        EdgeBindingSite::new("edge-1").with_persistent_name("curve-alpha"),
        contract,
        geometry.clone(),
    ))
    .expect("edge");
    let coedge = attach_pcurve_to_coedge(CoedgePCurveBindingSpec::new(
        CoedgeBindingSite::new("coedge-1").with_persistent_name("pcurve-alpha"),
        contract,
        geometry.clone(),
    ))
    .expect("coedge");
    let vertex = attach_vertex_geometry(VertexGeometryBindingSpec::new(
        VertexBindingSite::new("vertex-1").with_persistent_name("vertex-alpha"),
        contract,
        geometry,
        VertexGeometryProvenanceKind::CanonicalWitness,
        VertexToleranceRegime::ExactBits,
    ))
    .expect("vertex");

    assert_eq!(face.kind(), SpatialBindingKind::FaceSurface);
    assert_eq!(edge.kind(), SpatialBindingKind::EdgeCurve);
    assert_eq!(coedge.kind(), SpatialBindingKind::CoedgePCurve);
    assert_eq!(vertex.kind(), SpatialBindingKind::VertexGeometry);
    assert!(face.completeness().is_complete());
    assert!(edge.completeness().is_complete());
    assert!(coedge.completeness().is_complete());
    assert!(vertex.completeness().is_complete());
}

#[test]
fn spatial_public_facade_exports_phase_four_completeness_and_denial_taxonomy() {
    let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::ShellWithHole {
            outer_loop_edge_count: 4,
            hole_loop_edge_counts: vec![3],
        },
    );
    let unsupported_contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::WireBody { edge_count: 4 },
    );

    let partial = attach_surface_to_face(FaceSurfaceBindingSpec::new(
        FaceBindingSite::new("face-1"),
        contract,
        PrimitiveGeometryIdentityBundle::new(
            vec![PrimitiveSupportPlaneIdentity::new(
                "0".to_string(),
                "0".to_string(),
                "1".to_string(),
                "0".to_string(),
            )],
            vec![],
        ),
    ))
    .expect("partial");
    let denied = attach_curve_to_edge(EdgeCurveBindingSpec::new(
        EdgeBindingSite::new("edge-1"),
        contract,
        PrimitiveGeometryIdentityBundle::new(vec![], vec![]),
    ))
    .expect("denied");
    let unsupported = attach_surface_to_face(FaceSurfaceBindingSpec::new(
        FaceBindingSite::new("face-unsupported"),
        unsupported_contract,
        PrimitiveGeometryIdentityBundle::new(
            vec![PrimitiveSupportPlaneIdentity::new(
                "0".to_string(),
                "0".to_string(),
                "1".to_string(),
                "0".to_string(),
            )],
            vec![],
        ),
    ));
    let illegal = attach_surface_to_face(FaceSurfaceBindingSpec::new(
        FaceBindingSite::new(""),
        contract,
        PrimitiveGeometryIdentityBundle::new(vec![], vec![]),
    ));

    assert_eq!(
        partial.completeness(),
        &SpatialBindingCompleteness::AdmittedPartial(
            AdmittedPartialBindingPosture::FaceSurfaceMissingVertexGeometry,
        )
    );
    assert_eq!(
        denied.completeness(),
        &SpatialBindingCompleteness::DeniedIncomplete(
            SpatialBindingIncompleteness::EdgeCurveMissingCurveWitnessVertices,
        )
    );
    assert_eq!(
        unsupported,
        Err(SpatialBindingAuthorityError::Unsupported(
            SpatialBindingUnsupportedReason::TopologyBirthClassDoesNotAdmitBindingKind {
                binding_kind: SpatialBindingKind::FaceSurface,
                topology_birth_class: "planar_wire_body",
            },
        ))
    );
    assert_eq!(
        illegal,
        Err(SpatialBindingAuthorityError::Illegal(
            SpatialBindingIllegalityReason::MissingTopologyIdentity(
                SpatialBindingKind::FaceSurface,
            ),
        ))
    );
}

#[test]
fn spatial_public_facade_exports_phase_five_canonical_binding_declaration_projection() {
    let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::Orthotope,
    );
    let first = FaceSurfaceBindingSpec::new(
        FaceBindingSite::new("face-1").with_persistent_name("surface-alpha"),
        contract,
        PrimitiveGeometryIdentityBundle::new(
            vec![PrimitiveSupportPlaneIdentity::new(
                "0".to_string(),
                "0".to_string(),
                "1".to_string(),
                "0".to_string(),
            )],
            vec![PrimitiveVertexIdentity::from_position([0.0, 0.0, 0.0])],
        ),
    );
    let renamed = FaceSurfaceBindingSpec::new(
        FaceBindingSite::new("face-1").with_persistent_name("surface-beta"),
        contract,
        first.geometry_identity().clone(),
    );
    let changed = FaceSurfaceBindingSpec::new(
        FaceBindingSite::new("face-1").with_persistent_name("surface-alpha"),
        contract,
        PrimitiveGeometryIdentityBundle::new(
            vec![PrimitiveSupportPlaneIdentity::new(
                "0".to_string(),
                "0".to_string(),
                "1".to_string(),
                "1".to_string(),
            )],
            vec![PrimitiveVertexIdentity::from_position([2.0, 0.0, 0.0])],
        ),
    );

    let first_entries = first.canonical_declaration_fields();
    let renamed_entries = renamed.canonical_declaration_fields();
    let changed_entries = changed.canonical_declaration_fields();

    assert_eq!(first_entries, renamed_entries);
    assert!(first_entries
        .iter()
        .all(|entry| entry.locus() != "persistent_name"));
    assert_ne!(
        field_text(&first_entries, "geometry_digest"),
        field_text(&changed_entries, "geometry_digest")
    );
}

fn field_text<'a>(
    entries: &'a [worth_spatial::facade::bindings::SpatialCanonicalDeclarationField],
    locus: &str,
) -> &'a str {
    match entries
        .iter()
        .find(|entry| entry.locus() == locus)
        .map(|entry| entry.value())
    {
        Some(value) => value,
        _ => panic!("missing text field: {locus}"),
    }
}
