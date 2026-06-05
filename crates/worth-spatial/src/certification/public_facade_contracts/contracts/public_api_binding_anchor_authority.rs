use worth_geom::facade::{ParameterDomain, ParameterSpacePoint};
use worth_primitives::{
    PrimitiveConstructionFamilyContractRegistry, PrimitiveGeometryIdentityBundle,
    PrimitiveSupportPlaneIdentity, PrimitiveVertexIdentity, PrimitiveWitnessDescriptor,
};
use worth_spatial::facade::bindings::{
    attach_parameter_space_direction_to_face, attach_parameter_space_point_to_face,
    AnchorCarrierOwnership, AnchorDirectionRole, CarrierOwnedParameterDirectionAnchorSpec,
    CarrierOwnedParameterPointAnchorSpec, FaceBindingSite, FaceSurfaceBindingSpec,
    SpatialBindingKind,
};

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

#[test]
fn spatial_public_facade_exports_phase_five_anchor_canonical_declaration_projection() {
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
        vec![PrimitiveVertexIdentity::from_position([0.0, 0.0, 0.0])],
    );
    let first = CarrierOwnedParameterPointAnchorSpec::new(
        AnchorCarrierOwnership::for_face_surface("face-1", ParameterDomain::cylinder())
            .expect("first ownership"),
        ParameterSpacePoint::try_new([0.25, 3.0]).expect("first point"),
    )
    .expect("first anchor spec");
    let periodic = CarrierOwnedParameterPointAnchorSpec::new(
        AnchorCarrierOwnership::for_face_surface("face-1", ParameterDomain::cylinder())
            .expect("periodic ownership"),
        ParameterSpacePoint::try_new([std::f64::consts::TAU + 0.25, 3.0]).expect("periodic point"),
    )
    .expect("periodic anchor spec");
    let changed = CarrierOwnedParameterPointAnchorSpec::new(
        AnchorCarrierOwnership::for_face_surface("face-1", ParameterDomain::cylinder())
            .expect("changed ownership"),
        ParameterSpacePoint::try_new([0.5, 3.0]).expect("changed point"),
    )
    .expect("changed anchor spec");
    let binding_spec =
        FaceSurfaceBindingSpec::new(FaceBindingSite::new("face-1"), contract, geometry);

    let first_entries = first.canonical_declaration_fields();
    let periodic_entries = periodic.canonical_declaration_fields();
    let changed_entries = changed.canonical_declaration_fields();
    let anchored = attach_parameter_space_point_to_face(binding_spec, first).expect("anchored");

    assert_eq!(first_entries, periodic_entries);
    assert_ne!(
        field_text(&first_entries, "anchor_parameter_u_bits"),
        field_text(&changed_entries, "anchor_parameter_u_bits")
    );
    assert_eq!(
        anchored.identity().as_str(),
        attach_parameter_space_point_to_face(
            FaceSurfaceBindingSpec::new(
                FaceBindingSite::new("face-1"),
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
            ),
            periodic,
        )
        .expect("periodic anchored")
        .identity()
        .as_str()
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
