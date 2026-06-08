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
            attach_parameter_space_direction_to_face, attach_parameter_space_point_to_edge,
            attach_parameter_space_point_to_face, AnchorCarrierOwnership, AnchorDirectionRole,
            CarrierOwnedParameterDirectionAnchorSpec, CarrierOwnedParameterPointAnchorSpec,
            SpatialAnchorAuthorityError,
        },
        bindings::authority::{
            EdgeBindingSite, EdgeCurveBindingSpec, FaceBindingSite, FaceSurfaceBindingSpec,
        },
    };

    #[test]
    fn wrong_carrier_anchor_is_typed_denied_not_silently_coerced() {
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
    fn wrong_domain_anchor_is_denied_before_nearest_projection_or_repair() {
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
    fn parameter_space_direction_anchor_cannot_collapse_to_generic_vector_truth() {
        let ownership =
            AnchorCarrierOwnership::for_face_surface("face-1", ParameterDomain::plane())
                .expect("carrier ownership");
        let unsupported = CarrierOwnedParameterDirectionAnchorSpec::new(
            ownership.clone(),
            ParameterSpacePoint::try_new([0.25, 0.75]).expect("parameter point"),
            AnchorDirectionRole::Tangent,
        )
        .expect_err("unsupported direction role");
        let point = attach_parameter_space_point_to_face(
            face_binding_spec("face-1"),
            CarrierOwnedParameterPointAnchorSpec::new(
                ownership.clone(),
                ParameterSpacePoint::try_new([0.25, 0.75]).expect("point parameter"),
            )
            .expect("point spec"),
        )
        .expect("point anchor");
        let direction = attach_parameter_space_direction_to_face(
            face_binding_spec("face-1"),
            CarrierOwnedParameterDirectionAnchorSpec::new(
                ownership,
                ParameterSpacePoint::try_new([0.25, 0.75]).expect("direction parameter"),
                AnchorDirectionRole::Normal,
            )
            .expect("direction spec"),
        )
        .expect("direction anchor");

        assert!(matches!(
            unsupported,
            SpatialAnchorAuthorityError::UnsupportedDirectionRole { .. }
        ));
        assert_ne!(point.identity(), direction.identity());
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
    fn wrong_carrier_family_anchor_is_typed_denied_before_attachment() {
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
