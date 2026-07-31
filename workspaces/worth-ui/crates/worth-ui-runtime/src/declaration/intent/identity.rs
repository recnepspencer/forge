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
