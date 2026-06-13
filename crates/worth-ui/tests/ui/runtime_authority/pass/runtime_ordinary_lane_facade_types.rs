use worth_ui::facade::{
    WorthUiOrdinaryExecutionLane, WorthUiOrdinaryFrameTarget, WorthUiOrdinaryLaneCertification,
    WorthUiOrdinaryLaneCounters, WorthUiOrdinaryLaneFrameDenial,
    WorthUiOrdinaryLaneFrameDenialReason, WorthUiOrdinaryLaneFrameReceipt,
    WorthUiOrdinaryLaneNode, WorthUiOrdinaryLanePlan, WorthUiOrdinaryLanePlanDenial,
    WorthUiOrdinaryLanePlanDenialReason,
};

fn accepts_ordinary_lane_types(
    _lane: Option<WorthUiOrdinaryExecutionLane>,
    _target: Option<WorthUiOrdinaryFrameTarget>,
    _certification: Option<WorthUiOrdinaryLaneCertification>,
    _counters: Option<WorthUiOrdinaryLaneCounters>,
    _frame_denial: Option<WorthUiOrdinaryLaneFrameDenial>,
    _frame_reason: Option<WorthUiOrdinaryLaneFrameDenialReason>,
    _frame_receipt: Option<WorthUiOrdinaryLaneFrameReceipt>,
    _node: Option<WorthUiOrdinaryLaneNode>,
    _plan: Option<WorthUiOrdinaryLanePlan>,
    _plan_denial: Option<WorthUiOrdinaryLanePlanDenial>,
    _plan_reason: Option<WorthUiOrdinaryLanePlanDenialReason>,
) {
}

fn main() {
    let _ = WorthUiOrdinaryExecutionLane::WidgetShell;
    let _ = WorthUiOrdinaryLaneFrameDenialReason::NonOrdinaryLaneClaim;
    let _ = WorthUiOrdinaryLanePlanDenialReason::LaneAdmissionMissingOrdinarySupport;
    let _ = WorthUiOrdinaryFrameTarget::root_shell();
    accepts_ordinary_lane_types(
        None, None, None, None, None, None, None, None, None, None, None,
    );
}
