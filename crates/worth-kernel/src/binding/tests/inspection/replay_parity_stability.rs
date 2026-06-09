use super::super::support::{
    admitted_rebinding_handle, anchored_surface_candidate_from_declaration,
    anchored_surface_declaration, anchored_surface_prior_fact_from_declaration,
    progress_rebinding_entry, rebinding_receipt_for_entry, replacement_neighborhood,
    retained_digest_for_receipt, PrimitiveRebindingKernelQueryExt,
};
use worth_spatial::facade::bindings::{
    author_primitive_rebinding_declaration, BindingContinuityClass, NeighborhoodBindingFamily,
};
use worth_spatial::facade::inspection::PrimitiveRebindingReplaySource;

#[test]
fn replay_parity_does_not_depend_on_live_runtime_memory_or_host_order_accident() {
    let prior = anchored_surface_declaration("face-old", "surface-alpha", [0.25, 0.5], 1.0);
    let exact = anchored_surface_declaration("face-new-a", "surface-beta", [0.25, 0.5], 1.0);
    let weaker = anchored_surface_declaration("face-new-b", "surface-gamma", [0.25, 0.5], 2.0);
    let left = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            anchored_surface_prior_fact_from_declaration(&prior, "replay-stability-left-prior"),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                vec![
                    anchored_surface_candidate_from_declaration(
                        "weaker",
                        &weaker,
                        "replay-stability-left-weaker",
                    )
                    .expect("weaker candidate"),
                    anchored_surface_candidate_from_declaration(
                        "exact",
                        &exact,
                        "replay-stability-left-exact",
                    )
                    .expect("exact candidate"),
                ],
            ),
        ),
    );
    let right = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            anchored_surface_prior_fact_from_declaration(&prior, "replay-stability-right-prior"),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                vec![
                    anchored_surface_candidate_from_declaration(
                        "exact",
                        &exact,
                        "replay-stability-right-exact",
                    )
                    .expect("exact candidate"),
                    anchored_surface_candidate_from_declaration(
                        "weaker",
                        &weaker,
                        "replay-stability-right-weaker",
                    )
                    .expect("weaker candidate"),
                ],
            ),
        ),
    );
    let mutated_live = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            anchored_surface_prior_fact_from_declaration(&prior, "replay-stability-mutated-prior"),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                vec![anchored_surface_candidate_from_declaration(
                    "weaker",
                    &weaker,
                    "replay-stability-mutated-weaker",
                )
                .expect("weaker candidate")],
            ),
        ),
    );
    let mutated_live = rebinding_receipt_for_entry(&mutated_live, "phase-fifteen-mutated-live")
        .expect("mutated live decision");
    let handle = admitted_rebinding_handle("phase-fifteen-live-independence");

    let parity = left
        .replay_parity_with_query(
            &handle,
            PrimitiveRebindingReplaySource::Historical(
                left.historical_inspection_with_query(
                    &handle,
                    handle.orchestrate_envelope_from_progressed_checked(progress_rebinding_entry(
                        &left, &handle,
                    )),
                )
                .expect("left historical inspection")
                .retained_fact_receipt(),
            ),
            PrimitiveRebindingReplaySource::Historical(
                right
                    .historical_inspection_with_query(
                        &handle,
                        handle.orchestrate_envelope_from_progressed_checked(
                            progress_rebinding_entry(&right, &handle),
                        ),
                    )
                    .expect("right historical inspection")
                    .retained_fact_receipt(),
            ),
        )
        .expect("replay parity");
    let left_decision =
        rebinding_receipt_for_entry(&left, "phase-fifteen-left").expect("left decision");
    let right_decision =
        rebinding_receipt_for_entry(&right, "phase-fifteen-right").expect("right decision");

    assert_eq!(
        parity.binding_identity(),
        left_decision.prior_binding_identity()
    );
    assert_eq!(
        parity.anchor_identity(),
        left_decision.prior_site_identity()
    );
    assert_eq!(parity.outcome_class(), left_decision.outcome_class());
    assert_eq!(parity.continuity_class(), left_decision.continuity_class());
    assert_eq!(
        parity.selected_candidate_identity(),
        left_decision.selected_candidate_identity()
    );
    assert_eq!(
        parity.selected_candidate_label(),
        left_decision.selected_candidate_label()
    );
    assert_eq!(
        parity.binding_identity(),
        right_decision.prior_binding_identity()
    );
    assert_eq!(parity.ordinary_kind(), "ambiguous");
    assert_eq!(
        parity.next_step(),
        Some(forge_query::facade::ForgeQueryOrdinaryNextStep::NarrowInput)
    );
    assert_ne!(
        parity.replay_digest(),
        retained_digest_for_receipt(&mutated_live)
    );
    assert_eq!(
        mutated_live.continuity_class(),
        BindingContinuityClass::CorrespondenceOnly
    );
}
