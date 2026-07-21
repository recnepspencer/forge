use super::active_application_session_test_support::source_backed_component_session;
use super::virtualized_data_lane_test_support::{
    virtualized_data_denial_for_missing_support, virtualized_data_denial_for_stale_lane_admission,
    virtualized_data_fixture, VirtualizedDataFixture,
};
use crate::runtime::{
    WorthUiExecutionLane, WorthUiHandleSlotGeneration, WorthUiViewBindingHandle,
    WorthUiVirtualizedDataFrameDenialReason, WorthUiVirtualizedDataLane,
    WorthUiVirtualizedDataPlanDenialReason, WorthUiVirtualizedPlanAvailability,
    WorthUiVirtualizedPlanSummaryDenial, WorthUiVirtualizedPlanSummaryRequest, WorthUiVisibleRange,
    WorthUiVisibleRangeDenialReason,
};

#[test]
fn active_virtualized_query_plan_contract() {
    let mut fixture = virtualized_data_fixture();
    equivalent_active_reference_and_range_produce_equivalent_receipts(&mut fixture);
    active_lane_classifies_row_and_grid_breadth(&mut fixture);
    exact_query_native_value_reaches_the_active_plan_edge(&mut fixture);
    summary_is_budgeted_and_links_read_only_query_evidence(&fixture);
    active_frame_rejects_stale_or_absent_view_handles(&mut fixture);
}

fn equivalent_active_reference_and_range_produce_equivalent_receipts(
    fixture: &mut VirtualizedDataFixture,
) {
    let range = WorthUiVisibleRange::grid(120, 40, 0, 12).expect("range is valid");
    let target = fixture.summary().target(range);

    let first = fixture
        .execute(target)
        .expect("first active frame executes");
    let second = fixture
        .execute(target)
        .expect("second active frame executes");

    assert_eq!(first.certification(), second.certification());
    assert_eq!(first.binding_identity(), second.binding_identity());
    assert_eq!(first.evidence(), second.evidence());
    assert_eq!(first.visible_range(), second.visible_range());
    assert_eq!(first.lane(), WorthUiVirtualizedDataLane::CellGrid);
    assert_eq!(first.counters(), second.counters());
    assert_eq!(first.counters().visible_row_touch_count(), 40);
    assert_eq!(first.counters().visible_column_touch_count(), 12);
    assert_eq!(first.counters().visible_cell_touch_count(), 480);
}

fn active_lane_classifies_row_and_grid_breadth(fixture: &mut VirtualizedDataFixture) {
    let summary = fixture.summary();
    let row_target = summary.target(WorthUiVisibleRange::rows(20, 6).expect("row range"));
    let grid_target = summary.target(WorthUiVisibleRange::grid(20, 6, 4, 3).expect("grid range"));

    let row = fixture.execute(row_target).expect("row frame executes");
    let grid = fixture.execute(grid_target).expect("grid frame executes");

    assert_eq!(row.lane(), WorthUiVirtualizedDataLane::RowList);
    assert_eq!(grid.lane(), WorthUiVirtualizedDataLane::CellGrid);
    assert_eq!(row.counters().visible_column_touch_count(), 1);
    assert_eq!(grid.counters().visible_column_touch_count(), 3);
    for counters in [row.counters(), grid.counters()] {
        assert_eq!(counters.data_plan_row_count(), 0);
        assert_eq!(counters.unrelated_plan_row_count(), 0);
        assert_eq!(counters.family_index_read_count(), 0);
        assert_eq!(counters.regional_executable_read_count(), 0);
        assert_eq!(counters.full_collection_scan_count(), 0);
        assert_eq!(counters.offset_pagination_substitute_count(), 0);
        assert_eq!(counters.query_collection_execution_count(), 0);
        assert_eq!(counters.diagnostic_materialization_count(), 0);
        assert_eq!(counters.direct_row_lookup_count(), 1);
        assert_eq!(counters.evidence_reference_lookup_count(), 1);
    }
}

fn exact_query_native_value_reaches_the_active_plan_edge(fixture: &mut VirtualizedDataFixture) {
    let summary = fixture.summary();
    let summary_evidence = summary.evidence().expect("projection evidence is linked");
    let native = summary_evidence
        .native_fact(0)
        .and_then(|value| value.scalar())
        .expect("measurement projection retains a scalar native value");
    assert!(matches!(
        native,
        worth_foundational::AspectValue::Float32(value)
            if *value == worth_foundational::CanonicalF32::from_f32(240.0)
    ));

    let receipt = fixture
        .execute(summary.target(WorthUiVisibleRange::rows(0, 3).expect("range")))
        .expect("active frame executes");
    assert_eq!(
        receipt.evidence().evidence_identity_digest(),
        summary_evidence.evidence_identity_digest()
    );
    assert_eq!(receipt.evidence().native_fact_count(), 1);
    assert_eq!(receipt.evidence().observation_count(), 1);
}

fn summary_is_budgeted_and_links_read_only_query_evidence(fixture: &VirtualizedDataFixture) {
    let zero_budget = fixture
        .session
        .inspect_virtualized_plan(WorthUiVirtualizedPlanSummaryRequest::new(0))
        .expect_err("zero budget denies");
    let summary = fixture.summary();

    assert_eq!(
        zero_budget,
        WorthUiVirtualizedPlanSummaryDenial::ZeroRowBudget
    );
    assert_eq!(summary.total_view_row_count(), 1);
    assert_eq!(summary.family_index_lookup_count(), 1);
    assert_eq!(summary.direct_row_lookup_count(), 1);
    assert_eq!(summary.evidence_reference_lookup_count(), 1);
    assert_eq!(
        summary.definition(),
        summary.evidence().unwrap().definition()
    );
}

#[test]
fn plan_requires_exact_data_and_query_lane_support() {
    let missing_data =
        virtualized_data_denial_for_missing_support(WorthUiExecutionLane::VirtualizedData);
    let missing_query =
        virtualized_data_denial_for_missing_support(WorthUiExecutionLane::QueryBound);

    assert_eq!(
        missing_data.reason(),
        WorthUiVirtualizedDataPlanDenialReason::LaneAdmissionMissingVirtualizedDataSupport
    );
    assert_eq!(
        missing_query.reason(),
        WorthUiVirtualizedDataPlanDenialReason::LaneAdmissionMissingQuerySupport
    );
}

#[test]
fn plan_rejects_stale_lane_admission_before_index_reads() {
    let denial = virtualized_data_denial_for_stale_lane_admission();
    assert_eq!(
        denial.reason(),
        WorthUiVirtualizedDataPlanDenialReason::LaneAdmissionPlanMismatch
    );
    assert_eq!(denial.counters().certification_failure_count(), 1);
    assert_eq!(denial.counters().family_index_read_count(), 0);
}

fn active_frame_rejects_stale_or_absent_view_handles(fixture: &mut VirtualizedDataFixture) {
    let fresh = fixture
        .summary()
        .target(WorthUiVisibleRange::rows(0, 3).expect("range"));
    let handle = fresh.handle();
    let stale = WorthUiViewBindingHandle::new(
        handle.plan_index(),
        WorthUiHandleSlotGeneration::new(handle.slot_generation().as_u64() + 1),
        handle.arena_identity(),
    );
    let absent = WorthUiViewBindingHandle::new(
        handle.plan_index() + 10_000,
        handle.slot_generation(),
        handle.arena_identity(),
    );
    let range = fresh.visible_range();

    let stale_denial = fixture
        .execute(crate::runtime::WorthUiVirtualizedDataFrameTarget::view_binding(stale, range))
        .expect_err("stale handle denies");
    let absent_denial = fixture
        .execute(crate::runtime::WorthUiVirtualizedDataFrameTarget::view_binding(absent, range))
        .expect_err("absent handle denies");

    assert_eq!(
        stale_denial.reason(),
        WorthUiVirtualizedDataFrameDenialReason::TargetSlotGenerationMismatch
    );
    assert_eq!(stale_denial.counters().evidence_reference_lookup_count(), 0);
    assert_eq!(
        absent_denial.reason(),
        WorthUiVirtualizedDataFrameDenialReason::TargetNotInVirtualizedDataPlan
    );
    assert_eq!(
        absent_denial.counters().evidence_reference_lookup_count(),
        0
    );
}

#[test]
fn visible_range_rejects_empty_or_overflowing_windows() {
    let empty_rows = WorthUiVisibleRange::rows(0, 0).expect_err("empty rows deny");
    let empty_columns = WorthUiVisibleRange::rows(0, 1)
        .and_then(|range| range.with_columns(0, 0))
        .expect_err("empty columns deny");
    let row_overflow = WorthUiVisibleRange::rows(u32::MAX, 1).expect_err("row overflow denies");

    assert_eq!(
        empty_rows.reason(),
        WorthUiVisibleRangeDenialReason::EmptyRowRange
    );
    assert_eq!(
        empty_columns.reason(),
        WorthUiVisibleRangeDenialReason::EmptyColumnRange
    );
    assert_eq!(
        row_overflow.reason(),
        WorthUiVisibleRangeDenialReason::RangeOverflow
    );
}

#[test]
fn query_free_active_session_uses_explicit_cheap_posture() {
    let session = source_backed_component_session();
    assert_eq!(
        session.virtualized_plan_availability(),
        WorthUiVirtualizedPlanAvailability::QueryFree
    );
    assert_eq!(
        session
            .inspect_virtualized_plan(WorthUiVirtualizedPlanSummaryRequest::first_view())
            .expect_err("query-free summary denies explicitly"),
        WorthUiVirtualizedPlanSummaryDenial::ActivePlanIsQueryFree
    );
}
