use crate::live::{execute_live_change, BridgeChangeSummary};

use super::super::artifact::{LiveViewShapeArtifact, LiveViewShapeExecutionEnvelope};
use super::super::error::ViewShapeLiveError;
use super::super::family::LiveViewShapeFamily;
use super::super::grouped_execution::GroupedExecutionSurfaceArtifact;
use super::grouped_transition::{resolve_grouped_transition, GroupedLiveTransitionInput};
use super::ordinary_patch::resolve_ordinary_patch;
use super::result_assembly::assemble_live_execution_envelope;

pub(super) fn execute_live_view_shape_change_inner(
    live_view: &LiveViewShapeArtifact,
    change: &BridgeChangeSummary,
    next_grouped_execution: Option<&GroupedExecutionSurfaceArtifact>,
) -> Result<LiveViewShapeExecutionEnvelope, ViewShapeLiveError> {
    let family = live_view.lowering().family();
    let core_execution = execute_live_change(live_view.core_live_plan(), change);
    let mut counters = live_view.counters().clone();
    if let Ok(core_execution_counters) = &core_execution {
        counters = counters.with_core(core_execution_counters.counters().clone());
    }

    if family == LiveViewShapeFamily::KanbanGrouped {
        let assembly = resolve_grouped_transition(GroupedLiveTransitionInput {
            live_view,
            change,
            next_grouped_execution,
            core_execution,
            counters,
        })?;
        return Ok(assemble_live_execution_envelope(assembly));
    }

    let core_execution = core_execution?;
    let assembly = resolve_ordinary_patch(live_view, core_execution, counters)?;
    Ok(assemble_live_execution_envelope(assembly))
}
