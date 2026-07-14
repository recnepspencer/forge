use worth_query::facade::runtime::{attach_subscription_consumer, ActiveSubscriptionRuntime};

struct GenericHandle;

fn main() {
    let mut runtime = ActiveSubscriptionRuntime::new();
    let handle = GenericHandle;
    let _ = attach_subscription_consumer(&mut runtime, &handle, todo!(), todo!());
}
