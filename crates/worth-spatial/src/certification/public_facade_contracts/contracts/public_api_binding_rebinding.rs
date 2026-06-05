use worth_primitives::{
    PrimitiveConstructionFamilyContractRegistry, PrimitiveGeometryIdentityBundle,
    PrimitiveSupportPlaneIdentity, PrimitiveVertexIdentity, PrimitiveWitnessDescriptor,
};
use worth_spatial::facade::bindings::{
    attach_surface_to_face, attach_vertex_geometry, evaluate_binding_motion_posture,
    evaluate_continuity, explain_rebinding_decision, rebind_geometry_on_vertex,
    rebind_surface_on_face, BindingContinuityClass, BindingMotionSemanticsInput, FaceBindingSite,
    FaceSurfaceBindingSpec, LocalTopologyReplacementNeighborhood, MotionAwareBindingPosture,
    NeighborhoodBindingFamily, RebindingOutcomeClass, ReplacementCandidate,
    ReplacementCandidateSet, SpatialAdmittedPrimitiveBinding, UnsupportedRebindingReason,
    VertexBindingSite, VertexGeometryBindingSpec, VertexGeometryProvenanceKind,
    VertexToleranceRegime,
};
use worth_spatial::facade::motion::{admit_spatial_rotate, SpatialRotateSpec};

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
    let posture = evaluate_binding_motion_posture(
        &prior_binding,
        BindingMotionSemanticsInput::unresolved_without_motion_workflow(),
    )
    .expect("motion posture");
    let decision = rebind_surface_on_face(prior_binding, neighborhood).expect("rebinding");

    assert_eq!(posture, MotionAwareBindingPosture::Unresolved);
    assert_eq!(
        decision.outcome_class(),
        RebindingOutcomeClass::ExactReattachment
    );
    assert!(decision
        .explanation()
        .selected_candidate_identity()
        .is_some());
}

#[test]
fn spatial_public_facade_exports_phase_six_motion_posture_surface() {
    let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::Orthotope,
    );
    let prior = attach_surface_to_face(FaceSurfaceBindingSpec::new(
        FaceBindingSite::new("face-1"),
        contract,
        plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    ))
    .expect("prior");
    let prior_binding = SpatialAdmittedPrimitiveBinding::FaceSurface(prior);
    let preserved_rotate =
        admit_spatial_rotate(SpatialRotateSpec::shape_origin().by_radians(0.0)).expect("rotate");

    let posture = evaluate_binding_motion_posture(
        &prior_binding,
        BindingMotionSemanticsInput::for_rotate(&preserved_rotate),
    )
    .expect("posture");

    assert_eq!(posture, MotionAwareBindingPosture::Preserved);
}

#[test]
fn spatial_public_facade_exports_vertex_local_rebinding_surface() {
    let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::Orthotope,
    );
    let prior = attach_vertex_geometry(VertexGeometryBindingSpec::new(
        VertexBindingSite::new("vertex-old"),
        contract,
        plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
        VertexGeometryProvenanceKind::CanonicalWitness,
        VertexToleranceRegime::ExactBits,
    ))
    .expect("prior");
    let successor = attach_vertex_geometry(VertexGeometryBindingSpec::new(
        VertexBindingSite::new("vertex-new"),
        contract,
        plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
        VertexGeometryProvenanceKind::CanonicalWitness,
        VertexToleranceRegime::ExactBits,
    ))
    .expect("successor");

    let decision = rebind_geometry_on_vertex(
        SpatialAdmittedPrimitiveBinding::VertexGeometry(prior),
        LocalTopologyReplacementNeighborhood::new(
            NeighborhoodBindingFamily::VertexGeometry,
            "vertex-old",
            ReplacementCandidateSet::new(vec![ReplacementCandidate::new(
                "successor",
                SpatialAdmittedPrimitiveBinding::VertexGeometry(successor),
            )
            .expect("candidate")])
            .expect("candidate set"),
        )
        .expect("neighborhood"),
    )
    .expect("decision");

    assert_eq!(
        decision.outcome_class(),
        RebindingOutcomeClass::ExactReattachment
    );
}

#[test]
fn spatial_public_facade_exports_typed_unsupported_rebinding_outcome() {
    let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::Orthotope,
    );
    let prior = attach_vertex_geometry(VertexGeometryBindingSpec::new(
        VertexBindingSite::new("vertex-old"),
        contract,
        plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
        VertexGeometryProvenanceKind::CanonicalWitness,
        VertexToleranceRegime::ExactBits,
    ))
    .expect("prior");
    let decision = rebind_surface_on_face(
        SpatialAdmittedPrimitiveBinding::VertexGeometry(prior.clone()),
        LocalTopologyReplacementNeighborhood::new(
            NeighborhoodBindingFamily::VertexGeometry,
            "vertex-old",
            ReplacementCandidateSet::new(vec![ReplacementCandidate::new(
                "successor",
                SpatialAdmittedPrimitiveBinding::VertexGeometry(prior),
            )
            .expect("candidate")])
            .expect("candidate set"),
        )
        .expect("neighborhood"),
    )
    .expect("decision");

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

#[test]
fn spatial_public_facade_exports_continuity_and_rebinding_explanation_surface() {
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
        plane_geometry([[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]]),
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

    let continuity =
        evaluate_continuity(&prior_binding, &neighborhood).expect("continuity assessment");
    let decision = rebind_surface_on_face(prior_binding, neighborhood).expect("decision");
    let explanation = explain_rebinding_decision(&decision);

    assert_eq!(
        continuity.continuity_class(),
        BindingContinuityClass::AuthoritativeSuccessor
    );
    assert_eq!(
        explanation.continuity_class(),
        BindingContinuityClass::AuthoritativeSuccessor
    );
    assert_eq!(explanation.candidate_labels(), ["successor"]);
    assert!(explanation.selected_candidate_identity().is_some());
}
