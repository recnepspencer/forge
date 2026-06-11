use super::validation::normalize_machine_identity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUserResponseEvidence {
    digest: String,
    source_identity: String,
}

impl WorthUserResponseEvidence {
    pub(crate) fn new(digest: impl Into<String>, source_identity: impl Into<String>) -> Self {
        Self {
            digest: normalize_machine_identity(digest),
            source_identity: normalize_machine_identity(source_identity),
        }
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn source_identity(&self) -> &str {
        &self.source_identity
    }
}
