//! Progressive ordinary surface for runtime-installed operations.
//!
//! The root teaches only world → typed family → bound operation. Advanced
//! transitions remain discoverable by the guarantee they govern. Package
//! construction, provider registration, replay, raw Foundational carriers, and
//! generic proof machinery are deliberately absent.

pub use crate::domain_installation::{
    WorthQueryBoundDomainOperation, WorthQueryBranchHeadIdentity,
    WorthQueryBranchHeadIdentityError, WorthQueryInstalledOperatingWorld,
    WorthQueryOperatingWorldEntryDenial, WorthQueryOperatingWorldEntryDenialKind,
    WorthQueryOperationBindingDenial, WorthQueryOperationBindingDenialKind,
    WorthQueryOperationFamilyView,
};

pub mod transition {
    pub use super::super::installed_transitions::{
        collection_capability, collection_window_admission, collection_window_resolution,
        consumption, execution, publication, resource_admission, settlement,
        WorthQueryCollectionCapabilityTransition, WorthQueryCollectionWindowTransition,
        WorthQueryConsumptionTransition, WorthQueryExecutionTransition,
        WorthQueryPublicationTransition, WorthQueryResourceAdmissionStop,
        WorthQueryResourceAdmissionTransition, WorthQuerySettlementTransition,
    };
}

pub mod operation {
    pub use crate::domain_installation::{
        WorthQueryAdmittedDirectOperation, WorthQueryAdmittedExecutionResourcePlan,
        WorthQueryAdmittedWorkflowOperation, WorthQueryAdmittedWorkflowResourcePlan,
        WorthQueryBoundExecutionDenial, WorthQueryBoundExecutionDenialKind,
        WorthQueryBoundExecutionReceipt, WorthQueryBoundProjectionRequest,
        WorthQueryConsumedDomainProjection, WorthQueryConsumerAllocationPosture,
        WorthQueryConsumerBoundary, WorthQueryConsumerBoundaryRequirements,
        WorthQueryConsumerPresentationPosture, WorthQueryConsumerProjectionContract,
        WorthQueryConsumerProjectionContractDenial, WorthQueryConsumerSupportDimension,
        WorthQueryConsumerSupportPosture, WorthQueryDeferredDomainOperation,
        WorthQueryDerivedPublicationReceipt, WorthQueryDirectResourceAdmissionOutcome,
        WorthQueryExecutedDomainOperation, WorthQueryExecutionProviderSession,
        WorthQueryExecutionResourceAdmissionCounters, WorthQueryExecutionResourceAdmissionDenial,
        WorthQueryExecutionResourceAdmissionDenialKind,
        WorthQueryExecutionResourceAdmissionPosture, WorthQueryExecutionResourceAttemptEvidence,
        WorthQueryExecutionResourceSupport, WorthQueryExecutionResourceSupportSnapshot,
        WorthQueryNativeAccessCounters, WorthQueryNativeAccessDenial,
        WorthQueryNativeAccessDenialKind, WorthQueryNativeAccessKey, WorthQueryNativeFieldAccess,
        WorthQueryNativeProjectionRequestDenial, WorthQueryNativeProjectionRequestDenialKind,
        WorthQueryOperationExecutionCounters, WorthQueryOperationExecutionWarning,
        WorthQueryOperationLineageContract, WorthQueryOperationResultState,
        WorthQueryProgressionDenial, WorthQueryProjectionRequestBuilder,
        WorthQueryPublicationDenial, WorthQueryPublishedDomainOperation,
        WorthQuerySettledDomainProjection, WorthQueryWorkflowResourceAdmissionOutcome,
    };
    pub use crate::ordinary::read::project_facts;
    pub use worth_query_declaration::facade::domain_computation::{
        WorthQueryCancellationSafePointFamily, WorthQueryExecutionDegradation,
        WorthQueryExecutionMode, WorthQueryExecutionResourceRequest, WorthQueryResourceDimension,
        WorthQueryResourceLimitRequest, WorthQuerySemanticScaleAxis,
        WorthQuerySemanticScaleRequest,
    };
}

pub mod observation {
    pub use crate::domain_installation::{
        WorthQueryAdmittedConsumerInvalidation, WorthQueryConsumerInvalidationAdmissionStop,
        WorthQueryConsumerInvalidationCause, WorthQueryConsumerInvalidationContinuation,
        WorthQueryConsumerInvalidationCounters, WorthQueryConsumerInvalidationDelta,
        WorthQueryConsumerInvalidationDeltaStop, WorthQueryConsumerInvalidationDeltaStopKind,
        WorthQueryConsumerInvalidationDisposition, WorthQueryConsumerInvalidationLocality,
        WorthQueryLiveBoundDomainProjection, WorthQueryProjectionLeaseAdmissionDenialKind,
        WorthQueryProjectionLeaseAdmissionOutcome, WorthQueryProjectionLeaseAdmissionStop,
        WorthQueryProjectionPromotionDenialKind, WorthQueryProjectionPromotionOutcome,
        WorthQueryProjectionPromotionStop, WorthQuerySharedLiveProjectionLease,
        WorthQuerySharedProjectionDelivery, WorthQuerySharedProjectionDisposalOutcome,
        WorthQuerySharedProjectionDisposalStop, WorthQuerySharedProjectionDrainStop,
    };
}

pub mod collection {
    pub use crate::domain_installation::{
        WorthQueryAdmittedCollectionWindow, WorthQueryBoundCollection,
        WorthQueryBoundCollectionWindow, WorthQueryCollectionCapabilityDenial,
        WorthQueryCollectionCapabilityOutcome, WorthQueryCollectionCapabilityStop,
        WorthQueryCollectionConsumerPreparationDenial, WorthQueryCollectionConsumerWindow,
        WorthQueryCollectionContinuation, WorthQueryCollectionCursor,
        WorthQueryCollectionDeliveryCounters, WorthQueryCollectionDeliveryDenial,
        WorthQueryCollectionDeliveryDenialKind, WorthQueryCollectionDeliveryOutcome,
        WorthQueryCollectionPatch, WorthQueryCollectionPatchApplicationReceipt,
        WorthQueryCollectionPatchFact, WorthQueryCollectionPatchOperation,
        WorthQueryCollectionRowAccessDenial, WorthQueryCollectionRowHandle,
        WorthQueryCollectionWindowAdmissionOutcome, WorthQueryCollectionWindowBreadth,
        WorthQueryCollectionWindowBreadthDenial, WorthQueryCollectionWindowOutcome,
        WorthQueryCollectionWindowWarning,
    };
}

pub mod compatibility {
    pub use crate::domain_installation::{
        classify_owner_delivered_impact, WorthQueryArtifactReuseEquivalence,
        WorthQueryBasisCompatibilityDenial, WorthQueryBasisCompatibilityWitness,
        WorthQueryCompatibilityCounters, WorthQueryCompatibilityDenialKind,
        WorthQueryDependencyClosureReuseDenial, WorthQueryDependencyClosureReuseWitness,
        WorthQueryExecutionSharingDenial, WorthQueryExecutionSharingWitness, WorthQueryImpactClass,
        WorthQueryImpactCounters, WorthQueryImpactDecision,
        WorthQueryInvalidationCompatibilityOutcome, WorthQuerySameInstallationDenial,
        WorthQuerySameInstallationWitness,
    };
}

pub mod support {
    pub use crate::domain_installation::{
        WorthQueryConsumerSupportAdmissionCounters, WorthQueryConsumerSupportCompatibilityDenial,
        WorthQueryConsumerSupportDimension, WorthQueryConsumerSupportPosture,
    };
}

pub mod impact {
    pub use crate::domain_installation::{
        WorthQueryCompiledSemanticAspectDependency,
        WorthQueryCompiledSemanticAspectDependencyClosure,
        WorthQueryConditionalObservationEvidence, WorthQueryDependencyClosureReuseDenial,
        WorthQueryDependencyClosureReuseWitness, WorthQueryDependencyClosureSemanticComparison,
        WorthQueryImpactAdmissionDenial, WorthQueryImpactAdmissionDenialKind,
        WorthQueryImpactClass, WorthQueryImpactCounters, WorthQueryImpactDecision,
        WorthQuerySemanticAspectDependencyCompilationCounters,
        WorthQuerySemanticAspectDependencyCompilationDenial,
        WorthQuerySemanticAspectDependencyCompilationDenialKind,
        WorthQuerySemanticAspectDependencyView, WorthQuerySemanticDependencyClosureEvidence,
        WorthQuerySemanticDependencyEdge, WorthQuerySemanticDependencyRole,
    };
}

pub mod lineage {
    pub use crate::domain_installation::{
        WorthQueryDurableReferenceIntent, WorthQueryPersistentNameAdmission,
        WorthQueryPersistentNameDenial, WorthQueryPersistentNameIntent,
        WorthQueryPersistentNameOutcome, WorthQueryPersistentNameTarget,
        WorthQueryPromotedGraphIdentity, WorthQueryPromotionOnReferenceCapability,
        WorthQueryPromotionOnReferenceCounters, WorthQueryPromotionOnReferenceDenial,
        WorthQueryPromotionOnReferenceOutcome, WorthQueryTraceLineageCounters,
        WorthQueryTraceLineageEvidence, WorthQueryTraceLineageReport,
    };
}

pub mod inspection {
    pub use crate::domain_installation::{
        WorthQueryConsumptionCostExportDenial, WorthQueryConsumptionCostExportDenialKind,
        WorthQueryConsumptionCostRow, WorthQueryConsumptionCostSnapshot,
    };
}

pub mod conditional {
    pub use crate::domain_installation::{
        WorthQueryArtifactPosture, WorthQueryArtifactReuseEquivalence, WorthQueryComparatorFamily,
        WorthQueryComparatorRequirement, WorthQueryConditionalEvaluationCondition,
        WorthQueryConditionalGraphReadRole, WorthQueryConditionalNodeRole,
        WorthQueryConditionalOutcomeClass, WorthQueryConditionalProvenance,
        WorthQueryConditionalTrigger, WorthQueryDeltaComparisonDomain, WorthQueryDeltaThreshold,
        WorthQueryDomainConditionFamily, WorthQueryMaintenancePosture,
        WorthQueryOnDemandTriggerFamily, WorthQueryOperationProjectionRole,
        WorthQueryOutputEquivalenceRequirement, WorthQueryOutputRelationship,
        WorthQueryPortableConditionParameter, WorthQueryPortableConditionalNodeDeclaration,
        WorthQueryQuantityUnit, WorthQueryQuantityValueFamily, WorthQuerySemanticLocality,
        WorthQueryTemporalCondition, WorthQueryTemporalWake,
    };
}

pub mod workflow {
    pub use crate::domain_installation::WorthQueryReplayComparison as WorthQueryWorkflowTraceComparison;
    pub use crate::domain_installation::{
        compare_exact_workflow_traces, WorthQueryCompletedWorkflowTrace,
        WorthQueryConsumedWorkflowProjection, WorthQueryDeferredWorkflowStage,
        WorthQueryDeferredWorkflowStart, WorthQueryExecutedWorkflowAftermath,
        WorthQueryPublishedWorkflow, WorthQuerySettledWorkflowProjection,
        WorthQueryWorkflowAdvanceDenial, WorthQueryWorkflowAftermathOutcome,
        WorthQueryWorkflowCompletionDenial, WorthQueryWorkflowProjectionPromotionOutcome,
        WorthQueryWorkflowPublicationDenial, WorthQueryWorkflowReexecutionOutcome,
        WorthQueryWorkflowRun, WorthQueryWorkflowStageAttempt, WorthQueryWorkflowStageReceipt,
        WorthQueryWorkflowStartOutcome,
    };
}

pub mod recovery {
    pub use crate::domain_installation::{
        WorthQueryAftermathAdmission, WorthQueryAftermathAdmissionDenial,
        WorthQueryAftermathCounters, WorthQueryAftermathExecutionDenial,
        WorthQueryAftermathExecutionDenialKind, WorthQueryAftermathFailureRecoveryPosture,
        WorthQueryAftermathKind, WorthQueryAftermathPosture, WorthQueryAftermathRelationReceipt,
        WorthQueryCompensationCapability, WorthQueryDomainRebindDenial,
        WorthQueryDomainRebindNextAction, WorthQueryDomainRebindReceipt,
        WorthQueryDomainRebindRequest, WorthQueryExactInverseCapability,
        WorthQueryExecutedWorkflowAftermath, WorthQueryProjectionCancellationOutcome,
        WorthQueryProjectionDisposalOutcome, WorthQueryProjectionRebindOutcome,
        WorthQueryProjectionReplacementOutcome, WorthQueryRebindRequiredDomainProjection,
        WorthQueryReplacementDenial,
    };
}
