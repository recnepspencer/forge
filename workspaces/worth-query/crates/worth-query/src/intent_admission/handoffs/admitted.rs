use super::{
    WorthQueryAuthoritativeIntentExecutionHandoff,
    WorthQueryAuthoritativeMutationBatchExecutionHandoff,
    WorthQueryAuthoritativeMutationExecutionHandoff, WorthQueryDerivedInspectionExecutionHandoff,
    WorthQueryDerivedMaterializationExecutionHandoff,
    WorthQueryEffectTriggeredIntentExecutionHandoff, WorthQueryExistingTruthProbeExecutionHandoff,
    WorthQueryLiveReadExecutionHandoff, WorthQueryReadExecutionHandoff,
    WorthQueryUnifiedInspectionExecutionHandoff,
};

#[derive(Clone, Debug, PartialEq)]
pub enum WorthQueryAdmittedIntentExecutionHandoff {
    Authoritative(WorthQueryAuthoritativeIntentExecutionHandoff),
    EffectTriggered(WorthQueryEffectTriggeredIntentExecutionHandoff),
    AuthoritativeMutation(WorthQueryAuthoritativeMutationExecutionHandoff),
    AuthoritativeMutationBatch(WorthQueryAuthoritativeMutationBatchExecutionHandoff),
    ReadExecution(WorthQueryReadExecutionHandoff),
    LiveReadExecution(WorthQueryLiveReadExecutionHandoff),
    DerivedMaterialization(WorthQueryDerivedMaterializationExecutionHandoff),
    DerivedInspection(WorthQueryDerivedInspectionExecutionHandoff),
    UnifiedInspection(WorthQueryUnifiedInspectionExecutionHandoff),
    ExistingTruthProbeRouting(WorthQueryExistingTruthProbeExecutionHandoff),
}
