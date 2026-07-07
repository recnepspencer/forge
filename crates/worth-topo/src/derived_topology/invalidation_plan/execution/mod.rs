mod counters;
mod diagnostics;
mod error;
mod materialization_policy;
mod outcome;
mod product_execution;
mod receipt;
mod rows;

#[cfg(test)]
mod tests;

pub use counters::DerivedInvalidationExecutionCounters;
pub use diagnostics::{DerivedInvalidationDiagnosticProjection, DerivedInvalidationDiagnosticRow};
pub use error::{DerivedInvalidationExecutionError, DerivedInvalidationExecutionErrorKind};
pub(crate) use materialization_policy::admit_materialization_report_for_execution_outcome;
pub use outcome::DerivedInvalidationExecutionOutcome;
pub(crate) use product_execution::{
    DerivedInvalidationProductExecutionReport, DerivedInvalidationProductExecutor,
    PlannedDerivedInvalidationProductExecutor,
};
pub use receipt::DerivedInvalidationExecutionReceipt;
pub use rows::{
    DerivedInvalidationDeniedProductExecutionRow, DerivedInvalidationExecutedProductRow,
    DerivedInvalidationResidueExecutionRow, DerivedInvalidationUnaffectedProductExecutionRow,
};
