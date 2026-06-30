#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialReplayScopeProductIdentity {
    digest: String,
}

impl SpatialReplayScopeProductIdentity {
    pub(crate) fn new(digest: String) -> Self {
        Self { digest }
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}
