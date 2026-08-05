//! Bank transport adaptation.
//!
//! Authentik tokens and protocol types terminate in this crate.

#![forbid(unsafe_code)]

mod adapter;
mod authorization;
mod bank_identity;
mod client;
mod configuration;
mod credential;
mod error;
mod scope;
mod validation;

#[cfg(feature = "cold-certification")]
pub mod cold_certification;
pub use adapter::AuthentikOidcAdapter;
pub use authorization::{
    AuthentikAuthorizationCallback, AuthentikAuthorizationRequest, AuthentikPendingAuthorization,
};
pub use bank_identity::AuthentikBankIdentity;
pub use configuration::{
    AuthentikOidcConfiguration, AuthentikOidcConfigurationBuilder, AuthentikOidcConfigurationError,
};
pub use credential::AuthentikOidcCredential;
pub use error::{
    AuthentikBankAuthenticationError, AuthentikBankIdentityBuildError,
    AuthentikOidcAdapterBuildError, AuthentikOidcFlowError,
};
