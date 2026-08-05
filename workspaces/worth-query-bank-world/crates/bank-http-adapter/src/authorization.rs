use openidconnect::{AuthorizationCode, CsrfToken, Nonce, PkceCodeVerifier};

pub struct AuthentikAuthorizationRequest {
    authorization_url: String,
    pending: AuthentikPendingAuthorization,
}

impl AuthentikAuthorizationRequest {
    pub(crate) fn new(authorization_url: String, pending: AuthentikPendingAuthorization) -> Self {
        Self {
            authorization_url,
            pending,
        }
    }

    pub fn authorization_url(&self) -> &str {
        &self.authorization_url
    }

    pub fn into_pending(self) -> AuthentikPendingAuthorization {
        self.pending
    }
}

impl std::fmt::Debug for AuthentikAuthorizationRequest {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("AuthentikAuthorizationRequest")
            .finish()
    }
}

pub struct AuthentikPendingAuthorization {
    pub(crate) state: CsrfToken,
    pub(crate) nonce: Nonce,
    pub(crate) pkce_verifier: PkceCodeVerifier,
}

impl std::fmt::Debug for AuthentikPendingAuthorization {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("AuthentikPendingAuthorization")
            .finish_non_exhaustive()
    }
}

pub struct AuthentikAuthorizationCallback {
    pub(crate) code: AuthorizationCode,
    pub(crate) state: CsrfToken,
}

impl AuthentikAuthorizationCallback {
    pub fn new(
        code: impl Into<String>,
        state: impl Into<String>,
    ) -> Result<Self, crate::error::AuthentikOidcFlowError> {
        let code = code.into();
        let state = state.into();
        if !valid_callback_component(&code) {
            return Err(crate::error::AuthentikOidcFlowError::InvalidAuthorizationCode);
        }
        if !valid_callback_component(&state) {
            return Err(crate::error::AuthentikOidcFlowError::InvalidState);
        }
        Ok(Self {
            code: AuthorizationCode::new(code),
            state: CsrfToken::new(state),
        })
    }
}

fn valid_callback_component(value: &str) -> bool {
    !value.is_empty()
        && value.trim() == value
        && value.len() <= 2_048
        && !value.chars().any(char::is_control)
}

impl std::fmt::Debug for AuthentikAuthorizationCallback {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("AuthentikAuthorizationCallback")
            .finish_non_exhaustive()
    }
}
