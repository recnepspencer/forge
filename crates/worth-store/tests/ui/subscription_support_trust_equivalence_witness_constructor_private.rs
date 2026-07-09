#![allow(invalid_value)]

use worth_store::{
    SubscriptionSupportFamilyId, SubscriptionSupportOperationalBasis,
    SubscriptionSupportOperationalVerdict, SupportTrustEquivalenceWitness,
};

fn main() {
    let _ = SupportTrustEquivalenceWitness {
        source_basis: unsafe { std::mem::zeroed::<SubscriptionSupportOperationalBasis>() },
        target_family_id: unsafe { std::mem::zeroed::<SubscriptionSupportFamilyId>() },
        operational_verdict: SubscriptionSupportOperationalVerdict::ExactResumePreserved,
        equivalence_digest: "digest".to_string(),
    };
}
