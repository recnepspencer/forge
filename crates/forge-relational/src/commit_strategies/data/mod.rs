mod artifacts;
mod descriptor;
mod execution;
mod lowering;
mod registration;
mod request;
mod strategy_id;
mod validation;

pub use artifacts::{
    StrategyCommitArtifactBundle, StrategyMergeConflictClass, StrategyMergeDescriptor,
    StrategyReplayDescriptor,
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
pub use registration::{CommitStrategyRegistration, CommitStrategyRegistrationError};
pub use request::{
    CanonicalStrategyCommitRequest, CanonicalStrategyInputArtifact, CanonicalStrategyInputDigest,
    RawStrategyCommitRequest, StrategyCallerProvenance, StrategyCommitRequestError,
    StrategyRequestOrigin,
};
pub use strategy_id::CommitStrategyId;
pub(crate) use validation::PreparedStrategyAuthorityScope;
pub use validation::{StrategyPreviewValidationCostSummary, ValidatedStrategyCommitPlan};
