//! Authoritative bank runtime composition.
//!
//! Transport and Authentik protocol details belong to downstream adapters.

#![forbid(unsafe_code)]

mod authenticated_principal;
mod authentication_boundary;
mod bank_projection;
mod domain_package;
mod error;
mod graph_bootstrap;
mod identity_runtime;
mod operation_admission;
mod operation_commit;
mod operation_proposals;
mod ordinary;
mod principal_seed;
mod world_seed;

pub use authenticated_principal::BankAuthenticatedPrincipal;
pub use authentication_boundary::BankAuthenticationBoundary;
pub use bank_projection::BankProjectionDenial;
pub use error::{
    BankAuthenticationBoundaryBuildError, BankIdentityRuntimeBuildError,
    BankPrincipalAdmissionError, BankWorldSeedDenial,
};
pub use identity_runtime::{BankAuthenticationConfiguration, BankIdentityRuntime};
pub use operation_admission::{BankAdmittedOperation, BankOperationAdmissionError};
pub use operation_commit::{
    BankCommitPreparationDenial, BankCommitReceipt, BankMutationCommitOutcome,
};
pub use operation_proposals::{
    BankAuthorizedProposal, BankOperationProposalError, BankOperationProposals,
    BankSendMoneyPreparation,
};
pub use ordinary::{
    mutations, queries, BankActivityCursor, BankActivityCursorDenial, BankActivityLiveLease,
    BankActivityLiveOutcome, BankActivityLiveUpdate, BankActivityPage, BankApprovePendingPayment,
    BankLiveControlDenial, BankLiveControls, BankLiveOpenDenial, BankMutation,
    BankMutationControls, BankMutationDenial, BankMutationExplanation,
    BankMutationExplanationStage, BankMutationForPrincipal, BankMutationMetadata,
    BankMutationOutcome, BankMutationStatus, BankPaymentContinuationDenial,
    BankPaymentInitiationOutcome, BankPendingPaymentContinuation, BankQuery, BankQueryForPrincipal,
    BankReadControlDenial, BankReadControls, BankReadDenial, BankReadMetadata, BankReadOutcome,
    BankReadResult, BankReadyMutation, BankReadyQuery, BankRejectPendingPayment,
};
pub use principal_seed::BankPrincipalSeed;
pub use world_seed::{BankBusinessOwnerSeed, BankEmployeeAssignmentSeed, BankWorldSeed};
