mod capability;
mod config;
mod declaration;
mod declaration_aspect;
#[cfg(test)]
mod declaration_aspect_test_support;
mod declaration_authority_summary;
mod declaration_bridge_routing;
mod declaration_capability;
mod declaration_entry_orchestration;
mod declaration_entry_seam;
mod declaration_envelope;
mod declaration_evidence;
mod declaration_family;
mod declaration_legality;
mod declaration_progression;
mod declaration_publication;
mod declaration_receipt;
mod declaration_relational_routing;
mod declaration_route_plan;
mod declaration_signal_compatibility;
mod domain_entry;
mod domain_handle;
#[cfg(test)]
pub(crate) mod domain_test_support;
mod support;

pub use capability::{
    CapabilityAdmissionDecision, CapabilityAdmissionError, CapabilityAdmissionFailureClass,
    HistoricalEvaluationCapability, IdentityEvolutionCapability, LiveQueryCapability,
    PreviewSessionCapability, QueryCompositionCapability, QueryContextCapability,
    QueryReadCapability, WorkflowOrchestrationCapability, WorthQueryApplicationFacade,
    WorthQueryCapabilityResolution, WorthQueryFacadeCounters, WorthQueryFacadeError,
    WorthQueryFacadeFailureClass,
};
pub use config::{
    ConfigurationAdmissionError, ConfigurationAdmissionFailureClass, ValidatedWorthQueryConfig,
    WorthQueryConfig, WorthQueryConfigCounters, WorthQueryConfigSectionFamily,
    WorthQueryConfigSectionResolution, WorthQueryQueryConfig, WorthQueryRelationalConfig,
    WorthQueryRuntimeBridgeConfig, WorthQuerySignalConfig, WorthQueryStoreConfig,
    WorthQuerySubsystemOwner,
};
pub use declaration::{
    WorthQueryAsyncDeclarationClause, WorthQueryAsyncDeclarationSupport,
    WorthQueryAsyncFailurePosture, WorthQueryAsyncLoadingPosture,
    WorthQueryAsyncRequestIdentityPart, WorthQueryAsyncRequestIdentityValue,
    WorthQueryAsyncResourceRequestIdentity, WorthQueryAsyncResourceRequestIdentityError,
    WorthQueryAsyncSourceFamily, WorthQueryCanonicalDeclarationArtifact,
    WorthQueryCanonicalDeclarationComparison, WorthQueryDeclarationCanonicalEntry,
    WorthQueryDeclarationCanonicalEntryKind, WorthQueryDeclarationCanonicalValue,
    WorthQueryDeclarationCanonicalizationError, WorthQueryDeclarationCanonicalizationVersion,
    WorthQueryDeclarationFutureProjection, WorthQueryDeclarationInput,
    WorthQueryTemporalDeclarationClause, WorthQueryTemporalDeclarationSupport,
    WorthQueryTemporalDuration, WorthQueryTemporalWindowKind,
};
pub use declaration_aspect::{
    WorthQueryDeclarationAspectContract, WorthQueryDeclarationAspectCoverage,
    WorthQueryDeclarationAspectCoverageBasis, WorthQueryDeclarationAspectFit,
    WorthQueryDeclarationAuthorityAspectMismatch,
};
pub use declaration_authority_summary::{
    WorthQueryDeclarationBridgeAuthorityAspectSummary,
    WorthQueryDeclarationRelationalAuthorityAspectSummary,
    WorthQueryDeclarationSignalAuthorityAspectSummary,
};
pub(crate) use declaration_bridge_routing::{
    query_truth_commit_identity, query_truth_snapshot_identity,
};
pub use declaration_bridge_routing::{
    WorthQueryDeclarationBridgeBinding, WorthQueryDeclarationBridgeContinuationContract,
    WorthQueryDeclarationBridgeContinuationFamily, WorthQueryDeclarationBridgeContinuationMode,
    WorthQueryDeclarationBridgeContinuationRequest, WorthQueryDeclarationBridgeRouting,
    WorthQueryDeclarationBridgeRoutingChecked, WorthQueryDeclarationBridgeRoutingClass,
    WorthQueryDeclarationBridgeRoutingDeferred, WorthQueryDeclarationBridgeRoutingDenialCause,
    WorthQueryDeclarationBridgeRoutingDenied, WorthQueryDeclarationBridgeRoutingExplanation,
    WorthQueryDeclarationBridgeRoutingFailed, WorthQueryDeclarationBridgeRoutingInput,
    WorthQueryDeclarationBridgeRoutingSupportReport, WorthQueryDeclarationBridgeRoutingSupportRow,
    WorthQueryDeclarationBridgeRoutingSupportStatus,
    WorthQueryDeclarationBridgeRoutingTerminalError, WorthQueryDeclarationBridgeTruthContext,
    WorthQueryDeclarationEntryBridgeRoutingError,
};
pub use declaration_capability::{
    WorthQueryAsyncDeclarationDenial, WorthQueryBatchCapableDeclaration,
    WorthQueryBatchCapableGrouping, WorthQueryBridgeContinuationAuthority,
    WorthQueryBridgeContinuationDeclaration, WorthQueryDeclarationAdmissionError,
    WorthQueryDeclarationCapabilityDenial, WorthQueryDeclarationCapabilityStatus,
    WorthQueryDeclarationCapabilityVerb, WorthQueryDeclarationFamilySupportChecked,
    WorthQueryDeclarationFamilySupportReport, WorthQueryDeclarationFamilySupportRow,
    WorthQueryDeclarationGroupedPostureTag, WorthQueryDeclarationPrimaryAuthorityTag,
    WorthQueryDeclarationSignalCompatibilityTag, WorthQueryDeclarationSupportsBatchGrouping,
    WorthQueryDeclarationSupportsBridgeContinuation,
    WorthQueryDeclarationSupportsNeighborhoodGrouping,
    WorthQueryDeclarationSupportsRelationalTruth, WorthQueryDeclarationSupportsSignalCompatibility,
    WorthQueryDeclaredFamilyChecked, WorthQueryDescriptiveOnlyAuthority, WorthQueryMixedAuthority,
    WorthQueryNeighborhoodAndBatchCapableGrouping, WorthQueryNeighborhoodCapableDeclaration,
    WorthQueryNeighborhoodCapableGrouping, WorthQueryRelationalTruthAuthority,
    WorthQueryRelationalTruthDeclaration, WorthQuerySignalCompatibleDeclaration,
    WorthQuerySignalCompatiblePosture, WorthQuerySignalDeferredPosture,
    WorthQuerySignalNotCompatiblePosture, WorthQuerySingleOnlyGrouping,
    WorthQueryTemporalDeclarationDenial,
};
pub use declaration_entry_orchestration::{
    WorthQueryDeclarationEntryOrchestrationArtifactPolicy,
    WorthQueryDeclarationEntryOrchestrationAutomationBoundary,
    WorthQueryDeclarationEntryOrchestrationAutomationRefusal,
    WorthQueryDeclarationEntryOrchestrationAutomationRefusalClass,
    WorthQueryDeclarationEntryOrchestrationAutomationStep,
    WorthQueryDeclarationEntryOrchestrationChecked,
    WorthQueryDeclarationEntryOrchestrationCostPosture,
    WorthQueryDeclarationEntryOrchestrationDeferred, WorthQueryDeclarationEntryOrchestrationDenied,
    WorthQueryDeclarationEntryOrchestrationExposureLevel,
    WorthQueryDeclarationEntryOrchestrationFailed, WorthQueryDeclarationEntryOrchestrationInput,
    WorthQueryDeclarationEntryOrchestrationMaterializationGate,
    WorthQueryDeclarationEntryOrchestrationMaterializationPolicy,
    WorthQueryDeclarationEntryOrchestrationMaterializationTier,
    WorthQueryDeclarationEntryOrchestrationOutcome, WorthQueryDeclarationEntryOrchestrationPlan,
    WorthQueryDeclarationEntryOrchestrationProduct, WorthQueryDeclarationEntryOrchestrationProof,
    WorthQueryDeclarationEntryOrchestrationRebindRequired,
    WorthQueryDeclarationEntryOrchestrationRefusal,
    WorthQueryDeclarationEntryOrchestrationRefusalClass,
    WorthQueryDeclarationEntryOrchestrationStage,
    WorthQueryDeclarationEntryOrchestrationStageRecord,
    WorthQueryDeclarationEntryOrchestrationStale,
    WorthQueryDeclarationEntryOrchestrationStepDisposition,
    WorthQueryDeclarationEntryOrchestrationStepRecord,
    WorthQueryDeclarationEntryOrchestrationTerminalError,
    WorthQueryDeclarationEntryOrchestrationTranscript, WorthQueryDeclarationEntryOrchestrationVerb,
    WorthQueryDeclarationEntryOrchestrationVerbCeiling,
    WorthQueryDeclarationEntryOrchestrationVerbFamily,
    WorthQueryDeclarationEntryOrchestrationVerbInventory,
    WorthQueryDeclarationEnvelopeOrchestrationProof,
    WorthQueryDeclarationEnvelopeOrchestrationTranscript,
    WorthQueryDeclarationReceiptOrchestrationProof,
    WorthQueryDeclarationReceiptOrchestrationTranscript,
    WorthQueryDeclarationRouteOrchestrationProof,
    WorthQueryDeclarationRouteOrchestrationTranscript,
};
pub use declaration_entry_seam::{
    WorthQueryDeclarationEntryContributionCategoryFamily,
    WorthQueryDeclarationEntryContributionComposition,
    WorthQueryDeclarationEntryContributionCompositionError,
    WorthQueryDeclarationEntryContributionCompositionFailureClass,
    WorthQueryDeclarationEntryContributionEvidence,
    WorthQueryDeclarationEntryContributionEvidenceRecord,
    WorthQueryDeclarationEntryContributionEvidenceSet,
    WorthQueryDeclarationEntryContributionTargetFamily,
    WorthQueryDeclarationEntryCrossingInventory, WorthQueryDeclarationEntryCrossingRow,
    WorthQueryDeclarationEntryCrossingSurface, WorthQueryDeclarationEntryInspection,
    WorthQueryDeclarationEntryInspectionBridgePosture, WorthQueryDeclarationEntryInspectionError,
    WorthQueryDeclarationEntryInspectionRelationalPosture,
    WorthQueryDeclarationEntryInspectionSignalPosture, WorthQueryDeclarationEntryLowerOwnerCrate,
    WorthQueryDeclarationEntryReadinessReport, WorthQueryDeclarationEntryReadinessRow,
    WorthQueryDeclarationEntryReadinessStatus, WorthQueryDeclarationEntrySeamClassification,
};
#[cfg(test)]
pub(crate) use declaration_entry_seam::{
    WorthQueryDeclarationEntryInspectionInput, WorthQueryDeclarationEntryReadinessRequest,
    WorthQueryDeclarationEntryRetainedSubjectInput,
};
pub use declaration_envelope::{
    WorthQueryDeclarationEntryEnvelopeError, WorthQueryDeclarationEnvelope,
    WorthQueryDeclarationEnvelopeChecked, WorthQueryDeclarationEnvelopeClass,
    WorthQueryDeclarationEnvelopeDeferred, WorthQueryDeclarationEnvelopeDenied,
    WorthQueryDeclarationEnvelopeEvidenceOrigin, WorthQueryDeclarationEnvelopeExplanation,
    WorthQueryDeclarationEnvelopeFailed, WorthQueryDeclarationEnvelopeInput,
    WorthQueryDeclarationEnvelopeTerminalError,
};
pub use declaration_evidence::{
    WorthQueryDeclarationFoundationalEvidence, WorthQueryDeclarationFoundationalEvidenceChecked,
    WorthQueryDeclarationFoundationalEvidenceClass,
    WorthQueryDeclarationFoundationalEvidenceDenial,
    WorthQueryDeclarationFoundationalEvidenceInput,
};
pub use declaration_family::{
    WorthQueryDeclarationFamilyMarker, WorthQueryDeclarationFamilyTaxonomy,
    WorthQueryDeclarationPrimaryAuthorityFamily, WorthQueryGroupedDeclarationPosture,
    WorthQuerySignalCompatibilityPosture,
};
pub use declaration_legality::{
    WorthQueryAsyncLegalityDenialKind, WorthQueryDeclarationAdmissionOrLegalityError,
    WorthQueryDeclarationLegalityChecked, WorthQueryDeclarationLegalityClass,
    WorthQueryDeclarationLegalityContract, WorthQueryDeclarationLegalityDenial,
    WorthQueryDeclarationLegalityEvidence, WorthQueryDeclarationLegalityInput,
    WorthQueryTemporalLegalityDenialKind,
};
pub use declaration_progression::{
    WorthQueryAdmittedDeclarationProgression, WorthQueryDeclarationProgressionChecked,
    WorthQueryDeclarationProgressionContract, WorthQueryDeclarationProgressionContractClass,
    WorthQueryDeclarationProgressionDeferred, WorthQueryDeclarationProgressionDenied,
    WorthQueryDeclarationProgressionFailed, WorthQueryDeclarationProgressionOutcomeView,
    WorthQueryDeclarationProgressionRebindRequired, WorthQueryDeclarationProgressionRecipe,
    WorthQueryDeclarationProgressionStale, WorthQueryDeclarationProgressionTerminalError,
};
pub use declaration_publication::WorthQueryDeclarationAspectPublication;
pub use declaration_receipt::{
    WorthQueryDeclarationEntryReceiptError, WorthQueryDeclarationReceipt,
    WorthQueryDeclarationReceiptChecked, WorthQueryDeclarationReceiptClass,
    WorthQueryDeclarationReceiptDeferred, WorthQueryDeclarationReceiptDenialCause,
    WorthQueryDeclarationReceiptDenied, WorthQueryDeclarationReceiptExplanation,
    WorthQueryDeclarationReceiptFailed, WorthQueryDeclarationReceiptInput,
    WorthQueryDeclarationReceiptKind, WorthQueryDeclarationReceiptTerminalError,
};
pub use declaration_relational_routing::{
    WorthQueryDeclarationEntryRelationalRoutingError,
    WorthQueryDeclarationRelationalAuthorityFamily, WorthQueryDeclarationRelationalBinding,
    WorthQueryDeclarationRelationalRouting, WorthQueryDeclarationRelationalRoutingChecked,
    WorthQueryDeclarationRelationalRoutingClass, WorthQueryDeclarationRelationalRoutingDeferred,
    WorthQueryDeclarationRelationalRoutingDenialCause,
    WorthQueryDeclarationRelationalRoutingDenied,
    WorthQueryDeclarationRelationalRoutingExplanation,
    WorthQueryDeclarationRelationalRoutingFailed, WorthQueryDeclarationRelationalRoutingInput,
    WorthQueryDeclarationRelationalRoutingSupportReport,
    WorthQueryDeclarationRelationalRoutingSupportRow,
    WorthQueryDeclarationRelationalRoutingTerminalError, WorthQueryDeclarationRelationalTruthClaim,
    WorthQueryDeclarationRelationalTruthContract,
    WorthQueryDeclarationRelationalTruthRoutingSupportStatus,
};
pub use declaration_route_plan::{
    WorthQueryDeclarationEntryRoutePlanError, WorthQueryDeclarationRouteContract,
    WorthQueryDeclarationRouteIntent, WorthQueryDeclarationRouteIntentRequirement,
    WorthQueryDeclarationRouteMultiplicity, WorthQueryDeclarationRoutePlan,
    WorthQueryDeclarationRoutePlanChecked, WorthQueryDeclarationRoutePlanClass,
    WorthQueryDeclarationRoutePlanDeferred, WorthQueryDeclarationRoutePlanDenialCause,
    WorthQueryDeclarationRoutePlanDenied, WorthQueryDeclarationRoutePlanExplanation,
    WorthQueryDeclarationRoutePlanFailed, WorthQueryDeclarationRoutePlanInput,
    WorthQueryDeclarationRoutePlanTerminalError, WorthQueryDeclarationRouteSegment,
    WorthQueryDeclarationRouteSet, WorthQueryLowerAuthorityRouteFamily,
};
pub use declaration_signal_compatibility::{
    WorthQueryDeclarationEntrySignalCompatibilityError, WorthQueryDeclarationSignalCompatibility,
    WorthQueryDeclarationSignalCompatibilityChecked, WorthQueryDeclarationSignalCompatibilityClass,
    WorthQueryDeclarationSignalCompatibilityContract,
    WorthQueryDeclarationSignalCompatibilityDeferred,
    WorthQueryDeclarationSignalCompatibilityDenialCause,
    WorthQueryDeclarationSignalCompatibilityDenied,
    WorthQueryDeclarationSignalCompatibilityExplanation,
    WorthQueryDeclarationSignalCompatibilityFailed, WorthQueryDeclarationSignalCompatibilityInput,
    WorthQueryDeclarationSignalCompatibilitySupportReport,
    WorthQueryDeclarationSignalCompatibilitySupportRow,
    WorthQueryDeclarationSignalCompatibilitySupportStatus,
    WorthQueryDeclarationSignalCompatibilityTerminalError,
    WorthQueryDeclarationSignalExecutionFamily,
};
pub use domain_entry::{WorthQueryDomainEntryMarker, WorthQueryDomainEntrySupportSnapshot};
pub(crate) use domain_handle::compose_basis_lifecycle_support_identity;
pub use domain_handle::{
    WorthQueryAdmittedWorldBasis, WorthQueryContinuationExecutionReadmissionObservation,
    WorthQueryDeclarationEntryProgressionError, WorthQueryDomainOperatingContext,
    WorthQueryDomainOperatingContextIdentityDeclaration,
    WorthQueryDomainOperatingContextIdentityError, WorthQueryDomainOperatingRequirement,
    WorthQueryInstalledDomainDeclarationContext,
};
#[cfg(test)]
pub(crate) use support::scan_shared_read_mint_forbidden_patterns;
#[cfg(test)]
pub(crate) use support::*;
#[cfg(test)]
pub(crate) use support::{
    scan_journal_identity_forbidden_patterns, scan_journal_identity_required_pattern_failures,
    scan_shared_read_pin_hot_path_forbidden_patterns,
    scan_shared_read_pin_required_pattern_failures, scan_shared_read_pin_retire_forbidden_patterns,
    shared_read_pinning_operation_inventory, worth_query_journal_identity_inventory,
    WorthQueryJournalIdentityOperationKind, WorthQuerySharedReadPinningOperationKind,
};
pub use support::{
    QueryContextDeferredScopeMarker, WorthQueryCapabilityDescriptor, WorthQueryCapabilityFamily,
    WorthQueryCapabilityRegistry, WorthQueryCapabilityStatus, WorthQueryCapabilitySupportStatus,
    WorthQueryIdentityEvolutionSupportProfile, WorthQueryMilestoneClosureStatus,
    WorthQueryMilestoneNineSevenDerivedClosure, WorthQueryMilestoneNineSevenPhaseClosure,
    WorthQueryQueryCompositionSupportProfile, WorthQueryQueryContextSupportProfile,
    WorthQuerySupportMatrix, WorthQuerySupportReport, WorthQuerySupportReportCounters,
    WorthQuerySupportSectionPosture,
};
pub use support::{
    WorthQueryConcurrentHostileMatrixArtifact, WorthQueryConcurrentHostileMatrixPosture,
    WorthQueryConcurrentHostileMatrixSabotage, WorthQueryConcurrentHostileMatrixSabotageKind,
    WorthQueryPublicBridgeForbiddenAccessFinding, WorthQueryPublicBridgeForbiddenAccessPattern,
    WorthQueryPublicBridgeProjectionConsumptionEvidence,
    WorthQueryPublicBridgePublishedProjectionReader, WorthQueryPublicBridgeReaderLaneCertification,
    WorthQueryPublicBridgeReaderLaneInventory, WorthQueryPublicBridgeReaderLanePosture,
    WorthQueryPublicBridgeReaderLaneSabotage, WorthQueryPublicBridgeReaderLaneSabotageKind,
    WorthQueryPublicBridgeReaderLaneSabotageOutcome, WorthQuerySharedReadPinningCertification,
};

pub(crate) use declaration::worth_query_canonical_declaration;
pub(crate) use declaration_aspect::{
    aspect_coverage_from_publication, authority_mismatch_from_fit,
    authority_scoped_envelope_aspect_contract, merged_authority_aspect_contract,
    route_scoped_declaration_aspect_contract,
};
#[cfg(test)]
pub(crate) use declaration_aspect_test_support::{
    assert_declaration_aspect_projections, test_declaration_aspect_key,
    test_declaration_aspect_keys,
};
pub(crate) use declaration_authority_summary::{
    bridge_authority_summary_from_coverage, bridge_authority_summary_from_publication,
    relational_authority_summary_from_coverage, relational_authority_summary_from_publication,
    signal_authority_summary_from_coverage, signal_authority_summary_from_publication,
};
pub(crate) use declaration_bridge_routing::{
    derive_bridge_routing_support_report, worth_query_checked_declaration_bridge_routing_on_handle,
};
pub(crate) use declaration_capability::{
    worth_query_checked_family_declaration, worth_query_checked_family_support,
};
pub(crate) use declaration_entry_orchestration::materialized_profile_for_tier;
#[cfg(test)]
pub(crate) use declaration_entry_orchestration::{
    worth_query_checked_declaration_entry_orchestration_on_handle,
    worth_query_declaration_entry_orchestration_on_handle,
    worth_query_declaration_entry_orchestration_proof_on_handle,
    worth_query_declaration_receipt_orchestration_from_progressed_on_handle,
    worth_query_declaration_route_orchestration_from_progressed_on_handle,
};
pub(crate) use declaration_entry_orchestration::{
    worth_query_checked_declaration_envelope_orchestration_from_progressed_on_handle,
    worth_query_checked_declaration_receipt_orchestration_from_progressed_on_handle,
    worth_query_checked_declaration_route_orchestration_from_progressed_on_handle,
    worth_query_declaration_envelope_orchestration_from_progressed_on_handle,
    worth_query_declaration_envelope_orchestration_from_progressed_proof_on_handle,
    worth_query_declaration_receipt_orchestration_from_progressed_proof_on_handle,
    worth_query_declaration_route_orchestration_from_progressed_proof_on_handle,
    worth_query_lower_declaration_entry_product_orchestration_from_progressed_on_handle,
    WorthQueryDeclarationEntryProductChecked,
};
pub(crate) use declaration_entry_seam::{
    worth_query_bridge_routing_support_from_entry_readiness,
    worth_query_relational_routing_support_from_entry_readiness,
    worth_query_signal_compatibility_support_from_entry_readiness,
};
pub(crate) use declaration_entry_seam::{
    worth_query_declaration_entry_crossing_inventory,
    worth_query_declaration_entry_readiness_report,
};
#[cfg(test)]
pub(crate) use declaration_entry_seam::{
    worth_query_declaration_entry_inspection_on_handle,
    worth_query_declaration_entry_readiness_report_with_request,
};
pub(crate) use declaration_envelope::worth_query_checked_declaration_envelope;
pub(crate) use declaration_envelope::worth_query_declaration_envelope_terminal_from_receipt_terminal;
pub(crate) use declaration_evidence::worth_query_declaration_foundational_evidence;
pub(crate) use declaration_legality::review_declaration_legality;
pub(crate) use declaration_progression::{
    worth_query_checked_declaration_progression, worth_query_declaration_progression_recipe,
};
pub(crate) use declaration_receipt::{
    receipt_materialized_profile_for_tier, worth_query_checked_declaration_receipt,
    worth_query_checked_declaration_receipt_with_materialized_profile,
};
pub(crate) use declaration_relational_routing::{
    derive_relational_routing_support_report,
    worth_query_checked_declaration_relational_routing_on_handle,
};
pub(crate) use declaration_route_plan::worth_query_checked_declaration_route_plan;
pub(crate) use declaration_signal_compatibility::{
    derive_signal_compatibility_support_report,
    worth_query_checked_declaration_signal_compatibility_on_handle,
};
pub(crate) use domain_handle::checked_route_plan_from_progressed_with_profile;

#[cfg(test)]
mod tests;
