use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SavedQueryArtifactDigest(String);

impl SavedQueryArtifactDigest {
    pub fn from_parts(parts: &[String]) -> Self {
        Self(hash_parts(parts))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
