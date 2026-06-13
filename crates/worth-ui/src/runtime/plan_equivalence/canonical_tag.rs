use crate::runtime::{
    WorthUiEguiBoundaryContact, WorthUiEguiBoundaryInput, WorthUiPlanExecutionLane,
    WorthUiPlanNodeInputFamily,
};

pub(super) fn canonical_plan_node_family_tag(family: WorthUiPlanNodeInputFamily) -> u64 {
    match family {
        WorthUiPlanNodeInputFamily::ComponentInvocation => 1,
        WorthUiPlanNodeInputFamily::ChildRange => 2,
        WorthUiPlanNodeInputFamily::Command => 3,
        WorthUiPlanNodeInputFamily::TokenStyle => 4,
        WorthUiPlanNodeInputFamily::LayoutRegion => 5,
        WorthUiPlanNodeInputFamily::QueryViewBinding => 6,
        WorthUiPlanNodeInputFamily::Accessibility => 7,
        WorthUiPlanNodeInputFamily::DiagnosticsRef => 8,
        WorthUiPlanNodeInputFamily::LanePartitionRef => 9,
        WorthUiPlanNodeInputFamily::EguiBoundaryRef => 10,
        WorthUiPlanNodeInputFamily::RenderResourceRef => 11,
    }
}

pub(super) fn canonical_execution_lane_tag(lane: WorthUiPlanExecutionLane) -> u64 {
    match lane {
        WorthUiPlanExecutionLane::UiStructure => 1,
        WorthUiPlanExecutionLane::QueryView => 2,
        WorthUiPlanExecutionLane::Command => 3,
        WorthUiPlanExecutionLane::Style => 4,
        WorthUiPlanExecutionLane::Diagnostics => 5,
        WorthUiPlanExecutionLane::LaneBoundary => 6,
        WorthUiPlanExecutionLane::EguiBoundary => 7,
        WorthUiPlanExecutionLane::RenderResource => 8,
    }
}

pub(super) fn canonical_egui_boundary_input_tag(input: WorthUiEguiBoundaryInput) -> u64 {
    match input {
        WorthUiEguiBoundaryInput::Component => 1,
        WorthUiEguiBoundaryInput::Surface => 2,
        WorthUiEguiBoundaryInput::QueryBinding => 3,
        WorthUiEguiBoundaryInput::Token => 4,
        WorthUiEguiBoundaryInput::Diagnostics => 5,
    }
}

pub(super) fn canonical_egui_boundary_contact_tag(contact: WorthUiEguiBoundaryContact) -> u64 {
    match contact {
        WorthUiEguiBoundaryContact::Context => 1,
        WorthUiEguiBoundaryContact::Ui => 2,
        WorthUiEguiBoundaryContact::Response => 3,
        WorthUiEguiBoundaryContact::Id => 4,
        WorthUiEguiBoundaryContact::Input => 5,
        WorthUiEguiBoundaryContact::LayoutAllocation => 6,
        WorthUiEguiBoundaryContact::PaintSubmission => 7,
        WorthUiEguiBoundaryContact::MemoryStateBridge => 8,
        WorthUiEguiBoundaryContact::FrameTiming => 9,
    }
}
