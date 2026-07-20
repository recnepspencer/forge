use worth_ui::facade::{
    WorthUiViewBindingHandle, WorthUiVirtualizedDataCertification, WorthUiVirtualizedDataCounters,
    WorthUiVirtualizedDataFrameDenial, WorthUiVirtualizedDataFrameDenialReason,
    WorthUiVirtualizedDataFrameReceipt, WorthUiVirtualizedDataFrameTarget,
    WorthUiVirtualizedDataLane, WorthUiVirtualizedDataNode, WorthUiVirtualizedDataPlan,
    WorthUiVirtualizedDataPlanDenial, WorthUiVirtualizedDataPlanDenialReason,
    WorthUiVirtualizedPlanAvailability, WorthUiVirtualizedPlanSummary,
    WorthUiVirtualizedPlanSummaryDenial, WorthUiVirtualizedPlanSummaryRequest, WorthUiVisibleRange,
    WorthUiVisibleRangeDenial, WorthUiVisibleRangeDenialReason,
};

fn accepts_virtualized_data_lane_types(
    _certification: Option<WorthUiVirtualizedDataCertification>,
    _counters: Option<WorthUiVirtualizedDataCounters>,
    _frame_denial: Option<WorthUiVirtualizedDataFrameDenial>,
    _frame_reason: Option<WorthUiVirtualizedDataFrameDenialReason>,
    _frame_receipt: Option<WorthUiVirtualizedDataFrameReceipt>,
    _target: Option<WorthUiVirtualizedDataFrameTarget>,
    _lane: Option<WorthUiVirtualizedDataLane>,
    _node: Option<WorthUiVirtualizedDataNode>,
    _plan: Option<WorthUiVirtualizedDataPlan>,
    _plan_denial: Option<WorthUiVirtualizedDataPlanDenial>,
    _plan_reason: Option<WorthUiVirtualizedDataPlanDenialReason>,
    _availability: Option<WorthUiVirtualizedPlanAvailability>,
    _summary: Option<WorthUiVirtualizedPlanSummary>,
    _summary_denial: Option<WorthUiVirtualizedPlanSummaryDenial>,
    _summary_request: Option<WorthUiVirtualizedPlanSummaryRequest>,
    _range: Option<WorthUiVisibleRange>,
    _range_denial: Option<WorthUiVisibleRangeDenial>,
    _range_reason: Option<WorthUiVisibleRangeDenialReason>,
    _handle: Option<WorthUiViewBindingHandle>,
) {
}

fn main() {
    let range = WorthUiVisibleRange::grid(0, 20, 0, 8).expect("range is valid");
    let _ = range.end_row_exclusive();
    let _ = WorthUiVirtualizedDataFrameDenialReason::ProjectionNotAdmitted;
    let _ = WorthUiVirtualizedDataLane::CellGrid;
    let _ = WorthUiVirtualizedDataPlanDenialReason::LaneAdmissionMissingVirtualizedDataSupport;
    let _ = WorthUiVisibleRangeDenialReason::RangeOverflow;
    accepts_virtualized_data_lane_types(
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(range),
        None,
        None,
        None,
    );
}
