mod absence;
mod completeness;
mod coverage;
mod denial;
mod state;
#[cfg(test)]
mod tests;
mod watermark;

pub use absence::{S8AbsenceAuthorityClass, S8PhysicalAbsenceProof};
pub use completeness::{
    S8MaterializationCompleteness, S8PrefixCompletenessWitness, S8RangeCompletenessWitness,
};
pub use coverage::S8LayoutCoverageWitness;
pub use denial::{S8CoverageGapClass, S8CoverageGapWitness, S8MaterializationDenial};
pub use state::{S8LayoutMaterializationState, S8MaterializationStateClass};
pub use watermark::{S8CoverageBasisKind, S8LayoutWatermark, S8PhysicalCoverageBasis};
