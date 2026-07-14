use worth_query::facade::foundation::CanonicalQueryArtifact;
use worth_query::facade::policy::{lower_policy_aware_current_plan, PolicyAwareCurrentPlan};

fn expects_raw_query_lowerer(_: fn(&CanonicalQueryArtifact) -> PolicyAwareCurrentPlan) {}

fn main() {
    expects_raw_query_lowerer(lower_policy_aware_current_plan);
}
