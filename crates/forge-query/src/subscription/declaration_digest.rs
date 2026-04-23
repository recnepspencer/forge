use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct QuerySubscriptionDeclarationDigest(String);

impl QuerySubscriptionDeclarationDigest {
    pub(super) fn from_parts(parts: &[String]) -> Self {
        Self(hash_parts(parts))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
