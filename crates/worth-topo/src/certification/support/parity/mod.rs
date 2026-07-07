#[cfg(test)]
mod tests;

pub use crate::derived_topology::compiled_product_consumer_cutover::{
    build_derived_equivalence_contract, compare_derived_equivalence_contracts,
    digest_derived_validation_report, digest_materialized_topology_view,
    DerivedEquivalenceContractReport,
};
#[cfg(test)]
pub use crate::derived_topology::compiled_product_consumer_cutover::{
    digest_interpreted_topology_view,
};
