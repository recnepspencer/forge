use crate::subscription_support::{
    SubscriptionResumeClassification, SubscriptionSupportOperationalVerdict,
};

pub(super) fn operational_verdict_classification(
    verdict: SubscriptionSupportOperationalVerdict,
) -> Option<SubscriptionResumeClassification> {
    match verdict {
        SubscriptionSupportOperationalVerdict::ExactResumePreserved => {
            Some(SubscriptionResumeClassification::Exact)
        }
        SubscriptionSupportOperationalVerdict::DegradedResumePreserved => {
            Some(SubscriptionResumeClassification::Degraded)
        }
        SubscriptionSupportOperationalVerdict::RebuildRequired => {
            Some(SubscriptionResumeClassification::RebuildRequired)
        }
        SubscriptionSupportOperationalVerdict::NotResumable => {
            Some(SubscriptionResumeClassification::NotResumable)
        }
        SubscriptionSupportOperationalVerdict::RejectedByPolicy => None,
    }
}
