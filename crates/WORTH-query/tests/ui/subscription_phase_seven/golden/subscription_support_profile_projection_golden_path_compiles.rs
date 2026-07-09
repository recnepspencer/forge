use worth_query::facade::QuerySubscriptionSupportProfile;

fn profile_projection_golden_path(profile: &QuerySubscriptionSupportProfile) {
    let _ = profile.source_projection().label();
    let _ = profile.profile_projection().label();
}

fn main() {}
