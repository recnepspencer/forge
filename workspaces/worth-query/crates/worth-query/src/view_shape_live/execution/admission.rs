use crate::live::BridgeChangeSummary;

use super::super::artifact::{
    GroupedLiveViewShapeArtifact, LiveViewShapeArtifact, LiveViewShapeExecutionEnvelope,
};
use super::super::error::{ViewShapeLiveError, ViewShapeLiveFailureClass};
use super::super::family::LiveViewShapeFamily;
use super::super::grouped_execution::GroupedExecutionSurfaceArtifact;
use super::core_execution::execute_live_view_shape_change_inner;

pub(crate) fn execute_live_view_shape_change(
    live_view: &LiveViewShapeArtifact,
    change: &BridgeChangeSummary,
) -> Result<LiveViewShapeExecutionEnvelope, ViewShapeLiveError> {
    if live_view.lowering().family() == LiveViewShapeFamily::KanbanGrouped {
        return Err(ViewShapeLiveError::new(
            ViewShapeLiveFailureClass::GroupedRefreshForbidden,
            "grouped live execution requires grouped admission and next authoritative grouped truth",
            live_view.counters().clone(),
        ));
    }
    execute_live_view_shape_change_inner(live_view, change, None)
}

pub(crate) fn admit_grouped_live_view<'a>(
    live_view: &'a LiveViewShapeArtifact,
) -> Result<GroupedLiveViewShapeArtifact<'a>, ViewShapeLiveError> {
    if live_view.lowering().family() != LiveViewShapeFamily::KanbanGrouped {
        return Err(ViewShapeLiveError::new(
            ViewShapeLiveFailureClass::GroupedRefreshForbidden,
            format!(
                "view family '{}' is not admitted for grouped live execution",
                live_view.lowering().family().as_str()
            ),
            live_view.counters().clone(),
        ));
    }
    if live_view.grouped_state().is_none() || live_view.grouped_policy().is_none() {
        return Err(ViewShapeLiveError::new(
            ViewShapeLiveFailureClass::GroupedBaselineRequired,
            "grouped live artifact must retain grouped desired-state and grouped policy",
            live_view.counters().clone(),
        ));
    }
    Ok(GroupedLiveViewShapeArtifact::new(live_view))
}

pub(crate) fn execute_grouped_live_view_shape_change(
    live_view: GroupedLiveViewShapeArtifact<'_>,
    change: &BridgeChangeSummary,
    next_grouped_execution: &GroupedExecutionSurfaceArtifact,
) -> Result<LiveViewShapeExecutionEnvelope, ViewShapeLiveError> {
    execute_live_view_shape_change_inner(
        live_view.live_view(),
        change,
        Some(next_grouped_execution),
    )
}
