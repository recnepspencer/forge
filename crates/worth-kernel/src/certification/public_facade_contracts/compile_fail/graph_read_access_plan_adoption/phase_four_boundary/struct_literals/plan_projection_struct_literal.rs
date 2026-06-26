use worth_kernel::graph_read_access_plan_adoption::WorthGraphReadAccessSlicePlanProjection;

fn main() {
    let _ = WorthGraphReadAccessSlicePlanProjection {
        selected_slice_digest: String::new(),
        status: unimplemented!(),
        query_family_name: None,
        query_family_digest_seed: String::new(),
        query_posture: String::new(),
        executed_read_family_digest: None,
        query_requirement_set_digest: None,
        admitted_plan_digest: None,
        query_admission_digest: None,
        execution_strategy: None,
        required_worth_artifact: None,
        blocker: None,
        projection_digest: String::new(),
    };
}
