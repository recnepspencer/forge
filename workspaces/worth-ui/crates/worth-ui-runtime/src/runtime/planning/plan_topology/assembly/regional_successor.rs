use crate::runtime::{
    WorthUiExecutionPlan, WorthUiPlanTopologyCounters, WorthUiRuntimeHandleAllocation,
};

pub(crate) fn construct_regional_successor_plan(
    authority: &crate::runtime::planning::WorthUiExecutionPlanLoweringFacts,
    handle_allocation: &WorthUiRuntimeHandleAllocation,
    lane_admission_counters: crate::runtime::WorthUiLaneAdmissionCounters,
    region_successor: super::super::WorthUiPlanRegionSuccessor,
    counters: WorthUiPlanTopologyCounters,
) -> WorthUiExecutionPlan {
    let regional_storage = region_successor.counters();
    let construction_counters = super::super::WorthUiPlanConstructionCounters::new(
        authority.plan_input().counters(),
        handle_allocation.counters(),
        lane_admission_counters,
        counters,
        regional_storage,
    );
    let regional_evidence =
        super::super::WorthUiPlanRegionalEvidence::from_lowering(authority, &region_successor);
    WorthUiExecutionPlan::new_regional_successor(
        authority,
        handle_allocation.receipt(),
        region_successor.into_store(),
        construction_counters,
        regional_evidence,
        counters,
    )
}
