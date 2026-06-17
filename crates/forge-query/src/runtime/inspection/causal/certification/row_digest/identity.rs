#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct RepresentativeCausalObservationAnchorDigest(String);

impl RepresentativeCausalObservationAnchorDigest {
    pub(super) fn from_digest(digest: impl Into<String>) -> Self {
        Self(digest.into())
    }

    pub(super) fn as_str(&self) -> &str {
        &self.0
    }
}
