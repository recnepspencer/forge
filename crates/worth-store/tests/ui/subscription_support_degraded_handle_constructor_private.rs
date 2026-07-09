use worth_store::{
    DegradedSubscriptionResumeHandle, PublishedSubscriptionSupportArtifact,
    SubscriptionSupportDriftCause,
};

fn main() {
    let published: PublishedSubscriptionSupportArtifact =
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
    let _ = DegradedSubscriptionResumeHandle::new(
        &published,
        SubscriptionSupportDriftCause::SubscriptionSupportBasisDrift,
    );
}
