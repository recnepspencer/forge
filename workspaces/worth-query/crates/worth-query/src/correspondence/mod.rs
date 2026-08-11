mod ambiguity;
#[cfg(test)]
mod bridge_lowering;
#[cfg(test)]
mod bridge_lowering_fixtures;
#[cfg(test)]
mod bridge_lowering_tests;
mod candidate_set;
mod contracts;
mod cost;
mod counters;
mod error;
mod outcome;
mod report;
mod request;
mod resolution;
#[cfg(test)]
mod tests;

pub use ambiguity::{CorrespondenceAmbiguityEnvelope, CorrespondenceDisagreementEnvelope};
pub use candidate_set::CorrespondenceCandidateSet;
pub use contracts::{
    CorrespondenceComplexityContract, CorrespondencePerformanceStatusMarker,
    StructuralCandidateBudget, UniqueStructuralCorrespondenceWitness,
};
pub use cost::{
    CorrespondenceCostPosture, StructuralCandidateDiscoveryPlan,
    StructuralCandidateOrderingContract,
};
pub use counters::CorrespondenceCounterSnapshot;
pub use error::{CorrespondenceEvaluationError, CorrespondenceEvaluationFailureClass};
pub use outcome::{
    AdvisoryStructuralAmbiguous, AdvisoryStructuralUnique, CorrespondenceDenied,
    CorrespondenceOutcome, LineageContinuity, LineageStructuralDisagreement,
};
pub use report::CorrespondenceVocabularyReport;
pub use request::CorrespondenceEvaluationRequest;
pub(crate) use resolution::resolve_correspondence_evidence;
pub use resolution::CorrespondenceEvidenceResolved;
