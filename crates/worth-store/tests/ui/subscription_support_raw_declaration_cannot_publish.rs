use worth_store::{
    RawSubscriptionSupportDeclaration, SubscriptionSupportPublicationPipeline,
};

fn main() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let raw: RawSubscriptionSupportDeclaration =
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
    let _ = pipeline.publish(raw);
}
