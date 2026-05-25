mod classification;
mod contribution;
mod contribution_adapter;
mod contribution_reconciliation;
mod contribution_scope;
mod digest;
mod inspection;
mod inspection_artifact;
mod inventory;
mod readiness_projection;
mod row;
mod subject;
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
    ForgeQueryDeclarationEntryContributionEvidenceSet,
    ForgeQueryDeclarationEntryContributionTargetFamily,
};
pub use inspection_artifact::{
    ForgeQueryDeclarationEntryInspection, ForgeQueryDeclarationEntryInspectionBridgePosture,
    ForgeQueryDeclarationEntryInspectionError,
    ForgeQueryDeclarationEntryInspectionRelationalPosture,
    ForgeQueryDeclarationEntryInspectionSignalPosture,
};
pub use inventory::ForgeQueryDeclarationEntryCrossingInventory;
pub use row::{ForgeQueryDeclarationEntryCrossingRow, ForgeQueryDeclarationEntryCrossingSurface};
pub use subject::{
    ForgeQueryDeclarationEntryInspectionInput, ForgeQueryDeclarationEntryRetainedSubjectInput,
};
pub use support::{
    ForgeQueryDeclarationEntryReadinessReport, ForgeQueryDeclarationEntryReadinessRequest,
    ForgeQueryDeclarationEntryReadinessRow, ForgeQueryDeclarationEntryReadinessStatus,
};

pub(crate) use inspection::forge_query_declaration_entry_inspection_on_handle;
pub(crate) use inventory::forge_query_declaration_entry_crossing_inventory;
pub(crate) use readiness_projection::{
    forge_query_bridge_routing_support_from_entry_readiness,
    forge_query_relational_routing_support_from_entry_readiness,
    forge_query_signal_compatibility_support_from_entry_readiness,
};
pub(crate) use support::{
    forge_query_declaration_entry_readiness_report,
    forge_query_declaration_entry_readiness_report_with_request,
};

#[cfg(test)]
mod tests;
