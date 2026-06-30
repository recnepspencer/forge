#[cfg(test)]
mod tests;

#[allow(unused_imports)]
pub use crate::projection::diagnostic_surfaces::{
    build_derived_equivalence_contract, compare_derived_equivalence_contracts,
    digest_derived_validation_report, digest_interpreted_topology_view,
    digest_materialized_topology_view, DerivedEquivalenceContractReport,
    DerivedParityComparisonReport,
};
