use worth_spatial::facade::bindings::{
    author_primitive_rebinding_declaration, BindingContinuityClass, NeighborhoodBindingFamily,
};

use super::proof_fixture::{anchored_planar_surface, vertex_binding_declaration};
use crate::binding::tests::support::{
    admitted_rebinding_handle, anchored_surface_candidate_from_declaration,
    anchored_surface_prior_fact_from_declaration, certification_bundle_for_pair,
    rebinding_candidate_from_binding_declaration, rebinding_prior_fact_from_binding_declaration,
    rebinding_receipt_for_entry, replacement_neighborhood, scoped_branch_head_inspection_basis,
};

#[test]
fn binding_layer_certification_bundle_proves_determinism_replay_and_inspection_parity_under_hostile_order_variation(
) {
    let prior = anchored_planar_surface("face-old", [0.25, 0.5], 1.0);
    let exact = anchored_planar_surface("face-new-a", [0.25, 0.5], 1.0);
    let weaker = anchored_planar_surface("face-new-b", [0.25, 0.5], 2.0);
    let left_entry = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            anchored_surface_prior_fact_from_declaration(&prior, "closeout-left-prior"),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                vec![
                    anchored_surface_candidate_from_declaration(
                        "weaker",
                        &weaker,
                        "closeout-left-weaker",
                    )
                    .expect("weaker"),
                    anchored_surface_candidate_from_declaration(
                        "exact",
                        &exact,
                        "closeout-left-exact",
                    )
                    .expect("exact"),
                ],
            ),
        ),
    );
    let right_entry = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            anchored_surface_prior_fact_from_declaration(&prior, "closeout-right-prior"),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                vec![
                    anchored_surface_candidate_from_declaration(
                        "exact",
                        &exact,
                        "closeout-right-exact",
                    )
                    .expect("exact"),
                    anchored_surface_candidate_from_declaration(
                        "weaker",
                        &weaker,
                        "closeout-right-weaker",
                    )
                    .expect("weaker"),
                ],
            ),
        ),
    );
    let handle = admitted_rebinding_handle("rebinding-closeout-host-order");
    let branch_basis = scoped_branch_head_inspection_basis("branch:rebinding-closeout-host-order");
    let bundle = certification_bundle_for_pair(
        handle,
        branch_basis,
        left_entry.clone(),
        right_entry.clone(),
        "branch-evidence:left",
        "branch-evidence:right",
    );

    assert_eq!(
        bundle.deterministic_outcome_class(),
        worth_spatial::facade::bindings::RebindingOutcomeClass::Ambiguous
    );
    assert_eq!(
        bundle.deterministic_continuity_class(),
        BindingContinuityClass::Ambiguous
    );
    assert_eq!(
        bundle.binding_identity(),
        rebinding_receipt_for_entry(&left_entry, "closeout-left")
            .expect("left receipt")
            .prior_binding_identity()
    );
    assert!(bundle.selected_candidate_identity().is_none());
    assert!(!bundle.historical_digest().is_empty());
    assert!(!bundle.historical_inspection_digest().is_empty());
    assert!(!bundle.branch_local_digest().is_empty());
    assert!(!bundle.branch_local_inspection_digest().is_empty());
    assert!(!bundle.replay_digest().is_empty());
    assert_eq!(bundle.replay_ordinary_kind(), "ambiguous");
    assert!(!bundle.report_digest().is_empty());

    let denied_left = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            rebinding_prior_fact_from_binding_declaration(
                &vertex_binding_declaration("vertex-old"),
                "closeout-denied-left-prior",
            ),
            replacement_neighborhood(
                NeighborhoodBindingFamily::VertexGeometry,
                "vertex-old",
                vec![
                    rebinding_candidate_from_binding_declaration(
                        "a",
                        &vertex_binding_declaration("vertex-new-a"),
                        "closeout-denied-left-a",
                    )
                    .expect("a"),
                    rebinding_candidate_from_binding_declaration(
                        "b",
                        &vertex_binding_declaration("vertex-new-b"),
                        "closeout-denied-left-b",
                    )
                    .expect("b"),
                ],
            ),
        ),
    );
    let denied_right = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            rebinding_prior_fact_from_binding_declaration(
                &vertex_binding_declaration("vertex-old"),
                "closeout-denied-right-prior",
            ),
            replacement_neighborhood(
                NeighborhoodBindingFamily::VertexGeometry,
                "vertex-old",
                vec![
                    rebinding_candidate_from_binding_declaration(
                        "b",
                        &vertex_binding_declaration("vertex-new-b"),
                        "closeout-denied-right-b",
                    )
                    .expect("b"),
                    rebinding_candidate_from_binding_declaration(
                        "a",
                        &vertex_binding_declaration("vertex-new-a"),
                        "closeout-denied-right-a",
                    )
                    .expect("a"),
                ],
            ),
        ),
    );
    let denied_bundle = certification_bundle_for_pair(
        admitted_rebinding_handle("rebinding-closeout-host-order-denied"),
        scoped_branch_head_inspection_basis("branch:rebinding-closeout-host-order-denied"),
        denied_left,
        denied_right,
        "branch-evidence:denied-left",
        "branch-evidence:denied-right",
    );

    assert_eq!(
        denied_bundle.deterministic_outcome_class(),
        worth_spatial::facade::bindings::RebindingOutcomeClass::Unsupported
    );
    assert_eq!(denied_bundle.replay_ordinary_kind(), "unsupported");
}
