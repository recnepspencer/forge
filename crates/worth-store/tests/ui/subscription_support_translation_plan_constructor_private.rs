use worth_store::{
    ResumeClassificationTranslationPlan, SubscriptionSupportOperationalBasis,
    SubscriptionSupportOperationalVerdict,
};

fn attempt(basis: SubscriptionSupportOperationalBasis) {
    let _ = ResumeClassificationTranslationPlan::from_operational_verdict(
        SubscriptionSupportOperationalVerdict::ExactResumePreserved,
        basis,
        None,
        None,
    );
}

fn main() {}
