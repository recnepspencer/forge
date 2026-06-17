use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};

#[derive(Debug, Eq, PartialEq)]
pub struct SubscriptionConsumerAttachmentRequest {
    consumer_identity: ForgeQueryEvidenceIdentity,
    delivery_cursor_seed_identity: ForgeQueryEvidenceIdentity,
}

impl SubscriptionConsumerAttachmentRequest {
    pub fn admitted(
        consumer_id: impl Into<String>,
        delivery_cursor_seed: impl Into<String>,
    ) -> Self {
        let consumer_id = consumer_id.into();
        let delivery_cursor_seed = delivery_cursor_seed.into();
        let consumer_identity = ForgeQueryEvidenceIdentity::compose(
            ForgeQueryEvidenceScope::SubscriptionActivationReceipt,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "subscription_consumer_attachment_request_v1",
        )
        .field_value(ForgeQueryEvidenceTag::new("consumer"), consumer_id)
        .field_value(
            ForgeQueryEvidenceTag::new("cursor_seed"),
            &delivery_cursor_seed,
        )
        .seal();
        let delivery_cursor_seed_identity = delivery_cursor_seed_identity(&delivery_cursor_seed);
        Self {
            consumer_identity,
            delivery_cursor_seed_identity,
        }
    }

    pub fn from_consumer_identity(
        consumer_identity: ForgeQueryEvidenceIdentity,
        delivery_cursor_seed_identity: ForgeQueryEvidenceIdentity,
    ) -> Self {
        Self {
            consumer_identity,
            delivery_cursor_seed_identity,
        }
    }

    pub fn consumer_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.consumer_identity
    }

    pub fn delivery_cursor_seed_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.delivery_cursor_seed_identity
    }
}

fn delivery_cursor_seed_identity(seed: &str) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::SubscriptionActivationReceipt)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "subscription_delivery_cursor_seed_v1",
        )
        .field_value(ForgeQueryEvidenceTag::new("seed"), seed)
        .seal()
}
