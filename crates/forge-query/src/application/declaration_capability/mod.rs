mod declaration_outcome;
mod support;
mod support_checked;
mod support_rows;
mod tags;
mod witness;

pub use declaration_outcome::{
    ForgeQueryAsyncDeclarationDenial, ForgeQueryDeclarationAdmissionError,
    ForgeQueryDeclarationCapabilityDenial, ForgeQueryDeclaredFamilyChecked,
    ForgeQueryTemporalDeclarationDenial,
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
pub(crate) use support_rows::{
    batch_row, bridge_row, neighborhood_row, relational_row, row, signal_row,
};
