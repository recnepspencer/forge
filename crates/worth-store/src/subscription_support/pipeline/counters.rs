use super::super::SubscriptionSupportCounterSnapshot;
use super::SubscriptionSupportPublicationPipeline;

impl SubscriptionSupportPublicationPipeline {
    pub fn counters(&self) -> SubscriptionSupportCounterSnapshot {
        self.counters.clone()
    }
}
