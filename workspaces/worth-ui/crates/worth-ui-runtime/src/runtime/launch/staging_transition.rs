use crate::runtime::activation_staging::{WorthUiActivationStager, WorthUiActivationStagingInput};
use crate::runtime::replacement::WorthUiReplacementLoweringReady;
use crate::runtime::{WorthUiActivationStagingDenial, WorthUiPendingActivation};

use super::runtime_instance::WorthUiRuntime;

pub(crate) struct WorthUiActivationStagingPlans<'a> {
    pub(crate) reconciliation_plan:
        Option<&'a crate::runtime::WorthUiDurableStateReconciliationPlan>,
    pub(crate) query_rebind_plan: Option<&'a crate::runtime::WorthUiQueryLiveRebindPlan>,
}

impl<'a> WorthUiActivationStagingPlans<'a> {
    #[cfg(test)]
    pub(crate) fn new(
        reconciliation_plan: Option<&'a crate::runtime::WorthUiDurableStateReconciliationPlan>,
        query_rebind_plan: Option<&'a crate::runtime::WorthUiQueryLiveRebindPlan>,
    ) -> Self {
        Self {
            reconciliation_plan,
            query_rebind_plan,
        }
    }
}

impl WorthUiRuntime {
    pub(crate) fn stage_replacement_activation_from_lowering(
        &self,
        lowering: WorthUiReplacementLoweringReady,
    ) -> Result<WorthUiPendingActivation, WorthUiActivationStagingDenial> {
        self.stage_replacement_activation_with_authority(
            lowering.candidate_application_authority,
            lowering.admitted,
            &lowering.impact,
            &lowering.narrowing,
            &lowering.node_plan,
            WorthUiActivationStagingPlans {
                reconciliation_plan: Some(&lowering.reconciliation_plan),
                query_rebind_plan: Some(&lowering.query_rebind_plan),
            },
        )
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn stage_replacement_activation(
        &self,
        admitted: crate::runtime::WorthUiAdmittedReplacementCandidate,
        impact: &crate::runtime::WorthUiReplacementImpactClassification,
        narrowing: &crate::runtime::WorthUiRuntimeImpactNarrowing,
        node_plan: &crate::runtime::WorthUiNodeReplacementPlan,
        plans: WorthUiActivationStagingPlans<'_>,
    ) -> Result<WorthUiPendingActivation, WorthUiActivationStagingDenial> {
        let candidate_application_authority = self
            .active_application_lowering_authority
            .synthetic_successor_for_certification(&admitted);
        self.stage_replacement_activation_with_authority(
            candidate_application_authority,
            admitted,
            impact,
            narrowing,
            node_plan,
            plans,
        )
    }

    fn stage_replacement_activation_with_authority(
        &self,
        candidate_application_authority: crate::facade::prepared_application_authority::WorthUiPreparedApplicationLoweringAuthority,
        admitted: crate::runtime::WorthUiAdmittedReplacementCandidate,
        impact: &crate::runtime::WorthUiReplacementImpactClassification,
        narrowing: &crate::runtime::WorthUiRuntimeImpactNarrowing,
        node_plan: &crate::runtime::WorthUiNodeReplacementPlan,
        plans: WorthUiActivationStagingPlans<'_>,
    ) -> Result<WorthUiPendingActivation, WorthUiActivationStagingDenial> {
        let WorthUiActivationStagingPlans {
            reconciliation_plan,
            query_rebind_plan,
        } = plans;
        let active_before = self.inspect_active();
        let active_after = self.inspect_active();
        WorthUiActivationStager::stage(WorthUiActivationStagingInput {
            candidate_application_authority,
            active_before,
            active_after,
            admitted,
            impact,
            narrowing,
            node_plan,
            reconciliation_plan,
            query_rebind_plan,
        })
    }
}
