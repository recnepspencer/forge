//! Cold hostile transforms for the external Docker courtroom.
//!
//! These operations never mint authenticated Query authority and never expose
//! token text. The feature is absent from ordinary adapter builds.

use openidconnect::Nonce;
use worth_query_host::facade::admission::authenticated_principal::WorthQueryRequestScope;

use crate::{
    AuthentikBankIdentity, AuthentikBankIdentityBuildError, AuthentikOidcConfiguration,
    AuthentikOidcCredential,
};

pub async fn install_identity(
    configuration: AuthentikOidcConfiguration,
    seeds: impl IntoIterator<Item = bank_server::BankPrincipalSeed>,
    scope: &WorthQueryRequestScope,
) -> Result<AuthentikBankIdentity, AuthentikBankIdentityBuildError> {
    AuthentikBankIdentity::install_for_cold_certification(configuration, seeds, scope).await
}

pub fn corrupt_signature(
    mut credential: AuthentikOidcCredential,
) -> Result<AuthentikOidcCredential, HostileCredentialError> {
    let serialized = credential.id_token.to_string();
    let (signing_input, signature) = serialized
        .rsplit_once('.')
        .ok_or(HostileCredentialError::MalformedIdToken)?;
    let first = signature
        .chars()
        .next()
        .ok_or(HostileCredentialError::MalformedIdToken)?;
    let replacement = if first == 'A' { 'B' } else { 'A' };
    let mut corrupted_signature = signature.to_string();
    corrupted_signature.replace_range(0..first.len_utf8(), &replacement.to_string());
    credential.id_token = format!("{signing_input}.{corrupted_signature}")
        .parse()
        .map_err(|_| HostileCredentialError::MalformedIdToken)?;
    Ok(credential)
}

pub fn mismatch_nonce(mut credential: AuthentikOidcCredential) -> AuthentikOidcCredential {
    credential.nonce = Nonce::new("cold-certification-mismatched-nonce".to_string());
    credential
}

pub fn substitute_access_token(
    mut identity_credential: AuthentikOidcCredential,
    access_token_source: &AuthentikOidcCredential,
) -> AuthentikOidcCredential {
    identity_credential.access_token = access_token_source.access_token.clone();
    identity_credential
}

pub fn replace_with_malformed_id_token(
    mut credential: AuthentikOidcCredential,
) -> Result<AuthentikOidcCredential, HostileCredentialError> {
    credential.id_token = "not.a.valid-json-web-token"
        .parse()
        .map_err(|_| HostileCredentialError::MalformedIdToken)?;
    Ok(credential)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HostileCredentialError {
    MalformedIdToken,
}

impl std::fmt::Display for HostileCredentialError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("hostile credential is not a structurally valid ID token")
    }
}

impl std::error::Error for HostileCredentialError {}
