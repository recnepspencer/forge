use crate::runtime::{
    WorthUiExecutionLane, WorthUiLaneCostRegime, WorthUiLaneFailureMode, WorthUiPlanNodeInput,
    WorthUiPlanNodeInputFamily,
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct WorthUiExecutionLaneDescriptor {
    lane: WorthUiExecutionLane,
    cost_regime: WorthUiLaneCostRegime,
    failure_mode: WorthUiLaneFailureMode,
    query_bound: bool,
}

impl WorthUiExecutionLaneDescriptor {
    pub(crate) fn from_node_input(node_input: &WorthUiPlanNodeInput) -> Self {
        let lane = lane_for_node_input(node_input);
        Self::for_lane(lane, node_input.query_binding_identity().is_some())
    }

    pub(crate) fn for_lane(lane: WorthUiExecutionLane, query_bound: bool) -> Self {
        let (cost_regime, failure_mode) = lane_cost_and_failure(lane);
        Self {
            lane,
            cost_regime,
            failure_mode,
            query_bound,
        }
    }

    pub fn lane(&self) -> WorthUiExecutionLane {
        self.lane
    }

    pub fn cost_regime(&self) -> WorthUiLaneCostRegime {
        self.cost_regime
    }

    pub fn failure_mode(&self) -> WorthUiLaneFailureMode {
        self.failure_mode
    }

    pub fn is_query_bound(&self) -> bool {
        self.query_bound
    }
}

pub(crate) fn lane_for_family(family: WorthUiPlanNodeInputFamily) -> WorthUiExecutionLane {
    match family {
        WorthUiPlanNodeInputFamily::ComponentInvocation
        | WorthUiPlanNodeInputFamily::LayoutRegion
        | WorthUiPlanNodeInputFamily::ChildRange
        | WorthUiPlanNodeInputFamily::StateSlot => WorthUiExecutionLane::OrdinaryWidgetShell,
        WorthUiPlanNodeInputFamily::QueryViewBinding => WorthUiExecutionLane::QueryBound,
        WorthUiPlanNodeInputFamily::Command => WorthUiExecutionLane::CommandSurface,
        WorthUiPlanNodeInputFamily::TokenStyle => WorthUiExecutionLane::StyleToken,
        WorthUiPlanNodeInputFamily::Accessibility | WorthUiPlanNodeInputFamily::DiagnosticsRef => {
            WorthUiExecutionLane::DiagnosticsProjection
        }
        WorthUiPlanNodeInputFamily::LanePartitionRef => WorthUiExecutionLane::LaneBoundary,
        WorthUiPlanNodeInputFamily::CanvasSpatial => WorthUiExecutionLane::CanvasSpatial,
        WorthUiPlanNodeInputFamily::RealtimeOverlay => WorthUiExecutionLane::RealtimeOverlayHud,
        WorthUiPlanNodeInputFamily::RenderResourceRef => WorthUiExecutionLane::RenderResource,
    }
}

fn lane_for_node_input(node_input: &WorthUiPlanNodeInput) -> WorthUiExecutionLane {
    lane_for_family(node_input.family())
}

fn lane_cost_and_failure(
    lane: WorthUiExecutionLane,
) -> (WorthUiLaneCostRegime, WorthUiLaneFailureMode) {
    match lane {
        WorthUiExecutionLane::OrdinaryWidgetShell
        | WorthUiExecutionLane::CommandSurface
        | WorthUiExecutionLane::StyleToken => (
            WorthUiLaneCostRegime::LocalTraversal,
            WorthUiLaneFailureMode::LocalWidgetFailure,
        ),
        WorthUiExecutionLane::VirtualizedData => (
            WorthUiLaneCostRegime::WindowedTraversal,
            WorthUiLaneFailureMode::WindowInvalidationFailure,
        ),
        WorthUiExecutionLane::CanvasSpatial => (
            WorthUiLaneCostRegime::SpatialIndexTraversal,
            WorthUiLaneFailureMode::SpatialHitTestFailure,
        ),
        WorthUiExecutionLane::RealtimeOverlayHud => (
            WorthUiLaneCostRegime::FrameSynchronizedTraversal,
            WorthUiLaneFailureMode::RealtimeFrameMiss,
        ),
        WorthUiExecutionLane::QueryBound => (
            WorthUiLaneCostRegime::QueryRuntimeBacked,
            WorthUiLaneFailureMode::QuerySupportDenial,
        ),
        WorthUiExecutionLane::DiagnosticsProjection
        | WorthUiExecutionLane::LaneBoundary
        | WorthUiExecutionLane::RenderResource
        | WorthUiExecutionLane::SpecialCaseExtension => (
            WorthUiLaneCostRegime::BoundaryOnly,
            WorthUiLaneFailureMode::BoundaryAdmissionFailure,
        ),
    }
}
