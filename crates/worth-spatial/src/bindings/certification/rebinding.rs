#![cfg(test)]

use worth_primitives::{
    PrimitiveConstructionFamilyContractRegistry, PrimitiveGeometryIdentityBundle,
    PrimitiveSupportPlaneIdentity, PrimitiveVertexIdentity, PrimitiveWitnessDescriptor,
};

use crate::facade::bindings::{
    attach_curve_to_edge, attach_vertex_geometry, evaluate_continuity, rebind_curve_on_edge,
    rebind_geometry_on_vertex, rebind_surface_on_face, BindingContinuityClass, EdgeBindingSite,
    EdgeCurveBindingSpec, FaceBindingSite, FaceSurfaceBindingSpec,
    LocalTopologyReplacementNeighborhood, MotionAwareBindingPosture, NeighborhoodBindingFamily,
    RebindingOutcomeClass, ReplacementCandidate, ReplacementCandidateSet, VertexBindingSite,
    VertexGeometryBindingSpec, VertexGeometryProvenanceKind, VertexToleranceRegime,
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
                crate::bindings::admitted_binding::SpatialAdmittedPrimitiveBinding::FaceSurface(
                    weaker.clone(),
                ),
            )
            .expect("weaker"),
            ReplacementCandidate::new(
                "exact",
                crate::bindings::admitted_binding::SpatialAdmittedPrimitiveBinding::FaceSurface(
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
                crate::bindings::admitted_binding::SpatialAdmittedPrimitiveBinding::FaceSurface(
                    exact.clone(),
                ),
            )
            .expect("exact"),
            ReplacementCandidate::new(
                "weaker",
                crate::bindings::admitted_binding::SpatialAdmittedPrimitiveBinding::FaceSurface(
                    weaker,
                ),
            )
            .expect("weaker"),
        ])
        .expect("candidate set"),
    )
    .expect("right");

    let left_decision = rebind_surface_on_face(
        crate::bindings::admitted_binding::SpatialAdmittedPrimitiveBinding::FaceSurface(
            prior.clone(),
        ),
        left,
    )
    .expect("left decision");
    let right_decision = rebind_surface_on_face(
        crate::bindings::admitted_binding::SpatialAdmittedPrimitiveBinding::FaceSurface(prior),
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
        &MotionAwareBindingPosture::Unresolved
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
                crate::bindings::admitted_binding::SpatialAdmittedPrimitiveBinding::EdgeCurve(a),
            )
            .expect("a"),
            ReplacementCandidate::new(
                "b",
                crate::bindings::admitted_binding::SpatialAdmittedPrimitiveBinding::EdgeCurve(b),
            )
            .expect("b"),
        ])
        .expect("candidates"),
    )
    .expect("neighborhood");

    let decision = rebind_curve_on_edge(
        crate::bindings::admitted_binding::SpatialAdmittedPrimitiveBinding::EdgeCurve(prior),
        neighborhood,
    )
    .expect("decision");

    assert_eq!(decision.outcome_class(), RebindingOutcomeClass::Ambiguous);
    assert_eq!(
        decision.explanation().motion_posture(),
        &MotionAwareBindingPosture::Unresolved
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
            crate::bindings::admitted_binding::SpatialAdmittedPrimitiveBinding::FaceSurface(
                prior.clone(),
            ),
        )
        .expect("preserved")])
        .expect("candidate set"),
    )
    .expect("neighborhood");

    let decision = rebind_surface_on_face(
        crate::bindings::admitted_binding::SpatialAdmittedPrimitiveBinding::FaceSurface(
            prior.clone(),
        ),
        neighborhood,
    )
    .expect("decision");

    assert_eq!(decision.outcome_class(), RebindingOutcomeClass::Preserved);
    assert_eq!(
        decision.explanation().motion_posture(),
        &MotionAwareBindingPosture::Unresolved
    );
    assert_eq!(
        decision.explanation().selected_candidate_identity(),
        Some(prior.identity().as_str())
    );
}

#[test]
fn rebinding_continuity_preserves_partial_vs_denied_incomplete_distinction() {
    let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::Orthotope,
    );
    let prior = attach_curve_to_edge(EdgeCurveBindingSpec::new(
        EdgeBindingSite::new("edge-old"),
        contract,
        plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    ))
    .expect("prior");
    let partial = attach_curve_to_edge(EdgeCurveBindingSpec::new(
        EdgeBindingSite::new("edge-partial"),
        contract,
        PrimitiveGeometryIdentityBundle::new(
            vec![],
            vec![PrimitiveVertexIdentity::from_position([0.0, 0.0, 0.0])],
        ),
    ))
    .expect("partial");
    let denied = attach_curve_to_edge(EdgeCurveBindingSpec::new(
        EdgeBindingSite::new("edge-denied"),
        contract,
        PrimitiveGeometryIdentityBundle::new(vec![], vec![]),
    ))
    .expect("denied");
    let neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::EdgeCurve,
        "edge-old",
        ReplacementCandidateSet::new(vec![
            ReplacementCandidate::new(
                "partial",
                crate::bindings::admitted_binding::SpatialAdmittedPrimitiveBinding::EdgeCurve(
                    partial.clone(),
                ),
            )
            .expect("partial"),
            ReplacementCandidate::new(
                "denied",
                crate::bindings::admitted_binding::SpatialAdmittedPrimitiveBinding::EdgeCurve(
                    denied,
                ),
            )
            .expect("denied"),
        ])
        .expect("candidates"),
    )
    .expect("neighborhood");

    let continuity = evaluate_continuity(
        &crate::bindings::admitted_binding::SpatialAdmittedPrimitiveBinding::EdgeCurve(
            prior.clone(),
        ),
        &neighborhood,
    )
    .expect("continuity");

    assert_eq!(
        continuity.continuity_class(),
        BindingContinuityClass::InsufficientEvidenceFromAdmittedPartial
    );
    let denied_only_neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::EdgeCurve,
        "edge-old",
        ReplacementCandidateSet::new(vec![ReplacementCandidate::new(
            "denied",
            crate::bindings::admitted_binding::SpatialAdmittedPrimitiveBinding::EdgeCurve(
                attach_curve_to_edge(EdgeCurveBindingSpec::new(
                    EdgeBindingSite::new("edge-denied-only"),
                    contract,
                    PrimitiveGeometryIdentityBundle::new(vec![], vec![]),
                ))
                .expect("denied only"),
            ),
        )
        .expect("denied only")])
        .expect("candidates"),
    )
    .expect("denied-only neighborhood");
    let denied_only_continuity = evaluate_continuity(
        &crate::bindings::admitted_binding::SpatialAdmittedPrimitiveBinding::EdgeCurve(
            prior.clone(),
        ),
        &denied_only_neighborhood,
    )
    .expect("denied-only continuity");
    assert_eq!(
        denied_only_continuity.continuity_class(),
        BindingContinuityClass::InsufficientEvidenceFromDeniedIncomplete
    );

    let decision = rebind_curve_on_edge(
        crate::bindings::admitted_binding::SpatialAdmittedPrimitiveBinding::EdgeCurve(prior),
        neighborhood,
    )
    .expect("decision");
    assert_eq!(decision.outcome_class(), RebindingOutcomeClass::Orphaned);
}

#[test]
fn vertex_rebinding_uses_the_same_local_neighborhood_law_as_other_core_families() {
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
    let neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::VertexGeometry,
        "vertex-old",
        ReplacementCandidateSet::new(vec![ReplacementCandidate::new(
            "successor",
            crate::bindings::admitted_binding::SpatialAdmittedPrimitiveBinding::VertexGeometry(
                successor.clone(),
            ),
        )
        .expect("candidate")])
        .expect("candidate set"),
    )
    .expect("neighborhood");

    let decision = rebind_geometry_on_vertex(
        crate::bindings::admitted_binding::SpatialAdmittedPrimitiveBinding::VertexGeometry(prior),
        neighborhood,
    )
    .expect("decision");

    assert_eq!(
        decision.outcome_class(),
        RebindingOutcomeClass::ExactReattachment
    );
    assert_eq!(
        decision.explanation().neighborhood_family(),
        NeighborhoodBindingFamily::VertexGeometry
    );
    assert_eq!(
        decision.explanation().selected_candidate_identity(),
        Some(successor.identity().as_str())
    );
    assert_eq!(
        decision.explanation().motion_posture(),
        &MotionAwareBindingPosture::Unresolved
    );
}
