use std::collections::BTreeMap;

use crate::storage::overlay::PartitionState;

use super::{
    DurabilitySubsystem, HistorySubsystem, IndexingSubsystem, LineageSubsystem,
    PublicationSubsystem, RuntimeServices, VisibilitySubsystem,
};
use crate::logic::runtime::RelationalRuntimeConfig;

#[derive(Debug)]
pub struct RelationalRuntime {
    pub(crate) config: RelationalRuntimeConfig,
    pub(crate) partitions: BTreeMap<crate::identity::data::PartitionId, PartitionState>,
    pub(crate) visibility: VisibilitySubsystem,
    pub(crate) publication: PublicationSubsystem,
    pub(crate) history: HistorySubsystem,
    pub(crate) indexes: IndexingSubsystem,
    pub(crate) lineage: LineageSubsystem,
    pub(crate) durability: DurabilitySubsystem,
    pub(crate) services: RuntimeServices,
}

impl RelationalRuntime {
    pub fn config(&self) -> &RelationalRuntimeConfig {
        &self.config
    }

    #[cfg(test)]
    pub(crate) fn entity_history_len_for_test(
        &self,
        entity_id: crate::identity::data::EntityId,
    ) -> usize {
        self.partitions
            .get(&entity_id.partition_id)
            .and_then(|partition| {
                partition
                    .entity_arena
                    .payload_history_at(entity_id.local_slot.0 as usize)
            })
            .map(|history| history.len())
            .unwrap_or(0)
    }

    #[cfg(test)]
    pub(crate) fn relation_history_len_for_test(
        &self,
        relation_id: crate::identity::data::RelationId,
    ) -> usize {
        self.partitions
            .get(&relation_id.partition_id)
            .and_then(|partition| {
                partition
                    .relation_arena
                    .payload_history_at(relation_id.local_slot.0 as usize)
            })
            .map(|history| history.len())
            .unwrap_or(0)
    }
}
