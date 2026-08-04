use crate::live::*;
#[test]
fn in_region_detail_execution_can_lower_to_stream_contract() {
    let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");
    let region_plan =
        admit_region_scoped_live_plan(&live, LocalityPredicateContract::region("assembly-a"))
            .expect("detail live plan should admit region scope");
    let in_region_change = BridgeChangeSummary::default()
        .with_field_delta(BridgeFieldDelta::new(
            "identity",
            "id",
            Some("user-1"),
            Some("user-2"),
        ))
        .with_region_slice("assembly-a");

    let execution = execute_region_scoped_live_change(&region_plan, &in_region_change)
        .expect("in-region change should execute");
    let contract = lower_region_scoped_execution_to_stream_contract(
        &region_plan,
        &execution,
        StreamConsumerShape::DetailCurrentState,
    )
    .expect("detail execution should lower to stream contract");
    let counters = LivePolicyCounters::from_stream_lowered_delivery(&contract);

    assert_eq!(
        contract.consumer_shape(),
        &StreamConsumerShape::DetailCurrentState
    );
    assert_eq!(
        contract.query_delivery_contract().family(),
        &LiveQueryFamily::Detail
    );
    assert_eq!(
        contract.query_delivery_contract().locality_outcome(),
        &DeliveryLocalityOutcome::InRegionRegion
    );
    assert_eq!(
        contract
            .delivery_contract_lowering()
            .query_delivery_digest(),
        contract.query_delivery_contract().digest()
    );
    assert_eq!(
        contract.delivery_contract_lowering().request_digest(),
        contract.request().digest()
    );
    assert_eq!(
        contract.member_projection().consumer_shape(),
        &StreamConsumerShape::DetailCurrentState
    );
    assert_eq!(contract.member_projection().member_count(), 1);
    assert_eq!(contract.window_compatibility().window_width(), 1);
    assert_eq!(contract.window_compatibility().budget_limit(), 1);
    assert_eq!(
        contract.replay_record().stream_contract_digest(),
        Some(contract.stream_contract_digest())
    );
    assert_eq!(contract.member_count(), 1);
    assert_eq!(contract.delivery_width(), 1);
    assert_eq!(
        contract.cost_posture(),
        &StreamLoweringCostPosture::SingleDetailCurrentStateMember
    );
    assert_eq!(counters.stream_contract_admission_count(), 1);
    assert_eq!(counters.stream_lowered_delivery_member_count(), 1);
    assert_eq!(counters.stream_lowered_delivery_width(), 1);
}

#[test]
fn ordered_collection_partition_execution_admits_cdc_collection_stream_shape() {
    let preflight =
            crate::harness::fixtures::execution_preflights::ordered_collection_without_traversal_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("collection preflight should promote");
    let partition_plan =
        admit_region_scoped_live_plan(&live, LocalityPredicateContract::partition("tenant-a"))
            .expect("ordered collection should admit partition scope");
    let in_partition_change = BridgeChangeSummary::default()
        .with_field_delta(BridgeFieldDelta::new(
            "profile",
            "display_name",
            Some("Esther"),
            Some("Ess"),
        ))
        .with_partition_slice("tenant-a");

    let execution = execute_region_scoped_live_change(&partition_plan, &in_partition_change)
        .expect("in-partition change should execute");
    let contract = lower_region_scoped_execution_to_stream_contract(
        &partition_plan,
        &execution,
        StreamConsumerShape::CdcCollectionPatch,
    )
    .expect("ordered collection execution should lower to cdc collection stream shape");

    assert_eq!(
        contract.consumer_shape(),
        &StreamConsumerShape::CdcCollectionPatch
    );
    assert_eq!(
        contract.query_delivery_contract().family(),
        &LiveQueryFamily::OrderedCollection
    );
    assert_eq!(contract.member_projection().member_count(), 1);
    assert_eq!(contract.member_projection().delivery_width(), 2);
    assert_eq!(contract.window_compatibility().window_width(), 1);
    assert_eq!(contract.delivery_width(), 2);
    assert_eq!(
        contract.cost_posture(),
        &StreamLoweringCostPosture::CdcPatchWithProjectedDeltas
    );
}

#[test]
fn region_scoped_replay_bundle_carries_locality_native_replay_record() {
    let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");
    let region_plan =
        admit_region_scoped_live_plan(&live, LocalityPredicateContract::region("assembly-a"))
            .expect("detail live plan should admit region scope");
    let in_region_change = BridgeChangeSummary::default()
        .with_field_delta(BridgeFieldDelta::new(
            "identity",
            "id",
            Some("user-1"),
            Some("user-2"),
        ))
        .with_region_slice("assembly-a");

    let execution = execute_region_scoped_live_change(&region_plan, &in_region_change)
        .expect("in-region change should execute");
    let replay_record = execution.region_scoped_replay_bundle().replay_record();

    assert_eq!(
        replay_record.query_digest(),
        execution.report().query_digest()
    );
    assert_eq!(
        replay_record.delivery_digest(),
        execution.report().delivery_digest()
    );
    assert_eq!(
        replay_record.replay_digest(),
        execution.report().replay_digest()
    );
    assert_eq!(
        replay_record.locality_outcome(),
        &DeliveryLocalityOutcome::InRegionRegion
    );
    assert_eq!(replay_record.stream_contract_digest(), None);
    assert_eq!(
        execution
            .region_scoped_replay_bundle()
            .counter_snapshot()
            .locality_replay_change_count(),
        1
    );
    assert_eq!(
        execution
            .region_scoped_replay_bundle()
            .counter_snapshot()
            .locality_replay_divergence_count(),
        0
    );
}

#[test]
fn cdc_stream_shape_rejects_member_width_overflow() {
    let preflight =
            crate::harness::fixtures::execution_preflights::ordered_collection_without_traversal_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("collection preflight should promote");
    let partition_plan =
        admit_region_scoped_live_plan(&live, LocalityPredicateContract::partition("tenant-a"))
            .expect("ordered collection should admit partition scope");
    let wide_partition_change = BridgeChangeSummary::default()
        .with_field_delta(BridgeFieldDelta::new(
            "profile",
            "display_name",
            Some("Esther"),
            Some("Ess"),
        ))
        .with_field_delta(BridgeFieldDelta::new(
            "identity",
            "id",
            Some("user-1"),
            Some("user-2"),
        ))
        .with_partition_slice("tenant-a");

    let execution = execute_region_scoped_live_change(&partition_plan, &wide_partition_change)
        .expect("in-partition change should execute");
    let error = lower_region_scoped_execution_to_stream_contract(
        &partition_plan,
        &execution,
        StreamConsumerShape::CdcCollectionPatch,
    )
    .expect_err("two projected deltas should overflow the stream member width budget");
    let counters = LivePolicyCounters::from_region_scoped_error(&error);

    assert_eq!(
        error,
        RegionScopedLiveError::StreamMemberWidthBudgetExceeded {
            limit: 2,
            actual: 3
        }
    );
    assert_eq!(counters.stream_contract_denial_count(), 1);
    assert_eq!(counters.stream_member_width_budget_cross_count(), 1);
}

#[test]
fn widened_detail_stream_shape_rejects_window_width_overflow() {
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
        .expect("detail region widening should execute before stream lowering");
    let error = lower_region_scoped_execution_to_stream_contract(
        &region_plan,
        &execution,
        StreamConsumerShape::DetailCurrentState,
    )
    .expect_err("peer-widened detail delivery should overflow the window width budget");
    let counters = LivePolicyCounters::from_region_scoped_error(&error);

    assert_eq!(
        error,
        RegionScopedLiveError::StreamWindowWidthBudgetExceeded {
            limit: 1,
            actual: 2
        }
    );
    assert_eq!(counters.stream_contract_denial_count(), 1);
    assert_eq!(counters.stream_window_width_budget_cross_count(), 1);
    assert_eq!(counters.stream_member_width_budget_cross_count(), 0);
}
