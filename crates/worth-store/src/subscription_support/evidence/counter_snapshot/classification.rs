use super::SubscriptionSupportCounterSnapshot;
use crate::subscription_support::SubscriptionResumeClassification;

impl SubscriptionSupportCounterSnapshot {
    pub(crate) fn record_classification(
        &mut self,
        classification: SubscriptionResumeClassification,
    ) {
        match classification {
            SubscriptionResumeClassification::Exact => self.exact_classifications += 1,
            SubscriptionResumeClassification::Degraded => self.degraded_classifications += 1,
            SubscriptionResumeClassification::RebuildRequired => {
                self.rebuild_required_classifications += 1;
            }
            SubscriptionResumeClassification::NotResumable => self.denied_classifications += 1,
        }
    }
}
