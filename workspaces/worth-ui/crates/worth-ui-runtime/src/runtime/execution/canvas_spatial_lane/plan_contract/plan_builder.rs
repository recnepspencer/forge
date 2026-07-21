use crate::runtime::{
    WorthUiCanvasSpatialCounters, WorthUiCanvasSpatialPlan, WorthUiCanvasSpatialPlanDenial,
    WorthUiCanvasSpatialPlanDenialReason, WorthUiExecutionLane, WorthUiExecutionPlan,
    WorthUiLaneAdmission, WorthUiLaneSupportStatus, WorthUiPlanNodeInputFamily,
};

pub(crate) struct WorthUiCanvasSpatialPlanBuilder;

impl WorthUiCanvasSpatialPlanBuilder {
    pub(crate) fn build(
        execution_plan: &WorthUiExecutionPlan,
        lane_admission: &WorthUiLaneAdmission,
        host_binding: crate::facade::WorthUiHostPlanBinding,
    ) -> Result<WorthUiCanvasSpatialPlan, WorthUiCanvasSpatialPlanDenial> {
        let mut counters = WorthUiCanvasSpatialCounters::default();
        let row_count =
            execution_plan.regional_family_count(WorthUiPlanNodeInputFamily::CanvasSpatial);
        if row_count == 0 {
            counters.record_denial();
            return Err(WorthUiCanvasSpatialPlanDenial::new(
                WorthUiCanvasSpatialPlanDenialReason::NoCanvasSpatialRows,
                counters,
            ));
        }
        if lane_admission
            .posture_for(WorthUiExecutionLane::CanvasSpatial)
            .is_none_or(|row| row.status() != WorthUiLaneSupportStatus::Supported)
        {
            counters.record_denial();
            return Err(WorthUiCanvasSpatialPlanDenial::new(
                WorthUiCanvasSpatialPlanDenialReason::LaneAdmissionMissingCanvasSpatialSupport,
                counters,
            ));
        }
        if !host_binding.canvas_spatial_supported() {
            counters.record_denial();
            return Err(WorthUiCanvasSpatialPlanDenial::new(
                WorthUiCanvasSpatialPlanDenialReason::HostSupportMissing,
                counters,
            ));
        }
        counters.record_canvas_plan_rows(row_count);
        counters.record_admitted_hook_family();
        counters.record_renderer_reference(row_count);
        let spatial_slots =
            execution_plan.regional_family_slot_view([WorthUiPlanNodeInputFamily::CanvasSpatial]);
        let spatial_meaning_digest = execution_plan
            .regional_family_semantic_digest(WorthUiPlanNodeInputFamily::CanvasSpatial);
        let digest = fold(
            spatial_meaning_digest,
            fold(
                lane_admission.support_digest(),
                host_binding.capability_profile_digest(),
            ),
        );
        Ok(WorthUiCanvasSpatialPlan::new(
            super::WorthUiCanvasSpatialPlanInput {
                handle_receipt: execution_plan.handle_receipt(),
                support_digest: lane_admission.support_digest(),
                canvas_plan_digest: digest,
                host_binding,
                region_store: execution_plan.regional_store_clone(),
                spatial_slots,
                counters,
            },
        ))
    }
}

fn fold(digest: u64, value: u64) -> u64 {
    (digest ^ value).wrapping_mul(0x100000001b3)
}
