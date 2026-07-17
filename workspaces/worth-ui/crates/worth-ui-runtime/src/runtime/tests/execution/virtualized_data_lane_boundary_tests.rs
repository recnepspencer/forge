use super::virtualized_data_lane_test_support::{
    virtualized_data_denial_for_missing_support, virtualized_data_denial_for_stale_lane_admission,
    virtualized_data_fixture,
};
use crate::runtime::{
    WorthUiComponentHandle, WorthUiExecutionLane, WorthUiHandlePlanGeneration,
    WorthUiViewBindingHandle, WorthUiVirtualizedDataFrameDenialReason,
    WorthUiVirtualizedDataFrameTarget, WorthUiVirtualizedDataLane,
    WorthUiVirtualizedDataPlanDenialReason, WorthUiVisibleRange, WorthUiVisibleRangeDenialReason,
};

#[test]
fn equivalent_visible_range_inputs_produce_equivalent_data_lane_receipts() {
    let left = virtualized_data_fixture();
    let right = virtualized_data_fixture();
    let left_handle = first_view_binding_handle(&left.allocation);
    let right_handle = first_view_binding_handle(&right.allocation);
    let range = WorthUiVisibleRange::grid(120, 40, 0, 12).expect("range is valid");

    let left_receipt = left
        .runtime
        .execute_virtualized_data_frame(
            &left.data_plan,
            WorthUiVirtualizedDataFrameTarget::view_binding(left_handle, range),
        )
        .expect("runtime frame execution succeeds");
    let right_receipt = right
        .runtime
        .execute_virtualized_data_frame(
            &right.data_plan,
            WorthUiVirtualizedDataFrameTarget::view_binding(right_handle, range),
        )
        .expect("runtime frame execution succeeds");

    assert_eq!(
        left_receipt.certification().data_plan_digest(),
        right_receipt.certification().data_plan_digest()
    );
    assert_eq!(left_receipt.visible_range(), right_receipt.visible_range());
    assert_eq!(left_receipt.lane(), WorthUiVirtualizedDataLane::CellGrid);
    assert_eq!(left_receipt.counters(), right_receipt.counters());
    assert_eq!(left_receipt.counters().visible_row_touch_count(), 40);
    assert_eq!(left_receipt.counters().visible_column_touch_count(), 12);
    assert_eq!(left_receipt.counters().full_collection_scan_count(), 0);
}

#[test]
fn virtualized_data_lane_classifies_row_and_grid_frame_breadth() {
    let fixture = virtualized_data_fixture();
    let handle = first_view_binding_handle(&fixture.allocation);
    let row_range = WorthUiVisibleRange::rows(20, 6).expect("row range is valid");
    let grid_range = WorthUiVisibleRange::grid(20, 6, 4, 3).expect("grid range is valid");

    let row_receipt = fixture
        .runtime
        .execute_virtualized_data_frame(
            &fixture.data_plan,
            WorthUiVirtualizedDataFrameTarget::view_binding(handle, row_range),
        )
        .expect("runtime frame execution succeeds");
    let grid_receipt = fixture
        .runtime
        .execute_virtualized_data_frame(
            &fixture.data_plan,
            WorthUiVirtualizedDataFrameTarget::view_binding(handle, grid_range),
        )
        .expect("runtime frame execution succeeds");

    assert_eq!(row_receipt.lane(), WorthUiVirtualizedDataLane::RowList);
    assert_eq!(grid_receipt.lane(), WorthUiVirtualizedDataLane::CellGrid);
    assert_eq!(row_receipt.counters().visible_column_touch_count(), 1);
    assert_eq!(grid_receipt.counters().visible_column_touch_count(), 3);
}

#[test]
fn data_lane_rejects_full_collection_frame_scan() {
    let fixture = virtualized_data_fixture();
    let handle = first_view_binding_handle(&fixture.allocation);

    let denial = fixture
        .runtime
        .execute_virtualized_data_frame(
            &fixture.data_plan,
            WorthUiVirtualizedDataFrameTarget::full_collection_scan_for_test(handle),
        )
        .expect_err("runtime frame execution denies");

    assert_eq!(
        denial.reason(),
        WorthUiVirtualizedDataFrameDenialReason::FullCollectionScanCertificationFailure
    );
    assert_eq!(denial.counters().full_collection_scan_count(), 1);
    assert_eq!(denial.counters().certification_failure_count(), 1);
}

#[test]
fn query_shaped_patch_posture_preserved_in_data_lane() {
    let fixture = virtualized_data_fixture();
    let handle = first_view_binding_handle(&fixture.allocation);
    let expected = fixture
        .query_links
        .iter()
        .find(|links| links.plan_index() == handle.plan_index())
        .expect("query support links exist for handle");
    let range = WorthUiVisibleRange::rows(10, 5).expect("range is valid");

    let receipt = fixture
        .runtime
        .execute_virtualized_data_frame(
            &fixture.data_plan,
            WorthUiVirtualizedDataFrameTarget::view_binding(handle, range),
        )
        .expect("runtime frame execution succeeds");
    let posture = receipt.query_patch_posture();

    assert_eq!(posture.plan_index(), expected.plan_index());
    assert_eq!(posture.view_binding_id(), expected.view_binding_id());
    assert_eq!(posture.binding_identity(), expected.binding_identity());
    assert_eq!(posture.posture(), expected.posture());
    assert_eq!(posture.required_surfaces(), expected.required_surfaces());
}

#[test]
fn virtualized_data_lane_rejects_offset_pagination_as_cursor_substitute() {
    let fixture = virtualized_data_fixture();
    let handle = first_view_binding_handle(&fixture.allocation);

    let denial = fixture
        .runtime
        .execute_virtualized_data_frame(
            &fixture.data_plan,
            WorthUiVirtualizedDataFrameTarget::offset_pagination_for_test(handle),
        )
        .expect_err("runtime frame execution denies");

    assert_eq!(
        denial.reason(),
        WorthUiVirtualizedDataFrameDenialReason::OffsetPaginationSubstitute
    );
    assert_eq!(denial.counters().offset_pagination_substitute_count(), 1);
    assert_eq!(denial.counters().denial_count(), 1);
}

#[test]
fn virtualized_data_plan_requires_data_and_query_lane_support() {
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
fn virtualized_data_plan_rejects_stale_lane_admission_receipt() {
    let denial = virtualized_data_denial_for_stale_lane_admission();

    assert_eq!(
        denial.reason(),
        WorthUiVirtualizedDataPlanDenialReason::LaneAdmissionPlanMismatch
    );
    assert_eq!(denial.counters().certification_failure_count(), 1);
    assert_eq!(denial.counters().data_plan_row_count(), 0);
}

#[test]
fn virtualized_data_frame_rejects_stale_or_nondata_targets() {
    let fixture = virtualized_data_fixture();
    let fresh_handle = first_view_binding_handle(&fixture.allocation);
    let stale_generation =
        WorthUiHandlePlanGeneration::new(fresh_handle.plan_generation().as_u64() ^ 0xfeed);
    let stale_handle = WorthUiViewBindingHandle::new(fresh_handle.plan_index(), stale_generation);
    let absent_handle = WorthUiViewBindingHandle::new(
        fresh_handle.plan_index() + 10_000,
        fresh_handle.plan_generation(),
    );
    let component_handle =
        WorthUiComponentHandle::new(fresh_handle.plan_index(), fresh_handle.plan_generation());
    let range = WorthUiVisibleRange::rows(0, 3).expect("range is valid");

    let stale_denial = fixture
        .runtime
        .execute_virtualized_data_frame(
            &fixture.data_plan,
            WorthUiVirtualizedDataFrameTarget::view_binding(stale_handle, range),
        )
        .expect_err("runtime frame execution denies");
    let absent_denial = fixture
        .runtime
        .execute_virtualized_data_frame(
            &fixture.data_plan,
            WorthUiVirtualizedDataFrameTarget::view_binding(absent_handle, range),
        )
        .expect_err("runtime frame execution denies");
    let component_denial = fixture
        .runtime
        .execute_virtualized_data_frame(
            &fixture.data_plan,
            WorthUiVirtualizedDataFrameTarget::component_for_test(component_handle),
        )
        .expect_err("runtime frame execution denies");

    assert_eq!(
        stale_denial.reason(),
        WorthUiVirtualizedDataFrameDenialReason::TargetGenerationMismatch
    );
    assert_eq!(stale_denial.counters().certification_failure_count(), 1);
    assert_eq!(
        absent_denial.reason(),
        WorthUiVirtualizedDataFrameDenialReason::TargetNotInVirtualizedDataPlan
    );
    assert_eq!(
        component_denial.reason(),
        WorthUiVirtualizedDataFrameDenialReason::NonDataLaneClaim
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

fn first_view_binding_handle(
    allocation: &crate::runtime::WorthUiRuntimeHandleAllocation,
) -> WorthUiViewBindingHandle {
    allocation
        .view_binding_handles()
        .first()
        .copied()
        .expect("fixture has view binding handle")
}
