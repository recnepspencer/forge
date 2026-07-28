use openidconnect::core::CoreIdToken;
use openidconnect::{AccessToken, Nonce};

#[derive(Clone)]
pub struct AuthentikOidcCredential {
    pub(crate) id_token: CoreIdToken,
    pub(crate) access_token: AccessToken,
    pub(crate) nonce: Nonce,
}

impl std::fmt::Debug for AuthentikOidcCredential {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("AuthentikOidcCredential")
            .finish_non_exhaustive()
    }
}
