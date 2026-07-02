pub(crate) mod derived_read_diagnostics;

#[allow(unused_imports)]
pub use crate::compiled_product_family::DeterministicDigest;
pub use derived_read_diagnostics::{
    DerivedFallbackReport, DerivedInvalidationReport, DerivedReadDiagnostics, DerivedRebuildReport,
    DerivedValidationExecutionReport,
};

#[cfg(test)]
#[path = "equivalence_contract_tests.rs"]
mod equivalence_contract_tests;
