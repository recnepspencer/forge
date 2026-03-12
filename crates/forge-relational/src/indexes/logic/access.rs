use crate::history::data::BranchId;
use crate::indexes::data::{
    DerivedIndexDefinition, DerivedIndexGeneration, DerivedIndexId,
};
use crate::logic::runtime::{IndexedReadOutcome, RelationalRuntime};
use crate::query::data::QueryWorkPacket;
use crate::snapshots::data::SnapshotHandle;

pub struct IndexAccess<'runtime> {
    runtime: &'runtime RelationalRuntime,
}

impl RelationalRuntime {
    pub fn index_access(&self) -> IndexAccess<'_> {
        IndexAccess::new(self)
    }
}

impl<'runtime> IndexAccess<'runtime> {
    pub(crate) fn new(runtime: &'runtime RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub fn latest_generation(
        &self,
        index_id: DerivedIndexId,
        branch_id: &BranchId,
    ) -> Option<&DerivedIndexGeneration> {
        let definition = self.runtime.indexes.definitions.get(&index_id)?;
        self.runtime.indexes.generations.get(&index_id).and_then(|generations| {
            generations.iter().rev().find(|generation| {
                !definition.branch_scoped || generation.compatibility.branch_id == *branch_id
            })
        })
    }

    pub fn generations_for_version(
        &self,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<DerivedIndexGeneration> {
        let mut generations = self
            .runtime
            .indexes
            .generations
            .values()
            .flat_map(|generations| generations.iter())
            .filter(|generation| generation.compatibility.version_id <= version_id)
            .cloned()
            .collect::<Vec<_>>();
        generations.sort_by_key(|generation| {
            (
                generation.compatibility.branch_id.clone(),
                generation.source_commit_id,
                generation.generation_id,
            )
        });
        generations
    }

    pub fn read_with_storage_fallback(
        &self,
        handle: &SnapshotHandle,
        packet: &QueryWorkPacket,
    ) -> Option<IndexedReadOutcome> {
        let result = self.runtime.visibility_reads().execute_read_packet(handle, packet)?;
        let branch_id = self
            .branch_id_for_version(handle.version_id)
            .unwrap_or_else(|| self.runtime.config.history.main_branch.clone());
        let used_index_generation = self
            .compatible_generations_for_version(&branch_id, handle.version_id)
            .into_iter()
            .max_by_key(|generation| generation.generation_id)
            .map(|generation| generation.generation_id);
        Some(IndexedReadOutcome {
            result,
            used_index_generation,
        })
    }

    pub(crate) fn definitions_snapshot(&self) -> Vec<DerivedIndexDefinition> {
        self.runtime.indexes.definitions.values().cloned().collect()
    }

    pub(crate) fn generations_snapshot(&self) -> Vec<DerivedIndexGeneration> {
        self.runtime
            .indexes
            .generations
            .values()
            .flat_map(|generations| generations.iter().cloned())
            .collect()
    }

    pub(crate) fn entity_unique_field_ids(
        &self,
        field: &str,
        value: &str,
    ) -> Option<&std::collections::BTreeSet<crate::identity::data::EntityId>> {
        self.runtime
            .indexes
            .entity_unique_field_index
            .get(field)
            .and_then(|values| values.get(value))
    }

    fn branch_id_for_version(
        &self,
        version_id: crate::identity::data::VersionId,
    ) -> Option<BranchId> {
        self.runtime
            .history
            .commit_graph
            .values()
            .find(|node| node.commit.version_id == version_id)
            .map(|node| node.commit.branch_id.clone())
    }

    fn compatible_generations_for_version(
        &self,
        branch_id: &BranchId,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<&DerivedIndexGeneration> {
        self.runtime
            .indexes
            .generations
            .values()
            .flat_map(|generations| generations.iter())
            .filter(|generation| {
                generation.compatibility.version_id <= version_id
                    && self
                        .runtime
                        .indexes
                        .definitions
                        .get(&generation.index_id)
                        .is_some_and(|definition| {
                            !definition.branch_scoped
                                || generation.compatibility.branch_id == *branch_id
                        })
            })
            .collect()
    }
}
