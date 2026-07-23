mod artifact_reuse;
mod compatibility;
mod condition_resolution;
mod contract;
#[cfg(test)]
mod contract_tests;
mod decision;
mod dependency_versions;
mod execution;
mod execution_proof;
mod identity;
mod threshold_resolution;

pub use compatibility::{
    SignalConditionalArtifactReuseClass, SignalConditionalComparatorClass,
    SignalConditionalComparatorPosition, SignalConditionalComparisonWork,
    SignalConditionalConditionClass, SignalConditionalExecutionAffinity,
    SignalConditionalExecutionAffinityComparisonMismatch,
    SignalConditionalExecutionAffinityMismatch, SignalConditionalSemanticComparisonMismatch,
    SignalConditionalSemanticContinuity, SignalConditionalSemanticMismatch,
};
pub use contract::{
    InstalledSignalComparatorUse, InstalledSignalConditionalContract,
    SignalConditionalArtifactReuse, SignalConditionalArtifactReusePolicy,
    SignalConditionalCondition, SignalConditionalContractDefinition,
    SignalConditionalContractDenial, SignalConditionalVersionComparator,
    SignalDeltaThresholdContract, SignalThresholdBoundary, SignalThresholdComparisonDomain,
    SignalThresholdValueFamily,
};
pub use decision::{
    InstalledSignalConditionDecision, InstalledSignalConditionResolver,
    SignalConditionalDecisionClass, SignalConditionalDecisionCounters,
    SignalConditionalDecisionEvidence,
};
pub use execution::{SignalConditionalExecutionFailure, SignalConditionalExecutionRequest};
pub use identity::{
    SignalConditionalDecisionIdentityKind, SignalConditionalDecisionProjectionIdentity,
};
pub use threshold_resolution::resolve_signal_delta_threshold;
