use super::{
    WorthQueryAdmittedIntentExecutionHandoff, WorthQueryAdmittedIntentPlan,
    WorthQueryAuthoritativeIntentExecutionHandoff,
    WorthQueryAuthoritativeMutationBatchExecutionHandoff,
    WorthQueryAuthoritativeMutationExecutionHandoff, WorthQueryDerivedInspectionExecutionHandoff,
    WorthQueryDerivedMaterializationExecutionHandoff,
    WorthQueryEffectTriggeredIntentExecutionHandoff, WorthQueryExistingTruthProbeExecutionHandoff,
    WorthQueryLiveReadExecutionHandoff, WorthQueryReadExecutionHandoff,
    WorthQueryUnifiedInspectionExecutionHandoff,
};

impl WorthQueryAdmittedIntentPlan {
    pub fn into_execution_handoff(self) -> Option<WorthQueryAdmittedIntentExecutionHandoff> {
        match self {
            WorthQueryAdmittedIntentPlan::Authoritative(plan) => {
                Some(WorthQueryAdmittedIntentExecutionHandoff::Authoritative(
                    WorthQueryAuthoritativeIntentExecutionHandoff::from_plan(plan),
                ))
            }
            WorthQueryAdmittedIntentPlan::EffectTriggered(plan) => {
                Some(WorthQueryAdmittedIntentExecutionHandoff::EffectTriggered(
                    WorthQueryEffectTriggeredIntentExecutionHandoff::from_plan(plan),
                ))
            }
            WorthQueryAdmittedIntentPlan::AuthoritativeMutation(plan) => Some(
                WorthQueryAdmittedIntentExecutionHandoff::AuthoritativeMutation(
                    WorthQueryAuthoritativeMutationExecutionHandoff::from_plan(plan),
                ),
            ),
            WorthQueryAdmittedIntentPlan::AuthoritativeMutationBatch(plan) => Some(
                WorthQueryAdmittedIntentExecutionHandoff::AuthoritativeMutationBatch(
                    WorthQueryAuthoritativeMutationBatchExecutionHandoff::from_plan(plan),
                ),
            ),
            WorthQueryAdmittedIntentPlan::ReadExecution(plan) => {
                Some(WorthQueryAdmittedIntentExecutionHandoff::ReadExecution(
                    WorthQueryReadExecutionHandoff::from_plan(plan),
                ))
            }
            WorthQueryAdmittedIntentPlan::LiveReadExecution(plan) => {
                Some(WorthQueryAdmittedIntentExecutionHandoff::LiveReadExecution(
                    WorthQueryLiveReadExecutionHandoff::from_plan(plan),
                ))
            }
            WorthQueryAdmittedIntentPlan::DerivedMaterialization(plan) => Some(
                WorthQueryAdmittedIntentExecutionHandoff::DerivedMaterialization(
                    WorthQueryDerivedMaterializationExecutionHandoff::from_plan(plan),
                ),
            ),
            WorthQueryAdmittedIntentPlan::DerivedInspection(plan) => {
                Some(WorthQueryAdmittedIntentExecutionHandoff::DerivedInspection(
                    WorthQueryDerivedInspectionExecutionHandoff::from_plan(plan),
                ))
            }
            WorthQueryAdmittedIntentPlan::UnifiedInspection(plan) => {
                Some(WorthQueryAdmittedIntentExecutionHandoff::UnifiedInspection(
                    WorthQueryUnifiedInspectionExecutionHandoff::from_plan(plan),
                ))
            }
            WorthQueryAdmittedIntentPlan::ExistingTruthProbeRouting(plan) => Some(
                WorthQueryAdmittedIntentExecutionHandoff::ExistingTruthProbeRouting(
                    WorthQueryExistingTruthProbeExecutionHandoff::from_plan(plan),
                ),
            ),
            WorthQueryAdmittedIntentPlan::BasisObservation(_)
            | WorthQueryAdmittedIntentPlan::ProjectionConsumption(_) => None,
        }
    }
}
