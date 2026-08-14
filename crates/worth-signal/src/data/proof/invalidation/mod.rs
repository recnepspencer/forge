pub(crate) mod binding;
mod execution;
mod frontier_admission;
pub(crate) mod output_commit;
mod plan;
pub(crate) mod revalidation;
pub(crate) mod source_seed;

#[cfg(test)]
mod progression_contract_tests;

pub use execution::{
    FrontierExecutionCounters, FrontierExecutionSummary, FrontierWaveEntrySummary,
    FrontierWaveSummary, InvalidationTraceRecord, TransitiveFrontierEntrySummary,
    TransitiveFrontierWaveSummary,
};
pub use frontier_admission::{
    FrontierEntryClassification, FrontierInclusionBasis, FrontierSeedCause,
    FrontierValidationDecision, InvalidationSeed, InvalidationSeedBatch,
};
pub use plan::{FrontierPlan, FrontierPredictedCounters, FrontierWaveEntryPlan, FrontierWavePlan};
