#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubscriptionConsumerAttachmentDigest(String);

impl SubscriptionConsumerAttachmentDigest {
    pub(super) fn new(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
