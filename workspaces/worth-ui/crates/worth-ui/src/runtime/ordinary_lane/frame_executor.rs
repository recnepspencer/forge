use crate::runtime::ordinary_lane::frame_target::WorthUiOrdinaryFrameTargetKind;
use crate::runtime::{
    WorthUiOrdinaryExecutionLane, WorthUiOrdinaryFrameTarget, WorthUiOrdinaryLaneCounters,
    WorthUiOrdinaryLaneFrameDenial, WorthUiOrdinaryLaneFrameDenialReason,
    WorthUiOrdinaryLaneFrameReceipt, WorthUiOrdinaryLaneNode, WorthUiOrdinaryLanePlan,
};

pub(crate) struct WorthUiOrdinaryLaneFrameExecutor;

impl WorthUiOrdinaryLaneFrameExecutor {
    pub(crate) fn execute(
        plan: &WorthUiOrdinaryLanePlan,
        target: WorthUiOrdinaryFrameTarget,
    ) -> Result<WorthUiOrdinaryLaneFrameReceipt, WorthUiOrdinaryLaneFrameDenial> {
        let mut counters = WorthUiOrdinaryLaneCounters::default();
        counters.merge_plan_counters(plan.counters());

        match target.kind() {
            WorthUiOrdinaryFrameTargetKind::RootShell => execute_root_shell(plan, target, counters),
            WorthUiOrdinaryFrameTargetKind::Component(handle) => execute_indexed_target(
                plan,
                target,
                handle.plan_index(),
                handle.plan_generation(),
                WorthUiOrdinaryExecutionLane::WidgetShell,
                counters,
            ),
            WorthUiOrdinaryFrameTargetKind::Command(handle) => execute_indexed_target(
                plan,
                target,
                handle.plan_index(),
                handle.plan_generation(),
                WorthUiOrdinaryExecutionLane::CommandSurface,
                counters,
            ),
            WorthUiOrdinaryFrameTargetKind::TokenSupport(handle) => execute_indexed_target(
                plan,
                target,
                handle.plan_index(),
                handle.plan_generation(),
                WorthUiOrdinaryExecutionLane::TokenStyleSupport,
                counters,
            ),
            #[cfg(test)]
            WorthUiOrdinaryFrameTargetKind::VirtualizedData(plan_index)
            | WorthUiOrdinaryFrameTargetKind::CanvasSpatial(plan_index)
            | WorthUiOrdinaryFrameTargetKind::RealtimeOverlay(plan_index) => {
                counters.record_non_ordinary_claim_denial();
                Err(WorthUiOrdinaryLaneFrameDenial::new(
                    WorthUiOrdinaryLaneFrameDenialReason::NonOrdinaryLaneClaim,
                    Some(plan_index),
                    counters,
                ))
            }
            #[cfg(test)]
            WorthUiOrdinaryFrameTargetKind::ParseSourceForTest => {
                counters.record_source_parse();
                counters.record_denial();
                Err(WorthUiOrdinaryLaneFrameDenial::new(
                    WorthUiOrdinaryLaneFrameDenialReason::FramePathSourceParse,
                    None,
                    counters,
                ))
            }
            #[cfg(test)]
            WorthUiOrdinaryFrameTargetKind::RegistryLookupForTest => {
                counters.record_registry_lookup();
                counters.record_component_string_resolution();
                counters.record_command_string_resolution();
                counters.record_denial();
                Err(WorthUiOrdinaryLaneFrameDenial::new(
                    WorthUiOrdinaryLaneFrameDenialReason::FramePathRegistryLookup,
                    None,
                    counters,
                ))
            }
            #[cfg(test)]
            WorthUiOrdinaryFrameTargetKind::ArtifactScanForTest => {
                counters.record_artifact_tree_scan();
                counters.record_denial();
                Err(WorthUiOrdinaryLaneFrameDenial::new(
                    WorthUiOrdinaryLaneFrameDenialReason::FramePathArtifactScan,
                    None,
                    counters,
                ))
            }
            #[cfg(test)]
            WorthUiOrdinaryFrameTargetKind::FullPlanScanForTest => {
                counters.record_full_plan_scan();
                counters.record_certification_failure();
                Err(WorthUiOrdinaryLaneFrameDenial::new(
                    WorthUiOrdinaryLaneFrameDenialReason::FullPlanScanCertificationFailure,
                    None,
                    counters,
                ))
            }
        }
    }
}

fn execute_root_shell(
    plan: &WorthUiOrdinaryLanePlan,
    target: WorthUiOrdinaryFrameTarget,
    mut counters: WorthUiOrdinaryLaneCounters,
) -> Result<WorthUiOrdinaryLaneFrameReceipt, WorthUiOrdinaryLaneFrameDenial> {
    let mut touched_plan_indexes = Vec::new();
    let mut touched_runtime_handles = Vec::new();
    for row in plan.rows() {
        if matches!(
            row.lane(),
            WorthUiOrdinaryExecutionLane::WidgetShell
                | WorthUiOrdinaryExecutionLane::ShellRegion
                | WorthUiOrdinaryExecutionLane::ChildRangeTraversal
        ) {
            touch_row(row, &mut counters);
            touched_plan_indexes.push(row.plan_index());
            touched_runtime_handles.push(row.runtime_handle());
        }
    }

    if touched_plan_indexes.is_empty() {
        counters.record_denial();
        return Err(WorthUiOrdinaryLaneFrameDenial::new(
            WorthUiOrdinaryLaneFrameDenialReason::TargetNotInOrdinaryPlan,
            None,
            counters,
        ));
    }

    Ok(WorthUiOrdinaryLaneFrameReceipt::new(
        target,
        touched_plan_indexes,
        touched_runtime_handles,
        counters,
        plan.certification(WorthUiOrdinaryExecutionLane::WidgetShell),
    ))
}

fn execute_indexed_target(
    plan: &WorthUiOrdinaryLanePlan,
    target: WorthUiOrdinaryFrameTarget,
    plan_index: u32,
    plan_generation: crate::runtime::WorthUiHandlePlanGeneration,
    expected_lane: WorthUiOrdinaryExecutionLane,
    mut counters: WorthUiOrdinaryLaneCounters,
) -> Result<WorthUiOrdinaryLaneFrameReceipt, WorthUiOrdinaryLaneFrameDenial> {
    let Some(row) = plan.row_for_plan_index(plan_index) else {
        counters.record_denial();
        return Err(WorthUiOrdinaryLaneFrameDenial::new(
            WorthUiOrdinaryLaneFrameDenialReason::TargetNotInOrdinaryPlan,
            Some(plan_index),
            counters,
        ));
    };

    if row.runtime_handle().plan_generation() != plan_generation {
        counters.record_certification_failure();
        return Err(WorthUiOrdinaryLaneFrameDenial::new(
            WorthUiOrdinaryLaneFrameDenialReason::TargetGenerationMismatch,
            Some(plan_index),
            counters,
        ));
    }

    if row.lane() != expected_lane {
        counters.record_denial();
        return Err(WorthUiOrdinaryLaneFrameDenial::new(
            WorthUiOrdinaryLaneFrameDenialReason::TargetNotInOrdinaryPlan,
            Some(plan_index),
            counters,
        ));
    }

    touch_row(row, &mut counters);
    Ok(WorthUiOrdinaryLaneFrameReceipt::new(
        target,
        vec![row.plan_index()],
        vec![row.runtime_handle()],
        counters,
        plan.certification(expected_lane),
    ))
}

fn touch_row(row: &WorthUiOrdinaryLaneNode, counters: &mut WorthUiOrdinaryLaneCounters) {
    counters.record_frame_row_touch();
    if row.child_range().is_some() {
        counters.record_child_range_touch();
    }
    match row.lane() {
        WorthUiOrdinaryExecutionLane::CommandSurface => counters.record_command_surface_touch(),
        WorthUiOrdinaryExecutionLane::TokenStyleSupport => counters.record_token_support_touch(),
        _ => {}
    }
}
