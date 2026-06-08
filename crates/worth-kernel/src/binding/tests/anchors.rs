use worth_geom::facade::{ParameterDomain, ParameterSpacePoint, PolygonalTrimmedParameterRegion};
use worth_spatial::facade::bindings::{
    attach_parameter_space_direction_to_coedge, attach_parameter_space_direction_to_edge,
    attach_parameter_space_point_to_coedge, attach_parameter_space_point_to_edge,
    attach_parameter_space_point_to_face, AnchorCarrierKind, AnchorCarrierOwnership,
    AnchorDirectionRole, CarrierOwnedParameterDirectionAnchorSpec,
    CarrierOwnedParameterPointAnchorSpec, CoedgeBindingSite, CoedgePCurveBindingSpec,
    EdgeBindingSite, EdgeCurveBindingSpec, FaceBindingSite, FaceSurfaceBindingSpec,
    SpatialAnchorAuthorityError, SpatialBindingKind,
};

use crate::facade::authoring::anchoring::{
    author_primitive_anchor_binding_declaration, AuthorPrimitiveAnchorBindingIntent,
    PrimitiveAnchorBindingAuthoringError, PrimitiveAnchorBindingDeclarationEntry,
    PrimitiveAnchorBindingQueryDomain, PrimitiveAnchorBindingQueryWorld,
};

use super::support::{
    admitted_anchor_binding_handle, anchor_declaration_digest_string,
    anchor_inspection_digest_string, anchor_progression_digest_string, canonical_geometry,
    canonical_text_entries_for_anchor_binding, orthotope_contract,
};

#[test]
fn parameter_space_anchor_roundtrip_resolves_on_admitted_carrier() {
    let face_point = face_point_entry("face-cyl", "surface-periodic", [0.25, 3.0]);
    let periodic_face_point = face_point_entry(
        "face-cyl",
        "surface-periodic",
        [std::f64::consts::TAU + 0.25, 3.0],
    );
    let face_direction = author_primitive_anchor_binding_declaration(
        AuthorPrimitiveAnchorBindingIntent::attach_parameter_space_direction_to_face(
            face_binding_spec("face-cyl", "surface-periodic"),
            face_direction_spec("face-cyl", [0.25, 3.0], AnchorDirectionRole::Normal),
        ),
    );
    let edge_point = edge_point_entry(
        "edge-cyl",
        "curve-periodic",
        [std::f64::consts::TAU + 0.5, 2.0],
    );
    let periodic_edge_point = edge_point_entry("edge-cyl", "curve-periodic", [0.5, 2.0]);
    let coedge_point = coedge_point_entry(
        "coedge-cyl",
        "pcurve-periodic",
        [std::f64::consts::TAU + 0.75, 1.5],
    );
    let periodic_coedge_point = coedge_point_entry("coedge-cyl", "pcurve-periodic", [0.75, 1.5]);
    let direct_face = attach_parameter_space_point_to_face(
        face_binding_spec("face-cyl", "surface-periodic"),
        face_point_spec("face-cyl", [0.25, 3.0]),
    )
    .expect("direct face anchor");
    let direct_edge = attach_parameter_space_point_to_edge(
        edge_binding_spec("edge-cyl", "curve-periodic"),
        edge_point_spec("edge-cyl", [0.5, 2.0]),
    )
    .expect("direct edge anchor");
    let direct_coedge = attach_parameter_space_point_to_coedge(
        coedge_binding_spec("coedge-cyl", "pcurve-periodic"),
        coedge_point_spec("coedge-cyl", [0.75, 1.5]),
    )
    .expect("direct coedge anchor");
    let handle = admitted_anchor_binding_handle("anchors");

    let face_admitted = face_point.clone().admit().expect("face admitted");
    let direction_admitted = face_direction.clone().admit().expect("direction admitted");
    let edge_admitted = edge_point.clone().admit().expect("edge admitted");
    let coedge_admitted = coedge_point.clone().admit().expect("coedge admitted");

    assert_eq!(face_admitted.kind(), SpatialBindingKind::FaceSurface);
    assert_eq!(edge_admitted.kind(), SpatialBindingKind::EdgeCurve);
    assert_eq!(coedge_admitted.kind(), SpatialBindingKind::CoedgePCurve);
    assert_eq!(face_admitted.identity(), direct_face.identity().as_str());
    assert_eq!(edge_admitted.identity(), direct_edge.identity().as_str());
    assert_eq!(
        coedge_admitted.identity(),
        direct_coedge.identity().as_str()
    );
    assert_ne!(face_admitted.identity(), direction_admitted.identity());

    let face_entries = canonical_text_entries_for_anchor_binding(&face_point);
    let edge_entries = canonical_text_entries_for_anchor_binding(&edge_point);
    let coedge_entries = canonical_text_entries_for_anchor_binding(&coedge_point);
    let direction_entries = canonical_text_entries_for_anchor_binding(&face_direction);

    assert_eq!(
        face_entries.get("anchor_carrier_kind").map(String::as_str),
        Some("face_surface")
    );
    assert_eq!(
        edge_entries.get("anchor_carrier_kind").map(String::as_str),
        Some("edge_curve")
    );
    assert_eq!(
        coedge_entries
            .get("anchor_carrier_kind")
            .map(String::as_str),
        Some("coedge_pcurve")
    );
    assert_eq!(
        direction_entries
            .get("anchor_direction_role")
            .map(String::as_str),
        Some("normal")
    );

    assert_same_anchor_roundtrip(&handle, &face_point, &periodic_face_point);
    assert_same_anchor_roundtrip(&handle, &edge_point, &periodic_edge_point);
    assert_same_anchor_roundtrip(&handle, &coedge_point, &periodic_coedge_point);

    assert_ne!(
        anchor_declaration_digest_string(&face_point, &handle),
        anchor_declaration_digest_string(&face_direction, &handle)
    );
    assert_ne!(
        anchor_progression_digest_string(&face_point, &handle),
        anchor_progression_digest_string(&face_direction, &handle)
    );
    assert_ne!(
        anchor_inspection_digest_string(&face_point, &handle),
        anchor_inspection_digest_string(&face_direction, &handle)
    );
}

#[test]
fn wrong_carrier_anchor_is_typed_denied_not_silently_coerced() {
    let binding_spec = face_binding_spec("face-1", "surface-alpha");
    let wrong_carrier = CarrierOwnedParameterPointAnchorSpec::new(
        AnchorCarrierOwnership::for_face_surface("face-2", ParameterDomain::plane())
            .expect("wrong carrier ownership"),
        ParameterSpacePoint::try_new([0.25, 0.5]).expect("wrong carrier point"),
    )
    .expect("wrong carrier spec");
    let wrong_carrier_direct =
        attach_parameter_space_point_to_face(binding_spec.clone(), wrong_carrier.clone())
            .expect_err("wrong carrier direct denial");
    let wrong_carrier_kernel = author_primitive_anchor_binding_declaration(
        AuthorPrimitiveAnchorBindingIntent::attach_parameter_space_point_to_face(
            binding_spec,
            wrong_carrier,
        ),
    )
    .admit()
    .expect_err("wrong carrier kernel denial");

    assert!(matches!(
        wrong_carrier_direct,
        SpatialAnchorAuthorityError::CarrierIdentityMismatch { .. }
    ));
    assert!(matches!(
        wrong_carrier_kernel,
        PrimitiveAnchorBindingAuthoringError::Anchor(
            SpatialAnchorAuthorityError::CarrierIdentityMismatch { .. }
        )
    ));
}

#[test]
fn parameter_space_direction_anchor_cannot_collapse_to_generic_vector_truth() {
    let face_direction = author_primitive_anchor_binding_declaration(
        AuthorPrimitiveAnchorBindingIntent::attach_parameter_space_direction_to_face(
            face_binding_spec("face-1", "surface-alpha"),
            face_direction_spec("face-1", [0.25, 0.5], AnchorDirectionRole::Normal),
        ),
    );
    let face_point = face_point_entry("face-1", "surface-alpha", [0.25, 0.5]);
    let edge = attach_parameter_space_direction_to_edge(
        edge_binding_spec("edge-1", "curve-alpha"),
        edge_direction_spec("edge-1", [0.25, 0.0], AnchorDirectionRole::Tangent),
    )
    .expect("edge tangent anchor");
    let coedge = attach_parameter_space_direction_to_coedge(
        coedge_binding_spec("coedge-1", "pcurve-alpha"),
        coedge_direction_spec("coedge-1", [0.25, 0.0], AnchorDirectionRole::Tangent),
    )
    .expect("coedge tangent anchor");
    let unsupported = CarrierOwnedParameterDirectionAnchorSpec::new(
        AnchorCarrierOwnership::for_face_surface("face-1", ParameterDomain::plane())
            .expect("carrier ownership"),
        ParameterSpacePoint::try_new([0.25, 0.75]).expect("parameter point"),
        AnchorDirectionRole::Tangent,
    )
    .expect_err("unsupported direction role");
    let handle = admitted_anchor_binding_handle("anchor-direction-role");
    let direction_entries = canonical_text_entries_for_anchor_binding(&face_direction);

    assert!(matches!(
        unsupported,
        SpatialAnchorAuthorityError::UnsupportedDirectionRole { .. }
    ));
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
    assert_eq!(
        direction_entries
            .get("anchor_direction_role")
            .map(String::as_str),
        Some("normal")
    );
    assert_ne!(
        face_point
            .clone()
            .admit()
            .expect("point admitted")
            .identity(),
        face_direction
            .clone()
            .admit()
            .expect("direction admitted")
            .identity()
    );
    assert_ne!(
        anchor_declaration_digest_string(&face_point, &handle),
        anchor_declaration_digest_string(&face_direction, &handle)
    );
    assert_ne!(
        anchor_progression_digest_string(&face_point, &handle),
        anchor_progression_digest_string(&face_direction, &handle)
    );
    assert_ne!(
        anchor_inspection_digest_string(&face_point, &handle),
        anchor_inspection_digest_string(&face_direction, &handle)
    );
}

#[test]
fn wrong_domain_anchor_is_denied_before_nearest_projection_or_repair() {
    let wrong_domain = CarrierOwnedParameterPointAnchorSpec::new(
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
    .expect_err("point outside trimmed region should fail before authoring");

    assert!(matches!(
        wrong_domain,
        SpatialAnchorAuthorityError::ParameterDomainViolation(_)
    ));
}

fn assert_same_anchor_roundtrip(
    handle: &forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
        PrimitiveAnchorBindingQueryDomain,
        PrimitiveAnchorBindingQueryWorld,
    >,
    authored: &PrimitiveAnchorBindingDeclarationEntry,
    equivalent: &PrimitiveAnchorBindingDeclarationEntry,
) {
    assert_eq!(
        anchor_declaration_digest_string(authored, handle),
        anchor_declaration_digest_string(equivalent, handle)
    );
    assert_eq!(
        anchor_progression_digest_string(authored, handle),
        anchor_progression_digest_string(equivalent, handle)
    );
    assert_eq!(
        anchor_inspection_digest_string(authored, handle),
        anchor_inspection_digest_string(equivalent, handle)
    );
}

fn face_binding_spec(face_identity: &str, persistent_name: &str) -> FaceSurfaceBindingSpec {
    FaceSurfaceBindingSpec::new(
        FaceBindingSite::new(face_identity).with_persistent_name(persistent_name),
        orthotope_contract(),
        canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    )
}

fn edge_binding_spec(edge_identity: &str, persistent_name: &str) -> EdgeCurveBindingSpec {
    EdgeCurveBindingSpec::new(
        EdgeBindingSite::new(edge_identity).with_persistent_name(persistent_name),
        orthotope_contract(),
        canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    )
}

fn coedge_binding_spec(coedge_identity: &str, persistent_name: &str) -> CoedgePCurveBindingSpec {
    CoedgePCurveBindingSpec::new(
        CoedgeBindingSite::new(coedge_identity).with_persistent_name(persistent_name),
        orthotope_contract(),
        canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    )
}

fn face_point_entry(
    face_identity: &str,
    persistent_name: &str,
    parameter: [f64; 2],
) -> PrimitiveAnchorBindingDeclarationEntry {
    author_primitive_anchor_binding_declaration(
        AuthorPrimitiveAnchorBindingIntent::attach_parameter_space_point_to_face(
            face_binding_spec(face_identity, persistent_name),
            face_point_spec(face_identity, parameter),
        ),
    )
}

fn edge_point_entry(
    edge_identity: &str,
    persistent_name: &str,
    parameter: [f64; 2],
) -> PrimitiveAnchorBindingDeclarationEntry {
    author_primitive_anchor_binding_declaration(
        AuthorPrimitiveAnchorBindingIntent::attach_parameter_space_point_to_edge(
            edge_binding_spec(edge_identity, persistent_name),
            edge_point_spec(edge_identity, parameter),
        ),
    )
}

fn coedge_point_entry(
    coedge_identity: &str,
    persistent_name: &str,
    parameter: [f64; 2],
) -> PrimitiveAnchorBindingDeclarationEntry {
    author_primitive_anchor_binding_declaration(
        AuthorPrimitiveAnchorBindingIntent::attach_parameter_space_point_to_coedge(
            coedge_binding_spec(coedge_identity, persistent_name),
            coedge_point_spec(coedge_identity, parameter),
        ),
    )
}

fn face_point_spec(
    face_identity: &str,
    parameter: [f64; 2],
) -> CarrierOwnedParameterPointAnchorSpec {
    CarrierOwnedParameterPointAnchorSpec::new(
        AnchorCarrierOwnership::for_face_surface(face_identity, ParameterDomain::cylinder())
            .expect("face ownership"),
        ParameterSpacePoint::try_new(parameter).expect("face point"),
    )
    .expect("face point anchor spec")
}

fn edge_point_spec(
    edge_identity: &str,
    parameter: [f64; 2],
) -> CarrierOwnedParameterPointAnchorSpec {
    CarrierOwnedParameterPointAnchorSpec::new(
        AnchorCarrierOwnership::for_edge_curve(edge_identity, ParameterDomain::cylinder())
            .expect("edge ownership"),
        ParameterSpacePoint::try_new(parameter).expect("edge point"),
    )
    .expect("edge point anchor spec")
}

fn coedge_point_spec(
    coedge_identity: &str,
    parameter: [f64; 2],
) -> CarrierOwnedParameterPointAnchorSpec {
    CarrierOwnedParameterPointAnchorSpec::new(
        AnchorCarrierOwnership::for_coedge_pcurve(coedge_identity, ParameterDomain::cylinder())
            .expect("coedge ownership"),
        ParameterSpacePoint::try_new(parameter).expect("coedge point"),
    )
    .expect("coedge point anchor spec")
}

fn face_direction_spec(
    face_identity: &str,
    parameter: [f64; 2],
    role: AnchorDirectionRole,
) -> CarrierOwnedParameterDirectionAnchorSpec {
    CarrierOwnedParameterDirectionAnchorSpec::new(
        AnchorCarrierOwnership::for_face_surface(face_identity, ParameterDomain::cylinder())
            .expect("face direction ownership"),
        ParameterSpacePoint::try_new(parameter).expect("face direction point"),
        role,
    )
    .expect("face direction spec")
}

fn edge_direction_spec(
    edge_identity: &str,
    parameter: [f64; 2],
    role: AnchorDirectionRole,
) -> CarrierOwnedParameterDirectionAnchorSpec {
    CarrierOwnedParameterDirectionAnchorSpec::new(
        AnchorCarrierOwnership::for_edge_curve(edge_identity, ParameterDomain::plane())
            .expect("edge direction ownership"),
        ParameterSpacePoint::try_new(parameter).expect("edge direction point"),
        role,
    )
    .expect("edge direction spec")
}

fn coedge_direction_spec(
    coedge_identity: &str,
    parameter: [f64; 2],
    role: AnchorDirectionRole,
) -> CarrierOwnedParameterDirectionAnchorSpec {
    CarrierOwnedParameterDirectionAnchorSpec::new(
        AnchorCarrierOwnership::for_coedge_pcurve(coedge_identity, ParameterDomain::plane())
            .expect("coedge direction ownership"),
        ParameterSpacePoint::try_new(parameter).expect("coedge direction point"),
        role,
    )
    .expect("coedge direction spec")
}
