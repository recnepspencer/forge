use worth_query::facade::{advance_subscription_acknowledgement, ActiveSubscriptionRuntime};

fn main() {
    let mut runtime = ActiveSubscriptionRuntime::new();
    let _ = advance_subscription_acknowledgement(&mut runtime, todo!(), "no-receipt");
}
