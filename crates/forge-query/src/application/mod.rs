mod capability;
mod config;
mod declaration;
mod declaration_capability;
mod declaration_envelope;
mod declaration_evidence;
mod declaration_family;
mod declaration_legality;
mod declaration_progression;
mod declaration_receipt;
mod declaration_route_plan;
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
    ForgeQueryCanonicalDeclarationArtifact, ForgeQueryCanonicalDeclarationComparison,
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationCanonicalEntryKind,
    ForgeQueryDeclarationCanonicalValue, ForgeQueryDeclarationCanonicalizationError,
    ForgeQueryDeclarationCanonicalizationVersion, ForgeQueryDeclarationInput,
};
pub use declaration_capability::{
    ForgeQueryBatchCapableDeclaration, ForgeQueryBatchCapableGrouping,
    ForgeQueryBridgeContinuationAuthority, ForgeQueryBridgeContinuationDeclaration,
    ForgeQueryDeclarationAdmissionError, ForgeQueryDeclarationCapabilityDenial,
    ForgeQueryDeclarationCapabilityStatus, ForgeQueryDeclarationCapabilityVerb,
    ForgeQueryDeclarationFamilySupportChecked, ForgeQueryDeclarationFamilySupportReport,
    ForgeQueryDeclarationFamilySupportRow, ForgeQueryDeclarationGroupedPostureTag,
    ForgeQueryDeclarationPrimaryAuthorityTag, ForgeQueryDeclarationSignalCompatibilityTag,
    ForgeQueryDeclarationSupportsBatchGrouping, ForgeQueryDeclarationSupportsBridgeContinuation,
    ForgeQueryDeclarationSupportsNeighborhoodGrouping,
    ForgeQueryDeclarationSupportsRelationalTruth, ForgeQueryDeclarationSupportsSignalCompatibility,
    ForgeQueryDeclaredFamilyChecked, ForgeQueryDescriptiveOnlyAuthority, ForgeQueryMixedAuthority,
    ForgeQueryNeighborhoodAndBatchCapableGrouping, ForgeQueryNeighborhoodCapableDeclaration,
    ForgeQueryNeighborhoodCapableGrouping, ForgeQueryRelationalTruthAuthority,
    ForgeQueryRelationalTruthDeclaration, ForgeQuerySignalCompatibleDeclaration,
    ForgeQuerySignalCompatiblePosture, ForgeQuerySignalDeferredPosture,
    ForgeQuerySignalNotCompatiblePosture, ForgeQuerySingleOnlyGrouping,
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
    ForgeQueryDeclarationAdmissionOrLegalityError, ForgeQueryDeclarationLegalityChecked,
    ForgeQueryDeclarationLegalityClass, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationLegalityDenial, ForgeQueryDeclarationLegalityEvidence,
    ForgeQueryDeclarationLegalityInput,
};
pub use declaration_progression::{
    ForgeQueryAdmittedDeclarationProgression, ForgeQueryDeclarationProgressionChecked,
    ForgeQueryDeclarationProgressionContract, ForgeQueryDeclarationProgressionContractClass,
    ForgeQueryDeclarationProgressionDeferred, ForgeQueryDeclarationProgressionDenied,
    ForgeQueryDeclarationProgressionFailed, ForgeQueryDeclarationProgressionOutcomeView,
    ForgeQueryDeclarationProgressionRebindRequired, ForgeQueryDeclarationProgressionRecipe,
    ForgeQueryDeclarationProgressionStale, ForgeQueryDeclarationProgressionTerminalError,
};
pub use declaration_receipt::{
    ForgeQueryDeclarationEntryReceiptError, ForgeQueryDeclarationReceipt,
    ForgeQueryDeclarationReceiptChecked, ForgeQueryDeclarationReceiptClass,
    ForgeQueryDeclarationReceiptDeferred, ForgeQueryDeclarationReceiptDenialCause,
    ForgeQueryDeclarationReceiptDenied, ForgeQueryDeclarationReceiptExplanation,
    ForgeQueryDeclarationReceiptFailed, ForgeQueryDeclarationReceiptInput,
    ForgeQueryDeclarationReceiptKind, ForgeQueryDeclarationReceiptTerminalError,
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
pub use domain_entry::{
    ForgeQueryDomainEntryChecked, ForgeQueryDomainEntryDeferred, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainEntryProofRoot, ForgeQueryDomainEntryRoot,
    ForgeQueryDomainEntrySupportSnapshot, ForgeQueryDomainEntryUnsupported,
};
pub use domain_handle::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryConfiguredDomainHandleAdmissionError,
    ForgeQueryConfiguredDomainHandleChecked, ForgeQueryConfiguredDomainHandleDeferred,
    ForgeQueryConfiguredDomainHandleDraft, ForgeQueryConfiguredDomainHandleInvalidContext,
    ForgeQueryConfiguredDomainHandleUnsupported, ForgeQueryDeclarationEntryProgressionError,
    ForgeQueryDomainOperatingContext, ForgeQueryValidatedConfiguredDomainHandle,
};
pub use support::{
    ForgeQueryCapabilityDescriptor, ForgeQueryCapabilityFamily, ForgeQueryCapabilityRegistry,
    ForgeQueryCapabilityStatus, ForgeQueryCapabilitySupportStatus,
    ForgeQueryIdentityEvolutionSupportProfile, ForgeQueryQueryCompositionSupportProfile,
    ForgeQueryQueryContextSupportProfile, ForgeQuerySupportMatrix, ForgeQuerySupportReport,
    ForgeQuerySupportReportCounters, ForgeQuerySupportSectionPosture,
    QueryContextDeferredScopeMarker,
};

pub(crate) use declaration::forge_query_canonical_declaration;
pub(crate) use declaration_capability::{
    forge_query_checked_family_declaration, forge_query_checked_family_support,
};
pub(crate) use declaration_envelope::forge_query_checked_declaration_envelope;
pub(crate) use declaration_envelope::forge_query_declaration_envelope_terminal_from_receipt_terminal;
pub(crate) use declaration_evidence::forge_query_declaration_foundational_evidence;
pub(crate) use declaration_legality::review_declaration_legality;
pub(crate) use declaration_progression::{
    forge_query_checked_declaration_progression, forge_query_declaration_progression_recipe,
};
pub(crate) use declaration_receipt::forge_query_checked_declaration_receipt;
pub(crate) use declaration_route_plan::forge_query_checked_declaration_route_plan;

#[cfg(test)]
mod tests;
