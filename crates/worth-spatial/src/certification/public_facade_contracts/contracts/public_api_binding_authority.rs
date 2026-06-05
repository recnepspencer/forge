use worth_geom::facade::{ParameterDomain, ParameterSpacePoint};
use worth_primitives::{
    PrimitiveConstructionFamilyContractRegistry, PrimitiveGeometryIdentityBundle,
    PrimitiveSupportPlaneIdentity, PrimitiveVertexIdentity, PrimitiveWitnessDescriptor,
};
use worth_spatial::facade::bindings::{
    attach_curve_to_edge, attach_parameter_space_direction_to_face,
    attach_parameter_space_point_to_face, attach_pcurve_to_coedge, attach_surface_to_face,
    attach_vertex_geometry, AnchorCarrierOwnership, AnchorDirectionRole,
    CarrierOwnedParameterDirectionAnchorSpec, CarrierOwnedParameterPointAnchorSpec,
    CoedgeBindingSite, CoedgePCurveBindingSpec, EdgeBindingSite, EdgeCurveBindingSpec,
    FaceBindingSite, FaceSurfaceBindingSpec, SpatialBindingKind, VertexBindingSite,
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
fn spatial_public_facade_exports_parameter_space_anchor_authority_surface() {
    let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::Orthotope,
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
    let binding_spec = FaceSurfaceBindingSpec::new(
        FaceBindingSite::new("face-1").with_persistent_name("surface-alpha"),
        contract,
        geometry,
    );
    let point = attach_parameter_space_point_to_face(
        binding_spec.clone(),
        CarrierOwnedParameterPointAnchorSpec::new(
            AnchorCarrierOwnership::for_face_surface("face-1", ParameterDomain::plane())
                .expect("point ownership"),
            ParameterSpacePoint::try_new([0.25, 0.5]).expect("point anchor"),
        )
        .expect("point anchor spec"),
    )
    .expect("point anchor binding");
    let direction = attach_parameter_space_direction_to_face(
        binding_spec,
        CarrierOwnedParameterDirectionAnchorSpec::new(
            AnchorCarrierOwnership::for_face_surface("face-1", ParameterDomain::plane())
                .expect("direction ownership"),
            ParameterSpacePoint::try_new([0.25, 0.5]).expect("direction point"),
            AnchorDirectionRole::Normal,
        )
        .expect("direction anchor spec"),
    )
    .expect("direction anchor binding");

    assert_eq!(point.kind(), SpatialBindingKind::FaceSurface);
    assert_eq!(direction.kind(), SpatialBindingKind::FaceSurface);
    assert!(point.completeness().is_complete());
    assert!(direction.completeness().is_complete());
    assert_ne!(point.identity(), direction.identity());
}
