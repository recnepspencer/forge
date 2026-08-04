use crate::live::*;
#[test]
fn detail_live_plan_admits_region_scope() {
    let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");

    let region_plan =
        admit_region_scoped_live_plan(&live, LocalityPredicateContract::region("assembly-a"))
            .expect("detail live plan should admit region scope");

    assert_eq!(
        region_plan.admission_class(),
        &LocalityAdmissionClass::DetailRegion
    );
    assert!(!region_plan.locality_subscription_digest().is_empty());
    assert_eq!(
        region_plan.locality_cost_posture(),
        &LocalityCostPosture::SingleSliceNarrowing
    );
    assert_eq!(region_plan.locality_breadth_budget().limit(), 1);
    assert_eq!(
        region_plan.locality_widening_policy(),
        &LocalityWideningPolicy::AllowExactMatchWithSinglePeerSlice
    );
    assert_eq!(region_plan.locality_widening_budget().limit(), 1);
    assert_eq!(
        region_plan.stream_lowering_cost_posture(),
        &StreamLoweringCostPosture::SingleDetailCurrentStateMember
    );
    assert_eq!(region_plan.stream_member_width_budget().limit(), 1);
}

#[test]
fn ordered_collection_live_plan_rejects_region_scope() {
    let preflight =
            crate::harness::fixtures::execution_preflights::ordered_collection_without_traversal_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("collection preflight should promote");

    let error =
        admit_region_scoped_live_plan(&live, LocalityPredicateContract::region("assembly-a"))
            .expect_err("ordered collection should reject region scope in milestone 5.1");

    assert_eq!(error, RegionScopedLiveError::UnsupportedLocalityPredicate);
}

#[test]
fn off_region_change_suppresses_before_delivery() {
    let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");
    let region_plan =
        admit_region_scoped_live_plan(&live, LocalityPredicateContract::region("assembly-a"))
            .expect("detail live plan should admit region scope");
    let off_region_change = BridgeChangeSummary::default()
        .with_field_delta(BridgeFieldDelta::new(
            "identity",
            "id",
            Some("user-1"),
            Some("user-2"),
        ))
        .with_region_slice("assembly-b");

    let execution = execute_region_scoped_live_change(&region_plan, &off_region_change)
        .expect("off-region change should suppress, not fail");

    assert_eq!(
        execution.report().locality_outcome(),
        &DeliveryLocalityOutcome::OffRegionSuppressed
    );
    assert_eq!(
        execution.counters().locality_off_region_suppression_count(),
        1
    );
    assert_eq!(
        execution
            .counters()
            .locality_irrelevant_broad_control_count(),
        1
    );
    assert_eq!(execution.counters().locality_replay_change_count(), 1);
    match execution.patch_envelope().payload() {
        LivePatchPayload::Suppressed(SuppressionReason::OffRegionChange {
            scope_kind,
            scope,
            ..
        }) => {
            assert_eq!(scope_kind, &LocalityScopeKind::Region);
            assert_eq!(scope, "assembly-a");
        }
        other => panic!("expected off-region suppression payload, got {other:?}"),
    }
}

#[test]
fn coarse_fallback_slice_is_a_typed_widening_denial() {
    let preflight =
            crate::harness::fixtures::execution_preflights::ordered_collection_without_traversal_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("collection preflight should promote");
    let partition_plan =
        admit_region_scoped_live_plan(&live, LocalityPredicateContract::partition("tenant-a"))
            .expect("ordered collection should admit partition scope");
    let coarse_change = BridgeChangeSummary::default()
        .with_field_delta(BridgeFieldDelta::new(
            "profile",
            "display_name",
            Some("Esther"),
            Some("Ess"),
        ))
        .with_coarse_fallback_slice("tenant-a");

    let error = execute_region_scoped_live_change(&partition_plan, &coarse_change)
        .expect_err("coarse fallback should deny widening");

    match error {
        RegionScopedLiveError::WideningDenied { expected, received } => {
            assert!(expected.contains("entity_partition"));
            assert!(!received.is_empty());
        }
        other => panic!("expected widening denial, got {other:?}"),
    }
}

#[test]
fn detail_region_exact_hit_can_admit_single_peer_widening() {
    let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");
    let region_plan =
        admit_region_scoped_live_plan(&live, LocalityPredicateContract::region("assembly-a"))
            .expect("detail live plan should admit region scope");
    let widened_change = BridgeChangeSummary::default()
        .with_field_delta(BridgeFieldDelta::new(
            "identity",
            "id",
            Some("user-1"),
            Some("user-2"),
        ))
        .with_region_slice("assembly-a")
        .with_region_slice("assembly-b");

    let execution = execute_region_scoped_live_change(&region_plan, &widened_change)
        .expect("detail region widening should admit one exact hit plus one peer slice");

    assert_eq!(
        execution.report().locality_outcome(),
        &DeliveryLocalityOutcome::InRegionRegionWithPeerWidening {
            peer_scopes: vec!["assembly-b".to_string()],
        }
    );
    assert_eq!(execution.counters().locality_region_match_count(), 1);
    assert_eq!(execution.counters().locality_widening_admission_count(), 1);
    assert_eq!(execution.counters().locality_widening_denial_count(), 0);
    assert_eq!(execution.counters().locality_replay_change_count(), 1);
    match execution.report().widening_decision() {
        Some(LocalityWideningDecision::Admitted {
            matched_scope,
            peer_scopes,
        }) => {
            assert_eq!(matched_scope, "assembly-a");
            assert_eq!(peer_scopes, &vec!["assembly-b".to_string()]);
        }
        other => panic!("expected admitted widening decision, got {other:?}"),
    }
}

#[test]
fn detail_region_multiple_peer_slices_cross_the_widening_budget() {
    let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");
    let region_plan =
        admit_region_scoped_live_plan(&live, LocalityPredicateContract::region("assembly-a"))
            .expect("detail live plan should admit region scope");
    let widened_change = BridgeChangeSummary::default()
        .with_field_delta(BridgeFieldDelta::new(
            "identity",
            "id",
            Some("user-1"),
            Some("user-2"),
        ))
        .with_region_slice("assembly-a")
        .with_region_slice("assembly-b")
        .with_region_slice("assembly-c");

    let error = execute_region_scoped_live_change(&region_plan, &widened_change)
        .expect_err("detail region widening should reject peer width beyond the admitted budget");
    let counters = LivePolicyCounters::from_region_scoped_error(&error);

    match error {
        RegionScopedLiveError::WideningDenied { expected, received } => {
            assert_eq!(expected, "entity_region:assembly-a");
            assert_eq!(received.len(), 3);
        }
        other => panic!("expected widening denial, got {other:?}"),
    }
    assert_eq!(counters.locality_widening_denial_count(), 1);
    assert_eq!(counters.locality_widening_budget_cross_count(), 1);
}

#[test]
fn ordered_collection_exact_hit_with_peer_partition_still_denies_widening() {
    let preflight =
            crate::harness::fixtures::execution_preflights::ordered_collection_without_traversal_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("collection preflight should promote");
    let partition_plan =
        admit_region_scoped_live_plan(&live, LocalityPredicateContract::partition("tenant-a"))
            .expect("ordered collection should admit partition scope");
    let widened_change = BridgeChangeSummary::default()
        .with_field_delta(BridgeFieldDelta::new(
            "profile",
            "display_name",
            Some("Esther"),
            Some("Ess"),
        ))
        .with_partition_slice("tenant-a")
        .with_partition_slice("tenant-b");

    let error = execute_region_scoped_live_change(&partition_plan, &widened_change)
        .expect_err("ordered collection should still deny admitted widening");

    match error {
        RegionScopedLiveError::WideningDenied { expected, received } => {
            assert_eq!(expected, "entity_partition:tenant-a");
            assert_eq!(received.len(), 2);
        }
        other => panic!("expected widening denial, got {other:?}"),
    }
}

#[test]
fn duplicate_exact_locality_slices_cross_the_breadth_budget() {
    let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");
    let region_plan =
        admit_region_scoped_live_plan(&live, LocalityPredicateContract::region("assembly-a"))
            .expect("detail live plan should admit region scope");
    let duplicate_slice_change = BridgeChangeSummary::default()
        .with_field_delta(BridgeFieldDelta::new(
            "identity",
            "id",
            Some("user-1"),
            Some("user-2"),
        ))
        .with_region_slice("assembly-a")
        .with_region_slice("assembly-a");

    let error = execute_region_scoped_live_change(&region_plan, &duplicate_slice_change)
        .expect_err("duplicate exact locality slices should exceed the breadth budget");
    let counters = LivePolicyCounters::from_region_scoped_error(&error);

    assert_eq!(
        error,
        RegionScopedLiveError::LocalityBreadthBudgetExceeded {
            limit: 1,
            actual: 2
        }
    );
    assert_eq!(counters.locality_breadth_budget_cross_count(), 1);
}
