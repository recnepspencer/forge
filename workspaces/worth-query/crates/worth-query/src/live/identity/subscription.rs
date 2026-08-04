use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LiveSubscriptionDigest(String);

impl LiveSubscriptionDigest {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(in crate::live) fn from_parts(parts: &[String]) -> Self {
        Self(hash_parts(parts))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LiveChangeSequenceId(String);

impl LiveChangeSequenceId {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(in crate::live) fn from_subscription_digest(digest: &LiveSubscriptionDigest) -> Self {
        Self(hash_parts(&[format!(
            "live_change_sequence:{}",
            digest.as_str()
        )]))
    }
}
