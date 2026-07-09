use crate::facade::WORTHStoreBuilder;
use worth_relational::facade::runtime::RelationalRuntime;

use super::HostedRuntimeOwnershipProof;

#[derive(Debug)]
pub(crate) struct DurableModeConstructionPlan {
    store_builder: WORTHStoreBuilder,
    ownership: HostedRuntimeOwnershipProof,
}

impl DurableModeConstructionPlan {
    pub(crate) fn new(store_builder: WORTHStoreBuilder, runtime: RelationalRuntime) -> Self {
        Self {
            store_builder,
            ownership: HostedRuntimeOwnershipProof::verify(runtime),
        }
    }

    pub(crate) fn into_parts(self) -> (WORTHStoreBuilder, HostedRuntimeOwnershipProof) {
        (self.store_builder, self.ownership)
    }
}
