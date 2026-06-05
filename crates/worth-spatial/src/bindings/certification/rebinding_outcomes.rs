#![cfg(test)]

use worth_geom::facade::{ParameterDomain, ParameterSpacePoint};

use crate::bindings::rebinding::rebind_surface_on_face;
use crate::facade::bindings::{
    attach_parameter_space_point_to_face, attach_surface_to_face, AnchorCarrierOwnership,
    CarrierOwnedParameterPointAnchorSpec, FaceBindingSite, FaceSurfaceBindingSpec,
    LocalTopologyReplacementNeighborhood, NeighborhoodBindingFamily, RebindingOutcomeClass,
    ReplacementCandidate, ReplacementCandidateSet, SpatialAdmittedPrimitiveBinding,
    UnsupportedRebindingReason,
};
use worth_primitives::{
    PrimitiveConstructionFamilyContractRegistry, PrimitiveGeometryIdentityBundle,
    PrimitiveSupportPlaneIdentity, PrimitiveVertexIdentity, PrimitiveWitnessDescriptor,
};

fn plane_geometry(vertices: [[f64; 3]; 2]) -> PrimitiveGeometryIdentityBundle {
    PrimitiveGeometryIdentityBundle::new(
        vec![PrimitiveSupportPlaneIdentity::new(
            "0".to_string(),
            "0".to_string(),
            "1".to_string(),
            "0".to_string(),
        )],
        vertices
            .into_iter()
            .map(PrimitiveVertexIdentity::from_position)
            .collect(),
    )
}

fn point_binding(
    face_id: &str,
    persistent_name: &str,
    vertices: [[f64; 3]; 2],
    point: [f64; 2],
) -> crate::facade::bindings::AdmittedFaceSurfacePointAnchorBinding {
    attach_parameter_space_point_to_face(
        FaceSurfaceBindingSpec::new(
            FaceBindingSite::new(face_id).with_persistent_name(persistent_name),
            PrimitiveConstructionFamilyContractRegistry::contract_for(
                &PrimitiveWitnessDescriptor::Orthotope,
            ),
            plane_geometry(vertices),
        ),
        CarrierOwnedParameterPointAnchorSpec::new(
            AnchorCarrierOwnership::for_face_surface(face_id, ParameterDomain::plane())
                .expect("ownership"),
            ParameterSpacePoint::try_new(point).expect("parameter"),
        )
        .expect("anchor spec"),
    )
    .expect("binding")
}

fn surface_binding(
    face_id: &str,
    persistent_name: &str,
    vertices: [[f64; 3]; 2],
) -> crate::bindings::authority::AdmittedFaceSurfaceBinding {
    attach_surface_to_face(FaceSurfaceBindingSpec::new(
        FaceBindingSite::new(face_id).with_persistent_name(persistent_name),
        PrimitiveConstructionFamilyContractRegistry::contract_for(
            &PrimitiveWitnessDescriptor::Orthotope,
        ),
        plane_geometry(vertices),
    ))
    .expect("binding")
}

#[test]
fn rebinding_outcome_classes_keep_success_and_unsupported_posture_distinct() {
    let prior = surface_binding(
        "face-old",
        "surface-alpha",
        [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
    );
    let exact = surface_binding(
        "face-new",
        "surface-beta",
        [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
    );
    let successor = surface_binding(
        "face-successor",
        "surface-gamma",
        [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]],
    );
    let prior_point = point_binding(
        "face-anchor-old",
        "surface-anchor-alpha",
        [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
        [0.25, 0.5],
    );
    let correspondence = point_binding(
        "face-correspondence",
        "surface-delta",
        [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]],
        [0.5, 0.5],
    );

    let preserved = rebind_surface_on_face(
        SpatialAdmittedPrimitiveBinding::FaceSurface(prior.clone()),
        LocalTopologyReplacementNeighborhood::new(
            NeighborhoodBindingFamily::FaceSurface,
            "face-old",
            ReplacementCandidateSet::new(vec![ReplacementCandidate::new(
                "preserved",
                SpatialAdmittedPrimitiveBinding::FaceSurface(prior.clone()),
            )
            .expect("candidate")])
            .expect("candidate set"),
        )
        .expect("neighborhood"),
    )
    .expect("preserved");
    let exact = rebind_surface_on_face(
        SpatialAdmittedPrimitiveBinding::FaceSurface(prior.clone()),
        LocalTopologyReplacementNeighborhood::new(
            NeighborhoodBindingFamily::FaceSurface,
            "face-old",
            ReplacementCandidateSet::new(vec![ReplacementCandidate::new(
                "exact",
                SpatialAdmittedPrimitiveBinding::FaceSurface(exact),
            )
            .expect("candidate")])
            .expect("candidate set"),
        )
        .expect("neighborhood"),
    )
    .expect("exact");
    let successor = rebind_surface_on_face(
        SpatialAdmittedPrimitiveBinding::FaceSurface(prior.clone()),
        LocalTopologyReplacementNeighborhood::new(
            NeighborhoodBindingFamily::FaceSurface,
            "face-old",
            ReplacementCandidateSet::new(vec![ReplacementCandidate::new(
                "successor",
                SpatialAdmittedPrimitiveBinding::FaceSurface(successor),
            )
            .expect("candidate")])
            .expect("candidate set"),
        )
        .expect("neighborhood"),
    )
    .expect("successor");
    let correspondence = rebind_surface_on_face(
        SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(prior_point),
        LocalTopologyReplacementNeighborhood::new(
            NeighborhoodBindingFamily::FaceSurfacePointAnchor,
            "face-anchor-old",
            ReplacementCandidateSet::new(vec![ReplacementCandidate::new(
                "correspondence",
                SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(correspondence),
            )
            .expect("candidate")])
            .expect("candidate set"),
        )
        .expect("neighborhood"),
    )
    .expect("correspondence");

    assert_eq!(preserved.outcome_class(), RebindingOutcomeClass::Preserved);
    assert_eq!(
        exact.outcome_class(),
        RebindingOutcomeClass::ExactReattachment
    );
    assert_eq!(
        successor.outcome_class(),
        RebindingOutcomeClass::ContinuityJustifiedReattachment
    );
    assert_eq!(
        correspondence.outcome_class(),
        RebindingOutcomeClass::CorrespondenceOnly
    );
}

#[test]
fn rebinding_unsupported_is_typed_outcome_not_error_fallback() {
    let vertex_prior = crate::facade::bindings::attach_vertex_geometry(
        crate::facade::bindings::VertexGeometryBindingSpec::new(
            crate::facade::bindings::VertexBindingSite::new("vertex-old"),
            PrimitiveConstructionFamilyContractRegistry::contract_for(
                &PrimitiveWitnessDescriptor::Orthotope,
            ),
            plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
            crate::facade::bindings::VertexGeometryProvenanceKind::CanonicalWitness,
            crate::facade::bindings::VertexToleranceRegime::ExactBits,
        ),
    )
    .expect("vertex prior");
    let decision = rebind_surface_on_face(
        SpatialAdmittedPrimitiveBinding::VertexGeometry(vertex_prior.clone()),
        LocalTopologyReplacementNeighborhood::new(
            NeighborhoodBindingFamily::VertexGeometry,
            "vertex-old",
            ReplacementCandidateSet::new(vec![ReplacementCandidate::new(
                "vertex-successor",
                SpatialAdmittedPrimitiveBinding::VertexGeometry(vertex_prior),
            )
            .expect("candidate")])
            .expect("candidate set"),
        )
        .expect("neighborhood"),
    )
    .expect("unsupported decision");

    assert_eq!(decision.outcome_class(), RebindingOutcomeClass::Unsupported);
    assert_eq!(
        decision.explanation().unsupported_reason(),
        Some(
            UnsupportedRebindingReason::RequestedRebindingFamilyDoesNotAdmitBindingFamily {
                requested: NeighborhoodBindingFamily::FaceSurface,
                actual: NeighborhoodBindingFamily::VertexGeometry,
            },
        )
    );
}
