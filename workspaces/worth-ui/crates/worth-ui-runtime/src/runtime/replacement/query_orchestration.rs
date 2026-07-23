use crate::runtime::launch::runtime_instance::WorthUiRuntime;
use crate::runtime::replacement::query_binding::{
    WorthUiQueryBindingComparisonPlanner, WorthUiQueryBindingReplacementAuthority,
};
use crate::runtime::replacement::query_succession::WorthUiQueryLiveRebindPlanner;
use crate::runtime::replacement::transitions::WorthUiReplacementQueryComparisonReady;
use crate::runtime::{
    WorthUiAdmittedReplacementCandidate, WorthUiNodeReplacementPlan, WorthUiQueryBindingComparison,
    WorthUiQueryBindingComparisonDenial, WorthUiQueryLiveRebindPlan,
    WorthUiQueryLiveRebindPlanDenial, WorthUiReplacementLoweringReady,
    WorthUiRuntimeImpactNarrowing,
};

impl WorthUiRuntime {
    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn compare_query_bindings(
        &self,
        node_plan: &WorthUiNodeReplacementPlan,
        narrowing: &WorthUiRuntimeImpactNarrowing,
        admitted: &WorthUiAdmittedReplacementCandidate,
    ) -> Result<WorthUiQueryBindingComparison, WorthUiQueryBindingComparisonDenial> {
        WorthUiQueryBindingComparisonPlanner::compare(
            self.active.active_artifact(),
            node_plan,
            narrowing,
            admitted,
            WorthUiQueryBindingReplacementAuthority::new(
                self.active_application_lowering_authority
                    .query_binding_plan(),
                &self.query_binding,
            ),
            WorthUiQueryBindingReplacementAuthority::new(
                self.active_application_lowering_authority
                    .query_binding_plan(),
                &self.query_binding,
            ),
        )
    }

    pub(super) fn compare_query_bindings_for_narrowing_with_candidate_authority(
        &self,
        narrowing: &WorthUiRuntimeImpactNarrowing,
        admitted: &WorthUiAdmittedReplacementCandidate,
        candidate_plan: &worth_ui_query_binding::WorthUiQueryBindingPlan,
        candidate_binding: &worth_ui_query_binding::WorthUiRuntimeQueryBinding,
    ) -> Result<WorthUiQueryBindingComparison, WorthUiQueryBindingComparisonDenial> {
        WorthUiQueryBindingComparisonPlanner::compare_narrowed(
            self.active.active_artifact(),
            narrowing,
            admitted,
            WorthUiQueryBindingReplacementAuthority::new(
                self.active_application_lowering_authority
                    .query_binding_plan(),
                &self.query_binding,
            ),
            WorthUiQueryBindingReplacementAuthority::new(candidate_plan, candidate_binding),
        )
    }

    pub(crate) fn plan_query_live_rebinds(
        &self,
        comparison: &WorthUiQueryBindingComparison,
        node_plan: &WorthUiNodeReplacementPlan,
        narrowing: &WorthUiRuntimeImpactNarrowing,
        admitted: &WorthUiAdmittedReplacementCandidate,
    ) -> Result<WorthUiQueryLiveRebindPlan, WorthUiQueryLiveRebindPlanDenial> {
        WorthUiQueryLiveRebindPlanner::plan(comparison, node_plan, narrowing, admitted)
    }

    pub(crate) fn prepare_replacement_lowering_from_query_comparison(
        &self,
        ready: WorthUiReplacementQueryComparisonReady,
        candidate_application_authority: crate::facade::prepared_application_authority::WorthUiPreparedApplicationLoweringAuthority,
    ) -> Result<WorthUiReplacementLoweringReady, WorthUiQueryLiveRebindPlanDenial> {
        let WorthUiReplacementQueryComparisonReady {
            admitted,
            impact,
            narrowing,
            node_plan,
            reconciliation_plan,
            query_comparison,
            artifact_comparison_counters,
            identity_match_counters,
        } = ready;
        let query_rebind_plan =
            self.plan_query_live_rebinds(&query_comparison, &node_plan, &narrowing, &admitted)?;
        Ok(WorthUiReplacementLoweringReady {
            candidate_application_authority,
            admitted,
            impact,
            narrowing,
            node_plan,
            reconciliation_plan,
            query_rebind_plan,
            artifact_comparison_counters,
            identity_match_counters,
        })
    }
}
