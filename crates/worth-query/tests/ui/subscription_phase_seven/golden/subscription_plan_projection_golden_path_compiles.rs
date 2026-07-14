use worth_query::facade::runtime::LiveQueryAdmissionArtifact;

fn plan_projection_golden_path(live: &LiveQueryAdmissionArtifact) {
    let _ = live.plan_projection().label();
}

fn main() {}
