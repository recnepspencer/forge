mod candidate;
mod counters;
mod denial;
mod extraction;
mod identity;
mod source_interval_binding;

#[cfg(test)]
mod tests;

#[cfg(test)]
pub(crate) use candidate::PlanarBooleanIntervalSplitCandidateInput;
pub use candidate::{PlanarBooleanIntervalSplitCandidate, PlanarBooleanIntervalSplitCandidateSet};
pub use counters::PlanarBooleanIntervalSplitCandidateCounters;
pub use denial::{
    PlanarBooleanIntervalSplitCandidateDenial, PlanarBooleanIntervalSplitCandidateDenialKind,
};
