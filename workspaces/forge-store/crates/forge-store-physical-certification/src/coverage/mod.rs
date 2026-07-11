mod blob_dimensions;
mod denial;
mod harness_maturity;
mod matrix;
mod maturity;
mod mutation;
mod registration;
mod report;
mod row;
mod sequence;

pub use denial::{
    reject_edited_matrix_row, reject_manual_coverage_prose, reject_unchecked_maturity_claim,
    CoverageGapDenial,
};
pub use harness_maturity::{
    reject_copied_s5_simulation_harness_readiness_fields, reject_missing_s5_correctness_non_claim,
    PhysicalIsolationCorrectnessNonClaimEvidence, PhysicalIsolationHarnessMaturityDependency,
    PhysicalIsolationHarnessReadinessDenial,
};
pub use matrix::{GeneratedCoverageMatrix, PhysicalCoverageMatrix};
pub use maturity::{
    HarnessMaturityEvidence, HarnessMaturityLevel, HarnessSubsystemMaturity,
    PhysicalIsolationHarnessMaturityDependencyEvidence, PhysicalIsolationHarnessReadiness,
};
pub use mutation::{
    PhysicalIsolationCompactionMutationCoverageRow, PhysicalIsolationCompactionMutationKind,
    PhysicalIsolationMutationKind, PhysicalMutationCoverageEvidence,
};
pub use registration::PhysicalCoverageRegistry;
pub use report::PhysicalHarnessReadinessReport;
pub use row::{
    CoverageRowDimension, CoverageRowSatisfiedReceipt, CoverageSurfaceKind,
    MutationResultCoverageRow, MutationValidationPosture, PhysicalCoverageMatrixRow,
    RegisteredCounterCoverageRow, RegisteredOracleCoverageRow, RegisteredScenarioCoverageRow,
    RegisteredTranscriptCoverageRow,
};
pub use sequence::{
    HarnessCoverageStage, HarnessSubsystem, PhysicalIsolationReadinessDependencySet,
};
