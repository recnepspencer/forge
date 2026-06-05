mod corpus;
mod experiments;
mod frontier;
mod graph_memory;
mod hypotheses;
mod operations;
mod patterns;

pub use corpus::{
    HadwigerDiscoveryEvidenceReference, ResearchEvidenceCorpus, ResearchEvidenceCorpusBuilder,
};
pub use experiments::{
    DeadEndSignature, ExperimentBatch, ExperimentPlan, ExperimentResult,
    ExperimentSuppressionProof, SuppressionRelation,
};
pub use frontier::{
    DerivedFrontierState, DiscoveryFrontier, DiscoveryScorecard, HadwigerDiscoveryCounters,
};
pub use graph_memory::{FailureBasisFingerprint, FailureScope, GraphResidentFailure};
pub use hypotheses::{
    CounterexampleObligation, InvariantCandidate, InvariantHypothesis, ReactivationCondition,
    RetiredHypothesisRecord,
};
pub use operations::{
    attach_failure_to_research_graph, mine_research_patterns, plan_next_experiments,
    propose_invariant_hypotheses, recompute_derived_discovery_frontier, update_discovery_frontier,
    HadwigerDiscoveryError,
};
pub use patterns::{MotifObservation, PatternSignature};
