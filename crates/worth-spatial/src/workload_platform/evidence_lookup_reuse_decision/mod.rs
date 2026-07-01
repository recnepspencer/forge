mod counters;
mod decision;
mod denial;
mod execution;
mod execution_input;
mod mismatch_locus;
mod posture;
mod resolution;

#[cfg(test)]
mod tests;

pub use counters::EvidenceLookupReuseDecisionCounters;
pub use decision::EvidenceLookupIndexReuseDecision;
pub use denial::EvidenceLookupIndexRebuildDenial;
pub use execution::{decide_evidence_lookup_index_reuse, execute_evidence_lookup_index_reuse};
pub(crate) use execution_input::EvidenceLookupIndexReuseExecutionInput;
pub use mismatch_locus::EvidenceLookupReuseMismatchLocus;
pub use posture::EvidenceLookupReuseDecisionPosture;
pub use resolution::EvidenceLookupIndexReuseResolution;
