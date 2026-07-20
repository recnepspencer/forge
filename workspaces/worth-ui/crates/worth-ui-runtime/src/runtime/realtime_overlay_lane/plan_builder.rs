use crate::runtime::{
    WorthUiExecutionLane, WorthUiExecutionPlan, WorthUiHudPlan, WorthUiHudPlanDenial,
    WorthUiHudPlanDenialReason, WorthUiLaneAdmission, WorthUiLaneSupportStatus,
    WorthUiPlanNodeInputFamily, WorthUiRealtimeLaneCounters,
};

pub(crate) struct WorthUiHudPlanBuilder;

impl WorthUiHudPlanBuilder {
    pub(crate) fn build(
        execution_plan: &WorthUiExecutionPlan,
        lane_admission: &WorthUiLaneAdmission,
        host_binding: crate::facade::WorthUiHostPlanBinding,
    ) -> Result<WorthUiHudPlan, WorthUiHudPlanDenial> {
        let mut counters = WorthUiRealtimeLaneCounters::default();
        let row_count =
            execution_plan.regional_family_count(WorthUiPlanNodeInputFamily::RealtimeOverlay);
        if row_count == 0 {
            counters.record_denial();
            return Err(WorthUiHudPlanDenial::new(
                WorthUiHudPlanDenialReason::NoHudRows,
                counters,
            ));
        }
        if lane_admission
            .posture_for(WorthUiExecutionLane::RealtimeOverlayHud)
            .is_none_or(|row| row.status() != WorthUiLaneSupportStatus::Supported)
        {
            counters.record_denial();
            return Err(WorthUiHudPlanDenial::new(
                WorthUiHudPlanDenialReason::LaneAdmissionMissingRealtimeSupport,
                counters,
            ));
        }
        if !host_binding.realtime_overlay_supported() {
            counters.record_denial();
            return Err(WorthUiHudPlanDenial::new(
                WorthUiHudPlanDenialReason::HostSupportMissing,
                counters,
            ));
        }

        let slots =
            execution_plan.regional_family_slot_view([WorthUiPlanNodeInputFamily::RealtimeOverlay]);
        let store = execution_plan.regional_store_clone();
        if let Some((budget_millis, declared_cost_millis)) =
            store.first_realtime_budget_exhaustion()
        {
            counters.record_denial();
            return Err(WorthUiHudPlanDenial::new(
                WorthUiHudPlanDenialReason::FrameBudgetExhausted {
                    budget_millis,
                    declared_cost_millis,
                },
                counters,
            ));
        }
        counters.record_plan_rows(row_count);
        let realtime_meaning_digest = execution_plan
            .regional_family_semantic_digest(WorthUiPlanNodeInputFamily::RealtimeOverlay);
        let digest = fold(
            realtime_meaning_digest,
            fold(
                lane_admission.support_digest(),
                host_binding.capability_profile_digest(),
            ),
        );
        Ok(WorthUiHudPlan::new(super::WorthUiHudPlanInput {
            handle_receipt: execution_plan.handle_receipt(),
            support_digest: lane_admission.support_digest(),
            hud_plan_digest: digest,
            host_binding,
            region_store: store,
            realtime_slots: slots,
            counters,
        }))
    }
}

fn fold(digest: u64, value: u64) -> u64 {
    (digest ^ value).wrapping_mul(0x100000001b3)
}
