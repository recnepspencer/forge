use forge_query::facade::{
    ForgeQueryContinuationExecutionOutcome, ForgeQueryDeclarationBridgeRoutingSupportStatus,
    ForgeQueryDeclarationCanonicalValue, ForgeQueryDeclarationInput, ForgeQueryOrdinaryOutcome,
    ForgeQueryPreparedContinuationOutcome, ForgeQuerySignalCompatibilityOrchestrationOutcome,
};
use worth_spatial::facade::bindings::{
    author_primitive_binding_declaration, author_primitive_rebinding_declaration,
    primitive_rebinding_mutation_evidence, primitive_rebinding_retained_fact_source,
    AuthorPrimitiveBindingIntent, NeighborhoodBindingFamily, ReplacementCandidateSet,
    VertexBindingSite, VertexGeometryBindingSpec, VertexGeometryProvenanceKind,
    VertexToleranceRegime,
};
use worth_spatial::facade::continuation::{
    primitive_rebinding_continuation_target, primitive_rebinding_signal_workflow,
};
use worth_spatial::facade::neighborhood::{
    primitive_rebinding_neighborhood_replacement_facts,
    primitive_rebinding_neighborhood_replacement_source, topology_neighborhood_replacement_entry,
    TopologyNeighborhoodReplacementScope,
};
use worth_spatial::facade::projection::{
    geometry_projection_consumption_entry, primitive_rebinding_geometry_projection_consumption,
};
use worth_spatial::facade::recovery::{
    geometry_recovery_action_entry, primitive_rebinding_geometry_recovery_action,
    GeometryRecoveryAction, GeometryRecoveryTargetScope,
};

use super::support::{
    admitted_rebinding_handle, anchored_surface_candidate_from_declaration,
    anchored_surface_declaration, anchored_surface_prior_fact_from_declaration,
    branch_local_rebinding_inspection, canonical_geometry, certification_bundle_for_pair,
    historical_rebinding_inspection, orthotope_contract,
    rebinding_candidate_from_binding_declaration, rebinding_prior_fact_from_binding_declaration,
    replacement_neighborhood, scoped_branch_head_inspection_basis,
};

#[test]
fn geometry_hard_break_closeout_keeps_one_admitted_query_native_runtime_story() {
    let prior = anchored_surface_declaration("face-old", "surface-alpha", [0.25, 0.5], 1.0);
    let exact = anchored_surface_declaration("face-new-a", "surface-beta", [0.25, 0.5], 1.0);
    let weaker = anchored_surface_declaration("face-new-b", "surface-gamma", [0.25, 0.5], 2.0);
    let left = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            anchored_surface_prior_fact_from_declaration(&prior, "phase-nine-left-prior"),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                vec![
                    anchored_surface_candidate_from_declaration(
                        "weaker",
                        &weaker,
                        "phase-nine-left-weaker",
                    )
                    .expect("weaker candidate"),
                    anchored_surface_candidate_from_declaration(
                        "exact",
                        &exact,
                        "phase-nine-left-exact",
                    )
                    .expect("exact candidate"),
                ],
            ),
        ),
    );
    let right = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            anchored_surface_prior_fact_from_declaration(&prior, "phase-nine-right-prior"),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                vec![
                    anchored_surface_candidate_from_declaration(
                        "exact",
                        &exact,
                        "phase-nine-right-exact",
                    )
                    .expect("exact candidate"),
                    anchored_surface_candidate_from_declaration(
                        "weaker",
                        &weaker,
                        "phase-nine-right-weaker",
                    )
                    .expect("weaker candidate"),
                ],
            ),
        ),
    );
    let handle = admitted_rebinding_handle("phase-nine-closeout-live");
    let source =
        primitive_rebinding_retained_fact_source(&left, &handle).expect("retained fact source");
    let evidence = primitive_rebinding_mutation_evidence(&left, &handle).expect("evidence");
    let replacement_source = primitive_rebinding_neighborhood_replacement_source(&left, &handle)
        .expect("replacement source");
    let replacement_entry = topology_neighborhood_replacement_entry(replacement_source);
    let replacement =
        primitive_rebinding_neighborhood_replacement_facts(&replacement_entry, &handle)
            .expect("replacement facts");
    let projection = primitive_rebinding_geometry_projection_consumption(
        &geometry_projection_consumption_entry(source.clone()),
        &handle,
    )
    .expect("projection receipt");
    let historical = historical_rebinding_inspection(&left, &handle);
    let branch_basis = scoped_branch_head_inspection_basis("branch:phase-nine-closeout");
    let branch_local =
        branch_local_rebinding_inspection(&left, &handle, &branch_basis, "branch-evidence:left");
    let certification = certification_bundle_for_pair(
        admitted_rebinding_handle("phase-nine-closeout-bundle"),
        branch_basis,
        left.clone(),
        right,
        "branch-evidence:left",
        "branch-evidence:right",
    );

    let bridge_support = handle
        .bridge_continuation_support::<worth_spatial::facade::bindings::PrimitiveRebindingDeclarationEntry>(
        );
    let bridge_row = bridge_support.rows().first().expect("bridge row");
    assert_eq!(
        bridge_row.status(),
        ForgeQueryDeclarationBridgeRoutingSupportStatus::Admitted
    );

    let signal_envelope = match handle.orchestrate_declaration_entry_outcome(left.clone()) {
        ForgeQueryOrdinaryOutcome::Bound(envelope) => envelope,
        _ => panic!("expected bound envelope"),
    };
    let signal_checked = handle.orchestrate_signal_compatibility_checked(
        primitive_rebinding_signal_workflow(signal_envelope),
    );
    match signal_checked.outcome() {
        ForgeQuerySignalCompatibilityOrchestrationOutcome::Bound(_)
        | ForgeQuerySignalCompatibilityOrchestrationOutcome::BasisMismatch(_) => {}
        _ => panic!("unexpected signal outcome"),
    }

    let continuation_envelope = match handle.orchestrate_declaration_entry_outcome(left.clone()) {
        ForgeQueryOrdinaryOutcome::Bound(envelope) => envelope,
        _ => panic!("expected bound continuation envelope"),
    };
    let prepared = match handle.prepare_continuation_from_target(
        primitive_rebinding_continuation_target(continuation_envelope),
    ) {
        ForgeQueryPreparedContinuationOutcome::Prepared(prepared) => prepared,
        _ => panic!("unexpected continuation preparation outcome"),
    };
    let executed = handle.execute_prepared_continuation_checked(prepared);
    match executed.outcome() {
        ForgeQueryContinuationExecutionOutcome::Executed(_) => {}
        _ => panic!("unexpected continuation execution outcome"),
    }

    assert_eq!(
        replacement.replacement_scope(),
        TopologyNeighborhoodReplacementScope::LocalNeighborhood
    );
    let replacement_canonical_entries: std::collections::BTreeMap<String, String> =
        replacement_entry
            .canonical_declaration_entries()
            .into_iter()
            .filter_map(|entry| match entry.value() {
                ForgeQueryDeclarationCanonicalValue::ExactText(value)
                | ForgeQueryDeclarationCanonicalValue::DecimalText(value) => {
                    Some((entry.locus().to_string(), value.clone()))
                }
                _ => None,
            })
            .collect();
    assert_eq!(
        evidence.neighborhood_replacement().fact_digest(),
        replacement.fact_digest()
    );
    assert_eq!(
        projection.source_receipt_digest(),
        replacement_canonical_entries
            .get("geometry.neighborhood.source_receipt_digest")
            .expect("replacement canonical source digest")
    );
    assert_eq!(
        evidence.prior_binding_identity(),
        source.receipt().prior_binding_identity()
    );
    assert_eq!(
        historical.receipt().prior_binding_identity(),
        source.receipt().prior_binding_identity()
    );
    assert_eq!(
        branch_local.receipt().prior_binding_identity(),
        source.receipt().prior_binding_identity()
    );
    assert_eq!(
        certification.binding_identity(),
        source.receipt().prior_binding_identity()
    );
    assert_eq!(
        certification.deterministic_outcome_class(),
        source.receipt().outcome_class()
    );
    assert_eq!(
        certification.deterministic_continuity_class(),
        source.receipt().continuity_class()
    );
    assert_eq!(
        certification.selected_candidate_identity(),
        source.receipt().selected_candidate_identity()
    );
    assert_eq!(certification.replay_ordinary_kind(), "ambiguous");
    assert!(!projection.projection_digest().is_empty());
    assert!(!historical.historical_digest().is_empty());
    assert!(!branch_local.branch_local_digest().is_empty());
    assert!(!certification.replay_digest().is_empty());
}

#[test]
fn geometry_hard_break_closeout_keeps_denied_paths_typed_and_receipt_backed() {
    let prior = author_primitive_binding_declaration(
        AuthorPrimitiveBindingIntent::attach_vertex_geometry(VertexGeometryBindingSpec::new(
            VertexBindingSite::new("vertex-old"),
            orthotope_contract(),
            canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
            VertexGeometryProvenanceKind::CanonicalWitness,
            VertexToleranceRegime::ExactBits,
        )),
    );
    let a = author_primitive_binding_declaration(
        AuthorPrimitiveBindingIntent::attach_vertex_geometry(VertexGeometryBindingSpec::new(
            VertexBindingSite::new("vertex-new-a"),
            orthotope_contract(),
            canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
            VertexGeometryProvenanceKind::CanonicalWitness,
            VertexToleranceRegime::ExactBits,
        )),
    );
    let b = author_primitive_binding_declaration(
        AuthorPrimitiveBindingIntent::attach_vertex_geometry(VertexGeometryBindingSpec::new(
            VertexBindingSite::new("vertex-new-b"),
            orthotope_contract(),
            canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
            VertexGeometryProvenanceKind::CanonicalWitness,
            VertexToleranceRegime::ExactBits,
        )),
    );
    let left = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            rebinding_prior_fact_from_binding_declaration(&prior, "phase-nine-denied-left-prior"),
            worth_spatial::facade::bindings::LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::VertexGeometry,
                "vertex-old",
                ReplacementCandidateSet::new(vec![
                    rebinding_candidate_from_binding_declaration(
                        "a",
                        &a,
                        "phase-nine-denied-left-a",
                    )
                    .expect("candidate a"),
                    rebinding_candidate_from_binding_declaration(
                        "b",
                        &b,
                        "phase-nine-denied-left-b",
                    )
                    .expect("candidate b"),
                ])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    );
    let right = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            rebinding_prior_fact_from_binding_declaration(&prior, "phase-nine-denied-right-prior"),
            worth_spatial::facade::bindings::LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::VertexGeometry,
                "vertex-old",
                ReplacementCandidateSet::new(vec![
                    rebinding_candidate_from_binding_declaration(
                        "b",
                        &b,
                        "phase-nine-denied-right-b",
                    )
                    .expect("candidate b"),
                    rebinding_candidate_from_binding_declaration(
                        "a",
                        &a,
                        "phase-nine-denied-right-a",
                    )
                    .expect("candidate a"),
                ])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    );
    let handle = admitted_rebinding_handle("phase-nine-closeout-denied");
    let source =
        primitive_rebinding_retained_fact_source(&left, &handle).expect("retained fact source");
    let recovery = primitive_rebinding_geometry_recovery_action(
        &geometry_recovery_action_entry(source),
        &handle,
    )
    .expect("recovery action");
    let certification = certification_bundle_for_pair(
        admitted_rebinding_handle("phase-nine-closeout-denied-bundle"),
        scoped_branch_head_inspection_basis("branch:phase-nine-closeout-denied"),
        left,
        right,
        "branch-evidence:left",
        "branch-evidence:right",
    );

    assert_eq!(
        recovery.recovery_action_kind(),
        GeometryRecoveryAction::CheckSupport
    );
    assert_eq!(
        recovery.recovery_target_scope(),
        GeometryRecoveryTargetScope::SupportReadiness
    );
    assert_eq!(
        certification.deterministic_outcome_class(),
        worth_spatial::facade::bindings::RebindingOutcomeClass::Unsupported
    );
    assert_eq!(certification.replay_ordinary_kind(), "unsupported");
    assert!(!certification.replay_digest().is_empty());
}
