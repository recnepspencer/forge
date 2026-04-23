use crate::identity::hash_parts;

use super::attachment_digest::SubscriptionConsumerAttachmentDigest;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct QueryDeliverySequence(u64);

impl QueryDeliverySequence {
    pub(super) fn initial() -> Self {
        Self(0)
    }

    pub(super) fn next(self) -> Self {
        Self(self.0 + 1)
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubscriptionAcknowledgementFrontier {
    attachment_digest: SubscriptionConsumerAttachmentDigest,
    acknowledged_sequence: QueryDeliverySequence,
    frontier_digest: String,
}

impl SubscriptionAcknowledgementFrontier {
    pub(super) fn initial(attachment_digest: SubscriptionConsumerAttachmentDigest) -> Self {
        Self::new(attachment_digest, QueryDeliverySequence::initial())
    }

    pub(super) fn advance(&self, sequence: QueryDeliverySequence) -> Self {
        Self::new(self.attachment_digest.clone(), sequence)
    }

    fn new(
        attachment_digest: SubscriptionConsumerAttachmentDigest,
        acknowledged_sequence: QueryDeliverySequence,
    ) -> Self {
        let frontier_digest = hash_parts(&[
            "subscription_acknowledgement_frontier_v1".to_string(),
            format!("attachment:{}", attachment_digest.as_str()),
            format!("sequence:{}", acknowledged_sequence.get()),
        ]);
        Self {
            attachment_digest,
            acknowledged_sequence,
            frontier_digest,
        }
    }

    pub fn attachment_digest(&self) -> &SubscriptionConsumerAttachmentDigest {
        &self.attachment_digest
    }

    pub fn acknowledged_sequence(&self) -> QueryDeliverySequence {
        self.acknowledged_sequence
    }

    pub fn frontier_digest(&self) -> &str {
        &self.frontier_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryDeliveryBatchReceipt {
    attachment_digest: SubscriptionConsumerAttachmentDigest,
    sequence: QueryDeliverySequence,
    receipt_digest: String,
}

impl QueryDeliveryBatchReceipt {
    pub(super) fn new(
        attachment_digest: SubscriptionConsumerAttachmentDigest,
        sequence: QueryDeliverySequence,
    ) -> Self {
        let receipt_digest = hash_parts(&[
            "query_delivery_batch_receipt_v1".to_string(),
            format!("attachment:{}", attachment_digest.as_str()),
            format!("sequence:{}", sequence.get()),
        ]);
        Self {
            attachment_digest,
            sequence,
            receipt_digest,
        }
    }

    pub fn attachment_digest(&self) -> &SubscriptionConsumerAttachmentDigest {
        &self.attachment_digest
    }

    pub fn sequence(&self) -> QueryDeliverySequence {
        self.sequence
    }

    pub fn receipt_digest(&self) -> &str {
        &self.receipt_digest
    }
}
