mod core_access;
#[cfg(test)]
mod test_support;

use std::collections::BTreeMap;

use crate::logic::runtime::RelationalRuntimeConfig;
use crate::storage::overlay::PartitionState;

use super::{
    AspectSemanticsSubsystem, CommitStrategiesSubsystem, DurabilitySubsystem, HistorySubsystem,
    IndexingSubsystem, LineageSubsystem, PublicationSubsystem, RuntimeServices,
    VisibilitySubsystem,
};

#[derive(Debug)]
pub struct RelationalRuntime {
    pub(crate) config: RelationalRuntimeConfig,
    pub(crate) aspect_semantics: AspectSemanticsSubsystem,
    pub(crate) commit_strategies: CommitStrategiesSubsystem,
    pub(crate) partitions: BTreeMap<crate::identity::data::PartitionId, PartitionState>,
    pub(crate) visibility: VisibilitySubsystem,
    pub(crate) publication: PublicationSubsystem,
    pub(crate) history: HistorySubsystem,
    pub(crate) indexes: IndexingSubsystem,
    pub(crate) lineage: LineageSubsystem,
    pub(crate) durability: DurabilitySubsystem,
    pub(crate) services: RuntimeServices,
}

impl Drop for RelationalRuntime {
    fn drop(&mut self) {
        crate::indexes::logic::purge_index_query_scratch_hints(self.runtime_instance_id());
    }
}
