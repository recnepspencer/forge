//! Public contract for the internal execution authority.

pub mod domain_computation {
    pub use crate::domain_computation::*;
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
    pub use crate::domain_computation::primary_graph::{
        WorthQueryAdmittedApplicationOperation, WorthQueryAdmittedApplicationQueryPlan,
        WorthQueryApplicationAttemptDenial, WorthQueryApplicationAttemptDenialKind,
        WorthQueryApplicationAuthorizationWorkEvidence, WorthQueryApplicationCommitDenial,
        WorthQueryApplicationCommitDenialKind, WorthQueryApplicationCommitDenialStage,
        WorthQueryApplicationCommitOutcome, WorthQueryApplicationCommitReceipt,
        WorthQueryApplicationContinuationDenial, WorthQueryApplicationContinuationDenialKind,
        WorthQueryApplicationContinuationPageResult, WorthQueryApplicationEffectEntity,
        WorthQueryApplicationEffectProgram, WorthQueryApplicationEffectProgramBuilder,
        WorthQueryApplicationEntityIdentity, WorthQueryApplicationEntityKey,
        WorthQueryApplicationEntityKeyDenial, WorthQueryApplicationEntitySeed,
        WorthQueryApplicationHistoricalBasis, WorthQueryApplicationHistoricalBasisReleaseReceipt,
        WorthQueryApplicationHistoricalRead, WorthQueryApplicationHistoricalResult,
        WorthQueryApplicationIdempotencyBinding, WorthQueryApplicationIdempotencyResolution,
        WorthQueryApplicationIdempotencyResolutionDenial,
        WorthQueryApplicationIdempotencyResolutionDenialKind,
        WorthQueryApplicationInvariantProjectionAuthority,
        WorthQueryApplicationInvariantProjectionReader,
        WorthQueryApplicationInvariantProjectionSnapshot, WorthQueryApplicationLiveCauseDenialKind,
        WorthQueryApplicationLiveCloseOutcome, WorthQueryApplicationLiveControlDenial,
        WorthQueryApplicationLiveControls, WorthQueryApplicationLiveLease,
        WorthQueryApplicationLiveOpenDenial, WorthQueryApplicationLiveOpenDenialKind,
        WorthQueryApplicationLiveOutcome, WorthQueryApplicationLiveOverflow,
        WorthQueryApplicationLiveUpdate, WorthQueryApplicationOneShotDenial,
        WorthQueryApplicationOneShotDenialKind, WorthQueryApplicationOneShotResult,
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
        WorthQueryApplicationQueryResumeControls, WorthQueryApplicationReadAttempt,
        WorthQueryApplicationRelationSeed, WorthQueryApplicationStaleAttempt,
        WorthQueryAuthenticatedPrincipal, WorthQueryBoundedLaneDenial,
        WorthQueryBoundedLaneDenialKind, WorthQueryCompleteApplicationReadSet,
        WorthQueryCompletedInvariantProjection, WorthQueryCompletedOperationInvariantProjection,
        WorthQueryEntityResolutionDenial, WorthQueryEntityResolutionDenialKind,
        WorthQueryInspectedOperationInvariantProjection, WorthQueryInvariantAggregate,
        WorthQueryInvariantAggregateDenial, WorthQueryInvariantAggregateDenialKind,
        WorthQueryInvariantDecisionPlanDenial, WorthQueryInvariantDecisionPlanDenialKind,
        WorthQueryInvariantEntityIdentity, WorthQueryInvariantProjectionTraversalDenial,
        WorthQueryInvariantProjectionTraversalDenialKind, WorthQueryInvariantProjectionWork,
        WorthQueryInvariantRelation, WorthQueryMutationPreconditionComparisonEvidence,
        WorthQueryObservedApplicationRelation, WorthQueryOperationAuthorizationDenial,
        WorthQueryOperationAuthorizationDenialKind, WorthQueryOperationProjectionDenial,
        WorthQueryOperationProjectionDenialKind, WorthQueryOperationScopeFingerprint,
        WorthQueryOrdinaryApplicationRead, WorthQueryOrdinaryReadBatch,
        WorthQueryOrdinaryReadMetadata, WorthQueryOrdinaryReadProjection,
        WorthQueryOrdinaryReadVersion, WorthQueryPrimaryGraph,
        WorthQueryPrimaryGraphApplicationRuntime, WorthQueryPrimaryGraphBootstrap,
        WorthQueryPrimaryGraphInstallationDenial, WorthQueryPrimaryGraphInstallationDenialKind,
        WorthQueryPrimaryGraphPublication, WorthQueryPrincipalResolutionDenial,
        WorthQueryPrincipalResolutionDenialKind, WorthQueryPrincipalResolutionMode,
        WorthQueryProjectedApplicationMutation,
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
    pub mod legacy_provider_execution {
        pub use crate::domain_computation::provider_session::graph_provider::bounded_step::legacy_one_shot::execute_legacy_one_shot;
    }
}
