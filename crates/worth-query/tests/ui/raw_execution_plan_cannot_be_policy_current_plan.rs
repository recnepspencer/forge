use worth_query::facade::policy::{ExecutionPlanBundle, PolicyAwareCurrentPlan};

fn accepts_policy_current(_: PolicyAwareCurrentPlan) {}

fn expects_raw_execution_consumer(_: fn(ExecutionPlanBundle)) {}

fn main() {
    expects_raw_execution_consumer(accepts_policy_current);
}
