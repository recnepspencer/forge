#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ActiveSubscriptionLaneDigest(String);

impl ActiveSubscriptionLaneDigest {
    pub(super) fn new(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
