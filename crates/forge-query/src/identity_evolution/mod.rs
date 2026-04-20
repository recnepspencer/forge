mod admission;
mod contracts;
mod evidence;
mod execution;
mod families;
mod metadata;
mod performance;
mod replay;
mod request;
mod results;
mod support;
mod synthetic;

pub use admission::{
    admit_identity_evolution_query, AdmittedIdentityEvolutionQuery,
    IdentityEvolutionAdmissionError, IdentityEvolutionAdmissionFailureClass,
};
#[allow(unused_imports)]
pub(crate) use admission::admit_identity_evolution_query_for_scenario;
pub use contracts::{IdentityEvolutionComplexityContract, IdentityEvolutionComplexityStatus};
#[allow(unused_imports)]
pub use evidence::{
    IdentityEvolutionCertificationDenialEvidence, IdentityEvolutionCertificationEvidence,
    IdentityEvolutionCertificationResultEvidence, IdentityEvolutionCounterSnapshot,
};
pub use execution::{
    execute_admitted_identity_evolution_query, IdentityEvolutionExecutionArtifact,
    IdentityEvolutionExecutionCounters, IdentityEvolutionExecutionFamily,
};
pub use families::{
    IdentityEvolutionOutcomeFamily, IdentityEvolutionQueryFamily, LineageTraversalFamily,
};
pub use metadata::{
    BranchLocalityClass, IdentityEvolutionComplexityReport, IdentityEvolutionMetadata,
    PromotionOrMergeAuthorityState,
};
pub use performance::{
    IdentityEvolutionBudgetClass, IdentityEvolutionCostClass,
    IdentityEvolutionPredictionDriftOutcome, IdentityEvolutionPredictionReport,
};
#[allow(unused_imports)]
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
    IdentityEvolutionResultBundle, PluralIdentitySuccessorSet, SingularIdentityContinuityResult,
};
pub use support::{
    runtime_backed_direct_identity_evolution_support_profile,
    IdentityEvolutionDeferredScopeMarker, IdentityEvolutionSupportProfile,
};
#[allow(unused_imports)]
pub(crate) use synthetic::IdentityEvolutionSyntheticScenario;

#[cfg(test)]
mod tests;
