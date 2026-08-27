mod branch_authority;
mod core_access;
mod merge_authority;
mod publication_lifecycle;
mod publication_recovery;
#[cfg(test)]
mod test_support;

use std::collections::BTreeMap;

use crate::runtime::RelationalRuntimeConfig;
use crate::storage::overlay::PartitionState;

use super::{
    CommitStrategiesSubsystem, DurabilitySubsystem, HistorySubsystem, IndexingSubsystem,
    LineageSubsystem, PublicationSubsystem, RecordIdentitySubsystem, RuntimeServices,
    SchemaContractRuntimeSubsystem, VisibilitySubsystem,
};

pub(in crate::runtime) use publication_lifecycle::RelationalRuntimePublicationOwner;
pub(crate) use publication_lifecycle::{
    RelationalCandidateRegistrationDenial, RelationalRuntimePublicationBinding,
};

#[derive(Debug)]
pub struct RelationalRuntime {
    pub(crate) config: RelationalRuntimeConfig,
    pub(crate) schema_contract_runtime: SchemaContractRuntimeSubsystem,
    pub(crate) commit_strategies: CommitStrategiesSubsystem,
    pub(crate) partitions: BTreeMap<crate::identity::data::PartitionId, PartitionState>,
    pub(crate) visibility: VisibilitySubsystem,
    pub(crate) publication: PublicationSubsystem,
    pub(crate) history: HistorySubsystem,
    pub(crate) indexes: IndexingSubsystem,
    pub(crate) lineage: LineageSubsystem,
    pub(crate) durability: DurabilitySubsystem,
    pub(crate) record_identity: RecordIdentitySubsystem,
    pub(crate) services: RuntimeServices,
    pub(in crate::runtime) publication_owner: RelationalRuntimePublicationOwner,
}

impl Drop for RelationalRuntime {
    fn drop(&mut self) {
        self.publication_owner.close();
        crate::indexes::purge_index_query_scratch_hints(self.runtime_instance_id());
    }
}

impl RelationalRuntime {
    pub(crate) fn publication_binding(&self) -> RelationalRuntimePublicationBinding {
        self.publication_owner.binding()
    }
}
