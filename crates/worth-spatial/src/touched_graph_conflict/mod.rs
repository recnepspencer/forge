mod applicability;
mod catalog;
mod current_catalog;
mod declaration_admission;
mod family_declaration;
mod family_identity;
mod selection_closeout;
mod source_firewall;

#[cfg(test)]
mod tests;

#[cfg(test)]
pub(crate) use catalog::SpatialConflictFamilyApplicability;
pub use catalog::SpatialConflictFamilyCatalog;
pub use declaration_admission::{
    admit_spatial_conflict_family_declaration, SpatialConflictFamilyDeclarationInput,
};
pub use family_declaration::{
    SpatialConflictDiagnosticWitness, SpatialConflictFamilyDeclaration,
    SpatialConflictFamilyDeclarationDigest, SpatialConflictLocalityAuthorityRequirement,
    SpatialConflictPriorProofPosture, SpatialConflictSelectionProductPosture,
};
pub use family_identity::{
    admit_spatial_conflict_family_identity, SpatialConflictFamilyIdentity,
    SpatialConflictFamilyIdentityAuthority,
};
pub use selection_closeout::{
    current_spatial_conflict_family_catalog_closeout, SpatialConflictFamilyCatalogCloseout,
};
pub use source_firewall::WorthSpatialTouchedGraphConflictSourceFirewallRegion;
