use forge_query::facade::{
    lower_policy_aware_optimizer_input, PolicyAwareOptimizerInput, ValidatedQueryBundle,
};

fn expects_validated_bundle_optimizer(_: fn(&ValidatedQueryBundle) -> PolicyAwareOptimizerInput) {}

fn main() {
    expects_validated_bundle_optimizer(lower_policy_aware_optimizer_input);
}
