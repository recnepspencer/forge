use crate::bindings::rebinding::{
    LocalTopologyReplacementNeighborhood, MotionAwareBindingPosture, NeighborhoodBindingFamily,
    RebindingOutcomeClass, ReplacementCandidateSet,
};

#[test]
fn rebinding_authority_keeps_candidate_order_out_of_surface_decisions() {
    let prior_declaration = super::surface_binding_declaration(
        "face-old",
        super::plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    );
    let exact_declaration = super::surface_binding_declaration(
        "face-new-a",
        super::plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    );
    let weaker_declaration = super::surface_binding_declaration(
        "face-new-b",
        super::plane_geometry([[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]]),
    );
    let left = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::FaceSurface,
        "face-old",
        ReplacementCandidateSet::new(vec![
            super::super::rebinding_candidate_from_binding_declaration(
                "weaker",
                &weaker_declaration,
                "rebinding-order-weaker-left",
            )
            .expect("weaker"),
            super::super::rebinding_candidate_from_binding_declaration(
                "exact",
                &exact_declaration,
                "rebinding-order-exact-left",
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
            super::super::rebinding_candidate_from_binding_declaration(
                "exact",
                &exact_declaration,
                "rebinding-order-exact-right",
            )
            .expect("exact"),
            super::super::rebinding_candidate_from_binding_declaration(
                "weaker",
                &weaker_declaration,
                "rebinding-order-weaker-right",
            )
            .expect("weaker"),
        ])
        .expect("candidate set"),
    )
    .expect("right");

    let left_decision = super::super::rebind_surface_on_face_from_fact(
        super::super::rebinding_prior_fact_from_binding_declaration(
            &prior_declaration,
            "rebinding-order-prior-left",
        ),
        left,
    )
    .expect("left decision");
    let right_decision = super::super::rebind_surface_on_face_from_fact(
        super::super::rebinding_prior_fact_from_binding_declaration(
            &prior_declaration,
            "rebinding-order-prior-right",
        ),
        right,
    )
    .expect("right decision");
    let prior = super::super::rebinding_prior_fact_from_binding_declaration(
        &prior_declaration,
        "rebinding-order-prior-identity",
    );
    let exact = super::super::rebinding_candidate_from_binding_declaration(
        "exact",
        &exact_declaration,
        "rebinding-order-exact-identity",
    )
    .expect("exact identity");
    let weaker = super::super::rebinding_candidate_from_binding_declaration(
        "weaker",
        &weaker_declaration,
        "rebinding-order-weaker-identity",
    )
    .expect("weaker identity");

    assert_eq!(
        exact.binding_identity(),
        right_decision.selected_candidate_identity().unwrap()
    );
    assert_eq!(weaker.binding_identity() != exact.binding_identity(), true);
    assert_eq!(
        left_decision.outcome_class(),
        RebindingOutcomeClass::ExactReattachment
    );
    assert_eq!(
        left_decision.outcome_class(),
        right_decision.outcome_class()
    );
    assert_eq!(
        left_decision.motion_posture(),
        MotionAwareBindingPosture::Unresolved
    );
    assert_eq!(
        left_decision.selected_candidate_identity(),
        right_decision.selected_candidate_identity()
    );
    assert_eq!(
        Some(exact.binding_identity()),
        left_decision.selected_candidate_identity()
    );
    assert_eq!(
        prior.prior_binding_identity() != exact.binding_identity(),
        true
    );
}
