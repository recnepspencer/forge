use worth_geom::facade::{ParameterDomain, ParameterSpacePoint, PolygonalTrimmedParameterRegion};
use worth_spatial::facade::bindings::{
    attach_parameter_space_point_to_face, AnchorCarrierOwnership, AnchorDirectionRole,
    CarrierOwnedParameterDirectionAnchorSpec, CarrierOwnedParameterPointAnchorSpec,
    FaceBindingSite, FaceSurfaceBindingSpec, SpatialAnchorAuthorityError, SpatialBindingKind,
};

use crate::facade::authoring::binding::{
    author_primitive_binding_declaration, AuthorPrimitiveBindingIntent,
};

use super::support::{
    admitted_binding_handle, canonical_geometry, canonical_text_entries, declaration_digest_string,
    inspect_progressed_binding_entry, orthotope_contract, progress_binding_entry,
};

#[test]
fn parameter_space_anchor_roundtrip_resolves_on_admitted_carrier() {
    let binding_spec = FaceSurfaceBindingSpec::new(
        FaceBindingSite::new("face-cyl").with_persistent_name("surface-periodic"),
        orthotope_contract(),
        canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    );
    let point_ownership =
        AnchorCarrierOwnership::for_face_surface("face-cyl", ParameterDomain::cylinder())
            .expect("point ownership");
    let direction_ownership =
        AnchorCarrierOwnership::for_face_surface("face-cyl", ParameterDomain::cylinder())
            .expect("direction ownership");
    let kernel_point = author_primitive_binding_declaration(
        AuthorPrimitiveBindingIntent::attach_parameter_space_point_to_face(
            binding_spec.clone(),
            CarrierOwnedParameterPointAnchorSpec::new(
                point_ownership,
                ParameterSpacePoint::try_new([std::f64::consts::TAU + 0.25, 3.0])
                    .expect("kernel point"),
            )
            .expect("kernel point anchor spec"),
        ),
    );
    let direct_point = attach_parameter_space_point_to_face(
        binding_spec.clone(),
        CarrierOwnedParameterPointAnchorSpec::new(
            AnchorCarrierOwnership::for_face_surface("face-cyl", ParameterDomain::cylinder())
                .expect("direct ownership"),
            ParameterSpacePoint::try_new([0.25, 3.0]).expect("direct point"),
        )
        .expect("direct point anchor spec"),
    )
    .expect("direct anchored point binding");
    let direction_entry = author_primitive_binding_declaration(
        AuthorPrimitiveBindingIntent::attach_parameter_space_direction_to_face(
            binding_spec,
            CarrierOwnedParameterDirectionAnchorSpec::new(
                direction_ownership,
                ParameterSpacePoint::try_new([0.25, 3.0]).expect("direction point"),
                AnchorDirectionRole::Normal,
            )
            .expect("direction anchor spec"),
        ),
    );
    let handle = admitted_binding_handle("anchors");

    let kernel_point_admitted = kernel_point.clone().admit().expect("kernel point admitted");
    let direction_admitted = direction_entry.clone().admit().expect("direction admitted");
    let kernel_point_progressed = progress_binding_entry(&kernel_point, &handle);
    let direct_point_entry = author_primitive_binding_declaration(
        AuthorPrimitiveBindingIntent::attach_parameter_space_point_to_face(
            FaceSurfaceBindingSpec::new(
                FaceBindingSite::new("face-cyl").with_persistent_name("surface-periodic"),
                orthotope_contract(),
                canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
            ),
            CarrierOwnedParameterPointAnchorSpec::new(
                AnchorCarrierOwnership::for_face_surface("face-cyl", ParameterDomain::cylinder())
                    .expect("equivalent ownership"),
                ParameterSpacePoint::try_new([0.25, 3.0]).expect("equivalent point"),
            )
            .expect("equivalent point anchor spec"),
        ),
    );
    let direct_point_progressed = progress_binding_entry(&direct_point_entry, &handle);
    let direction_progressed = progress_binding_entry(&direction_entry, &handle);
    let kernel_point_inspection =
        inspect_progressed_binding_entry(&handle, kernel_point_progressed.clone());
    let direct_point_inspection =
        inspect_progressed_binding_entry(&handle, direct_point_progressed.clone());
    let direction_inspection =
        inspect_progressed_binding_entry(&handle, direction_progressed.clone());
    let kernel_point_entries = canonical_text_entries(&kernel_point);
    let direction_entries = canonical_text_entries(&direction_entry);

    assert_eq!(
        kernel_point_admitted.kind(),
        SpatialBindingKind::FaceSurface
    );
    assert_eq!(kernel_point_admitted.identity(), direct_point.identity());
    assert_ne!(
        kernel_point_admitted.identity(),
        direction_admitted.identity()
    );
    assert_eq!(
        kernel_point_entries
            .get("anchor_carrier_kind")
            .map(String::as_str),
        Some("face_surface")
    );
    assert_eq!(
        kernel_point_entries
            .get("anchor_carrier_identity")
            .map(String::as_str),
        Some("face-cyl")
    );
    assert_eq!(
        kernel_point_entries.get("anchor_kind").map(String::as_str),
        Some("parameter_space_point")
    );
    assert_eq!(
        direction_entries.get("anchor_kind").map(String::as_str),
        Some("parameter_space_direction")
    );
    assert_eq!(
        direction_entries
            .get("anchor_direction_role")
            .map(String::as_str),
        Some("normal")
    );
    assert_eq!(
        kernel_point_progressed.progression_digest(),
        direct_point_progressed.progression_digest()
    );
    assert_eq!(
        declaration_digest_string(&kernel_point_progressed),
        declaration_digest_string(&direct_point_progressed)
    );
    assert_ne!(
        declaration_digest_string(&kernel_point_progressed),
        declaration_digest_string(&direction_progressed)
    );
    assert_ne!(
        kernel_point_progressed.progression_digest(),
        direction_progressed.progression_digest()
    );
    assert_eq!(
        kernel_point_inspection.inspection_digest(),
        direct_point_inspection.inspection_digest()
    );
    assert_ne!(
        kernel_point_inspection.inspection_digest(),
        direction_inspection.inspection_digest()
    );
}

#[test]
fn wrong_carrier_anchor_is_typed_denied_not_silently_coerced() {
    let binding_spec = FaceSurfaceBindingSpec::new(
        FaceBindingSite::new("face-1").with_persistent_name("surface-alpha"),
        orthotope_contract(),
        canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    );
    let wrong_carrier = CarrierOwnedParameterPointAnchorSpec::new(
        AnchorCarrierOwnership::for_face_surface("face-2", ParameterDomain::plane())
            .expect("wrong carrier ownership"),
        ParameterSpacePoint::try_new([0.25, 0.5]).expect("wrong carrier point"),
    )
    .expect("wrong carrier spec");
    let outside_trimmed_region = CarrierOwnedParameterPointAnchorSpec::new(
        AnchorCarrierOwnership::for_trimmed_face_surface(
            "face-1",
            PolygonalTrimmedParameterRegion::new(
                ParameterDomain::plane(),
                vec![
                    ParameterSpacePoint::try_new([0.0, 0.0]).unwrap(),
                    ParameterSpacePoint::try_new([1.0, 0.0]).unwrap(),
                    ParameterSpacePoint::try_new([1.0, 1.0]).unwrap(),
                    ParameterSpacePoint::try_new([0.0, 1.0]).unwrap(),
                ],
                vec![],
            )
            .expect("trimmed region"),
        )
        .expect("trimmed ownership"),
        ParameterSpacePoint::try_new([1.5, 0.5]).expect("outside point"),
    )
    .expect_err("outside trimmed region should fail spec construction");

    let wrong_carrier_direct =
        attach_parameter_space_point_to_face(binding_spec.clone(), wrong_carrier.clone())
            .expect_err("wrong carrier direct denial");
    let wrong_carrier_kernel = author_primitive_binding_declaration(
        AuthorPrimitiveBindingIntent::attach_parameter_space_point_to_face(
            binding_spec.clone(),
            wrong_carrier,
        ),
    )
    .admit()
    .expect_err("wrong carrier kernel denial");
    let wrong_domain_direct = outside_trimmed_region.clone();
    let wrong_domain_kernel = outside_trimmed_region;

    assert!(matches!(
        wrong_carrier_direct,
        SpatialAnchorAuthorityError::CarrierIdentityMismatch { .. }
    ));
    assert!(matches!(
        wrong_carrier_kernel,
        crate::facade::authoring::binding::PrimitiveBindingAuthoringError::Anchor(
            SpatialAnchorAuthorityError::CarrierIdentityMismatch { .. }
        )
    ));
    assert!(matches!(
        wrong_domain_direct,
        SpatialAnchorAuthorityError::ParameterDomainViolation(_)
    ));
    assert!(matches!(
        wrong_domain_kernel,
        SpatialAnchorAuthorityError::ParameterDomainViolation(_)
    ));
}

#[test]
fn edge_and_coedge_tangent_anchor_roundtrip_preserve_family_specific_truth() {
    use worth_spatial::facade::bindings::{
        attach_parameter_space_direction_to_coedge, attach_parameter_space_direction_to_edge,
        AnchorCarrierKind, CoedgeBindingSite, CoedgePCurveBindingSpec, EdgeBindingSite,
        EdgeCurveBindingSpec,
    };

    let contract = orthotope_contract();
    let geometry = canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
    let edge = attach_parameter_space_direction_to_edge(
        EdgeCurveBindingSpec::new(
            EdgeBindingSite::new("edge-1").with_persistent_name("curve-alpha"),
            contract,
            geometry.clone(),
        ),
        CarrierOwnedParameterDirectionAnchorSpec::new(
            AnchorCarrierOwnership::for_edge_curve("edge-1", ParameterDomain::plane())
                .expect("edge ownership"),
            ParameterSpacePoint::try_new([0.25, 0.0]).expect("edge point"),
            AnchorDirectionRole::Tangent,
        )
        .expect("edge anchor spec"),
    )
    .expect("edge tangent anchor");
    let coedge = attach_parameter_space_direction_to_coedge(
        CoedgePCurveBindingSpec::new(
            CoedgeBindingSite::new("coedge-1").with_persistent_name("pcurve-alpha"),
            contract,
            geometry,
        ),
        CarrierOwnedParameterDirectionAnchorSpec::new(
            AnchorCarrierOwnership::for_coedge_pcurve("coedge-1", ParameterDomain::plane())
                .expect("coedge ownership"),
            ParameterSpacePoint::try_new([0.25, 0.0]).expect("coedge point"),
            AnchorDirectionRole::Tangent,
        )
        .expect("coedge anchor spec"),
    )
    .expect("coedge tangent anchor");

    assert_eq!(edge.kind(), SpatialBindingKind::EdgeCurve);
    assert_eq!(coedge.kind(), SpatialBindingKind::CoedgePCurve);
    assert_eq!(
        edge.anchor().ownership().carrier_kind(),
        AnchorCarrierKind::EdgeCurve
    );
    assert_eq!(
        coedge.anchor().ownership().carrier_kind(),
        AnchorCarrierKind::CoedgePCurve
    );
    assert_ne!(edge.identity(), coedge.identity());
}
