mod affinity;
mod mismatch;
mod semantic;
mod work;

pub use affinity::{
    SignalConditionalExecutionAffinity, SignalConditionalExecutionAffinityMismatch,
};
pub use mismatch::{
    SignalConditionalArtifactReuseClass, SignalConditionalComparatorClass,
    SignalConditionalComparatorPosition, SignalConditionalConditionClass,
    SignalConditionalSemanticMismatch,
};
pub use semantic::SignalConditionalSemanticContinuity;
pub use work::{
    SignalConditionalComparisonWork, SignalConditionalExecutionAffinityComparisonMismatch,
    SignalConditionalSemanticComparisonMismatch,
};

#[cfg(test)]
mod tests;
