pub(crate) mod derived_read_diagnostics;
mod equivalence_contract;

#[allow(unused_imports)]
pub use crate::compiled_product_family::DeterministicDigest;
pub use derived_read_diagnostics::{
    DerivedFallbackReport, DerivedInvalidationReport, DerivedReadDiagnostics, DerivedRebuildReport,
};
pub(crate) use equivalence_contract::build_derived_equivalence_contract_report;
pub use equivalence_contract::{
    build_derived_equivalence_contract, compare_derived_equivalence_contracts,
    digest_derived_validation_report, digest_interpreted_topology_view,
    digest_materialized_topology_view, DerivedEquivalenceContractReport,
    DerivedParityComparisonReport,
};
