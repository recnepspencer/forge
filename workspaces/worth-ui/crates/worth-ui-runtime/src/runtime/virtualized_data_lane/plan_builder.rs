use crate::runtime::{
    WorthUiExecutionLane, WorthUiExecutionPlan, WorthUiLaneAdmission, WorthUiLaneSupportStatus,
    WorthUiPlanNodeInputFamily, WorthUiVirtualizedDataCounters, WorthUiVirtualizedDataPlan,
    WorthUiVirtualizedDataPlanDenial, WorthUiVirtualizedDataPlanDenialReason,
};

pub(crate) struct WorthUiVirtualizedDataPlanBuilder;

impl WorthUiVirtualizedDataPlanBuilder {
    pub(crate) fn build(
        execution_plan: &WorthUiExecutionPlan,
        lane_admission: &WorthUiLaneAdmission,
    ) -> Result<WorthUiVirtualizedDataPlan, WorthUiVirtualizedDataPlanDenial> {
        let mut counters = WorthUiVirtualizedDataCounters::default();
        require_lane_support(
            lane_admission,
            WorthUiExecutionLane::VirtualizedData,
            WorthUiVirtualizedDataPlanDenialReason::LaneAdmissionMissingVirtualizedDataSupport,
            &mut counters,
        )?;
        require_lane_support(
            lane_admission,
            WorthUiExecutionLane::QueryBound,
            WorthUiVirtualizedDataPlanDenialReason::LaneAdmissionMissingQuerySupport,
            &mut counters,
        )?;
        require_matching_lane_admission(execution_plan, lane_admission, &mut counters)?;

        counters.record_family_index_read();
        let slots = execution_plan
            .regional_family_slot_view([WorthUiPlanNodeInputFamily::QueryViewBinding]);
        if slots.is_empty() {
            counters.record_denial();
            return Err(WorthUiVirtualizedDataPlanDenial::new(
                WorthUiVirtualizedDataPlanDenialReason::NoVirtualizedDataRows,
                counters,
            ));
        }

        counters.record_data_plan_rows(slots.len());
        counters.record_unrelated_plan_rows(execution_plan.region_count() - slots.len());
        let digest = fold(
            execution_plan.handle_receipt().basis_digest(),
            execution_plan
                .regional_family_semantic_digest(WorthUiPlanNodeInputFamily::QueryViewBinding),
        );
        Ok(WorthUiVirtualizedDataPlan::new(
            super::WorthUiVirtualizedDataPlanInput {
                handle_receipt: execution_plan.handle_receipt(),
                support_digest: lane_admission.support_digest(),
                data_plan_digest: digest,
                region_store: execution_plan.regional_store_clone(),
                query_slots: slots,
                counters,
            },
        ))
    }
}

fn require_matching_lane_admission(
    execution_plan: &WorthUiExecutionPlan,
    lane_admission: &WorthUiLaneAdmission,
    counters: &mut WorthUiVirtualizedDataCounters,
) -> Result<(), WorthUiVirtualizedDataPlanDenial> {
    if lane_admission.plan_input_basis_digest() == execution_plan.handle_receipt().basis_digest() {
        return Ok(());
    }
    counters.record_certification_failure();
    Err(WorthUiVirtualizedDataPlanDenial::new(
        WorthUiVirtualizedDataPlanDenialReason::LaneAdmissionPlanMismatch,
        *counters,
    ))
}

fn require_lane_support(
    lane_admission: &WorthUiLaneAdmission,
    lane: WorthUiExecutionLane,
    reason: WorthUiVirtualizedDataPlanDenialReason,
    counters: &mut WorthUiVirtualizedDataCounters,
) -> Result<(), WorthUiVirtualizedDataPlanDenial> {
    if lane_admission
        .posture_for(lane)
        .is_some_and(|row| row.status() == WorthUiLaneSupportStatus::Supported)
    {
        return Ok(());
    }
    counters.record_denial();
    Err(WorthUiVirtualizedDataPlanDenial::new(reason, *counters))
}

fn fold(digest: u64, value: u64) -> u64 {
    (digest ^ value).wrapping_mul(0x100000001b3)
}
