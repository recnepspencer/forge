use worth_store::{ExactResumePreservationWitness, SubscriptionSupportOperationalBasis};

fn main() {
    let basis: SubscriptionSupportOperationalBasis =
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
    let _ = ExactResumePreservationWitness::new(basis);
}
