use worth_geom::facade::{ParameterDomain, ParameterSpacePoint};
use worth_primitives::{
    PrimitiveConstructionFamilyContractRegistry, PrimitiveGeometryIdentityBundle,
    PrimitiveVertexIdentity, PrimitiveWitnessDescriptor,
};
use worth_spatial::facade::bindings::{
    attach_curve_to_edge, attach_parameter_space_point_to_face, attach_surface_to_face,
    evaluate_continuity, AnchorCarrierOwnership, BindingContinuityClass,
    CarrierOwnedParameterPointAnchorSpec, EdgeBindingSite, EdgeCurveBindingSpec, FaceBindingSite,
    FaceSurfaceBindingSpec, LocalTopologyReplacementNeighborhood, NeighborhoodBindingFamily,
    ReplacementCandidate, ReplacementCandidateSet, SpatialAdmittedPrimitiveBinding,
};

use crate::facade::authoring::binding::{
    author_primitive_rebinding_declaration, AuthorPrimitiveRebindingIntent,
};

use super::super::support::{canonical_geometry, orthotope_contract, shell_with_hole_contract};

fn face_surface_binding(
    face_id: &str,
    persistent_name: &str,
    vertices: [[f64; 3]; 2],
) -> SpatialAdmittedPrimitiveBinding {
    SpatialAdmittedPrimitiveBinding::FaceSurface(
        attach_surface_to_face(FaceSurfaceBindingSpec::new(
            FaceBindingSite::new(face_id).with_persistent_name(persistent_name),
            orthotope_contract(),
            canonical_geometry(vertices),
        ))
        .expect("face surface binding"),
    )
}

fn face_point_binding(
    face_id: &str,
    persistent_name: &str,
    vertices: [[f64; 3]; 2],
    point: [f64; 2],
) -> SpatialAdmittedPrimitiveBinding {
    SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(
        attach_parameter_space_point_to_face(
            FaceSurfaceBindingSpec::new(
                FaceBindingSite::new(face_id).with_persistent_name(persistent_name),
                orthotope_contract(),
                canonical_geometry(vertices),
            ),
            CarrierOwnedParameterPointAnchorSpec::new(
                AnchorCarrierOwnership::for_face_surface(face_id, ParameterDomain::plane())
                    .expect("ownership"),
                ParameterSpacePoint::try_new(point).expect("parameter"),
            )
            .expect("anchor spec"),
        )
        .expect("point anchor binding"),
    )
}

fn edge_curve_binding(edge_id: &str, vertices: [[f64; 3]; 2]) -> SpatialAdmittedPrimitiveBinding {
    SpatialAdmittedPrimitiveBinding::EdgeCurve(
        attach_curve_to_edge(EdgeCurveBindingSpec::new(
            EdgeBindingSite::new(edge_id),
            shell_with_hole_contract(),
            canonical_geometry(vertices),
        ))
        .expect("edge curve binding"),
    )
}

#[test]
fn continuity_classification_distinguishes_authoritative_successor_correspondence_and_insufficient_evidence(
) {
    let prior_surface = face_surface_binding(
        "face-old",
        "surface-alpha",
        [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
    );
    let successor_surface = face_surface_binding(
        "face-successor",
        "surface-beta",
        [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]],
    );
    let successor_neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::FaceSurface,
        "face-old",
        ReplacementCandidateSet::new(vec![ReplacementCandidate::new(
            "successor",
            successor_surface.clone(),
        )
        .expect("candidate")])
        .expect("candidate set"),
    )
    .expect("successor neighborhood");
    let successor_continuity =
        evaluate_continuity(&prior_surface, &successor_neighborhood).expect("successor continuity");
    let kernel_successor = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            prior_surface.clone(),
            successor_neighborhood,
        ),
    )
    .admit()
    .expect("kernel successor");

    let prior_anchor = face_point_binding(
        "face-anchor-old",
        "surface-anchor-alpha",
        [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
        [0.25, 0.5],
    );
    let correspondence_anchor = face_point_binding(
        "face-anchor-new",
        "surface-anchor-beta",
        [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]],
        [0.5, 0.5],
    );
    let correspondence_neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::FaceSurfacePointAnchor,
        "face-anchor-old",
        ReplacementCandidateSet::new(vec![ReplacementCandidate::new(
            "correspondence",
            correspondence_anchor,
        )
        .expect("candidate")])
        .expect("candidate set"),
    )
    .expect("correspondence neighborhood");
    let correspondence_continuity =
        evaluate_continuity(&prior_anchor, &correspondence_neighborhood)
            .expect("correspondence continuity");
    let kernel_correspondence = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            prior_anchor,
            correspondence_neighborhood,
        ),
    )
    .admit()
    .expect("kernel correspondence");

    let prior_edge = edge_curve_binding("edge-old", [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
    let partial_edge = SpatialAdmittedPrimitiveBinding::EdgeCurve(
        attach_curve_to_edge(EdgeCurveBindingSpec::new(
            EdgeBindingSite::new("edge-partial"),
            PrimitiveConstructionFamilyContractRegistry::contract_for(
                &PrimitiveWitnessDescriptor::Orthotope,
            ),
            PrimitiveGeometryIdentityBundle::new(
                vec![],
                vec![PrimitiveVertexIdentity::from_position([0.0, 0.0, 0.0])],
            ),
        ))
        .expect("partial edge"),
    );
    let admitted_partial_neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::EdgeCurve,
        "edge-old",
        ReplacementCandidateSet::new(vec![
            ReplacementCandidate::new("partial", partial_edge).expect("candidate")
        ])
        .expect("candidate set"),
    )
    .expect("partial neighborhood");
    let admitted_partial_continuity =
        evaluate_continuity(&prior_edge, &admitted_partial_neighborhood)
            .expect("admitted partial continuity");

    let denied_edge = SpatialAdmittedPrimitiveBinding::EdgeCurve(
        attach_curve_to_edge(EdgeCurveBindingSpec::new(
            EdgeBindingSite::new("edge-denied"),
            PrimitiveConstructionFamilyContractRegistry::contract_for(
                &PrimitiveWitnessDescriptor::Orthotope,
            ),
            PrimitiveGeometryIdentityBundle::new(vec![], vec![]),
        ))
        .expect("denied edge"),
    );
    let denied_incomplete_neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::EdgeCurve,
        "edge-old",
        ReplacementCandidateSet::new(vec![
            ReplacementCandidate::new("denied", denied_edge).expect("candidate")
        ])
        .expect("candidate set"),
    )
    .expect("denied neighborhood");
    let denied_incomplete_continuity =
        evaluate_continuity(&prior_edge, &denied_incomplete_neighborhood)
            .expect("denied incomplete continuity");

    let ambiguous_neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::EdgeCurve,
        "edge-old",
        ReplacementCandidateSet::new(vec![
            ReplacementCandidate::new(
                "a",
                edge_curve_binding("edge-a", [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]]),
            )
            .expect("candidate a"),
            ReplacementCandidate::new(
                "b",
                edge_curve_binding("edge-b", [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]]),
            )
            .expect("candidate b"),
        ])
        .expect("candidate set"),
    )
    .expect("ambiguous neighborhood");
    let ambiguous_continuity =
        evaluate_continuity(&prior_edge, &ambiguous_neighborhood).expect("ambiguous continuity");

    assert_eq!(
        successor_continuity.continuity_class(),
        BindingContinuityClass::AuthoritativeSuccessor
    );
    assert_eq!(
        kernel_successor.explanation().continuity_class(),
        BindingContinuityClass::AuthoritativeSuccessor
    );
    assert_eq!(
        correspondence_continuity.continuity_class(),
        BindingContinuityClass::CorrespondenceOnly
    );
    assert_eq!(
        kernel_correspondence.explanation().continuity_class(),
        BindingContinuityClass::CorrespondenceOnly
    );
    assert_eq!(
        admitted_partial_continuity.continuity_class(),
        BindingContinuityClass::InsufficientEvidenceFromAdmittedPartial
    );
    assert_eq!(
        denied_incomplete_continuity.continuity_class(),
        BindingContinuityClass::InsufficientEvidenceFromDeniedIncomplete
    );
    assert_eq!(
        ambiguous_continuity.continuity_class(),
        BindingContinuityClass::Ambiguous
    );
}
