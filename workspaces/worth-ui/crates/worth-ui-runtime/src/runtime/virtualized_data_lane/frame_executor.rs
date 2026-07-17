use crate::runtime::virtualized_data_lane::frame_target::WorthUiVirtualizedDataFrameTargetKind;
use crate::runtime::{
    WorthUiHandlePlanGeneration, WorthUiVirtualizedDataCounters, WorthUiVirtualizedDataFrameDenial,
    WorthUiVirtualizedDataFrameDenialReason, WorthUiVirtualizedDataFrameReceipt,
    WorthUiVirtualizedDataFrameTarget, WorthUiVirtualizedDataLane, WorthUiVirtualizedDataPlan,
    WorthUiVisibleRange,
};

pub(crate) struct WorthUiVirtualizedDataFrameExecutor;

impl WorthUiVirtualizedDataFrameExecutor {
    pub(crate) fn execute(
        plan: &WorthUiVirtualizedDataPlan,
        target: WorthUiVirtualizedDataFrameTarget,
    ) -> Result<WorthUiVirtualizedDataFrameReceipt, WorthUiVirtualizedDataFrameDenial> {
        let mut counters = WorthUiVirtualizedDataCounters::default();
        counters.merge_plan_counters(plan.counters());

        match target.kind() {
            WorthUiVirtualizedDataFrameTargetKind::ViewBinding(handle, range) => {
                execute_view_binding(
                    plan,
                    target,
                    handle.plan_index(),
                    handle.plan_generation(),
                    range,
                    counters,
                )
            }
            #[cfg(test)]
            WorthUiVirtualizedDataFrameTargetKind::FullCollectionScan(handle) => {
                counters.record_full_collection_scan();
                counters.record_certification_failure();
                Err(WorthUiVirtualizedDataFrameDenial::new(
                    WorthUiVirtualizedDataFrameDenialReason::FullCollectionScanCertificationFailure,
                    Some(handle.plan_index()),
                    counters,
                ))
            }
            #[cfg(test)]
            WorthUiVirtualizedDataFrameTargetKind::OffsetPagination(handle) => {
                counters.record_offset_pagination_substitute();
                counters.record_denial();
                Err(WorthUiVirtualizedDataFrameDenial::new(
                    WorthUiVirtualizedDataFrameDenialReason::OffsetPaginationSubstitute,
                    Some(handle.plan_index()),
                    counters,
                ))
            }
            #[cfg(test)]
            WorthUiVirtualizedDataFrameTargetKind::Component(handle) => {
                counters.record_denial();
                Err(WorthUiVirtualizedDataFrameDenial::new(
                    WorthUiVirtualizedDataFrameDenialReason::NonDataLaneClaim,
                    Some(handle.plan_index()),
                    counters,
                ))
            }
        }
    }
}

fn execute_view_binding(
    plan: &WorthUiVirtualizedDataPlan,
    target: WorthUiVirtualizedDataFrameTarget,
    plan_index: u32,
    plan_generation: WorthUiHandlePlanGeneration,
    range: WorthUiVisibleRange,
    mut counters: WorthUiVirtualizedDataCounters,
) -> Result<WorthUiVirtualizedDataFrameReceipt, WorthUiVirtualizedDataFrameDenial> {
    let Some(row) = plan.row_for_plan_index(plan_index) else {
        counters.record_denial();
        return Err(WorthUiVirtualizedDataFrameDenial::new(
            WorthUiVirtualizedDataFrameDenialReason::TargetNotInVirtualizedDataPlan,
            Some(plan_index),
            counters,
        ));
    };

    if row.runtime_handle().plan_generation() != plan_generation {
        counters.record_certification_failure();
        return Err(WorthUiVirtualizedDataFrameDenial::new(
            WorthUiVirtualizedDataFrameDenialReason::TargetGenerationMismatch,
            Some(plan_index),
            counters,
        ));
    }

    counters.record_visible_range(range);
    Ok(WorthUiVirtualizedDataFrameReceipt::new(
        super::WorthUiVirtualizedDataFrameReceiptInput {
            target,
            lane: WorthUiVirtualizedDataLane::from_visible_range(range),
            visible_range: range,
            touched_plan_indexes: vec![row.plan_index()],
            touched_runtime_handles: vec![row.runtime_handle()],
            query_patch_posture: row.query_patch_posture().clone(),
            counters,
            certification: plan.certification(),
        },
    ))
}
