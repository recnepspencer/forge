use crate::runtime::activation_staging::{WorthUiActivationStager, WorthUiActivationStagingInput};
use crate::runtime::replacement::WorthUiReplacementLoweringReady;
use crate::runtime::{WorthUiActivationStagingDenial, WorthUiPendingActivation};

use super::runtime_instance::WorthUiRuntime;

pub(crate) struct WorthUiActivationStagingPlans<'a> {
    pub(crate) reconciliation_plan:
        Option<&'a crate::runtime::WorthUiDurableStateReconciliationPlan>,
    pub(crate) query_rebind_plan: Option<&'a crate::runtime::WorthUiQueryLiveRebindPlan>,
    pub(crate) pending_execution_plan_lowering_input:
        Option<&'a crate::runtime::WorthUiPendingExecutionPlanLoweringInput>,
}

impl<'a> WorthUiActivationStagingPlans<'a> {
    #[cfg(test)]
    pub(crate) fn new(
        reconciliation_plan: Option<&'a crate::runtime::WorthUiDurableStateReconciliationPlan>,
        query_rebind_plan: Option<&'a crate::runtime::WorthUiQueryLiveRebindPlan>,
        pending_execution_plan_lowering_input: Option<
            &'a crate::runtime::WorthUiPendingExecutionPlanLoweringInput,
        >,
    ) -> Self {
        Self {
            reconciliation_plan,
            query_rebind_plan,
            pending_execution_plan_lowering_input,
        }
    }
}

impl WorthUiRuntime {
    pub(crate) fn stage_replacement_activation_from_lowering(
        &self,
        lowering: WorthUiReplacementLoweringReady,
    ) -> Result<WorthUiPendingActivation, WorthUiActivationStagingDenial> {
        self.stage_replacement_activation(
            lowering.admitted,
            &lowering.impact,
            &lowering.narrowing,
            &lowering.node_plan,
            WorthUiActivationStagingPlans {
                reconciliation_plan: Some(&lowering.reconciliation_plan),
                query_rebind_plan: Some(&lowering.query_rebind_plan),
                pending_execution_plan_lowering_input: Some(
                    &lowering.pending_execution_plan_lowering_input,
                ),
            },
        )
    }

    pub(crate) fn stage_replacement_activation(
        &self,
        admitted: crate::runtime::WorthUiAdmittedReplacementCandidate,
        impact: &crate::runtime::WorthUiReplacementImpactClassification,
        narrowing: &crate::runtime::WorthUiRuntimeImpactNarrowing,
        node_plan: &crate::runtime::WorthUiNodeReplacementPlan,
        plans: WorthUiActivationStagingPlans<'_>,
    ) -> Result<WorthUiPendingActivation, WorthUiActivationStagingDenial> {
        let WorthUiActivationStagingPlans {
            reconciliation_plan,
            query_rebind_plan,
            pending_execution_plan_lowering_input,
        } = plans;
        let active_before = self.inspect_active();
        let active_after = self.inspect_active();
        WorthUiActivationStager::stage(WorthUiActivationStagingInput {
            active_before,
            active_after,
            admitted,
            impact,
            narrowing,
            node_plan,
            reconciliation_plan,
            query_rebind_plan,
            pending_execution_plan_lowering_input,
        })
    }
}
