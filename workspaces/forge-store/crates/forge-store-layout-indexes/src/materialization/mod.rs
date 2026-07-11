mod absence;
mod absence_outcome;
mod completeness;
mod coverage;
mod denial;
mod state;
#[cfg(test)]
pub(crate) mod tests;
mod watermark;

pub use absence::{S8AbsenceAuthorityClass, S8PhysicalAbsenceProof};
pub(super) use absence_outcome::issue_physical_absence;
pub use absence_outcome::{S8PhysicalAbsenceOutcome, S8PhysicalAbsenceOutcomeView};
pub use completeness::{
    S8MaterializationCompleteness, S8PrefixCompletenessWitness, S8RangeCompletenessWitness,
};
pub use coverage::S8LayoutCoverageWitness;
pub use denial::{S8CoverageGapClass, S8CoverageGapWitness, S8MaterializationDenial};
pub use state::{S8LayoutMaterializationState, S8MaterializationStateClass};
pub use watermark::{S8CoverageBasisKind, S8LayoutWatermark, S8PhysicalCoverageBasis};
