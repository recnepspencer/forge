use worth_relational::facade::history::{BranchId, CommitId};
use worth_store::{BranchDeltaReadRequest, WORTHStoreBuilder};

fn main() {
    let store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let request = BranchDeltaReadRequest::new(BranchId("feature".to_string()), CommitId(1));
    let _ = store.read_branch_delta(request);
}
