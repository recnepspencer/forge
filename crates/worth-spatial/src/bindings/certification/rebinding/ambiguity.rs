use crate::bindings::rebinding::{
    LocalTopologyReplacementNeighborhood, MotionAwareBindingPosture, NeighborhoodBindingFamily,
    RebindingOutcomeClass, ReplacementCandidateSet,
};

#[test]
fn rebinding_authority_keeps_edge_curve_ambiguity_typed() {
    let prior_declaration = super::edge_binding_declaration(
        "edge-old",
        super::plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    );
    let a_declaration = super::edge_binding_declaration(
        "edge-a",
        super::plane_geometry([[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]]),
    );
    let b_declaration = super::edge_binding_declaration(
        "edge-b",
        super::plane_geometry([[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]]),
    );
    let neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::EdgeCurve,
        "edge-old",
        ReplacementCandidateSet::new(vec![
            super::super::rebinding_candidate_from_binding_declaration(
                "a",
                &a_declaration,
                "rebinding-ambiguity-a",
            )
            .expect("a"),
            super::super::rebinding_candidate_from_binding_declaration(
                "b",
                &b_declaration,
                "rebinding-ambiguity-b",
            )
            .expect("b"),
        ])
        .expect("candidates"),
    )
    .expect("neighborhood");

    let decision = super::super::rebind_curve_on_edge_from_fact(
        super::super::rebinding_prior_fact_from_binding_declaration(
            &prior_declaration,
            "rebinding-ambiguity-prior",
        ),
        neighborhood,
    )
    .expect("decision");
    let prior = super::super::rebinding_prior_fact_from_binding_declaration(
        &prior_declaration,
        "rebinding-ambiguity-prior-identity",
    );
    let a = super::super::rebinding_candidate_from_binding_declaration(
        "a",
        &a_declaration,
        "rebinding-ambiguity-a-identity",
    )
    .expect("a identity");
    let b = super::super::rebinding_candidate_from_binding_declaration(
        "b",
        &b_declaration,
        "rebinding-ambiguity-b-identity",
    )
    .expect("b identity");

    assert_eq!(a.binding_identity() != b.binding_identity(), true);
    assert_eq!(prior.prior_binding_identity() != a.binding_identity(), true);
    assert_eq!(decision.outcome_class(), RebindingOutcomeClass::Ambiguous);
    assert_eq!(
        decision.motion_posture(),
        MotionAwareBindingPosture::Unresolved
    );
    assert!(decision.selected_candidate_identity().is_none());
}
