use crate::runtime::lane_meaning_parity::WorthUiLaneMeaningParityPlanner;
use crate::runtime::WorthUiRuntime;
use crate::runtime::{
    WorthUiExecutionPlan, WorthUiLaneParityDenial, WorthUiLaneParityReport,
    WorthUiNodeReplacementPlan, WorthUiQueryBindingComparison, WorthUiQueryLiveRebindPlan,
    WorthUiRuntimeImpactNarrowing,
};

impl WorthUiRuntime {
    pub fn certify_lane_meaning_parity(
        &self,
        node_plan: &WorthUiNodeReplacementPlan,
        narrowing: &WorthUiRuntimeImpactNarrowing,
        active_plan: &WorthUiExecutionPlan,
        candidate_plan: &WorthUiExecutionPlan,
        query_comparison: &WorthUiQueryBindingComparison,
        query_rebind_plan: Option<&WorthUiQueryLiveRebindPlan>,
    ) -> Result<WorthUiLaneParityReport, WorthUiLaneParityDenial> {
        WorthUiLaneMeaningParityPlanner::certify(
            node_plan,
            narrowing,
            active_plan,
            candidate_plan,
            query_comparison,
            query_rebind_plan,
        )
    }
}
