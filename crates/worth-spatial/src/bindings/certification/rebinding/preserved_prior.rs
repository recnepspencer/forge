use crate::bindings::rebinding::{
    LocalTopologyReplacementNeighborhood, MotionAwareBindingPosture, NeighborhoodBindingFamily,
    RebindingOutcomeClass, ReplacementCandidateSet,
};

#[test]
fn rebinding_authority_preserves_when_prior_binding_remains_in_local_neighborhood() {
    let prior_declaration = super::surface_binding_declaration(
        "face-old",
        super::plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    );
    let neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::FaceSurface,
        "face-old",
        ReplacementCandidateSet::new(vec![
            super::super::rebinding_candidate_from_binding_declaration(
                "preserved",
                &prior_declaration,
                "rebinding-preserved-candidate",
            )
            .expect("preserved"),
        ])
        .expect("candidate set"),
    )
    .expect("neighborhood");

    let decision = super::super::rebind_surface_on_face_from_fact(
        super::super::rebinding_prior_fact_from_binding_declaration(
            &prior_declaration,
            "rebinding-preserved-prior",
        ),
        neighborhood,
    )
    .expect("decision");
    assert_eq!(decision.outcome_class(), RebindingOutcomeClass::Preserved);
    assert_eq!(
        decision.motion_posture(),
        MotionAwareBindingPosture::Unresolved
    );
    assert_eq!(
        decision.selected_candidate_identity(),
        Some(
            super::super::rebinding_prior_fact_from_binding_declaration(
                &prior_declaration,
                "rebinding-preserved-prior-identity",
            )
            .prior_binding_identity()
        )
    );
}
