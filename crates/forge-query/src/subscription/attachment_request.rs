use crate::identity::hash_parts;

#[derive(Debug, Eq, PartialEq)]
pub struct SubscriptionConsumerAttachmentRequest {
    consumer_digest: String,
    delivery_cursor_seed: String,
}

impl SubscriptionConsumerAttachmentRequest {
    pub fn admitted(
        consumer_id: impl Into<String>,
        delivery_cursor_seed: impl Into<String>,
    ) -> Self {
        let consumer_id = consumer_id.into();
        let delivery_cursor_seed = delivery_cursor_seed.into();
        let consumer_digest = hash_parts(&[
            "subscription_consumer_attachment_request_v1".to_string(),
            format!("consumer:{}", consumer_id),
            format!("cursor_seed:{}", delivery_cursor_seed),
        ]);
        Self {
            consumer_digest,
            delivery_cursor_seed,
        }
    }

    pub fn consumer_digest(&self) -> &str {
        &self.consumer_digest
    }

    pub fn delivery_cursor_seed(&self) -> &str {
        &self.delivery_cursor_seed
    }
}
