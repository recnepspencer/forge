//! The sole public aggregation surface for `worth-runtime-world`.
//!
//! This module contains exports only. Runtime behavior remains in the private
//! owner modules so callers cannot reach around the composition boundary.

// Public signatures below name the already-issued component owner bundles and
// tokens. Re-exporting those exact types here keeps the World facade complete
// without exposing a constructor or a second authority path.
pub use worth_relational::facade::branch::{
    AdmittedRelationalBranchBasis, RelationalBranchBasisPort, RelationalOwnerServicePorts,
};
pub use worth_relational::facade::history::RelationalCommitIdentity;
pub use worth_relational::facade::mvcc::RelationalTransactionIntent;
pub use worth_runtime_bridge::facade::{
    AdmittedRuntimeWorldCorrespondenceBasis, RuntimeWorldCorrespondencePort,
};
pub use worth_signal::facade::branch::{
    AdmittedSignalBranchBasis, SignalBranchBasisAdmissionIdentity, SignalOwnerCancellationToken,
    SignalOwnerServicePorts,
};
pub use worth_signal::facade::{SignalError, SignalTransaction};

pub use crate::basis::AdmittedCompositeRuntimeWorldBasis;
pub use crate::branch::{
    NoEffectRuntimeWorldBootstrap, PerformedRuntimeWorldBootstrap, ProductBranchComponentPosture,
    ProductBranchComponentPostures, ProductBranchCreationIntent, ProductBranchName,
    ProductBranchNameDenial, ProductBranchObservation, ProductBranchObservationMismatch,
    ProductBranchObservationMismatchAxis, RuntimeWorldBootstrapIntent,
    RuntimeWorldBootstrapNoEffectCause, RuntimeWorldBootstrapOutcome,
    RuntimeWorldBranchAdmissionDenial, RuntimeWorldBranchRetirementDenial,
};
pub use crate::budget::{
    RuntimeWorldBranchBudgetInstallation, RuntimeWorldBudgetDenial, RuntimeWorldBudgetInstallation,
    RuntimeWorldBudgetLimit, RuntimeWorldBudgetResource, RuntimeWorldBudgets,
    RuntimeWorldCustodyBudgetInstallation, RuntimeWorldHistoryBudgetInstallation,
    RuntimeWorldObservationBudgetInstallation, RuntimeWorldPublicationBudgetInstallation,
    RuntimeWorldRecoveryBudgetInstallation, RuntimeWorldRetentionBudgetInstallation,
};
pub use crate::history::{
    CompositeCallerCorrelation, CompositeCommitParent, CompositeCommitProvenance,
    CompositeComponentChangePosture, CompositeRuntimeWorldCommit,
    CompositeSignalPublicationIdentity, OrdinaryParent,
};
pub use crate::identity::{
    CompositeBasisIdentity, CompositeCommitIdentity, CompositePublicationAttemptIdentity,
    ProductBranchIdentity, ProductBranchLifecycleIncarnation, ProductBranchReferenceGeneration,
    ProductUnpublishedOwnerEffectsIdentity, RuntimeWorldBootstrapAttemptIdentity,
    RuntimeWorldIdentityExhaustion, RuntimeWorldIdentityFamily, RuntimeWorldOwnerIdentity,
};
pub use crate::lifecycle::{
    RuntimeWorldCancellationSource, RuntimeWorldCancellationToken, RuntimeWorldClock,
    RuntimeWorldClockSource, RuntimeWorldCloseDenial, RuntimeWorldInstant, RuntimeWorldOwnerInputs,
    RuntimeWorldOwnerLifecycleObservation, RuntimeWorldOwnerRoot, RuntimeWorldOwnerUnavailable,
};
pub use crate::publication::{
    CompositeAttemptCancellationPosture, CompositeAttemptProgress, CompositeComponentIntent,
    CompositeExecutionBorrow, CompositeLateCancellationPosture, CompositeOwnerExecutionResults,
    CompositePublicationCostCounters, CompositePublicationOrder, CompositePublicationReady,
    CompositeRelationalOwnerResult, CompositeSignalOwnerResult, LoweredOwnerComponentPlan,
    NoEffectCause, NoEffectCompositePublication, OwnerExecutionSettlement,
    PerformedCompositePublication, ProductBranchIntent, RelationalAttemptProgress,
    RelationalAttemptProgressPosture, RelationalComponentPlan, RelationalComponentPlanPosture,
    ReservedCompositePublicationAttempt, ResolvedExpectedProductHead,
    RuntimeWorldPublicationOutcome, RuntimeWorldPublicationPhase, SignalAttemptProgress,
    SignalAttemptProgressPosture, SignalComponentPlan, SignalComponentPlanPosture,
    SignalTransactionMutation,
};
pub use crate::recovery::{
    ProductUnpublishedCause, ProductUnpublishedNextAction, ProductUnpublishedOwnerEffects,
    ProductUnpublishedRecoveryHandle, RecoveryContinuationContract,
};
