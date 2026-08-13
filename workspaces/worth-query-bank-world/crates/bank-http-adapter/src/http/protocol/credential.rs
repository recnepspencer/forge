use std::str::FromStr;

use openidconnect::core::CoreIdToken;
use openidconnect::{AccessToken, Nonce};
use serde::{Deserialize, Serialize};

use crate::AuthentikOidcCredential;

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BankHttpCredential {
    id_token: String,
    access_token: String,
    nonce: String,
}

impl BankHttpCredential {
    pub fn from_authentik(credential: &AuthentikOidcCredential) -> Self {
        Self {
            id_token: credential.id_token.to_string(),
            access_token: credential.access_token.secret().to_owned(),
            nonce: credential.nonce.secret().to_owned(),
        }
    }

    pub(crate) fn into_authentik(self) -> Option<AuthentikOidcCredential> {
        Some(AuthentikOidcCredential {
            id_token: CoreIdToken::from_str(&self.id_token).ok()?,
            access_token: AccessToken::new(self.access_token),
            nonce: Nonce::new(self.nonce),
        })
    }
}

impl std::fmt::Debug for BankHttpCredential {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("BankHttpCredential")
            .finish_non_exhaustive()
    }
}
