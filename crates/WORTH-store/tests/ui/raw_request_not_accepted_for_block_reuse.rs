use worth_store::{
    AspectLayoutReadRequest, AspectLayoutTarget, AspectProjectionSet, AspectScopeClass,
    WORTHStoreBuilder, SingleEntityAspectScope,
};
use worth_relational::facade::history::{BranchId, CommitId};

fn main() {
    let store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let request = AspectLayoutReadRequest::new(
        AspectLayoutTarget::new(BranchId("main".to_string()), CommitId(1)),
        AspectScopeClass::SingleEntity(SingleEntityAspectScope::new("entity-a")),
        AspectProjectionSet::new(vec!["profile".to_string()]),
    );
    let _ = store.admit_structural_block_reuse(request);
}
