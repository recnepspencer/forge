mod admitted_contract;
mod reuse_decision_contract;
#[cfg(test)]
mod selected_family_contract;

pub use admitted_contract::{
    build_derived_equivalence_contract, build_derived_equivalence_contract_report,
    digest_derived_validation_report, digest_interpreted_topology_view,
    digest_materialized_topology_view, DerivedEquivalenceContractReport,
};
pub use reuse_decision_contract::{
    compare_derived_equivalence_contracts,
    topology_cutover_planned_disposition_from_update_posture,
    DerivedInvalidationPlannedDisposition, DerivedParityComparisonReport,
};
