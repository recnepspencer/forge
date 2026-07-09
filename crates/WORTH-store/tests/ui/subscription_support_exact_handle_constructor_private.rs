use worth_store::{ExactSubscriptionResumeHandle, PublishedSubscriptionSupportArtifact};

fn main() {
    let published: PublishedSubscriptionSupportArtifact =
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
    let _ = ExactSubscriptionResumeHandle::new(&published);
}
