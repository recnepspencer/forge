use worth_query::facade::{open_active_subscription_lane, ActiveSubscriptionRuntime};

fn main() {
    let mut runtime = ActiveSubscriptionRuntime::new();
    let _ = open_active_subscription_lane(&mut runtime, "raw-bridge-declaration");
}
