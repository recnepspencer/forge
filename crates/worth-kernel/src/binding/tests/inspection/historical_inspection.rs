use super::super::support::{
    admitted_rebinding_handle, anchored_surface, progress_rebinding_entry,
    replacement_neighborhood, retained_digest_for_decision,
};
use crate::{
    binding::rebinding::PrimitiveRebindingHistoricalInspectionError,
    facade::authoring::binding::{
        author_primitive_rebinding_declaration, AuthorPrimitiveRebindingIntent,
    },
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
use worth_spatial::facade::bindings::{
    rebind_surface_on_face, NeighborhoodBindingFamily, ReplacementCandidate,
    SpatialAdmittedPrimitiveBinding,
};

#[test]
fn historical_binding_inspection_reconstructs_transition_truth_without_live_state() {
    let prior = anchored_surface("face-old", "surface-alpha", [0.25, 0.5], 1.0);
    let exact = anchored_surface("face-new-a", "surface-beta", [0.25, 0.5], 1.0);
    let weaker = anchored_surface("face-new-b", "surface-gamma", [0.25, 0.5], 2.0);
    let historical_entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(prior.clone()),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                vec![
                    ReplacementCandidate::new(
                        "weaker",
                        SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(weaker.clone()),
                    )
                    .expect("weaker candidate"),
                    ReplacementCandidate::new(
                        "exact",
                        SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(exact.clone()),
                    )
                    .expect("exact candidate"),
                ],
            ),
        ),
    );
    let handle = admitted_rebinding_handle("phase-thirteen-history");
    let progression = progress_rebinding_entry(&historical_entry, &handle);
    let retained_subject = ForgeQueryDeclarationEntryInspectionInput::envelope_checked(
        handle.orchestrate_envelope_from_progressed_checked(progression.clone()),
    );
    let historical = historical_entry
        .historical_inspection_with_query(&handle, retained_subject)
        .expect("historical inspection");
    let direct = rebind_surface_on_face(
        SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(prior.clone()),
        replacement_neighborhood(
            NeighborhoodBindingFamily::FaceSurfacePointAnchor,
            "face-old",
            vec![
                ReplacementCandidate::new(
                    "weaker",
                    SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(weaker.clone()),
                )
                .expect("weaker candidate"),
                ReplacementCandidate::new(
                    "exact",
                    SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(exact.clone()),
                )
                .expect("exact candidate"),
            ],
        ),
    )
    .expect("direct decision");

    let mutated_live = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(prior.clone()),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                vec![ReplacementCandidate::new(
                    "weaker",
                    SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(weaker),
                )
                .expect("weaker candidate")],
            ),
        ),
    )
    .admit()
    .expect("mutated live decision");

    assert_eq!(
        historical.decision().outcome_class(),
        direct.outcome_class()
    );
    assert_eq!(
        historical.decision().explanation().continuity_class(),
        direct.explanation().continuity_class()
    );
    assert_eq!(
        historical
            .decision()
            .explanation()
            .selected_candidate_identity(),
        direct.explanation().selected_candidate_identity()
    );
    assert_eq!(
        historical.decision().explanation().motion_posture(),
        direct.explanation().motion_posture()
    );
    assert_eq!(
        historical.inspection().progression_digest(),
        Some(progression.progression_digest())
    );
    assert_ne!(
        historical.decision().outcome_class(),
        mutated_live.outcome_class()
    );
    assert_ne!(
        historical.historical_digest(),
        retained_digest_for_decision(&mutated_live)
    );
}

#[test]
fn historical_binding_inspection_rejects_wrong_or_truncated_basis_before_partial_interpretation() {
    let prior = anchored_surface("face-old", "surface-alpha", [0.25, 0.5], 1.0);
    let exact = anchored_surface("face-new-a", "surface-beta", [0.25, 0.5], 1.0);
    let weaker = anchored_surface("face-new-b", "surface-gamma", [0.25, 0.5], 2.0);
    let canonical_entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(prior.clone()),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                vec![
                    ReplacementCandidate::new(
                        "exact",
                        SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(exact.clone()),
                    )
                    .expect("exact candidate"),
                    ReplacementCandidate::new(
                        "weaker",
                        SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(weaker.clone()),
                    )
                    .expect("weaker candidate"),
                ],
            ),
        ),
    );
    let mismatched_entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(prior),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                vec![ReplacementCandidate::new(
                    "weaker",
                    SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(weaker),
                )
                .expect("weaker candidate")],
            ),
        ),
    );
    let handle = admitted_rebinding_handle("phase-thirteen-basis");
    let progression = progress_rebinding_entry(&canonical_entry, &handle);
    let retained_subject = ForgeQueryDeclarationEntryInspectionInput::envelope_checked(
        handle.orchestrate_envelope_from_progressed_checked(progression.clone()),
    );
    let mismatch = mismatched_entry.historical_inspection_with_query(&handle, retained_subject);

    assert!(matches!(
        mismatch,
        Err(PrimitiveRebindingHistoricalInspectionError::RetainedBasisMismatch { .. })
    ));

    let truncated_subject = ForgeQueryDeclarationEntryInspectionInput::envelope_checked(
        handle.orchestrate_envelope_from_progressed_checked_with_intent(
            progression,
            ForgeQueryDeclarationRouteIntent::DeferredRouting,
        ),
    );
    let truncated = canonical_entry.historical_inspection_with_query(&handle, truncated_subject);

    assert!(matches!(
        truncated,
        Err(PrimitiveRebindingHistoricalInspectionError::TruncatedRetainedBasis { .. })
    ));

    let source_handle = admitted_rebinding_handle("phase-thirteen-source");
    let source_progression = progress_rebinding_entry(&canonical_entry, &source_handle);
    let wrong_handle_subject = ForgeQueryDeclarationEntryInspectionInput::envelope_checked(
        source_handle.orchestrate_envelope_from_progressed_checked(source_progression),
    );
    let wrong_handle =
        canonical_entry.historical_inspection_with_query(&handle, wrong_handle_subject);

    match wrong_handle {
        Err(PrimitiveRebindingHistoricalInspectionError::Inspection(
            ForgeQueryDeclarationEntryInspectionError::RetainedSubjectMismatch { reason, .. },
        )) => assert!(reason.contains("same admitted handle")),
        _ => panic!("expected retained subject mismatch"),
    }
}

#[test]
fn historical_inspection_digest_is_stable_under_equivalent_retained_artifact_ordering() {
    let prior = anchored_surface("face-old", "surface-alpha", [0.25, 0.5], 1.0);
    let exact = anchored_surface("face-new-a", "surface-beta", [0.25, 0.5], 1.0);
    let weaker = anchored_surface("face-new-b", "surface-gamma", [0.25, 0.5], 2.0);
    let left = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(prior.clone()),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                vec![
                    ReplacementCandidate::new(
                        "weaker",
                        SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(weaker.clone()),
                    )
                    .expect("weaker candidate"),
                    ReplacementCandidate::new(
                        "exact",
                        SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(exact.clone()),
                    )
                    .expect("exact candidate"),
                ],
            ),
        ),
    );
    let right = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(prior),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                vec![
                    ReplacementCandidate::new(
                        "exact",
                        SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(exact),
                    )
                    .expect("exact candidate"),
                    ReplacementCandidate::new(
                        "weaker",
                        SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(weaker),
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
            ForgeQueryDeclarationEntryInspectionInput::envelope_checked(
                handle.orchestrate_envelope_from_progressed_checked(progress_rebinding_entry(
                    &left, &handle,
                )),
            ),
        )
        .expect("left historical inspection");
    let right_historical = right
        .historical_inspection_with_query(
            &handle,
            ForgeQueryDeclarationEntryInspectionInput::envelope_checked(
                handle.orchestrate_envelope_from_progressed_checked(progress_rebinding_entry(
                    &right, &handle,
                )),
            ),
        )
        .expect("right historical inspection");

    assert_eq!(
        left_historical.decision().outcome_class(),
        right_historical.decision().outcome_class()
    );
    assert_eq!(
        left_historical.decision().explanation().continuity_class(),
        right_historical.decision().explanation().continuity_class()
    );
    assert_eq!(
        left_historical
            .decision()
            .explanation()
            .selected_candidate_identity(),
        right_historical
            .decision()
            .explanation()
            .selected_candidate_identity()
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
    let prior = anchored_surface("face-old", "surface-alpha", [0.25, 0.5], 1.0);
    let exact = anchored_surface("face-new-a", "surface-beta", [0.25, 0.5], 1.0);
    let entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(prior),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                vec![ReplacementCandidate::new(
                    "exact",
                    SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(exact),
                )
                .expect("exact candidate")],
            ),
        ),
    );
    let handle = admitted_rebinding_handle("phase-thirteen-richness");
    let baseline = entry
        .historical_inspection_with_query(
            &handle,
            ForgeQueryDeclarationEntryInspectionInput::envelope_checked(
                handle.orchestrate_envelope_from_progressed_checked(progress_rebinding_entry(
                    &entry, &handle,
                )),
            ),
        )
        .expect("baseline historical inspection");
    let declaration_target =
        ForgeQueryDeclarationBoundContributionTarget::for_canonical_declaration(
            &handle
                .declare(entry.clone())
                .expect("canonical declaration"),
        );
    let enriched = entry
        .historical_inspection_with_query(
            &handle,
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
        .expect("enriched historical inspection");

    assert_eq!(
        baseline.decision().outcome_class(),
        enriched.decision().outcome_class()
    );
    assert_eq!(
        baseline.decision().explanation().continuity_class(),
        enriched.decision().explanation().continuity_class()
    );
    assert_eq!(baseline.historical_digest(), enriched.historical_digest());
    assert_eq!(
        baseline.inspection().matching_row_digests(),
        enriched.inspection().matching_row_digests()
    );
    assert_ne!(
        baseline.inspection().inspection_digest(),
        enriched.inspection().inspection_digest()
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
