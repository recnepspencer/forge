//! Public API boundary for `forge-signal`.
//! External components should import through this module.
//!
//! Contract:
//! - `forge-signal` owns evaluation DAG scheduling.
//! - Host crates own external structural or state graphs, including cyclic ones.
//! - Compute closures may consume opaque host snapshots directly.
//!
//! Public shape:
//! - top-level `facade::*` is the curated daily-use path
//! - `facade::core` covers graph authoring primitives
//! - `facade::runtime` covers production runtime and transaction workflows
//! - `facade::diagnostics` covers explain / compare / health / history flows
//! - `facade::specialist` and `facade::adapters` contain deeper specialist control

pub mod core {
    pub use crate::data::aspect::{Aspect, AspectMask, AspectVersion, MAX_ASPECTS};
    pub use crate::data::async_node::{
        async_node_compile_time_boundary_proof, async_node_milestone_d_certification_run,
        async_node_milestone_d_performance_closeout, async_node_milestone_d_scenario_matrix,
        AsyncCapableNode, AsyncKeyedNodeCapabilityBinding,
        AsyncKeyedNodeCapabilityEquivalenceDenialClass, AsyncKeyedNodeCapabilityEquivalenceReport,
        AsyncKeyedNodeHistoricalParityDenialClass, AsyncKeyedNodeHistoricalParityReport,
        AsyncNodeAdmissionClass, AsyncNodeAdmissionClassification,
        AsyncNodeCapabilityAliasLoweringProof, AsyncNodeCapabilityDeclaration,
        AsyncNodeCapabilityEquivalenceDenialClass, AsyncNodeCapabilityEquivalenceReport,
        AsyncNodeCompileTimeBoundaryProof, AsyncNodeConditionBlockClass,
        AsyncNodeDownstreamDependenceFact, AsyncNodeGateStateReport,
        AsyncNodeHierarchyCancellationReport, AsyncNodeHierarchyHistoricalParityDenialClass,
        AsyncNodeHierarchyHistoricalParityReport, AsyncNodeHierarchyReplaySummary,
        AsyncNodeHistoricalParityDenialClass, AsyncNodeHistoricalParityReport,
        AsyncNodeMilestoneDCertificationRun, AsyncNodeMilestoneDCertificationRunSummary,
        AsyncNodeMilestoneDPerformanceClaimId, AsyncNodeMilestoneDPerformanceCloseout,
        AsyncNodeMilestoneDPerformanceCloseoutRow, AsyncNodeMilestoneDPerformanceCloseoutSummary,
        AsyncNodeMilestoneDScenarioEvidenceKind, AsyncNodeMilestoneDScenarioId,
        AsyncNodeMilestoneDScenarioInputs, AsyncNodeMilestoneDScenarioMatrix,
        AsyncNodeMilestoneDScenarioMatrixSummary, AsyncNodeMilestoneDScenarioRow,
        AsyncNodePayloadContract, AsyncNodePayloadContractId, AsyncNodeRequestAdmissionReport,
        AsyncNodeRequestIntent, AsyncNodeRevalidationIntent, AsyncNodeRevalidationReport,
        DeniedAsyncKeyedNodeCapabilityEquivalence, DeniedAsyncKeyedNodeHistoricalParity,
        DeniedAsyncNodeCapabilityEquivalence, DeniedAsyncNodeHierarchyHistoricalParity,
        DeniedAsyncNodeHistoricalParity, FrozenAsyncNodeCapabilityDescriptor,
        LoweredAsyncNodeCapabilityBundle, ValidatedAsyncNodeCapabilityDeclaration,
        ASYNC_NODE_COMPILE_TIME_BOUNDARY_PROOF_SCHEMA_VERSION,
        ASYNC_NODE_MILESTONE_D_CERTIFICATION_RUN_SCHEMA_VERSION,
        ASYNC_NODE_MILESTONE_D_PERFORMANCE_CLOSEOUT_SCHEMA_VERSION,
        ASYNC_NODE_MILESTONE_D_SCENARIO_MATRIX_SCHEMA_VERSION,
        REQUIRED_ASYNC_NODE_COMPILE_TIME_FIXTURES,
        REQUIRED_ASYNC_NODE_MILESTONE_D_PERFORMANCE_CLAIMS,
        REQUIRED_ASYNC_NODE_MILESTONE_D_SCENARIOS,
    };
    pub use crate::data::core_profile::CORE_STORAGE_PROFILE_ID;
    pub use crate::data::dependency::DependencyEdge;
    pub use crate::data::error::SignalError;
    pub use crate::data::graph::{NodeBuilder, SignalGraph};
    pub use crate::data::handle::NodeId;
    pub use crate::data::host_computed::{
        AdmittedHostComputedReadSet, CommittedHostComputedArtifact, DeniedHostComputedEvaluation,
        DeniedHostComputedReadSet, HostComputedApiFamily, HostComputedDenialClass,
        HostComputedDependencyPatch, HostComputedDescriptor, HostComputedDescriptorId,
        HostComputedDiagnosticsSummary, HostComputedEvaluationOutcome,
        HostComputedEvaluationRequest, HostComputedEvaluationResponse, HostComputedEvaluator,
        HostComputedFailure, HostComputedFailureClass, HostComputedOutcomeClass,
        HostComputedPreparedResponse, PreparedHostComputedEvaluation, StagedHostComputedArtifact,
    };
    pub use crate::data::node::{EvaluationCondition, NodeState};
    pub use crate::data::output::{
        CanonicalChangedRegions, ChangedRegion, NodeEvaluationResult, OutputChange, OutputIdentity,
        PartitionMatchMode, PartitionSubscription, PartitionToken,
    };
    pub use crate::data::resource::{
        resource_certification_builder, resource_certification_bundle,
        resource_certification_bundle_parity_report, resource_milestone_b_certification_run,
        resource_milestone_b_hostile_scenario_evidence, resource_milestone_b_performance_closeout,
        resource_milestone_b_scenario_matrix, resource_milestone_c_certification_run,
        resource_milestone_c_policy_certification_builder,
        resource_milestone_c_policy_certification_bundle,
        resource_milestone_c_policy_performance_closeout,
        resource_milestone_c_policy_scenario_matrix, ActiveResourceRevalidationProof,
        AdmittedResourceCompletion, AdmittedResourceRequest, AdmittedResourceRetry,
        AdmittedResourceRevalidation, AsyncDenialId, CancelledResourceRequest,
        CommittedResourceCompletionArtifact, CompletionDenialClass, DeniedResourceCancellation,
        DeniedResourceCompletion, DeniedResourcePolicyRestoreCompatibility,
        DeniedResourceRejection, DeniedResourceRetry, DeniedResourceRevalidation,
        DeniedResourceTimeout, DeniedResourceTimeoutHeartbeatExtension,
        DependencyChangeResourceRevalidationProof, ExtendedResourceTimeoutHeartbeat,
        FrozenResourcePolicyDescriptor, FrozenResourcePolicyDescriptorSet,
        FrozenResourcePolicyRegistry, FulfilledLifecycleResourceRevalidationProof,
        InFlightResourceRequest, LoweredResourcePolicyBundle, ObservedResourceNodeState,
        ObserverDemandResourceRevalidationProof, RawCompletionEnvelope, RejectedResourceRequest,
        ResourceAttemptId, ResourceBranchEpoch, ResourceBranchRestoreReport,
        ResourceCancellationDecisionClass, ResourceCancellationDecisionPlan,
        ResourceCancellationDenialClass, ResourceCancellationGraceWindow,
        ResourceCancellationOrdinal, ResourceCancellationPolicyDeclaration,
        ResourceCancellationReason, ResourceCancellationReport, ResourceCertificationBuilder,
        ResourceCertificationBundle, ResourceCertificationBundleMismatchClass,
        ResourceCertificationBundleParityReport, ResourceCertificationFailure,
        ResourceCertificationFamily, ResourceCertificationRecord, ResourceCertificationSummary,
        ResourceCompletionAdmissionReport, ResourceCompletionBatchAdmissionReport,
        ResourceCompletionCommitReport, ResourceCompletionDenialStagingReport,
        ResourceCompletionOrdinal, ResourceCompletionRollbackReport,
        ResourceCompletionRollbackSubject, ResourceCompletionStagingReport, ResourceCostContractId,
        ResourceCostPosture, ResourceDensityStrategy, ResourceDependentCancellationPropagation,
        ResourceDiagnosticsDecisionClass, ResourceDiagnosticsDecisionPlan,
        ResourceDiagnosticsExpansionBudget, ResourceDiagnosticsExpansionDenial,
        ResourceDiagnosticsExpansionDenialClass, ResourceDiagnosticsPolicyDeclaration,
        ResourceDiagnosticsSummary, ResourceGeneration, ResourceHostCancellationAdvisory,
        ResourceInFlightStatus, ResourceInitialLifecycleClass, ResourceIntentEquivalenceCoalescing,
        ResourceLifecycleClass, ResourceLifecycleOrdinal, ResourceLifecyclePolicyDeclaration,
        ResourceLifecycleRetentionCompactionReport, ResourceLifecycleTransition,
        ResourceLifecycleTransitionKind, ResourceMilestoneBCertificationRun,
        ResourceMilestoneBCertificationRunSummary, ResourceMilestoneBHostileScenarioEvidence,
        ResourceMilestoneBHostileScenarioEvidenceRow, ResourceMilestoneBPerformanceClaimId,
        ResourceMilestoneBPerformanceCloseout, ResourceMilestoneBPerformanceCloseoutRow,
        ResourceMilestoneBPerformanceCloseoutSummary, ResourceMilestoneBScenarioEvidenceKind,
        ResourceMilestoneBScenarioId, ResourceMilestoneBScenarioMatrix,
        ResourceMilestoneBScenarioMatrixSummary, ResourceMilestoneBScenarioRow,
        ResourceMilestoneCCertificationRun, ResourceMilestoneCCertificationRunSummary,
        ResourceMilestoneCPolicyCertificationBuilder, ResourceMilestoneCPolicyCertificationBundle,
        ResourceMilestoneCPolicyCertificationFamily, ResourceMilestoneCPolicyCertificationRecord,
        ResourceMilestoneCPolicyCertificationSummary, ResourceMilestoneCPolicyPerformanceClaimId,
        ResourceMilestoneCPolicyPerformanceCloseout,
        ResourceMilestoneCPolicyPerformanceCloseoutRow,
        ResourceMilestoneCPolicyPerformanceCloseoutSummary,
        ResourceMilestoneCPolicyScenarioEvidenceKind, ResourceMilestoneCPolicyScenarioId,
        ResourceMilestoneCPolicyScenarioMatrix, ResourceMilestoneCPolicyScenarioMatrixSummary,
        ResourceMilestoneCPolicyScenarioRow, ResourceNodeDeclaration, ResourceNodeId,
        ResourceObservationBatchReport, ResourceObservationDecisionClass,
        ResourceObservationDecisionPlan, ResourceObservationEvent,
        ResourceObservationPolicyDeclaration, ResourceOldHostWorkCancellationAdvisory,
        ResourceOutputContinuity, ResourceOutputContinuityDecisionClass,
        ResourceOutputContinuityDecisionPlan, ResourceOutputContinuityPolicyDeclaration,
        ResourceOverlappingGenerationAdmission, ResourcePayloadContract,
        ResourcePayloadContractDigest, ResourcePayloadContractId, ResourcePolicyCompatibilityClass,
        ResourcePolicyCompatibilityFamilyReport, ResourcePolicyCompatibilityPosture,
        ResourcePolicyCompatibilityReport, ResourcePolicyDescriptor, ResourcePolicyDescriptorId,
        ResourcePolicyDigest, ResourcePolicyKind, ResourcePolicyName, ResourcePolicyRegistration,
        ResourcePolicyRegistryError, ResourcePolicyRegistryFreezeReport,
        ResourcePolicyResolutionError, ResourcePolicyRestoreCompatibilityDenialClass,
        ResourcePolicyRestoreCompatibilityProof, ResourcePolicySelectionBasis,
        ResourcePolicyVersion, ResourceRejectionDenialClass, ResourceRejectionOrdinal,
        ResourceRejectionReason, ResourceRejectionReport, ResourceReplayReconstructionReport,
        ResourceRequestHandle, ResourceRequestId, ResourceRequestIntent,
        ResourceRequestIntentDigest, ResourceResolvedPolicy, ResourceResolvedPolicyBundle,
        ResourceRetainedDeniedCompletionAvailability,
        ResourceRetainedDeniedCompletionAvailabilityClass, ResourceRetainedHistoryAvailability,
        ResourceRetainedHistoryAvailabilityClass, ResourceRetainedRetryLineageAvailability,
        ResourceRetainedRetryLineageAvailabilityClass, ResourceRetentionCompactionBudget,
        ResourceRetentionDecisionClass, ResourceRetentionDecisionPlan,
        ResourceRetentionPolicyDeclaration, ResourceRetryAdmissionReport, ResourceRetryBudgetScope,
        ResourceRetryDecisionClass, ResourceRetryDecisionPlan, ResourceRetryDenialClass,
        ResourceRetryOrdinal, ResourceRetryPolicyDeclaration, ResourceRetryReason,
        ResourceRetryScheduleReport, ResourceRevalidationCoalescing,
        ResourceRevalidationDecisionClass, ResourceRevalidationDecisionPlan,
        ResourceRevalidationDenialClass, ResourceRevalidationFreshnessClass,
        ResourceRevalidationFreshnessDecision, ResourceRevalidationIntent,
        ResourceRevalidationPolicyDeclaration, ResourceRevalidationReport,
        ResourceRuntimeSummaryReadReport, ResourceStaleAfterDecisionClass,
        ResourceStaleAfterDecisionPlan, ResourceStaleAfterPolicyDeclaration,
        ResourceSupersessionDecisionClass, ResourceSupersessionDecisionPlan,
        ResourceSupersessionOldHostWorkPosture, ResourceSupersessionOrdinal,
        ResourceSupersessionOverlapDisposition, ResourceSupersessionPolicyDeclaration,
        ResourceSupersessionRecord, ResourceTimeoutDeadlineAuthority, ResourceTimeoutDecisionClass,
        ResourceTimeoutDecisionPlan, ResourceTimeoutDenialClass,
        ResourceTimeoutHeartbeatExtensionDenialClass, ResourceTimeoutHeartbeatExtensionReport,
        ResourceTimeoutOrdinal, ResourceTimeoutOutcomeClass, ResourceTimeoutPolicyDeclaration,
        ResourceTimeoutReport, RetainedResourceRetryLineage, RolledBackResourceCompletionArtifact,
        ScheduledResourceRetry, StagedDeniedResourceCompletionEffect,
        StagedResourceCompletionEffect, TerminalStateResourceRevalidationProof,
        TimedOutResourceRequest, ValidatedCompletionEnvelope, ValidatedResourcePolicyDeclaration,
        ValidatedResourcePolicyReference, REQUIRED_RESOURCE_CERTIFICATION_FAMILIES,
        REQUIRED_RESOURCE_MILESTONE_B_HOSTILE_SCENARIOS,
        REQUIRED_RESOURCE_MILESTONE_B_PERFORMANCE_CLAIMS, REQUIRED_RESOURCE_MILESTONE_B_SCENARIOS,
        REQUIRED_RESOURCE_MILESTONE_C_POLICY_CERTIFICATION_FAMILIES,
        REQUIRED_RESOURCE_MILESTONE_C_POLICY_PERFORMANCE_CLAIMS,
        REQUIRED_RESOURCE_MILESTONE_C_POLICY_SCENARIOS,
        RESOURCE_CERTIFICATION_BUNDLE_PARITY_SCHEMA_VERSION,
        RESOURCE_CERTIFICATION_BUNDLE_SCHEMA_VERSION, RESOURCE_DIAGNOSTICS_SUMMARY_SCHEMA_VERSION,
        RESOURCE_MILESTONE_B_CERTIFICATION_RUN_SCHEMA_VERSION,
        RESOURCE_MILESTONE_B_HOSTILE_SCENARIO_EVIDENCE_SCHEMA_VERSION,
        RESOURCE_MILESTONE_B_PERFORMANCE_CLOSEOUT_SCHEMA_VERSION,
        RESOURCE_MILESTONE_B_SCENARIO_MATRIX_SCHEMA_VERSION,
        RESOURCE_MILESTONE_C_CERTIFICATION_RUN_SCHEMA_VERSION,
        RESOURCE_MILESTONE_C_POLICY_CERTIFICATION_BUNDLE_SCHEMA_VERSION,
        RESOURCE_MILESTONE_C_POLICY_PERFORMANCE_CLOSEOUT_SCHEMA_VERSION,
        RESOURCE_MILESTONE_C_POLICY_SCENARIO_MATRIX_SCHEMA_VERSION,
    };
    pub use crate::data::temporal::{
        AfterCondition, AtOrAfterCondition, ClockAdvanceOrdinal, ClockAdvanceRequest,
        ClockAuthority, ClockCheckpointId, ClockDomain, ClockTick, DebounceCondition,
        DeferredTemporalEligibility, IntervalAnchor, IntervalCondition, IntervalPeriod,
        IntervalWakeRegeneration, LoweredTemporalEligibility, MissedTickPolicy,
        PreviousValueRevision, ReadyTemporalEligibility, RuntimeClockBasis, StaleAfterCondition,
        TemporalClockAdvanceSummary, TemporalCondition, TemporalDuration,
        TemporalEligibilityAuthority, TemporalExecutionSummary, TemporalFrontierSnapshot,
        TemporalPreviousValueAccess, TemporalPreviousValueReference, TemporalReadyPromotionSummary,
        TemporalWakeAdmissionSummary, TemporalWakeOwner, TemporalWakeReschedule,
        TemporalWakeRetirementBatch, TemporalWakeReuse, ThrottleCondition, ValidatedClockAdvance,
    };
    pub use crate::logic::invalidation::mark_dirty_batch;

    pub fn mark_dirty(
        graph: impl std::ops::DerefMut<Target = SignalGraph>,
        source: NodeId,
        changed_aspect: Aspect,
    ) -> Result<(), SignalError> {
        #[cfg(any(test, doctest))]
        {
            return crate::logic::invalidation::mark_dirty(graph, source, changed_aspect);
        }
        #[cfg(not(any(test, doctest)))]
        {
            let _ = crate::logic::invalidation::mark_dirty_batch(
                graph,
                &crate::data::proof::DirtyBatch::singleton(
                    source,
                    changed_aspect,
                    Vec::<ChangedRegion>::new(),
                ),
            )?;
            Ok(())
        }
    }

    pub fn mark_changed(
        graph: impl std::ops::DerefMut<Target = SignalGraph>,
        source: NodeId,
        changed_aspect: Aspect,
    ) -> Result<(), SignalError> {
        mark_dirty(graph, source, changed_aspect)
    }

    pub fn mark_dirty_with_regions(
        graph: impl std::ops::DerefMut<Target = SignalGraph>,
        source: NodeId,
        changed_aspect: Aspect,
        changed_regions: &[ChangedRegion],
    ) -> Result<(), SignalError> {
        #[cfg(any(test, doctest))]
        {
            return crate::logic::invalidation::mark_dirty_with_regions(
                graph,
                source,
                changed_aspect,
                changed_regions,
            );
        }
        #[cfg(not(any(test, doctest)))]
        {
            let _ = crate::logic::invalidation::mark_dirty_batch(
                graph,
                &crate::data::proof::DirtyBatch::singleton(
                    source,
                    changed_aspect,
                    changed_regions.to_vec(),
                ),
            )?;
            Ok(())
        }
    }

    pub fn mark_changed_with_regions(
        graph: impl std::ops::DerefMut<Target = SignalGraph>,
        source: NodeId,
        changed_aspect: Aspect,
        changed_regions: &[ChangedRegion],
    ) -> Result<(), SignalError> {
        mark_dirty_with_regions(graph, source, changed_aspect, changed_regions)
    }
}

pub mod runtime {
    pub use crate::data::async_node::{
        async_node_compile_time_boundary_proof, async_node_milestone_d_certification_run,
        async_node_milestone_d_performance_closeout, async_node_milestone_d_scenario_matrix,
        AsyncKeyedNodeCapabilityBinding, AsyncKeyedNodeCapabilityEquivalenceDenialClass,
        AsyncKeyedNodeCapabilityEquivalenceReport, AsyncKeyedNodeHistoricalParityDenialClass,
        AsyncKeyedNodeHistoricalParityReport, AsyncNodeAdmissionClass,
        AsyncNodeAdmissionClassification, AsyncNodeCapabilityAliasLoweringProof,
        AsyncNodeCapabilityDeclaration, AsyncNodeCapabilityEquivalenceDenialClass,
        AsyncNodeCapabilityEquivalenceReport, AsyncNodeCompileTimeBoundaryProof,
        AsyncNodeConditionBlockClass, AsyncNodeDownstreamDependenceFact, AsyncNodeGateStateReport,
        AsyncNodeHierarchyCancellationReport, AsyncNodeHierarchyHistoricalParityDenialClass,
        AsyncNodeHierarchyHistoricalParityReport, AsyncNodeHierarchyReplaySummary,
        AsyncNodeMilestoneDCertificationRun, AsyncNodeMilestoneDCertificationRunSummary,
        AsyncNodeMilestoneDPerformanceClaimId, AsyncNodeMilestoneDPerformanceCloseout,
        AsyncNodeMilestoneDPerformanceCloseoutRow, AsyncNodeMilestoneDPerformanceCloseoutSummary,
        AsyncNodeMilestoneDScenarioEvidenceKind, AsyncNodeMilestoneDScenarioId,
        AsyncNodeMilestoneDScenarioInputs, AsyncNodeMilestoneDScenarioMatrix,
        AsyncNodeMilestoneDScenarioMatrixSummary, AsyncNodeMilestoneDScenarioRow,
        AsyncNodePayloadContract, AsyncNodePayloadContractId, AsyncNodeRequestAdmissionReport,
        AsyncNodeRequestIntent, AsyncNodeRevalidationIntent, AsyncNodeRevalidationReport,
        DeniedAsyncKeyedNodeCapabilityEquivalence, DeniedAsyncKeyedNodeHistoricalParity,
        DeniedAsyncNodeCapabilityEquivalence, DeniedAsyncNodeHierarchyHistoricalParity,
        FrozenAsyncNodeCapabilityDescriptor, LoweredAsyncNodeCapabilityBundle,
        ValidatedAsyncNodeCapabilityDeclaration,
        ASYNC_NODE_COMPILE_TIME_BOUNDARY_PROOF_SCHEMA_VERSION,
        ASYNC_NODE_MILESTONE_D_CERTIFICATION_RUN_SCHEMA_VERSION,
        ASYNC_NODE_MILESTONE_D_PERFORMANCE_CLOSEOUT_SCHEMA_VERSION,
        ASYNC_NODE_MILESTONE_D_SCENARIO_MATRIX_SCHEMA_VERSION,
        REQUIRED_ASYNC_NODE_COMPILE_TIME_FIXTURES,
        REQUIRED_ASYNC_NODE_MILESTONE_D_PERFORMANCE_CLAIMS,
        REQUIRED_ASYNC_NODE_MILESTONE_D_SCENARIOS,
    };
    pub use crate::data::checkpoint::CheckpointBarrier;
    pub use crate::data::checkpoint_policy::CheckpointPolicy as RuntimeCheckpointPolicy;
    pub use crate::data::output::ComputationFamily as RecipeFamily;
    pub use crate::data::output::ComputationKey as RecipeKey;
    pub use crate::data::output::KeyedComputation as KeyedRecipe;
    pub use crate::data::proof::DirtyBatch as ChangeBatch;
    pub use crate::data::proof::SemanticBatchCommit as ChangeBatchCommit;
    pub use crate::data::resource::{
        resource_certification_builder, resource_certification_bundle,
        resource_certification_bundle_parity_report, resource_milestone_b_certification_run,
        resource_milestone_b_hostile_scenario_evidence, resource_milestone_b_performance_closeout,
        resource_milestone_b_scenario_matrix, CommittedResourceCompletionArtifact,
        DeniedResourcePolicyRestoreCompatibility, DeniedResourceRejection,
        FrozenResourcePolicyDescriptor, FrozenResourcePolicyDescriptorSet,
        LoweredResourceDescriptor, LoweredResourcePolicyBundle, RejectedResourceRequest,
        ResourceBoundaryKind, ResourceBoundaryPerformanceEnvelope, ResourceBranchRestoreReport,
        ResourceCancellationDecisionClass, ResourceCancellationDecisionPlan,
        ResourceCertificationBuilder, ResourceCertificationBundle,
        ResourceCertificationBundleMismatchClass, ResourceCertificationBundleParityReport,
        ResourceCertificationFailure, ResourceCertificationFamily, ResourceCertificationRecord,
        ResourceCertificationSummary, ResourceCompletionBatchAdmissionReport,
        ResourceCompletionCommitReport, ResourceCompletionDenialStagingReport,
        ResourceCompletionRollbackReport, ResourceCompletionRollbackSubject,
        ResourceCompletionStagingReport, ResourceDeclarationReport, ResourceDensityStrategy,
        ResourceDependentCancellationPropagation, ResourceDescriptorId, ResourceDescriptorVersion,
        ResourceDiagnosticsDecisionClass, ResourceDiagnosticsDecisionPlan,
        ResourceDiagnosticsExpansionBudget, ResourceDiagnosticsExpansionDenial,
        ResourceDiagnosticsExpansionDenialClass, ResourceDiagnosticsPolicyDeclaration,
        ResourceDiagnosticsSummary, ResourceHostCancellationAdvisory,
        ResourceIntentEquivalenceCoalescing, ResourceLifecycleRetentionCompactionReport,
        ResourceLifecycleSummary, ResourceMilestoneBCertificationRun,
        ResourceMilestoneBCertificationRunSummary, ResourceMilestoneBHostileScenarioEvidence,
        ResourceMilestoneBHostileScenarioEvidenceRow, ResourceMilestoneBPerformanceClaimId,
        ResourceMilestoneBPerformanceCloseout, ResourceMilestoneBPerformanceCloseoutRow,
        ResourceMilestoneBPerformanceCloseoutSummary, ResourceMilestoneBScenarioEvidenceKind,
        ResourceMilestoneBScenarioId, ResourceMilestoneBScenarioMatrix,
        ResourceMilestoneBScenarioMatrixSummary, ResourceMilestoneBScenarioRow,
        ResourceOldHostWorkCancellationAdvisory, ResourceOverlappingGenerationAdmission,
        ResourcePayloadContractDigest, ResourcePolicyCompatibilityClass,
        ResourcePolicyCompatibilityFamilyReport, ResourcePolicyCompatibilityReport,
        ResourcePolicyDescriptor, ResourcePolicyDigest, ResourcePolicyKind,
        ResourcePolicyRegistryFreezeReport, ResourcePolicyRestoreCompatibilityDenialClass,
        ResourcePolicyRestoreCompatibilityProof, ResourcePolicySelectionBasis,
        ResourceRejectionReport, ResourceReplayReconstructionReport,
        ResourceRequestAdmissionReport, ResourceRequestIntentDigest, ResourceResolvedPolicy,
        ResourceResolvedPolicyBundle, ResourceRetainedDeniedCompletionAvailability,
        ResourceRetainedDeniedCompletionAvailabilityClass, ResourceRetainedHistoryAvailability,
        ResourceRetainedHistoryAvailabilityClass, ResourceRetainedRetryLineageAvailability,
        ResourceRetainedRetryLineageAvailabilityClass, ResourceRetentionCompactionBudget,
        ResourceRetentionDecisionClass, ResourceRetentionDecisionPlan,
        ResourceRetryAdmissionReport, ResourceRetryBudgetScope, ResourceRetryDecisionClass,
        ResourceRetryDecisionPlan, ResourceRetryScheduleReport, ResourceRevalidationReport,
        ResourceRuntimeSummary, ResourceRuntimeSummaryReadReport,
        ResourceSupersessionDecisionClass, ResourceSupersessionDecisionPlan,
        ResourceSupersessionOldHostWorkPosture, ResourceSupersessionOverlapDisposition,
        ResourceSupersessionRecord, ResourceTimeoutDecisionClass, ResourceTimeoutDecisionPlan,
        ResourceTimeoutReport, RetainedResourceRetryLineage, RolledBackResourceCompletionArtifact,
        StagedDeniedResourceCompletionEffect, StagedResourceCompletionEffect,
        ValidatedResourcePolicyDeclaration, ValidatedResourcePolicyReference,
        REQUIRED_RESOURCE_CERTIFICATION_FAMILIES, REQUIRED_RESOURCE_MILESTONE_B_HOSTILE_SCENARIOS,
        REQUIRED_RESOURCE_MILESTONE_B_PERFORMANCE_CLAIMS, REQUIRED_RESOURCE_MILESTONE_B_SCENARIOS,
        RESOURCE_CERTIFICATION_BUNDLE_PARITY_SCHEMA_VERSION,
        RESOURCE_CERTIFICATION_BUNDLE_SCHEMA_VERSION, RESOURCE_DIAGNOSTICS_SUMMARY_SCHEMA_VERSION,
        RESOURCE_MILESTONE_B_CERTIFICATION_RUN_SCHEMA_VERSION,
        RESOURCE_MILESTONE_B_HOSTILE_SCENARIO_EVIDENCE_SCHEMA_VERSION,
        RESOURCE_MILESTONE_B_PERFORMANCE_CLOSEOUT_SCHEMA_VERSION,
        RESOURCE_MILESTONE_B_SCENARIO_MATRIX_SCHEMA_VERSION,
    };
    pub use crate::data::temporal::{
        IntervalWakeRegeneration, PreviousValueRevision, ReadyTemporalWake, RetiredTemporalWake,
        ScheduledTemporalWake, TemporalClockAdvanceSummary, TemporalFrontierSnapshot,
        TemporalPreviousValueAccess, TemporalPreviousValueReference, TemporalReadyPromotionSummary,
        TemporalWakeAdmissionSummary, TemporalWakeId, TemporalWakeOwner, TemporalWakeReschedule,
        TemporalWakeRetirementBatch, TemporalWakeRetirementReason, TemporalWakeReuse,
        TemporalWakeSummary, WakeOrdinal,
    };
    pub use crate::data::tier::TierPolicy as RuntimeTierPolicy;
    pub use crate::data::tier::{DependencyMode, DirtyPropagation, EvaluationTrigger};
    pub use crate::diagnostics::policy::SignalRuntimePolicy as RuntimePolicy;
    pub use crate::logic::invalidation::mark_dirty_batch;
    pub use crate::logic::transaction::DefinedComputation as RecipeInstance;
    pub use crate::logic::transaction::DefinedKeyedComputation as KeyedRecipeInstance;
    pub use crate::logic::transaction::EvaluationSummary as RunSummary;
    pub use crate::logic::transaction::RuntimeExecutionRequest as RuntimeRunRequest;
    pub use crate::logic::transaction::RuntimeHistory as History;
    #[cfg(test)]
    pub use crate::logic::transaction::SignalRuntimeConfig;
    pub use crate::logic::transaction::SignalRuntimeConfig as RuntimeConfig;
    pub use crate::logic::transaction::TransactionExecutionRequest as TransactionRunRequest;
    pub use crate::logic::transaction::{
        bridge_signal_branch_basis_trust_boundary, BatchChangeSession, PlannedRuntimeMerge, Recipe,
        RequiredDerivedRebuildSet, RuntimeMerge, SignalBranchBasis, SignalBranchBasisArtifact,
        SignalBranchBasisCompactExplanation, SignalBranchBasisDenial, SignalBranchBasisIdentity,
        SignalBranchBasisValidationOutcome, SignalBranchForkDenial, SignalBranchForkReceipt,
        SignalBranchForkRequest, SignalBranchForkRequestBasis, SignalBranchHeadPosture,
        SignalBranchRestorePosture, SignalRuntime, SignalRuntimeBuilder, SignalTransaction,
        TemporalCertificationBuilder, TemporalCertificationBundle,
        TemporalCertificationBundleMismatchClass, TemporalCertificationBundleParityReport,
        TemporalCertificationFailure, TemporalCertificationFamily, TemporalCertificationRecord,
        TemporalCertificationSummary, TemporalEligibilityFact, TemporalReconstructabilityArtifact,
        TemporalReplayMismatchClass, TemporalReplayParityReport, TemporalStateRebuildProof,
        TemporalTransactionEvidence, TransactionOutcome, TransactionResult, TransactionTiming,
    };
    pub use crate::logic::transaction::{
        temporal_certification_builder, temporal_certification_bundle,
        temporal_certification_bundle_parity_report, temporal_certification_record,
        temporal_replay_parity_report, REQUIRED_TEMPORAL_CERTIFICATION_FAMILIES,
        TEMPORAL_CERTIFICATION_BUNDLE_PARITY_SCHEMA_VERSION,
        TEMPORAL_CERTIFICATION_BUNDLE_SCHEMA_VERSION, TEMPORAL_REPLAY_PARITY_SCHEMA_VERSION,
    };
    pub use crate::logic::transaction::{
        CommittedObservationEventSummary, MatchingObserverSet, ObservationBoundaryOutcome,
        ObservationBoundarySummary, ObservationDeliveryMode, ObservationHandle,
        ObservationHandleId, ObservationListener, ObservationNotice, ObservationPolicy,
        ObservationReadContext, ObservationRegistrySummary, ObservationTrigger, ObservedNodeSet,
        ObserverId,
    };
    pub use crate::schema::data::SignalSchemaRegistry;
    pub type BatchChange = ChangeBatch;
    pub type BatchChangeResult = ChangeBatchCommit;
    #[cfg(test)]
    pub type CheckpointPolicy<D> = RuntimeCheckpointPolicy<D>;
    #[cfg(test)]
    pub type ComputationFamily = RecipeFamily;
    #[cfg(test)]
    pub type ComputationKey = RecipeKey;
    #[cfg(test)]
    pub type KeyedComputation = KeyedRecipe;
    #[cfg(test)]
    pub type DirtyBatch = ChangeBatch;
    #[cfg(test)]
    pub type SemanticBatchCommit = ChangeBatchCommit;
    #[cfg(test)]
    pub type TierPolicy<T> = RuntimeTierPolicy<T>;
    #[cfg(test)]
    pub type SignalRuntimePolicy = RuntimePolicy;
    #[cfg(test)]
    pub type DefinedComputation<T, F> = RecipeInstance<T, F>;
    #[cfg(test)]
    pub type DefinedKeyedComputation<'a, T, F> = KeyedRecipeInstance<'a, T, F>;
    #[cfg(test)]
    pub type EvaluationSummary = RunSummary;
    #[cfg(test)]
    pub type RuntimeExecutionRequest<'a, D, I, E, Ctx, T> = RuntimeRunRequest<'a, D, I, E, Ctx, T>;
    #[cfg(test)]
    pub type RuntimeHistory<'a, D, I, E, Ctx, T> = History<'a, D, I, E, Ctx, T>;
    #[cfg(test)]
    pub type TransactionExecutionRequest<'tx, 'a, D, I, E, Ctx, T> =
        TransactionRunRequest<'tx, 'a, D, I, E, Ctx, T>;
}

pub mod schema {
    pub use crate::schema::facade::*;
}

pub mod specialist {
    pub use crate::data::comparator::DefaultComparatorPolicyResolver as DefaultComparatorResolver;
    pub use crate::data::comparator::DefaultComparatorResolver as ComparatorHookResolver;
    pub use crate::data::comparator::VersionComparatorPolicy as ComparatorPolicy;
    pub use crate::data::comparator::VersionComparatorResolver as ComparatorResolver;
    pub use crate::data::comparator::{ComparatorPolicyResolver, TierPolicyResolver};
    pub use crate::data::graph::{
        EvaluationStrategy, GcPressure, GraphMaterializer, GraphObserver, ObservationLevel,
        ParallelismHint,
    };
    pub use crate::logic::checkpoint::CheckpointRuntime;
    pub use crate::logic::context::EvaluationContext;
    pub use crate::logic::evaluation::EvaluationRequestMode as RunMode;
    pub use crate::logic::evaluation::{
        AppliedEffectReport, ConditionEvaluationContext, ConditionResolver,
        DefaultConditionResolver, DeferralReason, DiagnosticEnvelope, EvaluationExecutionMetadata,
        EvaluationOutput, EvaluationVerdict, IntoEvaluationOutput, OperationalEffect,
        SuppressionReason, TemporalConditionResolver,
    };
    pub use crate::logic::events::{EventBus, EventFlushError, SubscriberRegistryError};
    #[cfg(feature = "parallel")]
    pub use crate::logic::planner::ParallelExecutionPolicy;
    pub use crate::logic::planner::{
        build_evaluation_plan, execute_prepared_plan, CandidateTask, EligibleTask, EvaluationPlan,
        ExecutedTask, ExecutionPruneReason, ExecutionRecordId, ExecutionReport, ExecutionStage,
        FrontierRouteEvidenceReason, FrontierRouteEvidenceReceipt,
        FrontierRouteEvidenceReceiptError, FrontierRouteSerialFallbackReason,
        ParallelAdmissionReason, PlanSummary, ResolvedExecutionStrategy,
        ResolvedMaintenanceStrategy, SemanticSegmentId, SemanticTaskRange, StageBarrier,
        StageExecutionOutcome, StageExecutionRecord, StageExecutor, TaskExecutionOutcome,
        TaskExecutionRecord, TaskReason,
    };
    pub use crate::logic::prepared::ExecutionReadView as ReadView;
    pub use crate::logic::prepared::PreparedEvaluation as PlannedRun;
    pub use crate::state::SignalBranchHandle as RuntimeBranch;
    pub use crate::state::SignalBranchId as RuntimeBranchId;
    pub use crate::state::SignalSnapshotMeta as RuntimeSnapshotMeta;
    pub use crate::state::SignalSnapshotV1 as RuntimeSnapshot;
    pub use crate::state::{
        SignalSnapshotDiagnostics, SignalSnapshotId, SnapshotArtifactRestoreMode,
        SnapshotArtifactRetentionPolicy, SnapshotDependencyRestoreMode,
        SnapshotRestoreCoarseReason, SnapshotRestoreIntent, SnapshotRestorePlan,
        SnapshotStateRestoreMode,
    };
}

#[cfg(test)]
pub mod advanced {
    pub use super::specialist::*;
    pub use crate::data::comparator::DefaultComparatorPolicyResolver;
    pub use crate::data::comparator::DefaultComparatorResolver;
    pub use crate::data::comparator::VersionComparatorPolicy;
    pub use crate::data::comparator::VersionComparatorResolver;
    pub use crate::logic::evaluation::EvaluationRequestMode;
    pub use crate::logic::prepared::ExecutionReadView;
    pub use crate::logic::prepared::PreparedEvaluation;
}

pub mod adapters {
    pub use crate::data::dirty_set::{BatchedDirtySet, DomainImpact};
    pub use crate::data::effect_mapping::EffectMapping;
    pub use crate::data::evaluator::CheckpointEvaluator;
    pub use crate::data::event_subscriber::{EventSubscriber, SubscriberId};
    pub use crate::data::node::{
        ArtifactPolicyClass, AuthorityPolicy, CanonicalDependencyOrder, ComparatorBasis,
        CompileTimePerformanceContract, ContextRequirement, EquivalenceContract, IdentityBasis,
        MaintenanceMode, NodeAuthorityContract, NodeContract, NodeEvaluationConfig,
        NodeExecutionContract, NodeProjectionContract, NodeReuseContract, NodeSemanticContract,
        PathClass, PerformanceCounterSurface, PerformanceEnforcementLayer,
        ResolvedPerformancePolicy, SuppressionBasis,
    };
    pub use crate::data::proof::{
        CanonicalForm, DedupedNodeBatch, DeltaForm, DependencyBatchEdit, DependencySetEdit,
        DesiredState, DirtyBatch, DirtyBatchEntry, DirtyDelta, FrontierEntryClassification,
        FrontierExecutionCounters, FrontierExecutionSummary, FrontierInclusionBasis, FrontierPlan,
        FrontierPredictedCounters, FrontierSeedCause, FrontierValidationDecision, FrontierWave,
        FrontierWaveEntryPlan, FrontierWaveEntrySummary, FrontierWavePlan, FrontierWaveSummary,
        InvalidationFrontier, InvalidationSeed, InvalidationSeedBatch, InvalidationTraceRecord,
        LocalityFootprint, LocallyOrderedShard, LoweredForm, MergeableOrderedStream,
        MixedSnapshotBatchCommit, NarrowedPropagationSet, OrderedStreamItem,
        OrderedStreamMergeError, PartitionScopeSet, PatchPlan, PendingSnapshotBatch, ResolvedForm,
        SemanticBatchCommit, SingleConsumer, SnapshotBatchCommit, SortedSourceBatch,
        StableShapeSnapshotBatchCommit, StructuralDelta, SubscriberRepair, SubscriberRepairBatch,
        SummaryForm, TouchedScopeSummary, TransitiveFrontierRoot,
    };
    pub use crate::data::reuse::{
        ArtifactEquivalenceContract, ArtifactSemanticBoundary, PersistentCorrespondenceEvidence,
        ReuseBasis, ReuseBoundaryAuthority, ReuseBoundaryContext, ReuseBoundaryEvidence,
        ReuseBoundaryFailure, ReuseBoundaryProof, ReuseCertificationFailure,
        ReuseCertificationRecord, ReuseCrossing, ReuseOrigin, ReuseSemanticRegionIdentity,
        ReuseSource, ReuseStrategy, ReuseStrategyBoundaryAuthority,
    };
    pub use crate::data::subscriber_context::{SubscriberContext, SubscriberContextError};
    pub use crate::data::telemetry::{HostComputedTelemetry, ResourceTelemetry, RuntimeTelemetry};
    pub use crate::data::tier_policy_table::TierPolicyTable;
    pub use crate::logic::planner::{
        FrontierRouteEvidenceReason, FrontierRouteEvidenceReceipt,
        FrontierRouteEvidenceReceiptError, FrontierRouteSerialFallbackReason,
    };
    pub use crate::logic::transaction::{
        branch_state_proof_report, canonical_digest, lowered_strategy_bundle_digest,
        merge_lineage_digest, merge_plan_proof_report, merge_result_proof_report,
        replay_artifact_proof_report, replay_parity_proof_report, runtime_proof_report,
        ArtifactMergeAction, ArtifactMergeComparable, AspectMergeDecisionOutcome,
        AspectMergePolicy, AspectMergePolicyBinding, AspectMergePolicyDescriptor,
        AspectMergePolicyId, AspectMergePolicyName, AspectMergePolicyRegistration,
        AspectMergePolicySelectionBasis, AspectMergePolicyVersion, BranchMergeBase,
        BranchMergeConflictEvidence, BranchMergeConflictKind, BranchMergeConflictRecord,
        BranchMergeConflictSummary, BranchMergeCounters, BranchMergeDeletionFailureEvidence,
        BranchMergeDivergence, BranchMergeExecutionSummary, BranchMergeFailureEvidence,
        BranchMergeFailureKind, BranchMergeIdentityFailureEvidence, BranchMergeKind,
        BranchMergePlan, BranchMergeReconciliationPolicy, BranchMergeRequest,
        BranchMergeRequestDenial, BranchMergeRequestScope, BranchMergeRequestScopeFamily,
        BranchMergeResult, BranchMergeStrategy, BranchMutationJournalSlice, BranchMutationLedger,
        BranchStateDenseGridProofBasis, BranchStateProofBasis, BranchStateProofReport,
        CausalityCarryPolicy, ConflictIsolationGranularity, ConflictIsolationPolicyDescriptor,
        ConflictIsolationPolicyId, ConflictIsolationPolicyName,
        ConflictIsolationPolicyRegistration, ConflictIsolationPolicyVersion,
        ConflictIsolationSelectionBasis, ConflictMergePolicy, ConflictPolicyDescriptor,
        ConflictPolicyId, ConflictPolicyName, ConflictPolicyRegistration,
        ConflictPolicySelectionBasis, ConflictPolicyVersion, ConservativeOverlapExpansion,
        DeletionMergePolicy, DeletionPolicyDescriptor, DeletionPolicyId, DeletionPolicyName,
        DeletionPolicyRegistration, DeletionPolicySelectionBasis, DeletionPolicyVersion,
        DependencyFingerprint, DependencyRemapRecord, DuplicateAspectMergePolicyRegistration,
        DuplicateConflictIsolationPolicyRegistration, DuplicateConflictPolicyRegistration,
        DuplicateDeletionPolicyRegistration, DuplicateIdentityMatcherRegistration,
        DuplicateMergeBaseStrategyRegistration, DuplicateMergeStrategyRegistration,
        DuplicateSourceOnlyPolicyRegistration, ExistingTargetMergePolicy,
        FoundationalScopeLoweringDenial, FrozenAspectMergePolicyRegistry,
        FrozenConflictIsolationRegistry, FrozenConflictPolicyRegistry,
        FrozenDeletionPolicyRegistry, FrozenIdentityMatcherRegistry,
        FrozenMergeBaseStrategyRegistry, FrozenMergeStrategyRegistry,
        FrozenSourceOnlyPolicyRegistry, IdentityCorrespondenceBasis, IdentityCorrespondenceRecord,
        IdentityCorrespondenceStatus, IdentityMatchPolicy, IdentityMatcherDescriptor,
        IdentityMatcherId, IdentityMatcherName, IdentityMatcherRegistration,
        IdentityMatcherSelectionBasis, IdentityMatcherVersion, LoweredAspectMergeDecisionPlan,
        LoweredAspectMergeDecisionRecord, LoweredConflictIsolationPlan,
        LoweredConflictIsolationRecord, LoweredDeletionPolicyPlan, LoweredFoundationalMergeRequest,
        LoweredIdentityCorrespondencePlan, LoweredMergeBasePlan, LoweredMergePlan,
        MergeBaseSelectionBasis, MergeBaseSelectionPolicy, MergeBaseStrategyDescriptor,
        MergeBaseStrategyId, MergeBaseStrategyName, MergeBaseStrategyRegistration,
        MergeBaseStrategyVersion, MergeBoundaryWitness, MergeBoundaryWitnessKind,
        MergeDecisionBasis, MergeNodeMap, MergePlanProofReport, MergeResultProofReport,
        MergeStrategyDescriptor, MergeStrategyId, MergeStrategyName, MergeStrategyRegistration,
        MergeStrategySelectionBasis, MergeStrategyVersion, MergeTouchedNodeSet,
        MergedArtifactRecord, NodeMergeInputState, NodeMergePlan, NodeReconciliationDecision,
        NodeReconciliationShape, NormalizedBranchMergeRequest, NormalizedBranchMergeRequestScope,
        PlannedMergeCandidateSet, ProofMinimalOverlapBasis, ReplayArtifactProofInput,
        ReplayArtifactProofReport, ReplayMismatchClass, ReplayParityProofReport,
        RetainedArtifactCarryPolicy, RuntimeArtifactCarryPolicy, RuntimeMaterializer,
        RuntimeProofReport, ScopedMergeProofPacket, SelectedMergeSemanticsBundle,
        SignalAspectPolicyInventoryEntry, SignalBranchBasisInspectionRow,
        SignalCompatibilityInspectionRow, SignalDeliveryStrategyIdentity,
        SignalInvalidationStrategyIdentity, SignalMergeStrategyIdentity,
        SignalMergeStrategyWitness, SignalMergeStrategyWitnessDenial,
        SignalMergeStrategyWitnessDenialKind, SignalMergeSupportInspectionAbsence,
        SignalMergeSupportInspectionAbsenceKind, SignalMergeSupportInspectionOutcome,
        SignalMergeSupportInspectionWitness, SignalMergeSupportReadinessPosture,
        SignalScopedMergeCanonicalBasisBundle, SignalScopedMergeCanonicalLocatorBundle,
        SignalScopedMergeDiagnosticRow, SignalScopedMergeInspectionRow,
        SignalScopedMergeLocatorBundle, SignalSelectedAspectRequestEntry,
        SignalStrategyInspectionRow, SourceNodeAdoptionPlanCore, SourceOnlyMergePolicy,
        SourceOnlyPolicyDescriptor, SourceOnlyPolicyId, SourceOnlyPolicyName,
        SourceOnlyPolicyRegistration, SourceOnlyPolicySelectionBasis, SourceOnlyPolicyVersion,
        StructuralMergeCandidateRecord, StructuralMergeJournalSlice,
        BRANCH_STATE_PROOF_BASIS_VERSION, MERGE_PROOF_SCHEMA_VERSION,
    };
}

pub mod diagnostics {
    pub use crate::diagnostics::Diagnostics;
    pub use crate::diagnostics::DiagnosticsLevel;
    pub use crate::diagnostics::LineageEvent;
    pub use crate::diagnostics::ReplayView;
    pub use crate::diagnostics::{
        compare_execution_history, compare_execution_reports, compare_explanations,
        compare_failures, compare_flows, compare_graphs, compare_lineage_records, compare_plans,
        compare_replay_slices, diagnostics_for_graph, diagnostics_for_runtime,
        explanations_semantically_equivalent, graphs_semantically_equivalent, inspect_execution,
        inspect_flow, inspect_graph, inspect_plan, inspect_report, lineage_records_equivalent,
        plans_semantically_equivalent, render_execution_history_summary,
        render_execution_report_summary, render_explanation_summary, render_failure_summary,
        render_flow_summary, render_graph_summary, render_plan_summary, repeat_run_summaries_equal,
        replay_slices_equivalent, reports_semantically_equivalent,
        serial_parallel_reports_equivalent, ApplySummary, ArtifactRetentionPolicy,
        ArtifactTransitionKind, ChangeInputSummary, DiagnosticMismatch, DiagnosticMismatchCategory,
        DiagnosticsAvailability, EvaluationPlanSummary, EventEpochOutcome, EventEpochSummary,
        EventSubscriberOutcome, EventSubscriberOutcomeKind, ExecutionFailureContext,
        ExecutionFailurePhase, ExecutionHistoryNodeSummary, ExecutionHistorySummary,
        ExecutionInspector, ExecutionReportDiff, ExecutionReportSummary, ExplanationDiff,
        ExplanationSummary, FailureDiff, FailureSummary, FlowCauseSample, FlowDiff, FlowInspector,
        FlowSummary, FrontierCyclePolicy, FrontierPropagationPolicy, FrontierTracingPolicy,
        GraphComparisonDiagnostics, GraphDiagnostics, GraphDiff, GraphHealthDiagnostics,
        GraphInspectDiagnostics, GraphInspector, GraphSummary, HistoryDiff, InvalidationCause,
        InvalidationSummary, LineageArtifactId, LineageDiff, LineageRecordKind,
        ParallelAdmissionPolicy, PlanDiff, PlanInspector, PlanningSummary, PrecomputeSummary,
        ReconstructionBudget, ReplayCursor, ReplayDetailPolicy, ReplayDiff, ReplayEventKind,
        ReplayFrame, ReportInspector, RetentionBudget, RollbackDiagnostic, RollbackSummary,
        SemanticRetentionPolicy, SnapshotRestoreKind, SnapshotRestoreLineageMode,
        TemporalCostContractSummary, TemporalDiagnosticsSummary, TemporalPerformanceFailureMode,
    };
}

#[cfg(test)]
pub mod integration {
    pub use super::adapters::*;
}

pub mod history {
    pub use crate::diagnostics::LineageEvent;
    pub use crate::diagnostics::ReplayView;
    pub use crate::diagnostics::SnapshotRestoreKind;
    pub use crate::logic::transaction::{
        BoundaryBridgedSignalBranchBasisArtifact, SignalBranchBasis, SignalBranchBasisArtifact,
        SignalBranchBasisDenial, SignalBranchBasisIdentity, SignalBranchBasisValidationOutcome,
        SignalBranchForkDenial, SignalBranchForkReceipt, SignalBranchForkRequest,
        SignalBranchForkRequestBasis, SignalBranchHeadPosture, SignalBranchRestorePosture,
        StaleSignalBranchBasisArtifact,
    };
    pub use crate::state::SignalBranchHandle as RuntimeBranch;
    pub use crate::state::SignalBranchId as RuntimeBranchId;
    pub use crate::state::SignalSnapshotMeta as RuntimeSnapshotMeta;
    pub use crate::state::SignalSnapshotV1 as RuntimeSnapshot;
    pub use crate::state::{
        SignalSnapshotDiagnostics, SignalSnapshotId, SnapshotArtifactRestoreMode,
        SnapshotArtifactRetentionPolicy, SnapshotDependencyRestoreMode,
        SnapshotRestoreCoarseReason, SnapshotRestoreIntent, SnapshotRestorePlan,
        SnapshotStateRestoreMode,
    };
}

#[cfg(test)]
pub use self::adapters::*;
#[cfg(test)]
pub use self::core::*;
#[cfg(not(test))]
pub use self::core::{
    mark_changed, mark_changed_with_regions, mark_dirty, mark_dirty_with_regions,
    AdmittedHostComputedReadSet, AfterCondition, Aspect, AspectMask, AspectVersion,
    AtOrAfterCondition, CanonicalChangedRegions, ChangedRegion, ClockAdvanceOrdinal,
    ClockAdvanceRequest, ClockAuthority, ClockCheckpointId, ClockDomain, ClockTick,
    CommittedHostComputedArtifact, DebounceCondition, DeferredTemporalEligibility,
    DeniedHostComputedEvaluation, DeniedHostComputedReadSet, DependencyEdge, EvaluationCondition,
    HostComputedApiFamily, HostComputedDenialClass, HostComputedDependencyPatch,
    HostComputedDescriptor, HostComputedDescriptorId, HostComputedDiagnosticsSummary,
    HostComputedEvaluationOutcome, HostComputedEvaluationRequest, HostComputedEvaluationResponse,
    HostComputedEvaluator, HostComputedFailure, HostComputedFailureClass, HostComputedOutcomeClass,
    HostComputedPreparedResponse, IntervalAnchor, IntervalCondition, IntervalPeriod,
    LoweredTemporalEligibility, MissedTickPolicy, NodeBuilder, NodeEvaluationResult, NodeId,
    NodeState, OutputChange, OutputIdentity, PartitionMatchMode, PartitionSubscription,
    PartitionToken, PreparedHostComputedEvaluation, ReadyTemporalEligibility, RuntimeClockBasis,
    SignalError, SignalGraph, StagedHostComputedArtifact, StaleAfterCondition,
    TemporalClockAdvanceSummary, TemporalCondition, TemporalDuration, TemporalEligibilityAuthority,
    TemporalExecutionSummary, TemporalReadyPromotionSummary, TemporalWakeAdmissionSummary,
    TemporalWakeOwner, TemporalWakeRetirementBatch, ThrottleCondition, ValidatedClockAdvance,
    CORE_STORAGE_PROFILE_ID, MAX_ASPECTS,
};
#[cfg(test)]
pub use self::diagnostics::*;
#[cfg(not(test))]
pub use self::diagnostics::{diagnostics_for_graph, diagnostics_for_runtime};
#[cfg(test)]
pub use self::diagnostics::{
    DiagnosticsLevel as DiagnosticsTier, LineageEvent as LineageRecord, ReplayView as ReplaySlice,
};
#[cfg(not(test))]
pub use self::history::RuntimeBranchId as SignalBranchId;
#[cfg(test)]
pub use self::history::{
    RuntimeBranch as SignalBranchHandle, RuntimeBranchId as SignalBranchId,
    RuntimeSnapshot as SignalSnapshotV1, RuntimeSnapshotMeta as SignalSnapshotMeta,
};
#[cfg(test)]
pub use self::runtime::*;
#[cfg(not(test))]
pub use self::runtime::{
    bridge_signal_branch_basis_trust_boundary, mark_dirty_batch, BatchChange, BatchChangeResult,
    BatchChangeSession, ChangeBatch, ChangeBatchCommit, History, IntervalWakeRegeneration,
    PreviousValueRevision, ReadyTemporalWake, RecipeInstance, RetiredTemporalWake, RunSummary,
    RuntimeConfig, RuntimeMerge, RuntimePolicy, ScheduledTemporalWake, SignalBranchBasis,
    SignalBranchBasisArtifact, SignalBranchBasisDenial, SignalBranchBasisIdentity,
    SignalBranchBasisValidationOutcome, SignalBranchForkDenial, SignalBranchForkReceipt,
    SignalBranchForkRequest, SignalBranchForkRequestBasis, SignalBranchHeadPosture,
    SignalBranchRestorePosture, SignalRuntime, SignalTransaction, TemporalFrontierSnapshot,
    TemporalPreviousValueAccess, TemporalPreviousValueReference, TemporalWakeId,
    TemporalWakeReschedule, TemporalWakeRetirementReason, TemporalWakeReuse, TemporalWakeSummary,
    TransactionOutcome, TransactionResult, TransactionTiming, WakeOrdinal,
};
#[cfg(test)]
pub use self::runtime::{
    ChangeBatch as DirtyBatch, ChangeBatchCommit as SemanticBatchCommit, History as RuntimeHistory,
    KeyedRecipe as KeyedComputation, KeyedRecipeInstance as DefinedKeyedComputation,
    RecipeFamily as ComputationFamily, RecipeInstance as DefinedComputation,
    RunSummary as EvaluationSummary, RuntimeCheckpointPolicy as CheckpointPolicy,
    RuntimeConfig as SignalRuntimeConfig, RuntimePolicy as SignalRuntimePolicy,
    RuntimeRunRequest as RuntimeExecutionRequest, RuntimeTierPolicy as TierPolicy,
    TransactionRunRequest as TransactionExecutionRequest,
};
#[cfg(all(feature = "parallel", not(test)))]
pub use self::specialist::ParallelExecutionPolicy;
#[cfg(test)]
pub use self::specialist::{
    ComparatorPolicy as VersionComparatorPolicy, ComparatorResolver as VersionComparatorResolver,
    ConditionEvaluationContext, ConditionResolver, DefaultConditionResolver, EvaluationContext,
    EvaluationOutput, PlannedRun as PreparedEvaluation, ReadView as ExecutionReadView,
    RunMode as EvaluationRequestMode, TemporalConditionResolver,
};
#[cfg(not(test))]
pub use self::specialist::{EvaluationContext, RunMode};
#[cfg(not(test))]
pub use crate::data::async_node::{
    async_node_compile_time_boundary_proof, async_node_milestone_d_certification_run,
    async_node_milestone_d_performance_closeout, async_node_milestone_d_scenario_matrix,
    AsyncKeyedNodeCapabilityBinding, AsyncKeyedNodeCapabilityEquivalenceDenialClass,
    AsyncKeyedNodeCapabilityEquivalenceReport, AsyncKeyedNodeHistoricalParityDenialClass,
    AsyncKeyedNodeHistoricalParityReport, AsyncNodeAdmissionClass,
    AsyncNodeAdmissionClassification, AsyncNodeCapabilityAliasLoweringProof,
    AsyncNodeCapabilityDeclaration, AsyncNodeCapabilityEquivalenceDenialClass,
    AsyncNodeCapabilityEquivalenceReport, AsyncNodeCompileTimeBoundaryProof,
    AsyncNodeConditionBlockClass, AsyncNodeDownstreamDependenceFact, AsyncNodeGateStateReport,
    AsyncNodeHierarchyCancellationReport, AsyncNodeHierarchyHistoricalParityDenialClass,
    AsyncNodeHierarchyHistoricalParityReport, AsyncNodeHierarchyReplaySummary,
    AsyncNodeMilestoneDCertificationRun, AsyncNodeMilestoneDCertificationRunSummary,
    AsyncNodeMilestoneDPerformanceClaimId, AsyncNodeMilestoneDPerformanceCloseout,
    AsyncNodeMilestoneDPerformanceCloseoutRow, AsyncNodeMilestoneDPerformanceCloseoutSummary,
    AsyncNodeMilestoneDScenarioEvidenceKind, AsyncNodeMilestoneDScenarioId,
    AsyncNodeMilestoneDScenarioInputs, AsyncNodeMilestoneDScenarioMatrix,
    AsyncNodeMilestoneDScenarioMatrixSummary, AsyncNodeMilestoneDScenarioRow,
    AsyncNodePayloadContract, AsyncNodePayloadContractId, AsyncNodeRequestAdmissionReport,
    AsyncNodeRequestIntent, AsyncNodeRevalidationIntent, AsyncNodeRevalidationReport,
    DeniedAsyncKeyedNodeCapabilityEquivalence, DeniedAsyncKeyedNodeHistoricalParity,
    DeniedAsyncNodeCapabilityEquivalence, DeniedAsyncNodeHierarchyHistoricalParity,
    FrozenAsyncNodeCapabilityDescriptor, LoweredAsyncNodeCapabilityBundle,
    ValidatedAsyncNodeCapabilityDeclaration, ASYNC_NODE_COMPILE_TIME_BOUNDARY_PROOF_SCHEMA_VERSION,
    ASYNC_NODE_MILESTONE_D_CERTIFICATION_RUN_SCHEMA_VERSION,
    ASYNC_NODE_MILESTONE_D_PERFORMANCE_CLOSEOUT_SCHEMA_VERSION,
    ASYNC_NODE_MILESTONE_D_SCENARIO_MATRIX_SCHEMA_VERSION,
    REQUIRED_ASYNC_NODE_COMPILE_TIME_FIXTURES, REQUIRED_ASYNC_NODE_MILESTONE_D_PERFORMANCE_CLAIMS,
    REQUIRED_ASYNC_NODE_MILESTONE_D_SCENARIOS,
};
#[cfg(test)]
pub use crate::data::comparator::DefaultComparatorPolicyResolver;
#[cfg(test)]
pub use crate::data::comparator::DefaultComparatorResolver;
#[cfg(test)]
pub use crate::data::dependency::CanonicalDependencies;
#[cfg(test)]
pub use crate::data::graph::{GcPressure, ObservationLevel, ParallelismHint};
#[cfg(test)]
pub use crate::data::output::MemoizedResultOrigin;
pub use crate::data::resource::{
    resource_certification_builder, resource_certification_bundle,
    resource_certification_bundle_parity_report, resource_milestone_b_certification_run,
    resource_milestone_b_hostile_scenario_evidence, resource_milestone_b_performance_closeout,
    resource_milestone_b_scenario_matrix, resource_milestone_c_certification_run,
    resource_milestone_c_policy_certification_builder,
    resource_milestone_c_policy_certification_bundle,
    resource_milestone_c_policy_performance_closeout, resource_milestone_c_policy_scenario_matrix,
    ActiveResourceRevalidationProof, AdmittedResourceCompletion, AdmittedResourceRequest,
    AdmittedResourceRetry, AdmittedResourceRevalidation, AsyncDenialId, CancelledResourceRequest,
    CommittedResourceCompletionArtifact, CompletionDenialClass, DeniedResourceCancellation,
    DeniedResourceCompletion, DeniedResourcePolicyRestoreCompatibility, DeniedResourceRejection,
    DeniedResourceRetry, DeniedResourceRevalidation, DeniedResourceTimeout,
    DeniedResourceTimeoutHeartbeatExtension, DependencyChangeResourceRevalidationProof,
    ExtendedResourceTimeoutHeartbeat, FrozenResourcePolicyDescriptor,
    FrozenResourcePolicyDescriptorSet, FrozenResourcePolicyRegistry,
    FulfilledLifecycleResourceRevalidationProof, InFlightResourceRequest,
    LoweredResourceDescriptor, LoweredResourcePolicyBundle, ObservedResourceNodeState,
    ObserverDemandResourceRevalidationProof, RawCompletionEnvelope, RejectedResourceRequest,
    ResourceAttemptId, ResourceBoundaryKind, ResourceBoundaryPerformanceEnvelope,
    ResourceBranchEpoch, ResourceBranchRestoreReport, ResourceCancellationDecisionClass,
    ResourceCancellationDecisionPlan, ResourceCancellationDenialClass,
    ResourceCancellationGraceWindow, ResourceCancellationOrdinal,
    ResourceCancellationPolicyDeclaration, ResourceCancellationReason, ResourceCancellationReport,
    ResourceCertificationBuilder, ResourceCertificationBundle,
    ResourceCertificationBundleMismatchClass, ResourceCertificationBundleParityReport,
    ResourceCertificationFailure, ResourceCertificationFamily, ResourceCertificationRecord,
    ResourceCertificationSummary, ResourceCompletionAdmissionReport,
    ResourceCompletionBatchAdmissionReport, ResourceCompletionCommitReport,
    ResourceCompletionDenialStagingReport, ResourceCompletionOrdinal,
    ResourceCompletionRollbackReport, ResourceCompletionRollbackSubject,
    ResourceCompletionStagingReport, ResourceCostContractId, ResourceCostPosture,
    ResourceDeclarationReport, ResourceDensityStrategy, ResourceDependentCancellationPropagation,
    ResourceDescriptorId, ResourceDescriptorVersion, ResourceDiagnosticsDecisionClass,
    ResourceDiagnosticsDecisionPlan, ResourceDiagnosticsExpansionBudget,
    ResourceDiagnosticsExpansionDenial, ResourceDiagnosticsExpansionDenialClass,
    ResourceDiagnosticsPolicyDeclaration, ResourceDiagnosticsSummary, ResourceGeneration,
    ResourceHostCancellationAdvisory, ResourceInFlightStatus, ResourceInitialLifecycleClass,
    ResourceIntentEquivalenceCoalescing, ResourceLifecycleClass, ResourceLifecycleOrdinal,
    ResourceLifecyclePolicyDeclaration, ResourceLifecycleRetentionCompactionReport,
    ResourceLifecycleSummary, ResourceLifecycleTransition, ResourceLifecycleTransitionKind,
    ResourceMilestoneBCertificationRun, ResourceMilestoneBCertificationRunSummary,
    ResourceMilestoneBHostileScenarioEvidence, ResourceMilestoneBHostileScenarioEvidenceRow,
    ResourceMilestoneBPerformanceClaimId, ResourceMilestoneBPerformanceCloseout,
    ResourceMilestoneBPerformanceCloseoutRow, ResourceMilestoneBPerformanceCloseoutSummary,
    ResourceMilestoneBScenarioEvidenceKind, ResourceMilestoneBScenarioId,
    ResourceMilestoneBScenarioMatrix, ResourceMilestoneBScenarioMatrixSummary,
    ResourceMilestoneBScenarioRow, ResourceMilestoneCCertificationRun,
    ResourceMilestoneCCertificationRunSummary, ResourceMilestoneCPolicyCertificationBuilder,
    ResourceMilestoneCPolicyCertificationBundle, ResourceMilestoneCPolicyCertificationFamily,
    ResourceMilestoneCPolicyCertificationRecord, ResourceMilestoneCPolicyCertificationSummary,
    ResourceMilestoneCPolicyPerformanceClaimId, ResourceMilestoneCPolicyPerformanceCloseout,
    ResourceMilestoneCPolicyPerformanceCloseoutRow,
    ResourceMilestoneCPolicyPerformanceCloseoutSummary,
    ResourceMilestoneCPolicyScenarioEvidenceKind, ResourceMilestoneCPolicyScenarioId,
    ResourceMilestoneCPolicyScenarioMatrix, ResourceMilestoneCPolicyScenarioMatrixSummary,
    ResourceMilestoneCPolicyScenarioRow, ResourceNodeDeclaration, ResourceNodeId,
    ResourceObservationBatchReport, ResourceObservationDecisionClass,
    ResourceObservationDecisionPlan, ResourceObservationEvent,
    ResourceObservationPolicyDeclaration, ResourceOldHostWorkCancellationAdvisory,
    ResourceOutputContinuity, ResourceOutputContinuityDecisionClass,
    ResourceOutputContinuityDecisionPlan, ResourceOutputContinuityPolicyDeclaration,
    ResourceOverlappingGenerationAdmission, ResourcePayloadContract, ResourcePayloadContractDigest,
    ResourcePayloadContractId, ResourcePolicyCompatibilityClass,
    ResourcePolicyCompatibilityFamilyReport, ResourcePolicyCompatibilityPosture,
    ResourcePolicyCompatibilityReport, ResourcePolicyDescriptor, ResourcePolicyDescriptorId,
    ResourcePolicyDigest, ResourcePolicyKind, ResourcePolicyName, ResourcePolicyRegistration,
    ResourcePolicyRegistryError, ResourcePolicyRegistryFreezeReport, ResourcePolicyResolutionError,
    ResourcePolicyRestoreCompatibilityDenialClass, ResourcePolicyRestoreCompatibilityProof,
    ResourcePolicySelectionBasis, ResourcePolicyVersion, ResourceRejectionDenialClass,
    ResourceRejectionOrdinal, ResourceRejectionReason, ResourceRejectionReport,
    ResourceReplayAvailabilityClass, ResourceReplayAvailabilityDenialClass,
    ResourceReplayAvailabilityReport, ResourceReplayDecisionClass, ResourceReplayDecisionPlan,
    ResourceReplayPolicyDeclaration, ResourceReplayReconstructionReport,
    ResourceRequestAdmissionReport, ResourceRequestHandle, ResourceRequestId,
    ResourceRequestIntent, ResourceRequestIntentDigest, ResourceResolvedPolicy,
    ResourceResolvedPolicyBundle, ResourceRetainedDeniedCompletionAvailability,
    ResourceRetainedDeniedCompletionAvailabilityClass, ResourceRetainedHistoryAvailability,
    ResourceRetainedHistoryAvailabilityClass, ResourceRetainedRetryLineageAvailability,
    ResourceRetainedRetryLineageAvailabilityClass, ResourceRetentionCompactionBudget,
    ResourceRetentionDecisionClass, ResourceRetentionDecisionPlan,
    ResourceRetentionPolicyDeclaration, ResourceRetryAdmissionReport, ResourceRetryBudgetScope,
    ResourceRetryDecisionClass, ResourceRetryDecisionPlan, ResourceRetryDenialClass,
    ResourceRetryOrdinal, ResourceRetryPolicyDeclaration, ResourceRetryReason,
    ResourceRetryScheduleReport, ResourceRevalidationCoalescing, ResourceRevalidationDecisionClass,
    ResourceRevalidationDecisionPlan, ResourceRevalidationDenialClass,
    ResourceRevalidationFreshnessClass, ResourceRevalidationFreshnessDecision,
    ResourceRevalidationIntent, ResourceRevalidationPolicyDeclaration, ResourceRevalidationReport,
    ResourceRuntimeSummary, ResourceRuntimeSummaryReadReport, ResourceStaleAfterDecisionClass,
    ResourceStaleAfterDecisionPlan, ResourceStaleAfterPolicyDeclaration,
    ResourceSupersessionDecisionClass, ResourceSupersessionDecisionPlan,
    ResourceSupersessionOldHostWorkPosture, ResourceSupersessionOrdinal,
    ResourceSupersessionOverlapDisposition, ResourceSupersessionPolicyDeclaration,
    ResourceSupersessionRecord, ResourceTimeoutDeadlineAuthority, ResourceTimeoutDecisionClass,
    ResourceTimeoutDecisionPlan, ResourceTimeoutDenialClass,
    ResourceTimeoutHeartbeatExtensionDenialClass, ResourceTimeoutHeartbeatExtensionReport,
    ResourceTimeoutOrdinal, ResourceTimeoutOutcomeClass, ResourceTimeoutPolicyDeclaration,
    ResourceTimeoutReport, RetainedResourceRetryLineage, RolledBackResourceCompletionArtifact,
    ScheduledResourceRetry, StagedDeniedResourceCompletionEffect, StagedResourceCompletionEffect,
    TerminalStateResourceRevalidationProof, TimedOutResourceRequest, ValidatedCompletionEnvelope,
    ValidatedResourcePolicyDeclaration, ValidatedResourcePolicyReference,
    REQUIRED_RESOURCE_CERTIFICATION_FAMILIES, REQUIRED_RESOURCE_MILESTONE_B_HOSTILE_SCENARIOS,
    REQUIRED_RESOURCE_MILESTONE_B_PERFORMANCE_CLAIMS, REQUIRED_RESOURCE_MILESTONE_B_SCENARIOS,
    REQUIRED_RESOURCE_MILESTONE_C_POLICY_CERTIFICATION_FAMILIES,
    REQUIRED_RESOURCE_MILESTONE_C_POLICY_PERFORMANCE_CLAIMS,
    REQUIRED_RESOURCE_MILESTONE_C_POLICY_SCENARIOS,
    RESOURCE_CERTIFICATION_BUNDLE_PARITY_SCHEMA_VERSION,
    RESOURCE_CERTIFICATION_BUNDLE_SCHEMA_VERSION, RESOURCE_DIAGNOSTICS_SUMMARY_SCHEMA_VERSION,
    RESOURCE_MILESTONE_B_CERTIFICATION_RUN_SCHEMA_VERSION,
    RESOURCE_MILESTONE_B_HOSTILE_SCENARIO_EVIDENCE_SCHEMA_VERSION,
    RESOURCE_MILESTONE_B_PERFORMANCE_CLOSEOUT_SCHEMA_VERSION,
    RESOURCE_MILESTONE_B_SCENARIO_MATRIX_SCHEMA_VERSION,
    RESOURCE_MILESTONE_C_CERTIFICATION_RUN_SCHEMA_VERSION,
    RESOURCE_MILESTONE_C_POLICY_CERTIFICATION_BUNDLE_SCHEMA_VERSION,
    RESOURCE_MILESTONE_C_POLICY_PERFORMANCE_CLOSEOUT_SCHEMA_VERSION,
    RESOURCE_MILESTONE_C_POLICY_SCENARIO_MATRIX_SCHEMA_VERSION,
};
#[cfg(test)]
pub use crate::data::trace::{
    ArtifactAuthorityClass, ArtifactMergeAuthority, CausalityMetadata, MergeAdoptability,
    RetainedDiagnosticArtifact,
};
#[cfg(test)]
pub use crate::logic::evaluation::IntoEvaluationOutput;
#[cfg(test)]
pub use crate::logic::events::*;
#[cfg(test)]
pub use crate::logic::explain::{
    CausalDisposition, ConditionDecision, NodeExplanation, ScopeProvenanceKind, UpstreamCause,
};
#[cfg(test)]
pub use crate::logic::planner::*;
#[cfg(test)]
pub use crate::logic::transaction::{DecisionDetail, DecisionRecord};
#[cfg(test)]
pub use crate::presentation::boundaries::contracts::*;
#[cfg(test)]
pub use crate::presentation::boundaries::transaction_contract::*;
#[cfg(test)]
pub use crate::presentation::harness::*;
#[cfg(test)]
pub use crate::presentation::metrics::{GraphMetrics, RuntimeMetrics};
#[cfg(test)]
pub use crate::presentation::outputs::deployment::*;
#[cfg(test)]
pub use crate::state::{
    SignalSnapshotId, SnapshotArtifactRestoreMode, SnapshotDependencyRestoreMode,
    SnapshotRestoreCoarseReason, SnapshotRestoreIntent, SnapshotStateRestoreMode,
};
#[cfg(test)]
pub use crate::tests::support::GraphDependencyBatchExt;
