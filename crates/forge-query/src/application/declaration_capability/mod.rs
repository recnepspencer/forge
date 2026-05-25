mod declaration_outcome;
mod support;
mod support_checked;
mod tags;
mod witness;

pub use declaration_outcome::{
    ForgeQueryDeclarationAdmissionError, ForgeQueryDeclarationCapabilityDenial,
    ForgeQueryDeclaredFamilyChecked,
};
pub use support::{
    ForgeQueryDeclarationCapabilityStatus, ForgeQueryDeclarationCapabilityVerb,
    ForgeQueryDeclarationFamilySupportReport, ForgeQueryDeclarationFamilySupportRow,
};
pub use support_checked::ForgeQueryDeclarationFamilySupportChecked;
pub use tags::{
    ForgeQueryBatchCapableGrouping, ForgeQueryBridgeContinuationAuthority,
    ForgeQueryDeclarationGroupedPostureTag, ForgeQueryDeclarationPrimaryAuthorityTag,
    ForgeQueryDeclarationSignalCompatibilityTag, ForgeQueryDeclarationSupportsBatchGrouping,
    ForgeQueryDeclarationSupportsBridgeContinuation,
    ForgeQueryDeclarationSupportsNeighborhoodGrouping,
    ForgeQueryDeclarationSupportsRelationalTruth, ForgeQueryDeclarationSupportsSignalCompatibility,
    ForgeQueryDescriptiveOnlyAuthority, ForgeQueryMixedAuthority,
    ForgeQueryNeighborhoodAndBatchCapableGrouping, ForgeQueryNeighborhoodCapableGrouping,
    ForgeQueryRelationalTruthAuthority, ForgeQuerySignalCompatiblePosture,
    ForgeQuerySignalDeferredPosture, ForgeQuerySignalNotCompatiblePosture,
    ForgeQuerySingleOnlyGrouping,
};
pub use witness::{
    ForgeQueryBatchCapableDeclaration, ForgeQueryBridgeContinuationDeclaration,
    ForgeQueryNeighborhoodCapableDeclaration, ForgeQueryRelationalTruthDeclaration,
    ForgeQuerySignalCompatibleDeclaration,
};

pub(crate) use declaration_outcome::forge_query_checked_family_declaration;
pub(crate) use support_checked::forge_query_checked_family_support;
