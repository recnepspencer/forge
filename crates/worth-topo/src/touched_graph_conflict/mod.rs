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

pub(crate) use catalog::TopologyConflictFamilyApplicability;
pub use catalog::TopologyConflictFamilyCatalog;
pub use declaration_admission::{
    admit_topology_conflict_family_declaration, TopologyConflictFamilyDeclarationInput,
};
pub use family_declaration::{
    TopologyConflictDiagnosticWitness, TopologyConflictFamilyDeclaration,
    TopologyConflictFamilyDeclarationDigest, TopologyConflictLocalityAuthorityRequirement,
    TopologyConflictPriorProofPosture, TopologyConflictSelectionProductPosture,
};
pub use family_identity::{
    admit_topology_conflict_family_identity, TopologyConflictFamilyIdentity,
    TopologyConflictFamilyIdentityAuthority,
};
pub use selection_closeout::{
    current_topology_conflict_family_catalog_closeout, TopologyConflictFamilyCatalogCloseout,
};
pub use source_firewall::WorthTopologyTouchedGraphConflictSourceFirewallRegion;
