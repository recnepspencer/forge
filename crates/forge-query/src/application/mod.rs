mod capability;
mod config;
mod declaration;
mod declaration_family;
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
pub use declaration_family::{
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationFamilyTaxonomy,
    ForgeQueryDeclarationPrimaryAuthorityFamily, ForgeQueryGroupedDeclarationPosture,
    ForgeQuerySignalCompatibilityPosture,
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
    ForgeQueryConfiguredDomainHandleUnsupported, ForgeQueryDomainOperatingContext,
    ForgeQueryValidatedConfiguredDomainHandle,
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

#[cfg(test)]
mod tests;
