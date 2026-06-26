#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryViewBindingKey {
    digest_basis: String,
}

impl QueryViewBindingKey {
    pub(crate) fn from_digest_basis(digest_basis: impl Into<String>) -> Self {
        Self {
            digest_basis: digest_basis.into(),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.digest_basis
    }
}
