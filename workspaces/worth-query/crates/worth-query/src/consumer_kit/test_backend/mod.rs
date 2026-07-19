mod backend;
mod builder;
mod equivalence_report;
mod error;
mod residue_audit;
mod schema;
mod support_profile;

pub use builder::{in_memory_test_runtime, WorthQueryInMemoryTestRuntimeBuilder};
pub use equivalence_report::{
    compare_test_backend_write_receipts, WorthQueryTestBackendEquivalenceReport,
    WorthQueryTestBackendEquivalenceRow,
};
pub use error::{WorthQueryTestBackendError, WorthQueryTestBackendErrorKind};
pub use residue_audit::{
    query_test_backend_residue_audit, WorthQueryTestBackendResidueAudit,
    WorthQueryTestBackendResidueFinding, WorthQueryTestBackendResidueReport,
};
pub use schema::WorthQueryTestBackendSchema;

#[cfg(test)]
mod contract_fixtures;
#[cfg(test)]
mod schema_tests;
#[cfg(test)]
mod support_profile_tests;
#[cfg(test)]
mod workspace_behavior_tests;
