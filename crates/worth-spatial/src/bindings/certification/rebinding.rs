#![cfg(test)]

use worth_primitives::{
    PrimitiveConstructionFamilyContractRegistry, PrimitiveGeometryIdentityBundle,
    PrimitiveSupportPlaneIdentity, PrimitiveVertexIdentity, PrimitiveWitnessDescriptor,
};

use crate::facade::bindings::{
    attach_curve_to_edge, rebind_curve_on_edge, rebind_surface_on_face, EdgeBindingSite,
    EdgeCurveBindingSpec, FaceBindingSite, FaceSurfaceBindingSpec,
    LocalTopologyReplacementNeighborhood, MotionAwareBindingPosture, NeighborhoodBindingFamily,
    RebindingOutcomeClass, ReplacementCandidate, ReplacementCandidateSet,
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

#[test]
fn rebinding_authority_keeps_candidate_order_out_of_surface_decisions() {
    let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::Orthotope,
    );
    let prior = crate::facade::bindings::attach_surface_to_face(FaceSurfaceBindingSpec::new(
        FaceBindingSite::new("face-old"),
        contract,
        plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    ))
    .expect("prior");
    let exact = crate::facade::bindings::attach_surface_to_face(FaceSurfaceBindingSpec::new(
        FaceBindingSite::new("face-new-a"),
        contract,
        plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    ))
    .expect("exact");
    let weaker = crate::facade::bindings::attach_surface_to_face(FaceSurfaceBindingSpec::new(
        FaceBindingSite::new("face-new-b"),
        contract,
        plane_geometry([[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]]),
    ))
    .expect("weaker");
    let left = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::FaceSurface,
        "face-old",
        ReplacementCandidateSet::new(vec![
            ReplacementCandidate::new(
                "weaker",
                crate::bindings::authority::SpatialAdmittedPrimitiveBinding::FaceSurface(
                    weaker.clone(),
                ),
            )
            .expect("weaker"),
            ReplacementCandidate::new(
                "exact",
                crate::bindings::authority::SpatialAdmittedPrimitiveBinding::FaceSurface(
                    exact.clone(),
                ),
            )
            .expect("exact"),
        ])
        .expect("candidate set"),
    )
    .expect("left");
    let right = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::FaceSurface,
        "face-old",
        ReplacementCandidateSet::new(vec![
            ReplacementCandidate::new(
                "exact",
                crate::bindings::authority::SpatialAdmittedPrimitiveBinding::FaceSurface(
                    exact.clone(),
                ),
            )
            .expect("exact"),
            ReplacementCandidate::new(
                "weaker",
                crate::bindings::authority::SpatialAdmittedPrimitiveBinding::FaceSurface(weaker),
            )
            .expect("weaker"),
        ])
        .expect("candidate set"),
    )
    .expect("right");

    let left_decision = rebind_surface_on_face(
        crate::bindings::authority::SpatialAdmittedPrimitiveBinding::FaceSurface(prior.clone()),
        left,
    )
    .expect("left decision");
    let right_decision = rebind_surface_on_face(
        crate::bindings::authority::SpatialAdmittedPrimitiveBinding::FaceSurface(prior),
        right,
    )
    .expect("right decision");

    assert_eq!(
        left_decision.outcome_class(),
        RebindingOutcomeClass::ExactReattachment
    );
    assert_eq!(
        left_decision.outcome_class(),
        right_decision.outcome_class()
    );
    assert_eq!(
        left_decision.explanation().motion_posture(),
        &MotionAwareBindingPosture::RequiresRebinding
    );
    assert_eq!(
        left_decision.explanation().selected_candidate_identity(),
        right_decision.explanation().selected_candidate_identity()
    );
}

#[test]
fn rebinding_authority_keeps_edge_curve_ambiguity_typed() {
    let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::Orthotope,
    );
    let prior = attach_curve_to_edge(EdgeCurveBindingSpec::new(
        EdgeBindingSite::new("edge-old"),
        contract,
        plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    ))
    .expect("prior");
    let a = attach_curve_to_edge(EdgeCurveBindingSpec::new(
        EdgeBindingSite::new("edge-a"),
        contract,
        plane_geometry([[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]]),
    ))
    .expect("a");
    let b = attach_curve_to_edge(EdgeCurveBindingSpec::new(
        EdgeBindingSite::new("edge-b"),
        contract,
        plane_geometry([[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]]),
    ))
    .expect("b");
    let neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::EdgeCurve,
        "edge-old",
        ReplacementCandidateSet::new(vec![
            ReplacementCandidate::new(
                "a",
                crate::bindings::authority::SpatialAdmittedPrimitiveBinding::EdgeCurve(a),
            )
            .expect("a"),
            ReplacementCandidate::new(
                "b",
                crate::bindings::authority::SpatialAdmittedPrimitiveBinding::EdgeCurve(b),
            )
            .expect("b"),
        ])
        .expect("candidates"),
    )
    .expect("neighborhood");

    let decision = rebind_curve_on_edge(
        crate::bindings::authority::SpatialAdmittedPrimitiveBinding::EdgeCurve(prior),
        neighborhood,
    )
    .expect("decision");

    assert_eq!(decision.outcome_class(), RebindingOutcomeClass::Ambiguous);
    assert_eq!(
        decision.explanation().motion_posture(),
        &MotionAwareBindingPosture::RequiresRebinding
    );
    assert!(decision
        .explanation()
        .selected_candidate_identity()
        .is_none());
}

#[test]
fn rebinding_authority_preserves_when_prior_binding_remains_in_local_neighborhood() {
    let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::Orthotope,
    );
    let prior = crate::facade::bindings::attach_surface_to_face(FaceSurfaceBindingSpec::new(
        FaceBindingSite::new("face-old"),
        contract,
        plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    ))
    .expect("prior");
    let neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::FaceSurface,
        "face-old",
        ReplacementCandidateSet::new(vec![ReplacementCandidate::new(
            "preserved",
            crate::bindings::authority::SpatialAdmittedPrimitiveBinding::FaceSurface(prior.clone()),
        )
        .expect("preserved")])
        .expect("candidate set"),
    )
    .expect("neighborhood");

    let decision = rebind_surface_on_face(
        crate::bindings::authority::SpatialAdmittedPrimitiveBinding::FaceSurface(prior.clone()),
        neighborhood,
    )
    .expect("decision");

    assert_eq!(decision.outcome_class(), RebindingOutcomeClass::Preserved);
    assert_eq!(
        decision.explanation().motion_posture(),
        &MotionAwareBindingPosture::Preserved
    );
    assert_eq!(
        decision.explanation().selected_candidate_identity(),
        Some(prior.identity().as_str())
    );
}
