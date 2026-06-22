use forge_query::facade::{
    admit_policy_aware_live_plan, NarrowedPolicyQueryArtifact, PolicyDriftDisposition,
    PolicyLiveDensityPosture,
};

fn main() {
    let fields = vec!["profile.display_name".to_string()];
    let _ = admit_policy_aware_live_plan(
        narrowed_policy_query_artifact(),
        &fields,
        PolicyDriftDisposition::NoChange,
        PolicyLiveDensityPosture::SparseDelta,
    );
}

fn narrowed_policy_query_artifact() -> &'static NarrowedPolicyQueryArtifact {
    panic!("fixture only")
}
