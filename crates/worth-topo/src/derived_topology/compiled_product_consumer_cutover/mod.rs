mod closeout;
mod residue_manifest;
mod topology_derived_cluster;

#[cfg(test)]
mod tests;

pub use residue_manifest::{
    current_topology_consumer_residue_manifest, TopologyConsumerResidueDisposition,
    TopologyConsumerResidueOwner, TopologyConsumerResidueRow,
};
pub use topology_derived_cluster::{
    build_derived_equivalence_contract, build_derived_equivalence_contract_report,
    compare_derived_equivalence_contracts, digest_derived_validation_report,
    digest_interpreted_topology_view, digest_materialized_topology_view,
    topology_cutover_planned_disposition_from_update_posture, DerivedEquivalenceContractReport,
    DerivedInvalidationPlannedDisposition, DerivedParityComparisonReport,
};

#[cfg(test)]
pub(crate) use closeout::require_exact_topology_consumer_closeout;
