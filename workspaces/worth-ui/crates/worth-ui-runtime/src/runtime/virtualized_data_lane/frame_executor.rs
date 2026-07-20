use crate::runtime::handle_allocation::resolve_handle_row;
use crate::runtime::{
    WorthUiHandleResolutionOutcome, WorthUiPlanNodeInputFamily, WorthUiVirtualizedDataCounters,
    WorthUiVirtualizedDataFrameDenial, WorthUiVirtualizedDataFrameDenialReason,
    WorthUiVirtualizedDataFrameReceipt, WorthUiVirtualizedDataFrameTarget,
    WorthUiVirtualizedDataLane, WorthUiVirtualizedDataPlan,
};

pub(crate) struct WorthUiVirtualizedDataFrameExecutor;

impl WorthUiVirtualizedDataFrameExecutor {
    pub(crate) fn execute(
        plan: &WorthUiVirtualizedDataPlan,
        query_binding: &worth_ui_query_binding::WorthUiRuntimeQueryBinding,
        target: WorthUiVirtualizedDataFrameTarget,
    ) -> Result<WorthUiVirtualizedDataFrameReceipt, WorthUiVirtualizedDataFrameDenial> {
        let mut counters = WorthUiVirtualizedDataCounters::default();
        let handle = target.handle();
        let range = target.visible_range();
        let plan_index = handle.plan_index();
        counters.record_direct_row_lookup();
        let (row, resolution_evidence) = resolve_handle_row(
            plan.handle_receipt().arena_identity(),
            WorthUiPlanNodeInputFamily::QueryViewBinding,
            handle.locator(),
            |index| plan.row_for_plan_index(index),
            |row| row.runtime_handle(),
        )
        .map_err(|evidence| {
            let reason = resolution_denial(evidence.outcome());
            if reason == WorthUiVirtualizedDataFrameDenialReason::TargetNotInVirtualizedDataPlan {
                counters.record_denial();
            } else {
                counters.record_certification_failure();
            }
            WorthUiVirtualizedDataFrameDenial::new(reason, Some(plan_index), counters)
                .with_resolution_evidence(evidence)
        })?;

        counters.record_evidence_reference_lookup();
        let evidence = query_binding
            .execution_evidence_for(row.installed_reference())
            .map_err(|denial| {
                counters.record_denial();
                WorthUiVirtualizedDataFrameDenial::new(
                    query_evidence_denial(denial),
                    Some(plan_index),
                    counters,
                )
            })?;
        counters.record_visible_range(range);
        Ok(WorthUiVirtualizedDataFrameReceipt::new(
            super::WorthUiVirtualizedDataFrameReceiptInput {
                target,
                lane: WorthUiVirtualizedDataLane::from_visible_range(range),
                visible_range: range,
                touched_plan_index: row.plan_index(),
                touched_runtime_handle: row.runtime_handle(),
                binding_identity: row.binding_identity_reference(),
                evidence,
                counters,
                certification: plan.certification(),
                resolution_evidence,
                work_scope: crate::runtime::WorthUiFrameWorkScope::new(
                    u64::from(range.row_count()) * u64::from(range.column_count()),
                    counters.visible_cell_touch_count() as u64,
                ),
            },
        ))
    }
}

fn resolution_denial(
    outcome: WorthUiHandleResolutionOutcome,
) -> WorthUiVirtualizedDataFrameDenialReason {
    match outcome {
        WorthUiHandleResolutionOutcome::TargetMissing => {
            WorthUiVirtualizedDataFrameDenialReason::TargetNotInVirtualizedDataPlan
        }
        WorthUiHandleResolutionOutcome::ForeignSessionArena => {
            WorthUiVirtualizedDataFrameDenialReason::TargetArenaMismatch
        }
        WorthUiHandleResolutionOutcome::StaleSlotGeneration => {
            WorthUiVirtualizedDataFrameDenialReason::TargetSlotGenerationMismatch
        }
        WorthUiHandleResolutionOutcome::WrongFamily => {
            WorthUiVirtualizedDataFrameDenialReason::TargetFamilyMismatch
        }
        WorthUiHandleResolutionOutcome::Resolved => unreachable!("resolved evidence is not denial"),
    }
}

fn query_evidence_denial(
    denial: worth_ui_query_binding::WorthUiQueryViewExecutionEvidenceDenial,
) -> WorthUiVirtualizedDataFrameDenialReason {
    match denial {
        worth_ui_query_binding::WorthUiQueryViewExecutionEvidenceDenial::QueryNotInstalled => {
            WorthUiVirtualizedDataFrameDenialReason::QueryNotInstalled
        }
        worth_ui_query_binding::WorthUiQueryViewExecutionEvidenceDenial::ForeignInstalledReference => {
            WorthUiVirtualizedDataFrameDenialReason::ForeignInstalledReference
        }
        worth_ui_query_binding::WorthUiQueryViewExecutionEvidenceDenial::ProjectionNotAdmitted => {
            WorthUiVirtualizedDataFrameDenialReason::ProjectionNotAdmitted
        }
    }
}
