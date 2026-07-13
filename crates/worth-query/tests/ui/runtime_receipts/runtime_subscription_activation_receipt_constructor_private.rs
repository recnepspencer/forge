use worth_query::facade::runtime::{SubscriptionActivationInput, SubscriptionActivationReceipt};

fn main() {
    let activation = None::<SubscriptionActivationInput>;
    let activation = activation.as_ref().expect("fixture never executes");
    let _ = SubscriptionActivationReceipt::from_activation(
        "tasks.table",
        activation,
        "external-support",
    );
}
