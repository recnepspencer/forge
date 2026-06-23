use worth_ui::facade::{
    WorthUiCandidateRuntimeAuthoringSnapshot, WorthUiPageHostPlan, WorthUiPageHostRequest,
};

fn main() {}

fn plan_from_candidate(candidate: &WorthUiCandidateRuntimeAuthoringSnapshot) {
    let _plan = WorthUiPageHostPlan::from_active_authoring(
        candidate,
        candidate.witness(),
        WorthUiPageHostRequest::new("ProductsPage"),
    );
}
