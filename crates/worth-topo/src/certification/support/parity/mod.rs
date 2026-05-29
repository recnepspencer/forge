mod facade;
mod types;

#[cfg(test)]
mod tests;

pub use facade::{
    build_derived_equivalence_contract, build_derived_equivalence_contract_report,
    compare_derived_equivalence_contracts, digest_derived_validation_report,
    digest_interpreted_topology_view, digest_materialized_topology_view,
};
pub use types::{DerivedEquivalenceContractReport, DerivedParityComparisonReport};
