use crate::runtime::activation::WorthUiActivationLaneInput;
use crate::runtime::activation_staging::{WorthUiActivationStager, WorthUiActivationStagingInput};
use crate::runtime::replacement::WorthUiReplacementLoweringReady;
use crate::runtime::{WorthUiActivationStagingDenial, WorthUiPendingActivation};

use super::runtime_instance::WorthUiRuntime;

pub struct WorthUiActivationStagingPlans<'a> {
    pub reconciliation_plan: Option<&'a crate::runtime::WorthUiDurableStateReconciliationPlan>,
    pub query_rebind_plan: Option<&'a crate::runtime::WorthUiQueryLiveRebindPlan>,
    pub pending_execution_plan_lowering_input:
        Option<&'a crate::runtime::WorthUiPendingExecutionPlanLoweringInput>,
}

impl<'a> WorthUiActivationStagingPlans<'a> {
    pub fn new(
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
    pub fn stage_replacement_activation_from_lowering(
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

    pub fn stage_replacement_activation_from_lane_input(
        &self,
        input: WorthUiActivationLaneInput,
    ) -> Result<WorthUiPendingActivation, WorthUiActivationStagingDenial> {
        self.stage_replacement_activation_from_lowering(input.0)
    }

    pub fn stage_replacement_activation(
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
