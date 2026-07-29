pub use worth_ui_runtime::facade::rebind::{
    UiAffectedConsumer, UiAffectedFactLookup, UiAffectedScopeBasis, UiAffectedScopeCost,
    UiAffectedScopeDenial, UiAffectedScopeGeneration, UiAuthoredChangedFact, UiAuthoredFactKind,
    UiAuthoredFactSelector, UiChangeProfile, UiCommittedPortalAnchorChangedFact,
    UiCommittedScrollExtentChangedFact, UiConsumedFactContract, UiConsumedFactSelector,
    UiDuplicateObservationReceipt, UiGraphFactConsumerIdentity, UiGraphFactConsumerKey,
    UiGraphFactConsumerKind, UiHostDeviceScaleChangedFact, UiHostViewportChangedFact,
    UiIdentityLifecycleDecision, UiIdentityLifecycleDenial, UiIdentityLifecycleEntry,
    UiMeasurementChangedFact, UiPreparedRebind, UiPreparedRebindPosture, UiProducedFact,
    UiProducedFactContract, UiProducedFactFamily, UiProducedFactOwner, UiProducedFactResetPosture,
    UiQueryChangedFact, UiQueryChangedFactKind, UiQueryIncrementalChangedFact,
    UiQueryResetChangedFact, UiRebindArtifactPolicy, UiRebindBudgetInput,
    UiRebindCancellationPolicy, UiRebindCancellationReceipt, UiRebindCancellationRequest,
    UiRebindCandidatePreparationDenial, UiRebindCompletionHandle, UiRebindConcurrencyInput,
    UiRebindConflictFootprint, UiRebindDeadlinePolicy, UiRebindDeclarativeEffect,
    UiRebindDenialCause, UiRebindDenialReceipt, UiRebindDisclosurePolicy, UiRebindDisposition,
    UiRebindEffectSet, UiRebindExecutionPolicy, UiRebindExecutionRequest, UiRebindIdempotency,
    UiRebindInternalDefectKind, UiRebindInternalDefectOutcome, UiRebindLimit, UiRebindOutcome,
    UiRebindParallelAdmission, UiRebindPlan, UiRebindPlanBasis, UiRebindPlanCost,
    UiRebindPlanTarget, UiRebindPlanningDenial, UiRebindPreparationDenial, UiRebindProfile,
    UiRebindProfileConstructionDenial, UiRebindReceipt, UiRebindReconciliation,
    UiRebindReconciliationRequest, UiRebindRecoveryCompletionHandle, UiRebindRecoveryDenial,
    UiRebindRecoveryDenialCause, UiRebindRecoveryHandle, UiRebindRecoveryInternalDefect,
    UiRebindRecoveryInternalDefectKind, UiRebindRecoveryOutcome, UiRebindRecoveryReceipt,
    UiRebindRecoverySurfaceDenial, UiRebindReservationDenial, UiRebindResourceAccess,
    UiRebindRetryTolerance, UiRebindSafePoint, UiRebindSafePointPolicy, UiRebindSessionDeadline,
    UiRebindShutdownReport, UiRebindStoppedPhase, UiRebindSubsystemKind, UiRebindSubsystemPlan,
    UiRebindSupersededReceipt, UiRebindTimeoutReceipt, UiRebindValidNextAction,
    UiResolvedAffectedScope, UiResolvedIdentityLifecycle, UiSourceRebindRequest,
    UiSubsystemConsumedFactRule,
};

#[cfg(test)]
mod tests {
    use super::{
        UiPreparedRebind, UiRebindExecutionRequest, UiRebindPlan, UiRebindPreparationDenial,
    };
    use crate::facade::app::WorthUiActiveApplicationSession;

    #[test]
    fn curated_rebind_facade_exposes_the_typed_preparation_progression() {
        fn prepare<'session>(
            session: &'session mut WorthUiActiveApplicationSession,
            plan: UiRebindPlan,
            request: UiRebindExecutionRequest,
        ) -> Result<UiPreparedRebind<'session>, UiRebindPreparationDenial> {
            session.prepare_rebind(plan, request)
        }

        let _ = prepare;
    }
}
