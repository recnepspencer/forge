mod artifacts;
mod canonical_digest;
mod descriptor;
mod execution;
mod lowering;
mod native_codec;
mod native_strategy_intent_scope;
mod registration;
mod request;
mod strategy_id;
mod validation;

pub(crate) use canonical_digest::commit_strategy_registry_digest;
pub(super) use canonical_digest::strategy_mutation_program_digest;

#[allow(unused_imports)]
pub use artifacts::{
    StrategyCommitArtifactBundle, StrategyIntentScopeDigest, StrategyMergeConflictClass,
    StrategyMergeDescriptor, StrategyMergeSemantics, StrategyReplayDescriptor,
    StrategyRuntimeDeterminismBasis,
};
pub use descriptor::{
    CommitStrategyDescriptor, CommitStrategyDescriptorDigest, CommitStrategyFamilyName,
    CommitStrategySemanticName, CommitStrategyVersion, PersistentArtifactName,
    StrategyInputSchemaName, StrategyInputSchemaVersion, StrategyIntentName,
    StrategyOutputSchemaName, StrategyPacketContract, StrategyReadContract, StrategyReadCostClass,
    StrategyReadLocalityClass, StrategyReadScopeClass, StrategyRequestCanonicalization,
    StrategyTraversalBasis,
};
pub use execution::{
    CanonicalStrategyOutputArtifact, CanonicalStrategyOutputDigest,
    CommitStrategyExecutionRegistration, CommitStrategyExecutor, StrategyExecutionDraft,
    StrategyExecutionResult, StrategyExecutionSummary, StrategyExecutorFailure,
    StrategyExecutorFailureClass, StrategyMutationProgram, StrategyMutationProgramDigest,
    StrategyObservationContext, StrategyVisibilityReadView,
};
pub use lowering::{
    LoweredStrategyCommitPlan, StrategyLoweringError, StrategyLoweringProvenance,
    StrategyLoweringSummary,
};
pub(crate) use native_codec::*;
pub use registration::{CommitStrategyRegistration, CommitStrategyRegistrationError};
pub use request::{
    CanonicalStrategyCommitRequest, CanonicalStrategyInputArtifact, CanonicalStrategyInputDigest,
    RawStrategyCommitRequest, StrategyCallerProvenance, StrategyCommitRequestError,
    StrategyRequestOrigin,
};
pub use strategy_id::CommitStrategyId;
pub(crate) use validation::PreparedStrategyAuthorityScope;
pub use validation::{StrategyPreviewValidationCostSummary, ValidatedStrategyCommitPlan};
