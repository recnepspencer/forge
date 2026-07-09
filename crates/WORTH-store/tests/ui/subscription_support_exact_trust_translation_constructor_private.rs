#![allow(invalid_value)]

use worth_store::{
    SubscriptionResumeClassification, SubscriptionSupportOperationalBasis,
    SubscriptionSupportOperationalVerdict, SupportExactTrustTranslation,
};

fn main() {
    let _ = SupportExactTrustTranslation {
        basis: unsafe { std::mem::zeroed::<SubscriptionSupportOperationalBasis>() },
        resume_classification: SubscriptionResumeClassification::Exact,
        operational_verdict: SubscriptionSupportOperationalVerdict::ExactResumePreserved,
    };
}
