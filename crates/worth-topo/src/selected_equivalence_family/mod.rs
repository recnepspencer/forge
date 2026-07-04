mod basis_identity;
mod catalog;
mod comparator_contract;
mod declaration;
mod error;
mod family_identity;
mod posture;
mod selected_family;
mod selection;

#[cfg(test)]
mod tests;

pub use basis_identity::{
    TopologySelectedCompatibilityBasisIdentity, TopologySelectedEquivalenceBasisIdentity,
    TopologySelectedFutureProofSeedIdentity, TopologySelectedReuseBasisIdentity,
};
pub use catalog::{
    current_topology_selected_equivalence_family_catalog, TopologySelectedEquivalenceFamilyCatalog,
};
pub use comparator_contract::{
    TopologySelectedEquivalenceComparable, TopologySelectedEquivalenceComparatorContract,
    TopologySelectedEquivalenceComparisonReport, TopologySelectedEquivalenceDimension,
};
pub use error::{
    TopologySelectedEquivalenceFamilyError, TopologySelectedEquivalenceFamilyErrorKind,
};
pub use family_identity::TopologySelectedEquivalenceFamilyIdentity;
pub use posture::{
    TopologyCompatibilityPosture, TopologyFreshnessRequirementPosture,
    TopologyOrderingNoisePosture, TopologyRenderedOutputComparisonPosture,
};
pub use selected_family::SelectedTopologyEquivalenceFamily;
pub use selection::select_topology_equivalence_family;
