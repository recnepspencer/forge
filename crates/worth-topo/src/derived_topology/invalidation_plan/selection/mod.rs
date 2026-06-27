mod counters;
mod density_policy;
mod error;
mod lowering;
mod phase_four_seed;
mod row;
mod selected_plan;
mod support;
mod touched_closure;

#[cfg(any(test, feature = "test-support-lowering"))]
pub(crate) mod selection_test_fixtures;
#[cfg(test)]
mod tests;

pub use counters::DerivedInvalidationSelectionCounters;
pub use density_policy::DerivedInvalidationDensityPolicy;
pub use error::{DerivedInvalidationSelectionError, DerivedInvalidationSelectionErrorKind};
pub use phase_four_seed::DerivedInvalidationPhaseFourSeed;
pub use row::{
    DerivedInvalidationDenialKind, DerivedInvalidationDenialRow,
    DerivedInvalidationPlannedDisposition, DerivedInvalidationResidueRow,
    DerivedInvalidationSelectedRow, DerivedInvalidationUnaffectedRow,
};
pub use selected_plan::DerivedInvalidationSelectedPlan;
pub use support::{
    DerivedInvalidationExecutionAdmission, DerivedInvalidationLegalitySupportEvidence,
    DerivedInvalidationQuerySupportEvidence,
};
pub use touched_closure::DerivedInvalidationTouchedClosure;
