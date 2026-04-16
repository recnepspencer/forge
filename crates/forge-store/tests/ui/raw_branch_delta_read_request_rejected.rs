use forge_relational::facade::history::{BranchId, CommitId};
use forge_store::{BranchDeltaReadRequest, ForgeStoreBuilder};

fn main() {
    let store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let request = BranchDeltaReadRequest::new(BranchId("feature".to_string()), CommitId(1));
    let _ = store.read_branch_delta(request);
}
