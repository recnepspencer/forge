use super::super::support::{
    admitted_rebinding_handle, anchored_surface_candidate_from_declaration,
    anchored_surface_declaration, anchored_surface_prior_fact_from_declaration,
    progress_rebinding_entry, replacement_neighborhood, PrimitiveRebindingKernelQueryExt,
};
use forge_query::facade::{
    admit_basis_capability, evaluate_basis_inspection_eligibility, normalize_raw_basis_intent,
    scope_basis_for_inspection, LowerRuntimeBasisEvidence, RawBasisIntent, ScopedInspectionBasis,
};
use worth_spatial::facade::bindings::{
    author_primitive_rebinding_declaration, BindingContinuityClass, NeighborhoodBindingFamily,
};
use worth_spatial::facade::inspection::{
    PrimitiveRebindingReplayParityError, PrimitiveRebindingReplaySource,
};

#[test]
fn binding_and_rebinding_replay_is_identical_across_equivalent_retained_histories() {
    let prior = anchored_surface_declaration("face-old", "surface-alpha", [0.25, 0.5], 1.0);
    let exact = anchored_surface_declaration("face-new-a", "surface-beta", [0.25, 0.5], 1.0);
    let weaker = anchored_surface_declaration("face-new-b", "surface-gamma", [0.25, 0.5], 2.0);
    let historical_entry = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            anchored_surface_prior_fact_from_declaration(
                &prior,
                "replay-equivalent-historical-prior",
            ),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                vec![
                    anchored_surface_candidate_from_declaration(
                        "weaker",
                        &weaker,
                        "replay-equivalent-historical-weaker",
                    )
                    .expect("weaker candidate"),
                    anchored_surface_candidate_from_declaration(
                        "exact",
                        &exact,
                        "replay-equivalent-historical-exact",
                    )
                    .expect("exact candidate"),
                ],
            ),
        ),
    );
    let branch_entry = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            anchored_surface_prior_fact_from_declaration(&prior, "replay-equivalent-branch-prior"),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                vec![
                    anchored_surface_candidate_from_declaration(
                        "exact",
                        &exact,
                        "replay-equivalent-branch-exact",
                    )
                    .expect("exact candidate"),
                    anchored_surface_candidate_from_declaration(
                        "weaker",
                        &weaker,
                        "replay-equivalent-branch-weaker",
                    )
                    .expect("weaker candidate"),
                ],
            ),
        ),
    );
    let handle = admitted_rebinding_handle("replay-parity-equivalent");
    let historical = historical_entry
        .historical_inspection_with_query(
            &handle,
            handle.orchestrate_envelope_from_progressed_checked(progress_rebinding_entry(
                &historical_entry,
                &handle,
            )),
        )
        .expect("historical inspection");
    let branch_basis = scoped_branch_head_inspection_basis("branch:equivalent");
    let branch_local = branch_entry
        .branch_local_inspection_with_query(
            &handle,
            &branch_basis,
            branch_basis_evidence(&branch_basis, "branch-evidence:equivalent"),
            handle.orchestrate_envelope_from_progressed_checked(progress_rebinding_entry(
                &branch_entry,
                &handle,
            )),
        )
        .expect("branch-local inspection");
    let historical_receipt = historical.receipt().clone();
    let branch_receipt = branch_local.receipt().clone();

    let parity = historical_entry
        .replay_parity_with_query(
            &handle,
            PrimitiveRebindingReplaySource::Historical(historical.retained_fact_receipt()),
            PrimitiveRebindingReplaySource::BranchLocal(branch_local.retained_fact_receipt()),
        )
        .expect("replay parity");

    assert_eq!(parity.left_source_kind(), "historical");
    assert_eq!(parity.right_source_kind(), "branch_local");
    assert_eq!(
        parity.binding_identity(),
        historical_receipt.prior_binding_identity()
    );
    assert_eq!(
        parity.anchor_identity(),
        historical_receipt.prior_site_identity()
    );
    assert_eq!(parity.outcome_class(), historical_receipt.outcome_class());
    assert_eq!(
        parity.continuity_class(),
        historical_receipt.continuity_class()
    );
    assert_eq!(
        parity.selected_candidate_identity(),
        historical_receipt.selected_candidate_identity()
    );
    assert_eq!(
        parity.selected_candidate_label(),
        historical_receipt.selected_candidate_label()
    );
    assert_eq!(
        parity.binding_identity(),
        branch_receipt.prior_binding_identity()
    );
    assert_eq!(
        parity.anchor_identity(),
        branch_receipt.prior_site_identity()
    );
    assert_eq!(parity.ordinary_kind(), "ambiguous");
    assert_eq!(
        parity.next_step(),
        Some(forge_query::facade::ForgeQueryOrdinaryNextStep::NarrowInput)
    );
    assert_eq!(
        parity.outcome_class(),
        worth_spatial::facade::bindings::RebindingOutcomeClass::Ambiguous
    );
    assert_eq!(parity.continuity_class(), BindingContinuityClass::Ambiguous);
    assert!(parity.selected_candidate_identity().is_none());
    assert!(parity.selected_candidate_label().is_none());
    assert_eq!(parity.unsupported_reason(), "None");
    assert!(!parity.replay_digest().is_empty());
}

#[test]
fn replay_parity_fails_loudly_when_retained_identity_or_explanation_basis_is_semantically_different(
) {
    let prior = anchored_surface_declaration("face-old", "surface-alpha", [0.25, 0.5], 1.0);
    let exact = anchored_surface_declaration("face-new-a", "surface-beta", [0.25, 0.5], 1.0);
    let retained_identity_entry = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            anchored_surface_prior_fact_from_declaration(&prior, "replay-mismatch-retained-prior"),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                vec![anchored_surface_candidate_from_declaration(
                    "exact",
                    &exact,
                    "replay-mismatch-retained-exact",
                )
                .expect("exact candidate")],
            ),
        ),
    );
    let different_identity_prior =
        anchored_surface_declaration("face-other", "surface-delta", [0.25, 0.5], 1.0);
    let different_identity_entry = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            anchored_surface_prior_fact_from_declaration(
                &different_identity_prior,
                "replay-mismatch-different-prior",
            ),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-other",
                vec![anchored_surface_candidate_from_declaration(
                    "exact",
                    &exact,
                    "replay-mismatch-different-exact",
                )
                .expect("exact candidate")],
            ),
        ),
    );
    let explanation_left = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            anchored_surface_prior_fact_from_declaration(&prior, "replay-mismatch-left-prior"),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                vec![anchored_surface_candidate_from_declaration(
                    "exact",
                    &exact,
                    "replay-mismatch-left-exact",
                )
                .expect("exact candidate")],
            ),
        ),
    );
    let explanation_right = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            anchored_surface_prior_fact_from_declaration(&prior, "replay-mismatch-right-prior"),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                vec![anchored_surface_candidate_from_declaration(
                    "preferred",
                    &exact,
                    "replay-mismatch-right-preferred",
                )
                .expect("exact candidate")],
            ),
        ),
    );
    let handle = admitted_rebinding_handle("replay-parity-mismatch");

    let retained_identity_result =
        retained_identity_entry.replay_parity_with_query(
            &handle,
            PrimitiveRebindingReplaySource::Historical(
                retained_identity_entry
                    .historical_inspection_with_query(
                        &handle,
                        handle.orchestrate_envelope_from_progressed_checked(
                            progress_rebinding_entry(&retained_identity_entry, &handle),
                        ),
                    )
                    .expect("left historical inspection")
                    .retained_fact_receipt(),
            ),
            PrimitiveRebindingReplaySource::Historical(
                different_identity_entry
                    .historical_inspection_with_query(
                        &handle,
                        handle.orchestrate_envelope_from_progressed_checked(
                            progress_rebinding_entry(&different_identity_entry, &handle),
                        ),
                    )
                    .expect("right historical inspection")
                    .retained_fact_receipt(),
            ),
        );
    assert!(matches!(
        retained_identity_result,
        Err(PrimitiveRebindingReplayParityError::RetainedIdentityMismatch { .. })
    ));

    let explanation_result =
        explanation_left.replay_parity_with_query(
            &handle,
            PrimitiveRebindingReplaySource::Historical(
                explanation_left
                    .historical_inspection_with_query(
                        &handle,
                        handle.orchestrate_envelope_from_progressed_checked(
                            progress_rebinding_entry(&explanation_left, &handle),
                        ),
                    )
                    .expect("left explanation inspection")
                    .retained_fact_receipt(),
            ),
            PrimitiveRebindingReplaySource::Historical(
                explanation_right
                    .historical_inspection_with_query(
                        &handle,
                        handle.orchestrate_envelope_from_progressed_checked(
                            progress_rebinding_entry(&explanation_right, &handle),
                        ),
                    )
                    .expect("right explanation inspection")
                    .retained_fact_receipt(),
            ),
        );
    assert!(matches!(
        explanation_result,
        Err(PrimitiveRebindingReplayParityError::ExplanationBasisMismatch { .. })
    ));
}

fn scoped_branch_head_inspection_basis(branch_identity: &str) -> ScopedInspectionBasis {
    let normalized = normalize_raw_basis_intent(
        RawBasisIntent::BranchHead {
            branch_identity: branch_identity.to_string(),
            accessible: true,
        },
        "inspection",
    )
    .expect("branch-head inspection should normalize");
    let eligibility = evaluate_basis_inspection_eligibility(normalized)
        .expect("branch-head inspection should be eligible");

    scope_basis_for_inspection(admit_basis_capability(eligibility))
}

fn branch_basis_evidence(
    scoped_basis: &ScopedInspectionBasis,
    evidence_digest: &str,
) -> LowerRuntimeBasisEvidence {
    LowerRuntimeBasisEvidence::from_relational_facade(
        scoped_basis
            .expected_lower_runtime_binding_digest()
            .expect("branch basis digest"),
        evidence_digest,
        1,
    )
}
