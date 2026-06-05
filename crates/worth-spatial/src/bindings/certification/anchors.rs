#[cfg(test)]
mod tests {
    use worth_geom::facade::{
        ParameterDomain, ParameterSpacePoint, PolygonalTrimmedParameterRegion,
    };
    use worth_primitives::{
        PrimitiveConstructionFamilyContractRegistry, PrimitiveGeometryIdentityBundle,
        PrimitiveSupportPlaneIdentity, PrimitiveVertexIdentity, PrimitiveWitnessDescriptor,
    };

    use crate::{
        bindings::anchors::{
            attach_parameter_space_point_to_edge, attach_parameter_space_point_to_face,
            AnchorCarrierOwnership, AnchorDirectionRole, CarrierOwnedParameterDirectionAnchorSpec,
            CarrierOwnedParameterPointAnchorSpec, SpatialAnchorAuthorityError,
        },
        bindings::authority::{
            EdgeBindingSite, EdgeCurveBindingSpec, FaceBindingSite, FaceSurfaceBindingSpec,
        },
    };

    #[test]
    fn point_anchor_requires_matching_carrier_identity_before_geometry_folklore() {
        let binding_spec = face_binding_spec("face-1");
        let wrong_carrier =
            AnchorCarrierOwnership::for_face_surface("face-2", ParameterDomain::plane())
                .expect("carrier ownership");
        let anchor_spec = CarrierOwnedParameterPointAnchorSpec::new(
            wrong_carrier,
            ParameterSpacePoint::try_new([0.25, 0.75]).expect("parameter point"),
        )
        .expect("point anchor spec");

        let error = attach_parameter_space_point_to_face(binding_spec, anchor_spec)
            .expect_err("carrier mismatch");

        assert!(matches!(
            error,
            SpatialAnchorAuthorityError::CarrierIdentityMismatch { .. }
        ));
    }

    #[test]
    fn trimmed_face_surface_anchor_rejects_points_outside_the_trimmed_region() {
        let trimmed_region = PolygonalTrimmedParameterRegion::new(
            ParameterDomain::plane(),
            vec![
                ParameterSpacePoint::try_new([0.0, 0.0]).unwrap(),
                ParameterSpacePoint::try_new([1.0, 0.0]).unwrap(),
                ParameterSpacePoint::try_new([1.0, 1.0]).unwrap(),
                ParameterSpacePoint::try_new([0.0, 1.0]).unwrap(),
            ],
            vec![],
        )
        .expect("trimmed region");
        let ownership = AnchorCarrierOwnership::for_trimmed_face_surface("face-1", trimmed_region)
            .expect("trimmed ownership");
        let anchor_spec = CarrierOwnedParameterPointAnchorSpec::new(
            ownership,
            ParameterSpacePoint::try_new([1.5, 0.5]).expect("parameter point"),
        )
        .expect_err("point outside trimmed region should fail spec admission");

        assert!(matches!(
            anchor_spec,
            SpatialAnchorAuthorityError::ParameterDomainViolation(_)
        ));
    }

    #[test]
    fn unsupported_direction_role_is_denied_before_direction_anchor_is_admitted() {
        let ownership =
            AnchorCarrierOwnership::for_face_surface("face-1", ParameterDomain::plane())
                .expect("carrier ownership");
        let anchor_spec = CarrierOwnedParameterDirectionAnchorSpec::new(
            ownership,
            ParameterSpacePoint::try_new([0.25, 0.75]).expect("parameter point"),
            AnchorDirectionRole::Tangent,
        )
        .expect_err("unsupported direction role");

        assert!(matches!(
            anchor_spec,
            SpatialAnchorAuthorityError::UnsupportedDirectionRole { .. }
        ));
    }

    fn face_binding_spec(face_identity: &str) -> FaceSurfaceBindingSpec {
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
        FaceSurfaceBindingSpec::new(FaceBindingSite::new(face_identity), contract, geometry)
    }

    #[test]
    fn edge_anchor_rejects_face_owned_carrier_even_when_parameter_shape_is_valid() {
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
        let binding_spec =
            EdgeCurveBindingSpec::new(EdgeBindingSite::new("edge-1"), contract, geometry);
        let anchor_spec = CarrierOwnedParameterPointAnchorSpec::new(
            AnchorCarrierOwnership::for_face_surface("edge-1", ParameterDomain::plane())
                .expect("face-style ownership"),
            ParameterSpacePoint::try_new([0.25, 0.0]).expect("parameter point"),
        )
        .expect("point anchor spec");

        let error = attach_parameter_space_point_to_edge(binding_spec, anchor_spec)
            .expect_err("carrier family mismatch");

        assert!(matches!(
            error,
            SpatialAnchorAuthorityError::CarrierFamilyMismatch { .. }
        ));
    }
}
