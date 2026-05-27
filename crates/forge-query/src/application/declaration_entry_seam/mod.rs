mod authority_support;
mod classification;
mod contribution;
mod digest;
mod inspection;
mod inventory;
mod readiness_projection;
mod row;
mod support;

pub use classification::{
    ForgeQueryDeclarationEntryLowerOwnerCrate, ForgeQueryDeclarationEntrySeamClassification,
};
pub use contribution::{
    ForgeQueryDeclarationEntryContributionCategoryFamily,
    ForgeQueryDeclarationEntryContributionComposition,
    ForgeQueryDeclarationEntryContributionCompositionError,
    ForgeQueryDeclarationEntryContributionCompositionFailureClass,
    ForgeQueryDeclarationEntryContributionEvidence,
    ForgeQueryDeclarationEntryContributionEvidenceRecord,
    ForgeQueryDeclarationEntryContributionEvidenceSet,
    ForgeQueryDeclarationEntryContributionTargetFamily,
};
pub use inspection::{
    ForgeQueryDeclarationEntryInspection, ForgeQueryDeclarationEntryInspectionBridgePosture,
    ForgeQueryDeclarationEntryInspectionError,
    ForgeQueryDeclarationEntryInspectionRelationalPosture,
    ForgeQueryDeclarationEntryInspectionSignalPosture,
};
pub use inspection::{
    ForgeQueryDeclarationEntryInspectionInput, ForgeQueryDeclarationEntryRetainedSubjectInput,
};
pub use inventory::ForgeQueryDeclarationEntryCrossingInventory;
pub use row::{ForgeQueryDeclarationEntryCrossingRow, ForgeQueryDeclarationEntryCrossingSurface};
pub use support::{
    ForgeQueryDeclarationEntryReadinessReport, ForgeQueryDeclarationEntryReadinessRequest,
    ForgeQueryDeclarationEntryReadinessRow, ForgeQueryDeclarationEntryReadinessStatus,
};

pub(crate) use authority_support::{
    forge_query_bridge_routing_support_from_entry_readiness,
    forge_query_relational_routing_support_from_entry_readiness,
    forge_query_signal_compatibility_support_from_entry_readiness,
};
pub(crate) use inspection::forge_query_declaration_entry_inspection_on_handle;
pub(crate) use inventory::forge_query_declaration_entry_crossing_inventory;
pub(crate) use support::{
    forge_query_declaration_entry_readiness_report,
    forge_query_declaration_entry_readiness_report_with_request,
};

#[cfg(test)]
mod tests;
