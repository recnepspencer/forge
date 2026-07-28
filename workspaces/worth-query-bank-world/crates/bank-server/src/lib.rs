//! Authoritative bank runtime composition.
//!
//! Transport and Authentik protocol details belong to downstream adapters.

#![forbid(unsafe_code)]

mod authenticated_principal;
mod authentication_boundary;
mod domain_package;
mod error;
mod graph_bootstrap;
mod identity_runtime;
mod operation_admission;
mod operation_proposals;
mod principal_seed;
mod world_seed;

pub use authenticated_principal::BankAuthenticatedPrincipal;
pub use authentication_boundary::BankAuthenticationBoundary;
pub use error::{
    BankAuthenticationBoundaryBuildError, BankIdentityRuntimeBuildError,
    BankPrincipalAdmissionError, BankWorldSeedDenial,
};
pub use identity_runtime::{BankAuthenticationConfiguration, BankIdentityRuntime};
pub use operation_admission::{BankAdmittedOperation, BankOperationAdmissionError};
pub use operation_proposals::{BankAuthorizedProposal, BankOperationProposals};
pub use principal_seed::BankPrincipalSeed;
pub use world_seed::{BankBusinessOwnerSeed, BankEmployeeAssignmentSeed, BankWorldSeed};
