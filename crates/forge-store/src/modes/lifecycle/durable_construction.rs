use crate::facade::ForgeStoreBuilder;
use forge_relational::facade::runtime::RelationalRuntime;

use super::HostedRuntimeOwnershipProof;

#[derive(Debug)]
pub(crate) struct DurableModeConstructionPlan {
    store_builder: ForgeStoreBuilder,
    ownership: HostedRuntimeOwnershipProof,
}

impl DurableModeConstructionPlan {
    pub(crate) fn new(store_builder: ForgeStoreBuilder, runtime: RelationalRuntime) -> Self {
        Self {
            store_builder,
            ownership: HostedRuntimeOwnershipProof::verify(runtime),
        }
    }

    pub(crate) fn into_parts(self) -> (ForgeStoreBuilder, HostedRuntimeOwnershipProof) {
        (self.store_builder, self.ownership)
    }
}
