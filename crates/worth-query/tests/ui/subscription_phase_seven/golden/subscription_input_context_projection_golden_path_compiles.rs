use worth_query::facade::runtime::LiveQueryAdmissionArtifact;

fn input_context_projection_golden_path(live: &LiveQueryAdmissionArtifact) {
    let _ = live.policy_projection().label();
    let _ = live.tenant_projection().label();
    let _ = live.relationship_proof_projection().label();
    let _ = live.collection_projection().label();
}

fn main() {}
