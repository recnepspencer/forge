use super::support::*;

#[test]
fn pricing_shock_certification_matrix_distinguishes_control_replay_and_hostile_lanes() {
    let scenario = generated_pricing_scenario();
    let control = capture_pricing_certification_matrix(
        BridgeRuntimePolicy::development(),
        BridgePreviewSessionIdentity::new("pricing:preview-certification-control"),
    );

    let hostile_source = InMemoryRelationalBridgeSource::default();
    hostile_source.insert_committed_patch(pricing_patch(
        pricing_patch_envelope_identity(
            TruthBranchIdentity::new("main"),
            TruthCommitIdentity::new("commit:steel-missing-snapshot"),
            TruthPatchIdentity::new("patch:steel-missing-snapshot"),
            TruthSnapshotIdentity::new("snapshot:pricing-missing"),
        ),
        "steel",
    ));
    let hostile_runtime =
        build_pricing_runtime(hostile_source, RecordingSignalBridgeSink::default());
    let hostile = capture_pricing_missing_snapshot_failure_bundle(&hostile_runtime);

    assert_eq!(
        control.reference.route_snapshot,
        TruthSnapshotIdentity::new("snapshot:pricing-main")
    );
    assert_eq!(
        control.reference.source_branch,
        TruthBranchIdentity::new("main")
    );
    assert_eq!(
        control.reference.source_commit,
        TruthCommitIdentity::new("commit:steel-main")
    );
    assert_eq!(control.reference.route_entry_count, 2);
    assert_eq!(
        control.reference.main_rubber_cost_cents,
        scenario.main_rubber_cost
    );
    assert_eq!(
        control.reference.speculative_rubber_cost_cents,
        scenario.speculative_rubber_cost
    );
    assert!(!control
        .reference
        .evaluation_record_identity
        .as_str()
        .is_empty());
    assert!(!control
        .reference
        .evaluation_selector_identity
        .as_str()
        .is_empty());
    assert_eq!(
        control.replay.source_snapshot,
        control.reference.route_snapshot
    );
    assert_eq!(
        control.replay.source_commit,
        TruthCommitIdentity::new("commit:steel-main")
    );
    assert_eq!(
        hostile.failure_class,
        BridgeFailureClass::Delivery(BridgeDeliveryErrorKind::SnapshotAcquisitionFailure)
    );
    assert_eq!(
        hostile.source_commit,
        TruthCommitIdentity::new("commit:steel-missing-snapshot")
    );
    assert_eq!(
        hostile.source_snapshot,
        TruthSnapshotIdentity::new("snapshot:pricing-missing")
    );
}

#[test]
fn pricing_shock_aspect_lane_preserves_fine_grained_truth_and_history() {
    let aspect = capture_pricing_aspect_bundle(BridgeRuntimePolicy::development());

    assert_eq!(
        aspect.snapshot,
        TruthSnapshotIdentity::new("snapshot:pricing-aspect")
    );
    assert_eq!(aspect.source_branch, TruthBranchIdentity::new("main"));
    assert_eq!(
        aspect.source_commit,
        TruthCommitIdentity::new("commit:steel-aspect")
    );
    assert_eq!(
        aspect.truth_surface_kind,
        TruthDeltaSurfaceKind::EntityField
    );
    assert_eq!(
        aspect.fine_grained_match_status,
        FineGrainedMatchStatus::Matched
    );
    assert_eq!(
        aspect.aspect_registration_id,
        BridgeAspectRegistrationId::new("pricing-steel-usd-field")
    );
    assert_eq!(
        aspect.subscription_slice_kind,
        SubscriptionSliceKind::SignalField
    );
    assert_eq!(
        aspect.target_canonical_basis,
        expected_cost_usd_target_basis()
    );
    assert_eq!(aspect.invalidation_target, "price:bicycle");
}

#[test]
fn pricing_shock_workload_certification_bundle_is_profile_invariant_for_semantic_truth() {
    let baseline = capture_pricing_workload_certification_bundle(
        BridgeRuntimePolicy::development(),
        BridgePreviewSessionIdentity::new("pricing:preview-workload-baseline"),
    );
    let forensic = capture_pricing_workload_certification_bundle(
        BridgeRuntimePolicy::forensic(),
        BridgePreviewSessionIdentity::new("pricing:preview-workload-forensic"),
    );

    assert_eq!(baseline.matrix, forensic.matrix);
    assert_eq!(baseline.aspect, forensic.aspect);
    assert_eq!(baseline.discard, forensic.discard);
    assert_eq!(baseline.promotion, forensic.promotion);
    assert_eq!(baseline.fanout, forensic.fanout);
    assert_eq!(baseline.restart_replay, forensic.restart_replay);
    assert_eq!(baseline.restart_failure, forensic.restart_failure);
    assert_eq!(baseline.writeback, forensic.writeback);
    assert_eq!(baseline.merge, forensic.merge);
    assert_eq!(baseline.provenance, forensic.provenance);
    assert_eq!(baseline.hostile_failure, forensic.hostile_failure);
    assert_eq!(baseline.digest(), forensic.digest());

    let comparison = baseline.retained_comparison_evidence_against(&forensic);
    assert!(comparison.all_retained_artifacts_equal());
    assert!(comparison.matrix_equal);
    assert!(comparison.aspect_equal);
    assert!(comparison.discard_equal);
    assert!(comparison.promotion_equal);
    assert!(comparison.fanout_equal);
    assert!(comparison.restart_replay_equal);
    assert!(comparison.restart_failure_equal);
    assert!(comparison.writeback_equal);
    assert!(comparison.merge_equal);
    assert!(comparison.provenance_equal);
    assert!(comparison.portfolio_equal);
    assert!(comparison.crisis_equal);
    assert!(comparison.strategy_equal);
    assert!(comparison.simulation_equal);
    assert!(comparison.trust_attacks_equal);
    assert!(comparison.hostile_failure_equal);
    assert!(comparison.digest_equal);
}

#[test]
fn pricing_shock_workload_certification_bundle_exposes_phase_3_truth_edges() {
    let scenario = generated_pricing_scenario();
    let bundle = capture_pricing_workload_certification_bundle(
        BridgeRuntimePolicy::development(),
        BridgePreviewSessionIdentity::new("pricing:preview-workload-edges"),
    );
    let suite_25 = bundle.suite_25_digest_evidence();
    let suite_26 = bundle.suite_26_digest_evidence();
    let diagnostics_entrypoints = bundle.diagnostics_entrypoint_evidence();
    let completeness = bundle.bundle_completeness_evidence();
    let reference_comparison = bundle.reference_workload_comparison_evidence();
    let counters = bundle.certification_counter_evidence();

    assert_eq!(
        bundle.matrix.reference.route_snapshot,
        TruthSnapshotIdentity::new("snapshot:pricing-main")
    );
    assert_eq!(
        bundle.matrix.reference.main_rubber_cost_cents,
        scenario.main_rubber_cost
    );
    assert_eq!(
        bundle.matrix.reference.speculative_rubber_cost_cents,
        scenario.speculative_rubber_cost
    );
    assert_ne!(
        bundle.matrix.reference.main_snapshot,
        bundle.matrix.reference.speculative_snapshot
    );
    assert_ne!(
        bundle.matrix.reference.main_rubber_cost_cents,
        bundle.matrix.reference.speculative_rubber_cost_cents
    );
    assert_ne!(
        bundle.matrix.reference.speculative_truth_branch,
        bundle.matrix.reference.source_branch
    );
    assert_eq!(
        bundle.aspect.source_commit,
        TruthCommitIdentity::new("commit:steel-aspect")
    );
    assert_eq!(
        bundle.aspect.aspect_registration_id,
        BridgeAspectRegistrationId::new("pricing-steel-usd-field")
    );
    assert_eq!(
        bundle.matrix.replay.source_snapshot,
        TruthSnapshotIdentity::new("snapshot:pricing-main")
    );
    assert_eq!(
        bundle.discard.replay_outcome,
        BridgePreviewLifecycleStateKind::Discarded
    );
    assert_eq!(
        bundle.promotion.replay_outcome,
        BridgePreviewLifecycleStateKind::Promoted
    );
    assert_ne!(
        bundle.discard.lifecycle_state,
        bundle.promotion.lifecycle_state
    );
    assert!(!bundle.discard.has_promotion_record);
    assert!(bundle.promotion.has_promotion_explanation);
    assert_eq!(bundle.fanout.second_delivery_target_count, 100);
    assert_eq!(
        bundle.fanout.second_source_commit,
        TruthCommitIdentity::new("commit:steel-fanout-b")
    );
    assert_eq!(
        bundle.restart_replay.source_commit,
        TruthCommitIdentity::new("commit:steel-main")
    );
    assert_eq!(
        bundle.restart_failure.error_kind,
        BridgeReplayErrorKind::RouteMismatch
    );
    assert_eq!(
        bundle.writeback.commit_outcome_class,
        crate::facade::BridgeWritebackOutcomeClass::AuthoritativeCommit
    );
    assert_eq!(
        bundle.writeback.noop_outcome_class,
        crate::facade::BridgeWritebackOutcomeClass::CanonicalNoop
    );
    assert_eq!(bundle.writeback.authority_commit_count, 1);
    assert_eq!(
        bundle.writeback.rejection_error_kind,
        BridgeWritebackErrorKind::MergeAuthorityRejected
    );
    assert_eq!(
        bundle.merge.bridge_class,
        BridgeMergeConsumptionClass::AspectReconciliationMerge
    );
    assert_eq!(
        bundle.merge.outcome_class,
        crate::facade::BridgeMergeRoutingOutcomeClass::ContinuityCandidate
    );
    assert_eq!(
        bundle.merge.main_premerge_rubber_cost_cents,
        scenario.main_rubber_cost
    );
    assert_eq!(
        bundle.merge.speculative_rubber_cost_cents,
        scenario.speculative_rubber_cost
    );
    assert_eq!(
        bundle.merge.merged_rubber_cost_cents,
        scenario.speculative_rubber_cost
    );
    assert_ne!(
        bundle.merge.main_premerge_rubber_cost_cents,
        bundle.merge.merged_rubber_cost_cents
    );
    assert_eq!(
        bundle.merge.speculative_rubber_cost_cents,
        bundle.merge.merged_rubber_cost_cents
    );
    assert_eq!(
        bundle.merge.merged_aspect_registration_id,
        BridgeAspectRegistrationId::new("pricing-rubber-usd-field")
    );
    assert_eq!(
        bundle.provenance.main_commit,
        TruthCommitIdentity::new("commit:rubber-main")
    );
    assert_eq!(
        bundle.provenance.shock_commit,
        TruthCommitIdentity::new("commit:rubber-shock")
    );
    assert_eq!(
        bundle.provenance.shock_snapshot,
        TruthSnapshotIdentity::new("snapshot:pricing-shock")
    );
    assert_eq!(
        bundle.provenance.shock_delta_microunits,
        scenario.speculative_rubber_cost - scenario.main_rubber_cost
    );
    assert_eq!(bundle.provenance.shock_multiplier_per_mille, 4000);
    assert_eq!(bundle.provenance.representative_sku, "scooter-001");
    assert_eq!(
        bundle.hostile_failure.error_kind,
        BridgeDeliveryErrorKind::SnapshotAcquisitionFailure
    );
    assert_eq!(
        bundle.aspect.target_canonical_basis,
        expected_cost_usd_target_basis()
    );
    assert_eq!(
        bundle.merge.merged_fine_grained_match_status,
        FineGrainedMatchStatus::Matched
    );
    assert_eq!(bundle.portfolio.product_count, 100);
    assert_eq!(
        bundle.crisis.crisis_name,
        "energy-logistics-industrial-crunch"
    );
    assert_eq!(bundle.simulation.branch_count, 10);
    assert_eq!(bundle.simulation.iterations_per_branch, 10);
    assert_eq!(bundle.simulation.material_summaries.len(), 9);
    assert_eq!(bundle.simulation.iteration_traces.len(), 900);
    assert!(!bundle.simulation.ranked_materials_by_damage.is_empty());
    assert_eq!(
        bundle.trust_attacks.replay_policy_error_kind,
        crate::facade::BridgePolicyRejectionKind::ReplayPolicyConflict
    );
    assert_eq!(
        bundle.trust_attacks.replay_policy_failure_class,
        crate::facade::BridgePolicyFieldKind::ReplayArtifacts
    );
    assert_eq!(
        bundle.trust_attacks.route_policy_error_kind,
        crate::facade::BridgeRouteErrorKind::RoutePolicyMismatch
    );
    assert_eq!(
        bundle.trust_attacks.merge_denial_blocked_stage,
        crate::facade::BridgeMergePrecedenceStage::DeletionTopologyGate
    );
    assert_eq!(
        bundle.trust_attacks.merge_denial_class,
        crate::facade::BridgeMergeDenialClass::TopologyRewireGate
    );
    assert_eq!(counters.causality_bundle_count, 1);
    assert_eq!(counters.causality_bundle_replay_match_count, 3);
    assert_eq!(counters.causality_bundle_replay_mismatch_count, 1);
    assert_eq!(counters.failure_taxonomy_classification_count, 3);
    assert_eq!(counters.failure_taxonomy_unclassified_count, 0);
    assert_eq!(
        counters.diagnostics_entrypoint_request_count,
        diagnostics_entrypoints.entrypoint_count()
    );
    assert_eq!(counters.showcase_entrypoint_request_count, 1);
    assert_eq!(counters.simulation_trace_bundle_count, 1);
    assert_eq!(counters.trust_attack_classification_count, 8);
    assert_eq!(counters.diagnostics_entrypoint_reconstruction_count, 1);
    assert_eq!(counters.speculative_branch_bundle_count, 1);
    assert_eq!(counters.speculative_discard_residue_check_count, 1);
    assert_eq!(counters.speculative_discard_residue_nonzero_count, 0);
    assert_eq!(counters.branch_comparison_bundle_count, 1);
    assert_eq!(counters.offline_bundle_diagnosis_count, 1);
    assert_eq!(counters.offline_bundle_insufficiency_count, 0);
    assert!(!suite_25.causality_digest.is_empty());
    assert!(!suite_25.routing_digest.is_empty());
    assert!(!suite_25.explanation_digest.is_empty());
    assert!(!suite_25.replay_digest.is_empty());
    assert!(!suite_25.reference_workload_bundle_digest.is_empty());
    assert_ne!(suite_25.causality_digest, suite_25.routing_digest);
    assert_ne!(suite_25.routing_digest, suite_25.replay_digest);
    assert!(!suite_26.failure_digest.is_empty());
    assert!(!suite_26.replay_failure_digest.is_empty());
    assert_ne!(suite_26.failure_digest, suite_26.replay_failure_digest);
    assert_eq!(
        bundle.hostile_failure.failure_class,
        BridgeFailureClass::Delivery(BridgeDeliveryErrorKind::SnapshotAcquisitionFailure)
    );
    assert_eq!(
        bundle.writeback.rejection_error_kind,
        BridgeWritebackErrorKind::MergeAuthorityRejected
    );
    assert_eq!(
        bundle.restart_failure.error_kind,
        BridgeReplayErrorKind::RouteMismatch
    );
    assert_eq!(counters.speculative_discard_residue_nonzero_count, 0);
    assert!(completeness.offline_sufficient);
    assert_eq!(completeness.insufficiency_count, 0);
    assert!(diagnostics_entrypoints.all_entrypoints_available());
    assert!(diagnostics_entrypoints.routing);
    assert!(diagnostics_entrypoints.merge);
    assert!(diagnostics_entrypoints.historical_provenance);
    assert!(diagnostics_entrypoints.portfolio);
    assert!(diagnostics_entrypoints.crisis);
    assert!(diagnostics_entrypoints.strategy);
    assert!(diagnostics_entrypoints.simulation);
    assert!(diagnostics_entrypoints.trust_attacks);
    assert!(reference_comparison.main_vs_speculative_snapshot_distinct);
    assert!(reference_comparison.merged_vs_speculative_rubber_cost_equal);
    assert!(reference_comparison.merged_vs_premerge_rubber_cost_distinct);
    assert!(reference_comparison.historical_provenance_commit_matches_shock);
    assert!(reference_comparison.portfolio_reports_positive_blast_radius);
    assert!(reference_comparison.crisis_affects_portfolio_breadth);
    assert!(reference_comparison.strategy_recommends_non_hold_response);
    assert!(reference_comparison.promotion_strategy_prefers_authoritative_action);
    assert!(reference_comparison.simulation_identifies_at_least_one_damaging_material);
    assert!(reference_comparison.trust_attack_matrix_is_typed);
    assert!(!bundle.digest().is_empty());
}

fn expected_cost_usd_target_basis() -> &'static str {
    "committed-patch-target|locator=version=bridge.committed-patch-target.v1;domain=locator;entries=[locus=named:aspect_field.aspect_key,kind=locator,value=exact-text:cost;locus=named:aspect_field.authority,kind=locator,value=exact-text:authoritative;locus=named:aspect_field.field_path,kind=locator,value=exact-text:usd;locus=named:aspect_field.kind,kind=locator,value=exact-text:aspect]|mutation-mask=version=bridge.committed-patch-target.v1;domain=aspect-mask;entries=[locus=named:cost.mutation.field.usd,kind=mask,value=exact-text:usd]|projection-mask=version=bridge.committed-patch-target.v1;domain=aspect-mask;entries=[locus=named:cost.projection.field.usd,kind=mask,value=exact-text:usd]|kind=entity-field"
}
