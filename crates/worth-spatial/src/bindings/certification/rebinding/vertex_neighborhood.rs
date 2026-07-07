use crate::bindings::rebinding::{
    LocalTopologyReplacementNeighborhood, MotionAwareBindingPosture, NeighborhoodBindingFamily,
    RebindingOutcomeClass, ReplacementCandidateSet,
};

#[test]
fn vertex_rebinding_uses_the_same_local_neighborhood_law_as_other_core_families() {
    let prior_declaration = super::vertex_binding_declaration(
        "vertex-old",
        super::plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    );
    let successor_declaration = super::vertex_binding_declaration(
        "vertex-new",
        super::plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    );
    let neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::VertexGeometry,
        "vertex-old",
        ReplacementCandidateSet::new(vec![
            super::super::rebinding_candidate_from_binding_declaration(
                "successor",
                &successor_declaration,
                "rebinding-vertex-successor-candidate",
            )
            .expect("candidate"),
        ])
        .expect("candidate set"),
    )
    .expect("neighborhood");

    let decision = super::super::rebind_geometry_on_vertex_from_fact(
        super::super::rebinding_prior_fact_from_binding_declaration(
            &prior_declaration,
            "rebinding-vertex-prior",
        ),
        neighborhood,
    )
    .expect("decision");
    let prior = super::super::rebinding_prior_fact_from_binding_declaration(
        &prior_declaration,
        "rebinding-vertex-prior-identity",
    );
    let successor = super::super::rebinding_candidate_from_binding_declaration(
        "successor",
        &successor_declaration,
        "rebinding-vertex-successor-identity",
    )
    .expect("successor identity");

    assert_eq!(
        decision.outcome_class(),
        RebindingOutcomeClass::ExactReattachment
    );
    assert_eq!(
        decision.neighborhood_family(),
        NeighborhoodBindingFamily::VertexGeometry
    );
    assert_eq!(
        decision.selected_candidate_identity(),
        Some(successor.binding_identity())
    );
    assert_eq!(
        decision.motion_posture(),
        MotionAwareBindingPosture::Unresolved
    );
    assert_eq!(
        prior.prior_binding_identity() != successor.binding_identity(),
        true
    );
}
