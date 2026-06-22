#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerCompatibilityProductSessionContinuation {
    product_session_identity: String,
    canonical_digest: String,
}

impl ForgeServerCompatibilityProductSessionContinuation {
    pub(crate) fn new(product_session_identity: impl Into<String>) -> Self {
        let product_session_identity = product_session_identity.into();
        let canonical_digest = format!(
            "forge-server-compat-product-session-continuation-v1|session={product_session_identity}"
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
pub struct ForgeServerCompatibilityOpenedProductSession {
    session: crate::ForgeServerProductSession,
    continuation: ForgeServerCompatibilityProductSessionContinuation,
    canonical_digest: String,
}

impl ForgeServerCompatibilityOpenedProductSession {
    pub(crate) fn new(session: crate::ForgeServerProductSession) -> Self {
        let continuation =
            ForgeServerCompatibilityProductSessionContinuation::new(session.identity().as_str());
        let canonical_digest = format!(
            "forge-server-compat-opened-product-session-v1|session={}|continuation={}",
            session.canonical_digest(),
            continuation.canonical_digest()
        );
        Self {
            session,
            continuation,
            canonical_digest,
        }
    }

    pub fn session(&self) -> &crate::ForgeServerProductSession {
        &self.session
    }

    pub fn continuation(&self) -> &ForgeServerCompatibilityProductSessionContinuation {
        &self.continuation
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }

    pub fn into_session(self) -> crate::ForgeServerProductSession {
        self.session
    }
}
