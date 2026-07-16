mod admission;
mod certification_evidence;
mod contracts;
mod evidence;
mod execution;
mod families;
mod inspector;
mod metadata;
mod performance;
mod replay;
mod request;
mod results;
mod support;
mod synthetic;
#[cfg(test)]
pub(crate) use admission::admit_identity_evolution_query_for_scenario;
pub use admission::{
    admit_identity_evolution_query, AdmittedIdentityEvolutionQuery,
    IdentityEvolutionAdmissionError, IdentityEvolutionAdmissionFailureClass,
};
pub use certification_evidence::IdentityEvolutionCertificationEvidence;
pub use contracts::{IdentityEvolutionComplexityContract, IdentityEvolutionComplexityStatus};
pub use evidence::{
    IdentityEvolutionCertificationDenialEvidence, IdentityEvolutionCertificationResultEvidence,
    IdentityEvolutionCounterSnapshot,
};
pub use execution::{
    execute_admitted_identity_evolution_query, IdentityEvolutionExecutionArtifact,
    IdentityEvolutionExecutionCounters, IdentityEvolutionExecutionFamily,
};
#[cfg(test)]
pub(crate) use families::{
    IdentityEvolutionAmbiguityReason, IdentityEvolutionDenialReason,
    IdentityEvolutionIdentityBreakReason,
};
pub use families::{
    IdentityEvolutionOutcomeFamily, IdentityEvolutionQueryFamily, LineageTraversalFamily,
};
pub use inspector::{
    InspectorIdentityArtifact, InspectorIdentityClassification, InspectorIdentityDigest,
};
pub use metadata::{
    BranchLocalityClass, IdentityEvolutionComplexityReport, IdentityEvolutionMetadata,
    PromotionOrMergeAuthorityState,
};
pub use performance::{
    IdentityEvolutionBudgetClass, IdentityEvolutionCostClass,
    IdentityEvolutionPredictionDriftOutcome, IdentityEvolutionPredictionReport,
};
pub use replay::{
    compare_identity_evolution_denial_classification, compare_identity_evolution_denial_replay,
    compare_identity_evolution_result_classification, compare_identity_evolution_result_replay,
    IdentityEvolutionReplayArtifact, IdentityEvolutionReplayParityClass,
};
pub use request::{
    CorrespondenceIdentityComparison, IdentityComparisonIntent,
    IdentityEvolutionComparisonBasisFamily, IdentityEvolutionQueryContext,
    LineageTraversalDescriptor,
};
pub use results::{
    AdvisoryIdentityCandidateSet, IdentityEvolutionAmbiguityBundle, IdentityEvolutionDeniedBundle,
    IdentityEvolutionIdentityBreakBundle, IdentityEvolutionResultBundle,
    PluralIdentitySuccessorSet, SingularIdentityContinuityResult,
};
pub use support::{
    runtime_backed_direct_identity_evolution_support_profile, IdentityEvolutionDeferredScopeMarker,
    IdentityEvolutionSupportProfile,
};
#[cfg(test)]
pub(crate) use synthetic::IdentityEvolutionSyntheticScenario;

#[cfg(test)]
mod tests;
