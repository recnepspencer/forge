use worth_query::facade::{
    DeliveryBackpressurePolicy, SubscriptionConsumerAttachmentBudget,
};

fn main() {
    let _budget = SubscriptionConsumerAttachmentBudget::admitted(
        1,
        1,
        1,
        DeliveryBackpressurePolicy::RetainWithinWindow,
    );
}
