use worth_query::facade::{admit_active_subscription_lane, ActiveSubscriptionWorkBudget};

fn main() {
    let _ = admit_active_subscription_lane("raw-activation", todo::<ActiveSubscriptionWorkBudget>());
}

fn todo<T>() -> T {
    unimplemented!()
}
