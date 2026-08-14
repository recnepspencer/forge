//! Public contract for the internal execution authority.

pub mod domain_computation {
    pub use super::primary_graph::*;
    pub use crate::domain_computation::artifact_owner::*;
    pub use crate::domain_computation::convergence_epoch::*;
    pub use crate::domain_computation::execution_runtime::*;
    pub use crate::domain_computation::managed_run::*;
    pub use crate::domain_computation::operation_binding::*;
    pub use crate::domain_computation::provider_session::*;
    pub use crate::domain_computation::{
        canonical_indexed_operation_material, canonical_operation_material,
        WorthQueryConvergenceDomainEvidenceBindingDenial,
    };
}

pub mod runtime {
    pub use crate::domain_computation::execution_runtime::*;
    pub use crate::domain_computation::{
        WorthQueryExecutionBoundOperationAuthority, WorthQueryExecutionOperationBindingDenial,
        WorthQueryInstalledDomainExecutionAuthority,
    };
}

pub mod provider_session {
    pub use crate::domain_computation::provider_session::*;
}

pub mod primary_graph {
    pub use crate::domain_computation::application_aftermath::{
        compensate_recovery_handle, dispose_recovery_handle, expire_recovery_handle,
        inspect_recovery_handle, map_ordinary_commit_conflict, reconcile_recovery_handle,
        resolve_recovery_handle, safe_retry_recovery_handle, ExternalEffectCausalLink,
        ExternalEffectClassification, ExternalEffectCorrelationIdentity, ExternalEffectPosture,
        ExternalEffectPostureIdentity, ExternalEffectPostureKind, ExternalRailTransportFault,
        WorthQueryAdmittedIdempotencyRead, WorthQueryAftermathDerivationFailure,
        WorthQueryDispatchOutboxDurabilityPosture, WorthQueryDispatchOutboxRecord,
        WorthQueryExternalDispatchCausalRelation, WorthQueryExternalDispatchPosture,
        WorthQueryExternalDispatchPostureKind, WorthQueryExternalDispatchRequest,
        WorthQueryExternalEffectCausalLadder, WorthQueryExternalEffectDispatch,
        WorthQueryExternalEffectTransport, WorthQueryExternalTransportOutcome,
        WorthQueryOpaqueRecoveryWireIdentity, WorthQueryPerformedExternalRedispatch,
        WorthQueryRecoveryCompensateAdmission, WorthQueryRecoveryCurrentDecision,
        WorthQueryRecoveryDisclosureAdmission, WorthQueryRecoveryDisposalReceipt,
        WorthQueryRecoveryDurabilityPosture, WorthQueryRecoveryEffectAuthority,
        WorthQueryRecoveryExpiryDecision, WorthQueryRecoveryExpiryEvaluation,
        WorthQueryRecoveryHandle, WorthQueryRecoveryHandleBinding, WorthQueryRecoveryHandleDenial,
        WorthQueryRecoveryHandleDenialKind, WorthQueryRecoveryInspectAuthority,
        WorthQueryRecoveryInspectionView, WorthQueryRecoveryReconcileAdmission,
        WorthQueryRecoverySafeRetryAdmission,
    };
    pub use crate::domain_computation::primary_graph::{
        WorthQueryAdmittedApplicationCapabilityAccess, WorthQueryAdmittedApplicationOperation,
        WorthQueryAdmittedApplicationQueryControls, WorthQueryAdmittedApplicationQueryPlan,
        WorthQueryAdmittedDisclosedApplicationResult, WorthQueryApplicationAttemptDenial,
        WorthQueryApplicationAttemptDenialKind, WorthQueryApplicationAuthorizationExplanationCause,
        WorthQueryApplicationAuthorizationWorkEvidence, WorthQueryApplicationBasisObservation,
        WorthQueryApplicationBasisObserver, WorthQueryApplicationCommitAuthorityBinding,
        WorthQueryApplicationCommitDenial, WorthQueryApplicationCommitDenialKind,
        WorthQueryApplicationCommitDenialStage, WorthQueryApplicationCommitOutcome,
        WorthQueryApplicationCommitOutcomeIdentity,
        WorthQueryApplicationCommitPublicationExternalEffect,
        WorthQueryApplicationCommitPublicationSource, WorthQueryApplicationCommitReceipt,
        WorthQueryApplicationCommitRecoveryKind, WorthQueryApplicationCommitTerminalEvidence,
        WorthQueryApplicationCommitTerminalKind, WorthQueryApplicationContinuationDenial,
        WorthQueryApplicationContinuationDenialKind, WorthQueryApplicationContinuationPageResult,
        WorthQueryApplicationDisclosed, WorthQueryApplicationDisclosureDecisionFact,
        WorthQueryApplicationDisclosureOutcome, WorthQueryApplicationDisclosureOutcomeIdentity,
        WorthQueryApplicationDisclosureReceipt, WorthQueryApplicationDisclosureReceiptPosture,
        WorthQueryApplicationEffectEntity, WorthQueryApplicationEffectProgram,
        WorthQueryApplicationEffectProgramBuilder, WorthQueryApplicationEntityIdentity,
        WorthQueryApplicationEntityKey, WorthQueryApplicationEntityKeyDenial,
        WorthQueryApplicationEntitySeed, WorthQueryApplicationHistoricalBasis,
        WorthQueryApplicationHistoricalBasisReleaseReceipt, WorthQueryApplicationHistoricalRead,
        WorthQueryApplicationHistoricalResult, WorthQueryApplicationIdempotencyBinding,
        WorthQueryApplicationIdempotencyResolution,
        WorthQueryApplicationIdempotencyResolutionDenial,
        WorthQueryApplicationIdempotencyResolutionDenialKind,
        WorthQueryApplicationInvariantProjectionAuthority,
        WorthQueryApplicationInvariantProjectionReader,
        WorthQueryApplicationInvariantProjectionSnapshot, WorthQueryApplicationLiveCauseDenialKind,
        WorthQueryApplicationLiveCloseOutcome, WorthQueryApplicationLiveControlDenial,
        WorthQueryApplicationLiveControls, WorthQueryApplicationLiveLease,
        WorthQueryApplicationLiveOpenDenial, WorthQueryApplicationLiveOpenDenialKind,
        WorthQueryApplicationLiveOutcome, WorthQueryApplicationLiveOverflow,
        WorthQueryApplicationLiveUpdate, WorthQueryApplicationOmission,
        WorthQueryApplicationOneShotDenial, WorthQueryApplicationOneShotDenialKind,
        WorthQueryApplicationOneShotResult,
        WorthQueryApplicationOperationInvariantProjectionReader,
        WorthQueryApplicationOperationInvariantProjectionSnapshot,
        WorthQueryApplicationPinnedBasis, WorthQueryApplicationPinnedBasisDenial,
        WorthQueryApplicationPinnedBasisDenialKind, WorthQueryApplicationPinnedBasisReleaseReceipt,
        WorthQueryApplicationPreviewBasis, WorthQueryApplicationPreviewBasisReleaseReceipt,
        WorthQueryApplicationPreviewResult, WorthQueryApplicationPreviewSession,
        WorthQueryApplicationPreviewSessionDenial, WorthQueryApplicationPreviewSessionDenialKind,
        WorthQueryApplicationPreviewSessionDiscardReceipt,
        WorthQueryApplicationPreviewSessionIdentity, WorthQueryApplicationPrincipalIdentity,
        WorthQueryApplicationPrincipalKey, WorthQueryApplicationPrincipalKeyDenial,
        WorthQueryApplicationProjection, WorthQueryApplicationProjectionDenial,
        WorthQueryApplicationProjectionDenialKind, WorthQueryApplicationProjectionRow,
        WorthQueryApplicationProjectionRows, WorthQueryApplicationQueryAccessContext,
        WorthQueryApplicationQueryAccessReceipt, WorthQueryApplicationQueryAdmissionDenial,
        WorthQueryApplicationQueryAdmissionDenialKind, WorthQueryApplicationQueryBasisPosture,
        WorthQueryApplicationQueryConsistency, WorthQueryApplicationQueryContinuation,
        WorthQueryApplicationQueryControls, WorthQueryApplicationQueryFreshness,
        WorthQueryApplicationQueryOmissionPosture, WorthQueryApplicationQueryResumeControls,
        WorthQueryApplicationQueryWorkEvidence, WorthQueryApplicationReadAttempt,
        WorthQueryApplicationRelationSeed, WorthQueryApplicationResultBufferEvidence,
        WorthQueryApplicationResultBufferObservation, WorthQueryApplicationResultBufferObserver,
        WorthQueryApplicationStaleAttempt, WorthQueryApplicationUnresolvedCommitEvidence,
        WorthQueryApprovedElevation, WorthQueryAuthenticatedPrincipal, WorthQueryBoundedLaneDenial,
        WorthQueryBoundedLaneDenialKind, WorthQueryCapabilityRevocationProgram,
        WorthQueryCompleteApplicationReadSet, WorthQueryCompletedInvariantProjection,
        WorthQueryCompletedOperationInvariantProjection,
        WorthQueryConditionalApplicationRuntimeInstallation, WorthQueryConditionalClockHandle,
        WorthQueryConditionalClockObservationDenial,
        WorthQueryConditionalClockObservationDenialKind,
        WorthQueryConditionalClockObservationFailure,
        WorthQueryConditionalClockObservationFailureKind,
        WorthQueryConditionalClockObservationOutcome, WorthQueryConditionalClockObservationPort,
        WorthQueryConditionalClockObservationReceipt, WorthQueryConditionalExecutionProvenance,
        WorthQueryConditionalExecutionTerminal, WorthQueryConditionalRuntimeInspection,
        WorthQueryConditionalRuntimeInstallationDenial,
        WorthQueryConditionalRuntimeInstallationDenialKind,
        WorthQueryConditionalRuntimeLifecycleProbe,
        WorthQueryConditionalRuntimeReinstallationReceipt, WorthQueryConditionalSignalDecision,
        WorthQueryDelegationActivationProgram, WorthQueryElevationApprovalAuthorizationDenial,
        WorthQueryElevationApprovalOutcome, WorthQueryElevationApprovalProgram,
        WorthQueryElevationCloseAuthorizationDenial, WorthQueryElevationCloseOutcome,
        WorthQueryElevationCloseProgram, WorthQueryElevationClosureKind,
        WorthQueryElevationRequestOutcome, WorthQueryElevationRequestProgram,
        WorthQueryEntityResolutionDenial, WorthQueryEntityResolutionDenialKind,
        WorthQueryExternalDispatchPreparationDenial, WorthQueryExternalRedispatchDenial,
        WorthQueryExternalTransportInstallationDenial,
        WorthQueryGovernedTemporalOperationAuthorization,
        WorthQueryGovernedTemporalQueryAuthorization,
        WorthQueryInspectedOperationInvariantProjection, WorthQueryInvariantAggregate,
        WorthQueryInvariantAggregateDenial, WorthQueryInvariantAggregateDenialKind,
        WorthQueryInvariantDecisionPlanDenial, WorthQueryInvariantDecisionPlanDenialKind,
        WorthQueryInvariantEntityIdentity, WorthQueryInvariantMutationTarget,
        WorthQueryInvariantProjectionTraversalDenial,
        WorthQueryInvariantProjectionTraversalDenialKind, WorthQueryInvariantProjectionWork,
        WorthQueryInvariantRelation, WorthQueryMandatoryReview,
        WorthQueryMandatoryReviewAuthorizationDenial, WorthQueryMandatoryReviewOutcome,
        WorthQueryMandatoryReviewProgram, WorthQueryMutationPreconditionComparisonEvidence,
        WorthQueryObservedApplicationRelation, WorthQueryOperationAuthorizationDenial,
        WorthQueryOperationAuthorizationDenialIdentity, WorthQueryOperationAuthorizationDenialKind,
        WorthQueryOperationProjectionDenial, WorthQueryOperationProjectionDenialKind,
        WorthQueryOperationScopeBinding, WorthQueryOperationScopeEntityBinding,
        WorthQueryOrdinaryApplicationRead, WorthQueryOrdinaryReadBatch,
        WorthQueryOrdinaryReadMetadata, WorthQueryOrdinaryReadProjection,
        WorthQueryOrdinaryReadVersion, WorthQueryPrimaryGraph,
        WorthQueryPrimaryGraphApplicationRuntime, WorthQueryPrimaryGraphBootstrap,
        WorthQueryPrimaryGraphInstallationDenial, WorthQueryPrimaryGraphInstallationDenialKind,
        WorthQueryPrimaryGraphPublication, WorthQueryPrimaryMutationWorkEvidence,
        WorthQueryPrincipalResolutionDenial, WorthQueryPrincipalResolutionDenialKind,
        WorthQueryPrincipalResolutionMode, WorthQueryProjectedApplicationMutation,
        WorthQueryPublicTemporalOperationAuthorization, WorthQueryPublicTemporalQueryAuthorization,
        WorthQueryRequestedElevation, WorthQueryReviewedElevation, WorthQueryRuntimeTimeSource,
        WorthQueryRuntimeTimeSourceDenial, WorthQueryTemporalInvocationFailure,
        WorthQueryTemporalInvocationFailureKind, WorthQueryTemporalOperationAuthorization,
        WorthQueryTemporalOperationExecution, WorthQueryTemporalOperationInvoker,
        WorthQueryTemporalPrincipalAdmission, WorthQueryTemporalPrincipalFailure,
        WorthQueryTemporalPrincipalFailureKind, WorthQueryTemporalPrincipalSource,
        WorthQueryTemporalQueryAuthorization, WorthQueryTemporalReconstructionAccess,
        WorthQueryTouchedRecordIdentity,
    };
    pub use crate::domain_computation::primary_graph::{
        WorthQueryCommittedDispatchOutboxObservation, WorthQueryCommittedDispatchOutboxReadDenial,
        WorthQueryCommittedDispatchOutboxReadWork,
    };
    pub use crate::domain_computation::runtime_time::WorthQueryRuntimeTimeSample;
}

/// Compatibility surface for the current undo/redo experiment.
///
/// These types remain compiled but are not accepted Phase 8 product contracts.
pub mod provisional_aftermath {
    pub use crate::domain_computation::application_aftermath::{
        admit_undo, consume_redo_progression, consume_unresolved_undo_progression,
        deny_irreversible_undo_attempt, map_ordinary_commit_conflict_to_redo,
        progress_admitted_reconciliation, progress_admitted_redo, progress_admitted_undo,
        WorthQueryAftermathCausalRole, WorthQueryCommittedAftermathCausality, WorthQueryProvedUndo,
        WorthQueryRedoAdmission, WorthQueryRedoDenial, WorthQueryRedoDenialKind,
        WorthQueryRedoIntent, WorthQueryRedoIntentIdentity, WorthQueryRedoProgressionHandoff,
        WorthQueryRedoRecovery, WorthQueryRetainedPreImage, WorthQueryUndoAdmission,
        WorthQueryUndoDenial, WorthQueryUndoDenialKind, WorthQueryUndoDerivedRequest,
        WorthQueryUndoIntentIdentity, WorthQueryUndoProgressionHandoff,
    };
}

pub mod convergence_epoch {
    pub use crate::domain_computation::convergence_epoch::*;
}

pub mod installed {
    pub use super::{convergence_epoch, domain_computation, provider_session, runtime};
}

#[doc(hidden)]
pub mod integration {
    use worth_query_installation::facade::{
        ApplicationSchema, WorthQueryInstalledApplicationSchema,
    };
    use worth_relational::facade::runtime::RelationalRuntime;

    use crate::domain_computation::execution_runtime::{
        WorthQueryExecutionInstallationAuthority, WorthQueryExecutionRuntime,
    };
    use crate::domain_computation::primary_graph::{
        WorthQueryPrimaryGraphBootstrap, WorthQueryPrimaryGraphInstallationDenial,
        WorthQueryPrimaryGraphPublication,
    };

    pub use crate::domain_computation::artifact_owner::{
        WorthQueryArtifactAccessAuthority, WorthQueryArtifactProductionAuthority,
        WorthQueryArtifactTransferAdmission, WorthQueryWorkflowArtifactAuthority,
        WorthQueryWorkflowArtifactRegistry,
    };
    pub use crate::domain_computation::provider_session::graph_provider::bounded_step::provider_anchor::WorthQueryGraphProviderAnchor;
    pub use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphIntegrationHandle;
    pub use crate::domain_computation::primary_graph::{
        WorthQueryPrimaryGraphIndexRefreshDenial,
        WorthQueryPrimaryGraphIndexRefreshDenialKind,
    };

    pub fn prepare_primary_graph_with_relational_runtime<Schema>(
        authority: &WorthQueryExecutionInstallationAuthority,
        runtime: &WorthQueryExecutionRuntime,
        installed_schema: &WorthQueryInstalledApplicationSchema<Schema>,
        relational_runtime: RelationalRuntime,
    ) -> Result<WorthQueryPrimaryGraphBootstrap<Schema>, WorthQueryPrimaryGraphInstallationDenial>
    where
        Schema: ApplicationSchema,
    {
        authority.prepare_primary_graph_with_relational_runtime(
            runtime,
            installed_schema,
            relational_runtime,
        )
    }

    pub fn retain_primary_graph_integration_handle(
        runtime: &WorthQueryExecutionRuntime,
    ) -> Option<WorthQueryPrimaryGraphIntegrationHandle> {
        runtime.retain_primary_graph_integration_handle()
    }

    pub fn publish_primary_graph<Schema>(
        bootstrap: WorthQueryPrimaryGraphBootstrap<Schema>,
        runtime: &mut WorthQueryExecutionRuntime,
        authority: &WorthQueryExecutionInstallationAuthority,
    ) -> Result<WorthQueryPrimaryGraphPublication, WorthQueryPrimaryGraphInstallationDenial>
    where
        Schema: ApplicationSchema,
    {
        bootstrap.publish(runtime, authority)
    }

    #[doc(hidden)]
    pub fn classify_conditional_signal_for_certification(
        evidence: &worth_runtime_bridge::facade::BridgeConditionalDecisionEvidence,
    ) -> crate::domain_computation::primary_graph::WorthQueryConditionalSignalDecision {
        crate::domain_computation::primary_graph::classify_bridge_signal(evidence)
    }

    #[doc(hidden)]
    pub mod legacy_provider_execution {
        pub use crate::domain_computation::provider_session::graph_provider::bounded_step::legacy_one_shot::execute_legacy_one_shot;
    }
}
