use worth_relational::facade::history::{BranchId, CommitId};
use worth_store::{BranchDeltaRewriteRequest, WORTHStoreBuilder};

fn main() {
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let request = BranchDeltaRewriteRequest::new(BranchId("feature".to_string()), CommitId(1));
    let _ = store.rewrite_branch_delta(request);
}
