#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeOutcomeDenialPosture {
    digest_basis: String,
}

impl RuntimeOutcomeDenialPosture {
    pub fn structured_status() -> Self {
        Self {
            digest_basis: "structured_status".to_string(),
        }
    }

    pub fn with_digest_basis(digest_basis: impl Into<String>) -> Self {
        Self {
            digest_basis: digest_basis.into(),
        }
    }

    pub(crate) fn digest_basis(&self) -> &str {
        &self.digest_basis
    }
}
