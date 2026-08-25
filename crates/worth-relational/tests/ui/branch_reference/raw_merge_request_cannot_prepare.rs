use worth_relational::facade::merge::{MergeExecutionRequest, MergeIntent};
use worth_relational::facade::runtime::RelationalRuntime;
use worth_relational::facade::history::BranchId;

pub fn attempt(runtime: &RelationalRuntime) {
    let request = MergeExecutionRequest::new(
        BranchId("main".to_owned()),
        BranchId("feature".to_owned()),
        MergeIntent::ReconcileIntoTarget,
    );
    let _ = runtime.prepare_merge_execution(request);
}

fn main() {}
