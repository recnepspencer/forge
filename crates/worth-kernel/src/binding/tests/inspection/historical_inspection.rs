use super::super::support::{
    admitted_rebinding_handle, anchored_surface_candidate_from_declaration,
    anchored_surface_declaration, anchored_surface_prior_fact_from_declaration,
    progress_rebinding_entry, rebind_surface_on_face, rebinding_receipt_for_entry,
    replacement_neighborhood, retained_digest_for_receipt, PrimitiveRebindingKernelQueryExt,
};
use forge_proof::TransitionOutcome;
use forge_query::facade::{
    admit_eligible_domain_capability_contribution,
    evaluate_requested_domain_capability_contribution,
    ForgeQueryDeclarationBoundContributionTarget, ForgeQueryDeclarationEntryContributionEvidence,
    ForgeQueryDeclarationEntryContributionEvidenceSet, ForgeQueryDeclarationEntryInspectionError,
    ForgeQueryDeclarationEntryInspectionInput, ForgeQueryDeclarationRouteIntent,
    ForgeQueryExplanationContributionAuthoring,
};
use worth_spatial::facade::bindings::author_primitive_rebinding_declaration;
use worth_spatial::facade::bindings::NeighborhoodBindingFamily;
use worth_spatial::facade::inspection::PrimitiveRebindingHistoricalInspectionError;

#[test]
fn historical_binding_inspection_reconstructs_transition_truth_without_live_state() {
    let prior = anchored_surface_declaration("face-old", "surface-alpha", [0.25, 0.5], 1.0);
    let exact = anchored_surface_declaration("face-new-a", "surface-beta", [0.25, 0.5], 1.0);
    let weaker = anchored_surface_declaration("face-new-b", "surface-gamma", [0.25, 0.5], 2.0);
    let historical_entry = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            anchored_surface_prior_fact_from_declaration(&prior, "historical-truth-prior"),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                vec![
                    anchored_surface_candidate_from_declaration(
                        "weaker",
                        &weaker,
                        "historical-truth-weaker",
                    )
                    .expect("weaker candidate"),
                    anchored_surface_candidate_from_declaration(
                        "exact",
                        &exact,
                        "historical-truth-exact",
                    )
                    .expect("exact candidate"),
                ],
            ),
        ),
    );
    let handle = admitted_rebinding_handle("phase-thirteen-history");
    let progression = progress_rebinding_entry(&historical_entry, &handle);
    let retained_subject = handle.orchestrate_envelope_from_progressed_checked(progression.clone());
    let historical = historical_entry
        .historical_inspection_with_query(&handle, retained_subject)
        .expect("historical inspection");
    let direct = rebind_surface_on_face(
        anchored_surface_prior_fact_from_declaration(&prior, "historical-inspection-direct-prior"),
        replacement_neighborhood(
            NeighborhoodBindingFamily::FaceSurfacePointAnchor,
            "face-old",
            vec![
                anchored_surface_candidate_from_declaration(
                    "weaker",
                    &weaker,
                    "historical-direct-weaker",
                )
                .expect("weaker candidate"),
                anchored_surface_candidate_from_declaration(
                    "exact",
                    &exact,
                    "historical-direct-exact",
                )
                .expect("exact candidate"),
            ],
        ),
    )
    .expect("direct decision");

    let mutated_live = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            anchored_surface_prior_fact_from_declaration(&prior, "historical-mutated-prior"),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                vec![anchored_surface_candidate_from_declaration(
                    "weaker",
                    &weaker,
                    "historical-mutated-weaker",
                )
                .expect("weaker candidate")],
            ),
        ),
    );
    let mutated_live_receipt =
        rebinding_receipt_for_entry(&mutated_live, "historical-mutated-live")
            .expect("mutated live receipt");

    assert_eq!(historical.receipt().outcome_class(), direct.outcome_class());
    assert_eq!(
        historical.receipt().continuity_class(),
        direct.continuity_class()
    );
    assert_eq!(
        historical.receipt().selected_candidate_identity(),
        direct.selected_candidate_identity()
    );
    assert_eq!(
        historical.receipt().motion_posture(),
        direct.motion_posture()
    );
    assert_eq!(
        historical.inspection().progression_digest(),
        Some(progression.progression_digest())
    );
    assert_ne!(
        historical.receipt().outcome_class(),
        mutated_live_receipt.outcome_class()
    );
    assert_ne!(
        historical.historical_digest(),
        retained_digest_for_receipt(&mutated_live_receipt)
    );
}

#[test]
fn historical_binding_inspection_rejects_wrong_handle_or_truncated_basis_before_partial_interpretation(
) {
    let prior = anchored_surface_declaration("face-old", "surface-alpha", [0.25, 0.5], 1.0);
    let exact = anchored_surface_declaration("face-new-a", "surface-beta", [0.25, 0.5], 1.0);
    let weaker = anchored_surface_declaration("face-new-b", "surface-gamma", [0.25, 0.5], 2.0);
    let canonical_entry = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            anchored_surface_prior_fact_from_declaration(&prior, "historical-basis-prior"),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                vec![
                    anchored_surface_candidate_from_declaration(
                        "exact",
                        &exact,
                        "historical-basis-exact",
                    )
                    .expect("exact candidate"),
                    anchored_surface_candidate_from_declaration(
                        "weaker",
                        &weaker,
                        "historical-basis-weaker",
                    )
                    .expect("weaker candidate"),
                ],
            ),
        ),
    );
    let handle = admitted_rebinding_handle("phase-thirteen-basis");
    let progression = progress_rebinding_entry(&canonical_entry, &handle);
    let retained_subject = handle.orchestrate_envelope_from_progressed_checked(progression.clone());
    let canonical = canonical_entry
        .historical_inspection_with_query(&handle, retained_subject)
        .expect("canonical historical inspection");
    assert_eq!(
        canonical.receipt().outcome_class(),
        canonical.source().receipt().outcome_class()
    );
    assert_eq!(
        canonical.inspection().progression_digest(),
        Some(progression.progression_digest())
    );

    let truncated_subject = handle.orchestrate_envelope_from_progressed_checked_with_intent(
        progression,
        ForgeQueryDeclarationRouteIntent::DeferredRouting,
    );
    let truncated = canonical_entry.historical_inspection_with_query(&handle, truncated_subject);

    assert!(matches!(
        truncated,
        Err(PrimitiveRebindingHistoricalInspectionError::TruncatedRetainedBasis { .. })
    ));

    let source_handle = admitted_rebinding_handle("phase-thirteen-source");
    let source_progression = progress_rebinding_entry(&canonical_entry, &source_handle);
    let wrong_handle_subject =
        source_handle.orchestrate_envelope_from_progressed_checked(source_progression);
    let wrong_handle =
        canonical_entry.historical_inspection_with_query(&handle, wrong_handle_subject);

    assert!(matches!(
        wrong_handle,
        Err(PrimitiveRebindingHistoricalInspectionError::RetainedBasisMismatch { .. })
            | Err(PrimitiveRebindingHistoricalInspectionError::Inspection(
                ForgeQueryDeclarationEntryInspectionError::RetainedSubjectMismatch { .. },
            ))
    ));
}

#[test]
fn historical_inspection_digest_is_stable_under_equivalent_retained_artifact_ordering() {
    let prior = anchored_surface_declaration("face-old", "surface-alpha", [0.25, 0.5], 1.0);
    let exact = anchored_surface_declaration("face-new-a", "surface-beta", [0.25, 0.5], 1.0);
    let weaker = anchored_surface_declaration("face-new-b", "surface-gamma", [0.25, 0.5], 2.0);
    let left = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            anchored_surface_prior_fact_from_declaration(&prior, "historical-left-prior"),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                vec![
                    anchored_surface_candidate_from_declaration(
                        "weaker",
                        &weaker,
                        "historical-left-weaker",
                    )
                    .expect("weaker candidate"),
                    anchored_surface_candidate_from_declaration(
                        "exact",
                        &exact,
                        "historical-left-exact",
                    )
                    .expect("exact candidate"),
                ],
            ),
        ),
    );
    let right = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            anchored_surface_prior_fact_from_declaration(&prior, "historical-right-prior"),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                vec![
                    anchored_surface_candidate_from_declaration(
                        "exact",
                        &exact,
                        "historical-right-exact",
                    )
                    .expect("exact candidate"),
                    anchored_surface_candidate_from_declaration(
                        "weaker",
                        &weaker,
                        "historical-right-weaker",
                    )
                    .expect("weaker candidate"),
                ],
            ),
        ),
    );
    let handle = admitted_rebinding_handle("phase-thirteen-ordering");
    let left_historical = left
        .historical_inspection_with_query(
            &handle,
            handle.orchestrate_envelope_from_progressed_checked(progress_rebinding_entry(
                &left, &handle,
            )),
        )
        .expect("left historical inspection");
    let right_historical = right
        .historical_inspection_with_query(
            &handle,
            handle.orchestrate_envelope_from_progressed_checked(progress_rebinding_entry(
                &right, &handle,
            )),
        )
        .expect("right historical inspection");

    assert_eq!(
        left_historical.receipt().outcome_class(),
        right_historical.receipt().outcome_class()
    );
    assert_eq!(
        left_historical.receipt().continuity_class(),
        right_historical.receipt().continuity_class()
    );
    assert_eq!(
        left_historical.receipt().selected_candidate_identity(),
        right_historical.receipt().selected_candidate_identity()
    );
    assert_eq!(
        left_historical.historical_digest(),
        right_historical.historical_digest()
    );
    assert_eq!(
        left_historical.inspection().inspection_digest(),
        right_historical.inspection().inspection_digest()
    );
}

#[test]
fn historical_binding_inspection_truth_is_not_perturbed_by_admitted_explanation_richness() {
    let prior = anchored_surface_declaration("face-old", "surface-alpha", [0.25, 0.5], 1.0);
    let exact = anchored_surface_declaration("face-new-a", "surface-beta", [0.25, 0.5], 1.0);
    let entry = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            anchored_surface_prior_fact_from_declaration(&prior, "historical-richness-prior"),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                vec![anchored_surface_candidate_from_declaration(
                    "exact",
                    &exact,
                    "historical-richness-exact",
                )
                .expect("exact candidate")],
            ),
        ),
    );
    let handle = admitted_rebinding_handle("phase-thirteen-richness");
    let baseline = entry
        .historical_inspection_with_query(
            &handle,
            handle.orchestrate_envelope_from_progressed_checked(progress_rebinding_entry(
                &entry, &handle,
            )),
        )
        .expect("baseline historical inspection");
    let declaration_target =
        ForgeQueryDeclarationBoundContributionTarget::for_canonical_declaration(
            &handle
                .declare(entry.clone())
                .expect("canonical declaration"),
        );
    let enriched = handle
        .inspect_declaration_entry(
            ForgeQueryDeclarationEntryInspectionInput::envelope_checked(
                handle.orchestrate_envelope_from_progressed_checked(progress_rebinding_entry(
                    &entry, &handle,
                )),
            )
            .with_contribution_evidence(
                ForgeQueryDeclarationEntryContributionEvidenceSet::new(vec![
                    ForgeQueryDeclarationEntryContributionEvidence::from(
                        admitted_declaration_explanation(
                            declaration_target,
                            "explain",
                            "retained context",
                        ),
                    ),
                ]),
            ),
        )
        .unwrap_or_else(|error| panic!("enriched retained inspection: {}", error.reason()));

    assert_eq!(
        baseline.inspection().matching_row_digests(),
        enriched.matching_row_digests()
    );
    assert_ne!(
        baseline.inspection().inspection_digest(),
        enriched.inspection_digest()
    );
}

fn admitted_declaration_explanation(
    declaration_target: ForgeQueryDeclarationBoundContributionTarget,
    semantic_code: &str,
    detail: &str,
) -> forge_query::facade::ForgeQueryAdmittedExplanationContribution<
    ForgeQueryDeclarationBoundContributionTarget,
> {
    let requested =
        ForgeQueryExplanationContributionAuthoring::requires_context(semantic_code, detail)
            .bind_to_declaration_target(declaration_target);
    let eligible = match evaluate_requested_domain_capability_contribution(requested) {
        TransitionOutcome::Success(value) => value,
        _ => panic!("expected eligible explanation contribution"),
    };
    match admit_eligible_domain_capability_contribution(eligible) {
        TransitionOutcome::Success(value) => value,
        _ => panic!("expected admitted explanation contribution"),
    }
}
