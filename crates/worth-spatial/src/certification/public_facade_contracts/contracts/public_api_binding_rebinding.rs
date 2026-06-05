use worth_primitives::{
    PrimitiveConstructionFamilyContractRegistry, PrimitiveGeometryIdentityBundle,
    PrimitiveSupportPlaneIdentity, PrimitiveVertexIdentity, PrimitiveWitnessDescriptor,
};
use worth_spatial::facade::bindings::{
    attach_surface_to_face, evaluate_binding_motion_posture, rebind_surface_on_face,
    FaceBindingSite, FaceSurfaceBindingSpec, LocalTopologyReplacementNeighborhood,
    MotionAwareBindingPosture, NeighborhoodBindingFamily, RebindingOutcomeClass,
    ReplacementCandidate, ReplacementCandidateSet, SpatialAdmittedPrimitiveBinding,
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
fn spatial_public_facade_exports_local_neighborhood_rebinding_surface() {
    let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::Orthotope,
    );
    let prior = attach_surface_to_face(FaceSurfaceBindingSpec::new(
        FaceBindingSite::new("face-old"),
        contract,
        plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    ))
    .expect("prior");
    let successor = attach_surface_to_face(FaceSurfaceBindingSpec::new(
        FaceBindingSite::new("face-new"),
        contract,
        plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    ))
    .expect("successor");
    let neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::FaceSurface,
        "face-old",
        ReplacementCandidateSet::new(vec![ReplacementCandidate::new(
            "successor",
            SpatialAdmittedPrimitiveBinding::FaceSurface(successor),
        )
        .expect("candidate")])
        .expect("candidate set"),
    )
    .expect("neighborhood");
    let prior_binding = SpatialAdmittedPrimitiveBinding::FaceSurface(prior);
    let posture =
        evaluate_binding_motion_posture(&prior_binding, &neighborhood).expect("motion posture");
    let decision = rebind_surface_on_face(prior_binding, neighborhood).expect("rebinding");

    assert_eq!(posture, MotionAwareBindingPosture::RequiresRebinding);
    assert_eq!(
        decision.outcome_class(),
        RebindingOutcomeClass::ExactReattachment
    );
    assert!(decision
        .explanation()
        .selected_candidate_identity()
        .is_some());
}
