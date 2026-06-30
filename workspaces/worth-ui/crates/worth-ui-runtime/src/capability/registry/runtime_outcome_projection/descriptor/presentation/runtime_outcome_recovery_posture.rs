#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeOutcomeRecoveryPosture {
    digest_basis: String,
}

impl RuntimeOutcomeRecoveryPosture {
    pub fn action_hint() -> Self {
        Self {
            digest_basis: "action_hint".to_string(),
        }
    }

    pub fn retry_hint() -> Self {
        Self {
            digest_basis: "retry_hint".to_string(),
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
