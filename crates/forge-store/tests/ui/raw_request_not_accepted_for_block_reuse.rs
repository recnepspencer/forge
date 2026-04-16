use forge_store::{
    AspectLayoutReadRequest, AspectLayoutTarget, AspectProjectionSet, AspectScopeClass,
    ForgeStoreBuilder, SingleEntityAspectScope,
};
use forge_relational::facade::history::{BranchId, CommitId};

fn main() {
    let store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let request = AspectLayoutReadRequest::new(
        AspectLayoutTarget::new(BranchId("main".to_string()), CommitId(1)),
        AspectScopeClass::SingleEntity(SingleEntityAspectScope::new("entity-a")),
        AspectProjectionSet::new(vec!["profile".to_string()]),
    );
    let _ = store.admit_structural_block_reuse(request);
}
