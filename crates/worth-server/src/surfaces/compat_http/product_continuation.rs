#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerCompatibilityProductSessionContinuation {
    product_session_identity: String,
    canonical_digest: String,
}

impl WorthServerCompatibilityProductSessionContinuation {
    pub(crate) fn new(product_session_identity: impl Into<String>) -> Self {
        let product_session_identity = product_session_identity.into();
        let canonical_digest = format!(
            "worth-server-compat-product-session-continuation-v1|session={product_session_identity}"
        );
        Self {
            product_session_identity,
            canonical_digest,
        }
    }

    pub fn product_session_identity(&self) -> &str {
        &self.product_session_identity
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}

#[derive(Clone, Debug)]
pub struct WorthServerCompatibilityOpenedProductSession {
    session: crate::WorthServerProductSession,
    continuation: WorthServerCompatibilityProductSessionContinuation,
    canonical_digest: String,
}

impl WorthServerCompatibilityOpenedProductSession {
    pub(crate) fn new(session: crate::WorthServerProductSession) -> Self {
        let continuation =
            WorthServerCompatibilityProductSessionContinuation::new(session.identity().as_str());
        let canonical_digest = format!(
            "worth-server-compat-opened-product-session-v1|session={}|continuation={}",
            session.canonical_digest(),
            continuation.canonical_digest()
        );
        Self {
            session,
            continuation,
            canonical_digest,
        }
    }

    pub fn session(&self) -> &crate::WorthServerProductSession {
        &self.session
    }

    pub fn continuation(&self) -> &WorthServerCompatibilityProductSessionContinuation {
        &self.continuation
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }

    pub fn into_session(self) -> crate::WorthServerProductSession {
        self.session
    }
}
