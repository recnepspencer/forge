use forge_store::{
    PublishedSubscriptionSupportArtifact, SubscriptionSupportFamilyKind,
    SubscriptionSupportRebuildPlanHandle,
};

fn main() {
    let published: PublishedSubscriptionSupportArtifact =
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
    let _ = SubscriptionSupportRebuildPlanHandle::new(
        &published,
        "basis:retained",
        vec![SubscriptionSupportFamilyKind::BasisBoundContinuationSupport],
    );
}
