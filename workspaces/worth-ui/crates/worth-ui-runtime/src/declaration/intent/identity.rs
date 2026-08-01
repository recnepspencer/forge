use std::sync::Arc;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UiIntentDeclarationIdentity(Arc<str>);

impl UiIntentDeclarationIdentity {
    pub(crate) fn new(identity: impl AsRef<str>) -> Self {
        Self(Arc::from(identity.as_ref()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub(crate) fn valid_intent_identity(identity: &str) -> bool {
    !identity.is_empty()
        && identity
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b'/'))
}
