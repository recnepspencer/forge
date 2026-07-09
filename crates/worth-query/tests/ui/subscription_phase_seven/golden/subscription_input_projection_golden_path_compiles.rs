use worth_query::facade::{LiveQueryAdmissionArtifact, QuerySubscriptionSupportProfile};

fn input_projection_golden_path(live: &LiveQueryAdmissionArtifact) {
    let _ = live.query_projection().label();
}

fn support_profile_projection_golden_path(profile: &QuerySubscriptionSupportProfile) {
    let _ = profile.profile_projection().label();
}

fn main() {}
