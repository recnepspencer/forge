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

pub use catalog::current_topology_selected_equivalence_family_catalog;
pub use comparator_contract::{
    TopologySelectedEquivalenceComparable, TopologySelectedEquivalenceComparatorContract,
    TopologySelectedEquivalenceComparisonReport,
};
#[cfg(test)]
pub use comparator_contract::TopologySelectedEquivalenceDimension;
pub use family_identity::TopologySelectedEquivalenceFamilyIdentity;
pub use posture::{
    TopologyCompatibilityPosture, TopologyFreshnessRequirementPosture,
    TopologyOrderingNoisePosture, TopologyRenderedOutputComparisonPosture,
};
pub use selected_family::SelectedTopologyEquivalenceFamily;
pub use selection::select_topology_equivalence_family;
