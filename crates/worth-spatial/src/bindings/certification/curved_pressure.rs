#![cfg(test)]

use std::f64::consts::{PI, TAU};

use worth_geom::facade::{ParameterDomain, ParameterSpacePoint, PolygonalTrimmedParameterRegion};
use worth_primitives::{
    PrimitiveConstructionFamilyContractRegistry, PrimitiveGeometryIdentityBundle,
    PrimitiveSupportPlaneIdentity, PrimitiveVertexIdentity, PrimitiveWitnessDescriptor,
};

use crate::facade::bindings::{
    attach_parameter_space_point_to_face, evaluate_continuity, rebind_surface_on_face,
    AnchorCarrierOwnership, BindingContinuityClass, CarrierOwnedParameterPointAnchorSpec,
    FaceBindingSite, FaceSurfaceBindingSpec, LocalTopologyReplacementNeighborhood,
    MotionAwareBindingPosture, NeighborhoodBindingFamily, RebindingOutcomeClass,
    ReplacementCandidate, ReplacementCandidateSet, SpatialAdmittedPrimitiveBinding,
    SpatialAnchorAuthorityError,
};

fn planar_geometry() -> PrimitiveGeometryIdentityBundle {
    PrimitiveGeometryIdentityBundle::new(
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
    )
}

fn anchored_face_binding(
    face_id: &str,
    domain: ParameterDomain,
    point: [f64; 2],
) -> SpatialAdmittedPrimitiveBinding {
    let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::Orthotope,
    );
    SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(
        attach_parameter_space_point_to_face(
            FaceSurfaceBindingSpec::new(FaceBindingSite::new(face_id), contract, planar_geometry()),
            CarrierOwnedParameterPointAnchorSpec::new(
                AnchorCarrierOwnership::for_face_surface(face_id, domain).expect("ownership"),
                ParameterSpacePoint::try_new(point).expect("parameter"),
            )
            .expect("anchor spec"),
        )
        .expect("point anchor binding"),
    )
}

fn trimmed_face_binding(
    face_id: &str,
    outer_boundary: [[f64; 2]; 4],
    point: [f64; 2],
) -> SpatialAdmittedPrimitiveBinding {
    let trimmed_region = PolygonalTrimmedParameterRegion::new(
        ParameterDomain::plane(),
        outer_boundary
            .into_iter()
            .map(|coords| ParameterSpacePoint::try_new(coords).expect("boundary point"))
            .collect(),
        vec![],
    )
    .expect("trimmed region");
    let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::Orthotope,
    );
    SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(
        attach_parameter_space_point_to_face(
            FaceSurfaceBindingSpec::new(FaceBindingSite::new(face_id), contract, planar_geometry()),
            CarrierOwnedParameterPointAnchorSpec::new(
                AnchorCarrierOwnership::for_trimmed_face_surface(face_id, trimmed_region)
                    .expect("ownership"),
                ParameterSpacePoint::try_new(point).expect("parameter"),
            )
            .expect("anchor spec"),
        )
        .expect("point anchor binding"),
    )
}

#[test]
fn curved_carrier_pressure_breaks_planar_anchor_and_rebinding_shortcuts() {
    let prior = anchored_face_binding(
        "face-curved",
        ParameterDomain::cylinder(),
        [TAU + 0.25, 0.5],
    );
    let preserved = anchored_face_binding("face-curved", ParameterDomain::cylinder(), [0.25, 0.5]);
    let planarized = anchored_face_binding("face-curved", ParameterDomain::plane(), [0.25, 0.5]);

    let curved_neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::FaceSurfacePointAnchor,
        "face-curved",
        ReplacementCandidateSet::new(vec![
            ReplacementCandidate::new("curved", preserved.clone()).expect("candidate")
        ])
        .expect("candidate set"),
    )
    .expect("curved neighborhood");
    let curved_continuity = evaluate_continuity(&prior, &curved_neighborhood).expect("continuity");
    let curved_decision =
        rebind_surface_on_face(prior.clone(), curved_neighborhood).expect("decision");

    let planarized_neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::FaceSurfacePointAnchor,
        "face-curved",
        ReplacementCandidateSet::new(vec![
            ReplacementCandidate::new("planarized", planarized).expect("candidate")
        ])
        .expect("candidate set"),
    )
    .expect("planarized neighborhood");
    let planarized_continuity =
        evaluate_continuity(&prior, &planarized_neighborhood).expect("continuity");
    let planarized_decision =
        rebind_surface_on_face(prior.clone(), planarized_neighborhood).expect("decision");

    assert_eq!(prior.identity(), preserved.identity());
    assert_eq!(
        curved_continuity.continuity_class(),
        BindingContinuityClass::Exact
    );
    assert_eq!(
        curved_decision.outcome_class(),
        RebindingOutcomeClass::Preserved
    );
    assert_eq!(
        curved_decision.explanation().motion_posture(),
        &MotionAwareBindingPosture::Unresolved
    );

    assert_ne!(
        prior.identity(),
        planarized_decision.selected_binding().unwrap().identity()
    );
    assert_eq!(
        planarized_continuity.continuity_class(),
        BindingContinuityClass::CorrespondenceOnly
    );
    assert_eq!(
        planarized_decision.outcome_class(),
        RebindingOutcomeClass::CorrespondenceOnly
    );
    assert_eq!(
        planarized_decision.explanation().continuity_class(),
        BindingContinuityClass::CorrespondenceOnly
    );
}

#[test]
fn curved_binding_and_rebinding_do_not_fall_back_to_planarized_identity_or_domain_assumptions() {
    let periodic = anchored_face_binding(
        "face-periodic",
        ParameterDomain::cylinder(),
        [TAU + 0.25, 0.5],
    );
    let canonical =
        anchored_face_binding("face-periodic", ParameterDomain::cylinder(), [0.25, 0.5]);
    let planarized = anchored_face_binding("face-periodic", ParameterDomain::plane(), [0.25, 0.5]);
    let trimmed_a = trimmed_face_binding(
        "face-trimmed",
        [[0.0, 0.0], [3.0, 0.0], [3.0, 3.0], [0.0, 3.0]],
        [1.0, 1.0],
    );
    let trimmed_b = trimmed_face_binding(
        "face-trimmed",
        [[0.5, 0.0], [3.0, 0.0], [3.0, 3.0], [0.5, 3.0]],
        [1.0, 1.0],
    );
    let trimmed_neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::FaceSurfacePointAnchor,
        "face-trimmed",
        ReplacementCandidateSet::new(vec![ReplacementCandidate::new(
            "trimmed-b",
            trimmed_b.clone(),
        )
        .expect("candidate")])
        .expect("candidate set"),
    )
    .expect("trimmed neighborhood");

    let denied = CarrierOwnedParameterPointAnchorSpec::new(
        AnchorCarrierOwnership::for_face_surface("face-sphere", ParameterDomain::sphere())
            .expect("ownership"),
        ParameterSpacePoint::try_new([0.25, PI]).expect("parameter"),
    )
    .expect_err("sphere latitude outside admitted domain should deny");

    assert_eq!(periodic.identity(), canonical.identity());
    assert_ne!(periodic.identity(), planarized.identity());
    assert_ne!(trimmed_a.identity(), trimmed_b.identity());
    assert_eq!(
        evaluate_continuity(&trimmed_a, &trimmed_neighborhood)
            .expect("trimmed continuity")
            .continuity_class(),
        BindingContinuityClass::CorrespondenceOnly
    );
    assert!(matches!(
        denied,
        SpatialAnchorAuthorityError::ParameterDomainViolation(_)
    ));
}
