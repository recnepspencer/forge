use crate::runtime::{WorthUiPlanExecutionLane, WorthUiPlanNodeInputFamily};

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
        WorthUiPlanNodeInputFamily::RenderResourceRef => 11,
        WorthUiPlanNodeInputFamily::StateSlot => 12,
        WorthUiPlanNodeInputFamily::CanvasSpatial => 13,
        WorthUiPlanNodeInputFamily::RealtimeOverlay => 14,
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
        WorthUiPlanExecutionLane::RenderResource => 8,
        WorthUiPlanExecutionLane::CanvasSpatial => 9,
        WorthUiPlanExecutionLane::RealtimeOverlay => 10,
    }
}
