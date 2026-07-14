use worth_query::facade::policy::{lower_policy_aware_optimizer_input, PolicyAwareOptimizerInput};
use worth_query::facade::runtime::ValidatedQueryBundle;

fn expects_validated_bundle_optimizer(_: fn(&ValidatedQueryBundle) -> PolicyAwareOptimizerInput) {}

fn main() {
    expects_validated_bundle_optimizer(lower_policy_aware_optimizer_input);
}
