use worth_ui::facade::{
    WorthUiCandidateAuthoringLane, WorthUiFileRustReplacementParityCounters,
    WorthUiFileRustReplacementPipelineReport,
};

fn main() {
    let _report = WorthUiFileRustReplacementPipelineReport {
        authoring_lane: WorthUiCandidateAuthoringLane::FileAuthored,
        candidate_basis: uninitialized_field(),
        provenance_handle: uninitialized_field(),
        active_artifact_digest: 1,
        candidate_artifact_digest: 2,
        artifact_comparison_outcome: uninitialized_field(),
        candidate_plan_digest: 3,
        lane_support_digest: 4,
        plan_node_count: 5,
        swap_receipt: uninitialized_field(),
        counters: WorthUiFileRustReplacementParityCounters::default(),
    };
}

fn uninitialized_field<T>() -> T {
    unimplemented!()
}
