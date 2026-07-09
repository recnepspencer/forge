#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionSourceReferenceIdentity {
    label: &'static str,
    identity: String,
}

impl ProjectionSourceReferenceIdentity {
    pub fn label(&self) -> &'static str {
        self.label
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub(crate) fn synthetic(label: &'static str, identity: impl Into<String>) -> Self {
        Self {
            label,
            identity: identity.into(),
        }
    }

    #[cfg(test)]
    pub(crate) fn test_only(label: &'static str, identity: impl Into<String>) -> Self {
        Self::synthetic(label, identity)
    }
}
