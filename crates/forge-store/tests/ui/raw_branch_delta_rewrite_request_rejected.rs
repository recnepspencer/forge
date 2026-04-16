use forge_relational::facade::history::{BranchId, CommitId};
use forge_store::{BranchDeltaRewriteRequest, ForgeStoreBuilder};

fn main() {
    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let request = BranchDeltaRewriteRequest::new(BranchId("feature".to_string()), CommitId(1));
    let _ = store.rewrite_branch_delta(request);
}
