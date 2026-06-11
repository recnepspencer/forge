mod capability;
mod config;
mod declaration;
mod declaration_aspect;
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
mod support;

pub use capability::{
    CapabilityAdmissionDecision, CapabilityAdmissionError, CapabilityAdmissionFailureClass,
    ForgeQueryApplicationFacade, ForgeQueryCapabilityResolution, ForgeQueryFacadeCounters,
    ForgeQueryFacadeError, ForgeQueryFacadeFailureClass, HistoricalEvaluationCapability,
    IdentityEvolutionCapability, LiveQueryCapability, PreviewSessionCapability,
    QueryCompositionCapability, QueryContextCapability, QueryReadCapability,
    WorkflowOrchestrationCapability,
};
pub use config::{
    ConfigurationAdmissionError, ConfigurationAdmissionFailureClass, ForgeQueryConfig,
    ForgeQueryConfigCounters, ForgeQueryConfigSectionFamily, ForgeQueryConfigSectionResolution,
    ForgeQueryQueryConfig, ForgeQueryRelationalConfig, ForgeQueryRuntimeBridgeConfig,
    ForgeQuerySignalConfig, ForgeQueryStoreConfig, ForgeQuerySubsystemOwner,
    ValidatedForgeQueryConfig,
};
pub use declaration::{
    ForgeQueryAsyncDeclarationClause, ForgeQueryAsyncDeclarationSupport,
    ForgeQueryAsyncFailurePosture, ForgeQueryAsyncLoadingPosture,
    ForgeQueryAsyncRequestIdentityPart, ForgeQueryAsyncSourceFamily,
    ForgeQueryCanonicalDeclarationArtifact, ForgeQueryCanonicalDeclarationComparison,
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationCanonicalEntryKind,
    ForgeQueryDeclarationCanonicalValue, ForgeQueryDeclarationCanonicalizationError,
    ForgeQueryDeclarationCanonicalizationVersion, ForgeQueryDeclarationFutureProjection,
    ForgeQueryDeclarationInput, ForgeQueryTemporalDeclarationClause,
    ForgeQueryTemporalDeclarationSupport, ForgeQueryTemporalDuration, ForgeQueryTemporalWindowKind,
};
pub use declaration_aspect::{
    ForgeQueryDeclarationAspectContract, ForgeQueryDeclarationAspectCoverage,
    ForgeQueryDeclarationAspectCoverageBasis, ForgeQueryDeclarationAspectFit,
    ForgeQueryDeclarationAuthorityAspectMismatch,
};
pub use declaration_authority_summary::{
    ForgeQueryDeclarationBridgeAuthorityAspectSummary,
    ForgeQueryDeclarationRelationalAuthorityAspectSummary,
    ForgeQueryDeclarationSignalAuthorityAspectSummary,
};
pub use declaration_bridge_routing::{
    ForgeQueryDeclarationBridgeBinding, ForgeQueryDeclarationBridgeContinuationContract,
    ForgeQueryDeclarationBridgeContinuationFamily, ForgeQueryDeclarationBridgeContinuationMode,
    ForgeQueryDeclarationBridgeContinuationRequest, ForgeQueryDeclarationBridgeRouting,
    ForgeQueryDeclarationBridgeRoutingChecked, ForgeQueryDeclarationBridgeRoutingClass,
    ForgeQueryDeclarationBridgeRoutingDeferred, ForgeQueryDeclarationBridgeRoutingDenialCause,
    ForgeQueryDeclarationBridgeRoutingDenied, ForgeQueryDeclarationBridgeRoutingExplanation,
    ForgeQueryDeclarationBridgeRoutingFailed, ForgeQueryDeclarationBridgeRoutingInput,
    ForgeQueryDeclarationBridgeRoutingSupportReport, ForgeQueryDeclarationBridgeRoutingSupportRow,
    ForgeQueryDeclarationBridgeRoutingSupportStatus,
    ForgeQueryDeclarationBridgeRoutingTerminalError, ForgeQueryDeclarationBridgeTruthContext,
    ForgeQueryDeclarationEntryBridgeRoutingError,
};
pub use declaration_capability::{
    ForgeQueryAsyncDeclarationDenial, ForgeQueryBatchCapableDeclaration,
    ForgeQueryBatchCapableGrouping, ForgeQueryBridgeContinuationAuthority,
    ForgeQueryBridgeContinuationDeclaration, ForgeQueryDeclarationAdmissionError,
    ForgeQueryDeclarationCapabilityDenial, ForgeQueryDeclarationCapabilityStatus,
    ForgeQueryDeclarationCapabilityVerb, ForgeQueryDeclarationFamilySupportChecked,
    ForgeQueryDeclarationFamilySupportReport, ForgeQueryDeclarationFamilySupportRow,
    ForgeQueryDeclarationGroupedPostureTag, ForgeQueryDeclarationPrimaryAuthorityTag,
    ForgeQueryDeclarationSignalCompatibilityTag, ForgeQueryDeclarationSupportsBatchGrouping,
    ForgeQueryDeclarationSupportsBridgeContinuation,
    ForgeQueryDeclarationSupportsNeighborhoodGrouping,
    ForgeQueryDeclarationSupportsRelationalTruth, ForgeQueryDeclarationSupportsSignalCompatibility,
    ForgeQueryDeclaredFamilyChecked, ForgeQueryDescriptiveOnlyAuthority, ForgeQueryMixedAuthority,
    ForgeQueryNeighborhoodAndBatchCapableGrouping, ForgeQueryNeighborhoodCapableDeclaration,
    ForgeQueryNeighborhoodCapableGrouping, ForgeQueryRelationalTruthAuthority,
    ForgeQueryRelationalTruthDeclaration, ForgeQuerySignalCompatibleDeclaration,
    ForgeQuerySignalCompatiblePosture, ForgeQuerySignalDeferredPosture,
    ForgeQuerySignalNotCompatiblePosture, ForgeQuerySingleOnlyGrouping,
    ForgeQueryTemporalDeclarationDenial,
};
pub use declaration_entry_orchestration::{
    ForgeQueryDeclarationEntryOrchestrationArtifactPolicy,
    ForgeQueryDeclarationEntryOrchestrationAutomationBoundary,
    ForgeQueryDeclarationEntryOrchestrationAutomationRefusal,
    ForgeQueryDeclarationEntryOrchestrationAutomationRefusalClass,
    ForgeQueryDeclarationEntryOrchestrationAutomationStep,
    ForgeQueryDeclarationEntryOrchestrationChecked,
    ForgeQueryDeclarationEntryOrchestrationCostPosture,
    ForgeQueryDeclarationEntryOrchestrationDeferred, ForgeQueryDeclarationEntryOrchestrationDenied,
    ForgeQueryDeclarationEntryOrchestrationExposureLevel,
    ForgeQueryDeclarationEntryOrchestrationFailed, ForgeQueryDeclarationEntryOrchestrationInput,
    ForgeQueryDeclarationEntryOrchestrationMaterializationGate,
    ForgeQueryDeclarationEntryOrchestrationMaterializationPolicy,
    ForgeQueryDeclarationEntryOrchestrationMaterializationTier,
    ForgeQueryDeclarationEntryOrchestrationOutcome, ForgeQueryDeclarationEntryOrchestrationPlan,
    ForgeQueryDeclarationEntryOrchestrationProduct, ForgeQueryDeclarationEntryOrchestrationProof,
    ForgeQueryDeclarationEntryOrchestrationRebindRequired,
    ForgeQueryDeclarationEntryOrchestrationRefusal,
    ForgeQueryDeclarationEntryOrchestrationRefusalClass,
    ForgeQueryDeclarationEntryOrchestrationStage,
    ForgeQueryDeclarationEntryOrchestrationStageRecord,
    ForgeQueryDeclarationEntryOrchestrationStale,
    ForgeQueryDeclarationEntryOrchestrationStepDisposition,
    ForgeQueryDeclarationEntryOrchestrationStepRecord,
    ForgeQueryDeclarationEntryOrchestrationTerminalError,
    ForgeQueryDeclarationEntryOrchestrationTranscript, ForgeQueryDeclarationEntryOrchestrationVerb,
    ForgeQueryDeclarationEntryOrchestrationVerbCeiling,
    ForgeQueryDeclarationEntryOrchestrationVerbFamily,
    ForgeQueryDeclarationEntryOrchestrationVerbInventory,
    ForgeQueryDeclarationEnvelopeOrchestrationProof,
    ForgeQueryDeclarationEnvelopeOrchestrationTranscript,
    ForgeQueryDeclarationReceiptOrchestrationProof,
    ForgeQueryDeclarationReceiptOrchestrationTranscript,
    ForgeQueryDeclarationRouteOrchestrationProof,
    ForgeQueryDeclarationRouteOrchestrationTranscript,
};
pub use declaration_entry_seam::{
    ForgeQueryDeclarationEntryContributionCategoryFamily,
    ForgeQueryDeclarationEntryContributionComposition,
    ForgeQueryDeclarationEntryContributionCompositionError,
    ForgeQueryDeclarationEntryContributionCompositionFailureClass,
    ForgeQueryDeclarationEntryContributionEvidence,
    ForgeQueryDeclarationEntryContributionEvidenceRecord,
    ForgeQueryDeclarationEntryContributionEvidenceSet,
    ForgeQueryDeclarationEntryContributionTargetFamily,
    ForgeQueryDeclarationEntryCrossingInventory, ForgeQueryDeclarationEntryCrossingRow,
    ForgeQueryDeclarationEntryCrossingSurface, ForgeQueryDeclarationEntryInspection,
    ForgeQueryDeclarationEntryInspectionBridgePosture, ForgeQueryDeclarationEntryInspectionError,
    ForgeQueryDeclarationEntryInspectionInput,
    ForgeQueryDeclarationEntryInspectionRelationalPosture,
    ForgeQueryDeclarationEntryInspectionSignalPosture, ForgeQueryDeclarationEntryLowerOwnerCrate,
    ForgeQueryDeclarationEntryReadinessReport, ForgeQueryDeclarationEntryReadinessRequest,
    ForgeQueryDeclarationEntryReadinessRow, ForgeQueryDeclarationEntryReadinessStatus,
    ForgeQueryDeclarationEntryRetainedSubjectInput, ForgeQueryDeclarationEntrySeamClassification,
};
pub use declaration_envelope::{
    ForgeQueryDeclarationEntryEnvelopeError, ForgeQueryDeclarationEnvelope,
    ForgeQueryDeclarationEnvelopeChecked, ForgeQueryDeclarationEnvelopeClass,
    ForgeQueryDeclarationEnvelopeDeferred, ForgeQueryDeclarationEnvelopeDenied,
    ForgeQueryDeclarationEnvelopeEvidenceOrigin, ForgeQueryDeclarationEnvelopeExplanation,
    ForgeQueryDeclarationEnvelopeFailed, ForgeQueryDeclarationEnvelopeInput,
    ForgeQueryDeclarationEnvelopeTerminalError,
};
pub use declaration_evidence::{
    ForgeQueryDeclarationFoundationalEvidence, ForgeQueryDeclarationFoundationalEvidenceChecked,
    ForgeQueryDeclarationFoundationalEvidenceClass,
    ForgeQueryDeclarationFoundationalEvidenceDenial,
    ForgeQueryDeclarationFoundationalEvidenceInput,
};
pub use declaration_family::{
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationFamilyTaxonomy,
    ForgeQueryDeclarationPrimaryAuthorityFamily, ForgeQueryGroupedDeclarationPosture,
    ForgeQuerySignalCompatibilityPosture,
};
pub use declaration_legality::{
    ForgeQueryAsyncLegalityDenialKind, ForgeQueryDeclarationAdmissionOrLegalityError,
    ForgeQueryDeclarationLegalityChecked, ForgeQueryDeclarationLegalityClass,
    ForgeQueryDeclarationLegalityContract, ForgeQueryDeclarationLegalityDenial,
    ForgeQueryDeclarationLegalityEvidence, ForgeQueryDeclarationLegalityInput,
    ForgeQueryTemporalLegalityDenialKind,
};
pub use declaration_progression::{
    ForgeQueryAdmittedDeclarationProgression, ForgeQueryDeclarationProgressionChecked,
    ForgeQueryDeclarationProgressionContract, ForgeQueryDeclarationProgressionContractClass,
    ForgeQueryDeclarationProgressionDeferred, ForgeQueryDeclarationProgressionDenied,
    ForgeQueryDeclarationProgressionFailed, ForgeQueryDeclarationProgressionOutcomeView,
    ForgeQueryDeclarationProgressionRebindRequired, ForgeQueryDeclarationProgressionRecipe,
    ForgeQueryDeclarationProgressionStale, ForgeQueryDeclarationProgressionTerminalError,
};
pub use declaration_publication::ForgeQueryDeclarationAspectPublication;
pub use declaration_receipt::{
    ForgeQueryDeclarationEntryReceiptError, ForgeQueryDeclarationReceipt,
    ForgeQueryDeclarationReceiptChecked, ForgeQueryDeclarationReceiptClass,
    ForgeQueryDeclarationReceiptDeferred, ForgeQueryDeclarationReceiptDenialCause,
    ForgeQueryDeclarationReceiptDenied, ForgeQueryDeclarationReceiptExplanation,
    ForgeQueryDeclarationReceiptFailed, ForgeQueryDeclarationReceiptInput,
    ForgeQueryDeclarationReceiptKind, ForgeQueryDeclarationReceiptTerminalError,
};
pub use declaration_relational_routing::{
    ForgeQueryDeclarationEntryRelationalRoutingError,
    ForgeQueryDeclarationRelationalAuthorityFamily, ForgeQueryDeclarationRelationalBinding,
    ForgeQueryDeclarationRelationalRouting, ForgeQueryDeclarationRelationalRoutingChecked,
    ForgeQueryDeclarationRelationalRoutingClass, ForgeQueryDeclarationRelationalRoutingDeferred,
    ForgeQueryDeclarationRelationalRoutingDenialCause,
    ForgeQueryDeclarationRelationalRoutingDenied,
    ForgeQueryDeclarationRelationalRoutingExplanation,
    ForgeQueryDeclarationRelationalRoutingFailed, ForgeQueryDeclarationRelationalRoutingInput,
    ForgeQueryDeclarationRelationalRoutingSupportReport,
    ForgeQueryDeclarationRelationalRoutingSupportRow,
    ForgeQueryDeclarationRelationalRoutingTerminalError, ForgeQueryDeclarationRelationalTruthClaim,
    ForgeQueryDeclarationRelationalTruthContract,
    ForgeQueryDeclarationRelationalTruthRoutingSupportStatus,
};
pub use declaration_route_plan::{
    ForgeQueryDeclarationEntryRoutePlanError, ForgeQueryDeclarationRouteContract,
    ForgeQueryDeclarationRouteIntent, ForgeQueryDeclarationRouteIntentRequirement,
    ForgeQueryDeclarationRouteMultiplicity, ForgeQueryDeclarationRoutePlan,
    ForgeQueryDeclarationRoutePlanChecked, ForgeQueryDeclarationRoutePlanClass,
    ForgeQueryDeclarationRoutePlanDeferred, ForgeQueryDeclarationRoutePlanDenialCause,
    ForgeQueryDeclarationRoutePlanDenied, ForgeQueryDeclarationRoutePlanExplanation,
    ForgeQueryDeclarationRoutePlanFailed, ForgeQueryDeclarationRoutePlanInput,
    ForgeQueryDeclarationRoutePlanTerminalError, ForgeQueryDeclarationRouteSegment,
    ForgeQueryDeclarationRouteSet, ForgeQueryLowerAuthorityRouteFamily,
};
pub use declaration_signal_compatibility::{
    ForgeQueryDeclarationEntrySignalCompatibilityError, ForgeQueryDeclarationSignalCompatibility,
    ForgeQueryDeclarationSignalCompatibilityChecked, ForgeQueryDeclarationSignalCompatibilityClass,
    ForgeQueryDeclarationSignalCompatibilityContract,
    ForgeQueryDeclarationSignalCompatibilityDeferred,
    ForgeQueryDeclarationSignalCompatibilityDenialCause,
    ForgeQueryDeclarationSignalCompatibilityDenied,
    ForgeQueryDeclarationSignalCompatibilityExplanation,
    ForgeQueryDeclarationSignalCompatibilityFailed, ForgeQueryDeclarationSignalCompatibilityInput,
    ForgeQueryDeclarationSignalCompatibilitySupportReport,
    ForgeQueryDeclarationSignalCompatibilitySupportRow,
    ForgeQueryDeclarationSignalCompatibilitySupportStatus,
    ForgeQueryDeclarationSignalCompatibilityTerminalError,
    ForgeQueryDeclarationSignalExecutionFamily,
};
pub use domain_entry::{
    ForgeQueryDomainEntryChecked, ForgeQueryDomainEntryDeferred, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainEntryProofRoot, ForgeQueryDomainEntryRoot,
    ForgeQueryDomainEntrySupportSnapshot, ForgeQueryDomainEntryUnsupported,
};
pub use domain_handle::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryAdmittedWorldBasis,
    ForgeQueryConfiguredDomainHandleAdmissionError, ForgeQueryConfiguredDomainHandleChecked,
    ForgeQueryConfiguredDomainHandleDeferred, ForgeQueryConfiguredDomainHandleDraft,
    ForgeQueryConfiguredDomainHandleInvalidContext, ForgeQueryConfiguredDomainHandleUnsupported,
    ForgeQueryContinuationExecutionReadmissionObservation,
    ForgeQueryDeclarationEntryProgressionError, ForgeQueryDomainOperatingContext,
    ForgeQueryDomainOperatingRequirement, ForgeQueryValidatedConfiguredDomainHandle,
};
pub use support::{
    ForgeQueryCapabilityDescriptor, ForgeQueryCapabilityFamily, ForgeQueryCapabilityRegistry,
    ForgeQueryCapabilityStatus, ForgeQueryCapabilitySupportStatus,
    ForgeQueryEvidenceIdentityBoundaryClosure, ForgeQueryFolkloreResidueStatus,
    ForgeQueryIdentityBoundaryClosure, ForgeQueryIdentityEvolutionSupportProfile,
    ForgeQueryMilestoneClosureStatus, ForgeQueryQueryCompositionSupportProfile,
    ForgeQueryQueryContextSupportProfile, ForgeQuerySessionLabelBoundaryClosure,
    ForgeQueryStopClassBoundaryClosure, ForgeQuerySupportMatrix, ForgeQuerySupportReport,
    ForgeQuerySupportReportCounters, ForgeQuerySupportSectionPosture,
    QueryContextDeferredScopeMarker,
};

pub(crate) use declaration::forge_query_canonical_declaration;
pub(crate) use declaration_aspect::{
    aspect_coverage_from_publication, authority_mismatch_from_fit,
    authority_scoped_envelope_aspect_contract, merged_authority_aspect_contract,
    route_scoped_declaration_aspect_contract,
};
pub(crate) use declaration_authority_summary::{
    bridge_authority_summary_from_coverage, bridge_authority_summary_from_publication,
    relational_authority_summary_from_coverage, relational_authority_summary_from_publication,
    signal_authority_summary_from_coverage, signal_authority_summary_from_publication,
};
pub(crate) use declaration_bridge_routing::{
    derive_bridge_routing_support_report, forge_query_checked_declaration_bridge_routing_on_handle,
};
pub(crate) use declaration_capability::{
    forge_query_checked_family_declaration, forge_query_checked_family_support,
};
pub(crate) use declaration_entry_orchestration::{
    forge_query_checked_declaration_entry_orchestration_on_handle,
    forge_query_checked_declaration_envelope_orchestration_from_progressed_on_handle,
    forge_query_checked_declaration_receipt_orchestration_from_progressed_on_handle,
    forge_query_checked_declaration_route_orchestration_from_progressed_on_handle,
    forge_query_declaration_entry_orchestration_on_handle,
    forge_query_declaration_entry_orchestration_proof_on_handle,
    forge_query_declaration_envelope_orchestration_from_progressed_on_handle,
    forge_query_declaration_envelope_orchestration_from_progressed_proof_on_handle,
    forge_query_declaration_receipt_orchestration_from_progressed_on_handle,
    forge_query_declaration_receipt_orchestration_from_progressed_proof_on_handle,
    forge_query_declaration_route_orchestration_from_progressed_on_handle,
    forge_query_declaration_route_orchestration_from_progressed_proof_on_handle,
    forge_query_lower_declaration_entry_product_orchestration_from_progressed_on_handle,
    materialized_profile_for_tier, ForgeQueryDeclarationEntryProductChecked,
};
pub(crate) use declaration_entry_seam::{
    forge_query_bridge_routing_support_from_entry_readiness,
    forge_query_declaration_entry_crossing_inventory,
    forge_query_declaration_entry_inspection_on_handle,
    forge_query_declaration_entry_readiness_report,
    forge_query_declaration_entry_readiness_report_with_request,
    forge_query_relational_routing_support_from_entry_readiness,
    forge_query_signal_compatibility_support_from_entry_readiness,
};
pub(crate) use declaration_envelope::forge_query_checked_declaration_envelope;
pub(crate) use declaration_envelope::forge_query_declaration_envelope_terminal_from_receipt_terminal;
pub(crate) use declaration_evidence::forge_query_declaration_foundational_evidence;
pub(crate) use declaration_legality::review_declaration_legality;
pub(crate) use declaration_progression::{
    forge_query_checked_declaration_progression, forge_query_declaration_progression_recipe,
};
pub(crate) use declaration_receipt::{
    forge_query_checked_declaration_receipt,
    forge_query_checked_declaration_receipt_with_materialized_profile,
    receipt_materialized_profile_for_tier,
};
pub(crate) use declaration_relational_routing::{
    derive_relational_routing_support_report,
    forge_query_checked_declaration_relational_routing_on_handle,
};
pub(crate) use declaration_route_plan::forge_query_checked_declaration_route_plan;
pub(crate) use declaration_signal_compatibility::{
    derive_signal_compatibility_support_report,
    forge_query_checked_declaration_signal_compatibility_on_handle,
};
pub(crate) use domain_handle::checked_route_plan_from_progressed_with_profile;

#[cfg(test)]
mod tests;
