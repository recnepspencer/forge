use worth_query::facade::{
    ActiveSubscriptionAllocationPolicy, ActiveSubscriptionWorkBudget,
};

fn main() {
    let _budget = ActiveSubscriptionWorkBudget::admitted(
        1,
        1,
        1,
        ActiveSubscriptionAllocationPolicy::LifecycleArena,
    );
}
