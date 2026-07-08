use crate::runtime::activation::WorthUiActivationLaneInput;
use crate::runtime::activation_staging::WorthUiActivationStager;
use crate::runtime::replacement::WorthUiReplacementLoweringReady;
use crate::runtime::{WorthUiActivationStagingDenial, WorthUiPendingActivation};

use super::host::WorthUiRuntimeHost;

impl WorthUiRuntimeHost {
    pub fn stage_replacement_activation_from_lowering(
        &self,
        lowering: WorthUiReplacementLoweringReady,
    ) -> Result<WorthUiPendingActivation, WorthUiActivationStagingDenial> {
        self.stage_replacement_activation(
            lowering.admitted,
            &lowering.impact,
            &lowering.narrowing,
            &lowering.node_plan,
            Some(&lowering.reconciliation_plan),
            Some(&lowering.query_rebind_plan),
            Some(&lowering.pending_execution_plan_lowering_input),
        )
    }

    pub fn stage_replacement_activation_from_lane_input(
        &self,
        input: WorthUiActivationLaneInput,
    ) -> Result<WorthUiPendingActivation, WorthUiActivationStagingDenial> {
        self.stage_replacement_activation_from_lowering(input.0)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn stage_replacement_activation(
        &self,
        admitted: crate::runtime::WorthUiAdmittedReplacementCandidate,
        impact: &crate::runtime::WorthUiReplacementImpactClassification,
        narrowing: &crate::runtime::WorthUiRuntimeImpactNarrowing,
        node_plan: &crate::runtime::WorthUiNodeReplacementPlan,
        reconciliation_plan: Option<&crate::runtime::WorthUiDurableStateReconciliationPlan>,
        query_rebind_plan: Option<&crate::runtime::WorthUiQueryLiveRebindPlan>,
        pending_execution_plan_lowering_input: Option<
            &crate::runtime::WorthUiPendingExecutionPlanLoweringInput,
        >,
    ) -> Result<WorthUiPendingActivation, WorthUiActivationStagingDenial> {
        let active_before = self.inspect_active();
        let active_after = self.inspect_active();
        WorthUiActivationStager::stage(
            active_before,
            active_after,
            admitted,
            impact,
            narrowing,
            node_plan,
            reconciliation_plan,
            query_rebind_plan,
            pending_execution_plan_lowering_input,
        )
    }
}