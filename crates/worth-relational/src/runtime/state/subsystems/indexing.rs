use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use worth_foundational::facade::AspectFieldLocator;

use crate::history::data::CommitId;
use crate::indexes::data::{
    DerivedIndexArtifacts, DerivedIndexDefinition, DerivedIndexGeneration, DerivedIndexId,
};
use crate::runtime::state::subsystems::{RuntimeOwnedState, RuntimeSubsystem};
use crate::storage::data::AuthoritativeFieldComparisonKey;

/// Every entity that currently carries a tracked unique aspect field value.
pub(crate) type UniqueEntityAspectFieldIndex = BTreeMap<
    AspectFieldLocator,
    BTreeMap<AuthoritativeFieldComparisonKey, BTreeSet<crate::identity::data::EntityId>>,
>;

/// The derived-index subsystem's authoritative contents.
///
/// Definitions and generations are held behind `Arc` so a reader can carry one
/// out of the subsystem lock without copying its entries and without retaining
/// the guard.
#[derive(Debug, Clone, Default)]
pub(crate) struct IndexingState {
    pub(crate) definitions: BTreeMap<DerivedIndexId, Arc<DerivedIndexDefinition>>,
    pub(crate) generations: BTreeMap<DerivedIndexId, Vec<Arc<DerivedIndexGeneration>>>,
    pub(crate) entity_unique_aspect_field_index: UniqueEntityAspectFieldIndex,
    pub(crate) next_index_id: u64,
    pub(crate) next_generation_id: u64,
}

impl IndexingState {
    fn empty() -> Self {
        Self {
            definitions: BTreeMap::new(),
            generations: BTreeMap::new(),
            entity_unique_aspect_field_index: BTreeMap::new(),
            next_index_id: 1,
            next_generation_id: 1,
        }
    }

    pub(crate) fn insert_definition(&mut self, definition: DerivedIndexDefinition) {
        self.definitions
            .insert(definition.index_id, Arc::new(definition));
    }
}

#[derive(Debug, Default)]
pub(crate) struct IndexingSubsystem {
    state: RuntimeOwnedState<IndexingState>,
}

impl IndexingSubsystem {
    /// Replace the whole subsystem, for checkpoint restore.
    pub(crate) fn install(&self, state: IndexingState) {
        *self.state.write() = state;
    }

    pub(crate) fn snapshot(&self) -> IndexingState {
        self.state.read().clone()
    }

    pub(crate) fn definition(
        &self,
        index_id: DerivedIndexId,
    ) -> Option<Arc<DerivedIndexDefinition>> {
        self.state.read().definitions.get(&index_id).map(Arc::clone)
    }

    pub(crate) fn definitions(&self) -> Vec<Arc<DerivedIndexDefinition>> {
        self.state
            .read()
            .definitions
            .values()
            .map(Arc::clone)
            .collect()
    }

    pub(crate) fn generations_for(
        &self,
        index_id: DerivedIndexId,
    ) -> Vec<Arc<DerivedIndexGeneration>> {
        self.state
            .read()
            .generations
            .get(&index_id)
            .map(|generations| generations.iter().map(Arc::clone).collect())
            .unwrap_or_default()
    }

    pub(crate) fn all_generations(&self) -> Vec<Arc<DerivedIndexGeneration>> {
        self.state
            .read()
            .generations
            .values()
            .flat_map(|generations| generations.iter().map(Arc::clone))
            .collect()
    }

    /// Allocate the next definition identity and record the definition.
    pub(crate) fn register_definition(
        &self,
        mut definition: DerivedIndexDefinition,
    ) -> DerivedIndexDefinition {
        let mut state = self.state.write();
        definition.index_id = DerivedIndexId(state.next_index_id);
        state.next_index_id += 1;
        state.insert_definition(definition.clone());
        definition
    }

    /// Reserve the next generation identity without publishing anything.
    pub(crate) fn next_generation_id(&self) -> u64 {
        let mut state = self.state.write();
        let generation_id = state.next_generation_id;
        state.next_generation_id += 1;
        generation_id
    }

    pub(crate) fn publish_generation(&self, generation: DerivedIndexGeneration) {
        self.state
            .write()
            .generations
            .entry(generation.index_id)
            .or_default()
            .push(Arc::new(generation));
    }

    /// Install a generation carried by a canonical envelope, replacing any
    /// earlier copy of the same generation identity.
    pub(crate) fn restore_generation(&self, generation: DerivedIndexGeneration) {
        let mut state = self.state.write();
        let generations = state.generations.entry(generation.index_id).or_default();
        if let Some(existing) = generations
            .iter_mut()
            .find(|candidate| candidate.generation_id == generation.generation_id)
        {
            *existing = Arc::new(generation);
        } else {
            generations.push(Arc::new(generation));
            generations.sort_by_key(|candidate| candidate.generation_id);
        }
    }

    pub(crate) fn derived_artifacts_for_commit(
        &self,
        commit_id: CommitId,
    ) -> DerivedIndexArtifacts {
        DerivedIndexArtifacts::new(
            self.state
                .read()
                .generations
                .values()
                .flat_map(|generations| generations.iter())
                .filter(|generation| generation.source_commit_id == commit_id)
                .map(|generation| generation.as_ref().clone())
                .collect(),
        )
    }

    /// Read the unique aspect field index without letting the guard escape.
    pub(crate) fn with_unique_index<R>(
        &self,
        read: impl FnOnce(&UniqueEntityAspectFieldIndex) -> R,
    ) -> R {
        read(&self.state.read().entity_unique_aspect_field_index)
    }

    pub(crate) fn with_unique_index_mut<R>(
        &self,
        write: impl FnOnce(&mut UniqueEntityAspectFieldIndex) -> R,
    ) -> R {
        write(&mut self.state.write().entity_unique_aspect_field_index)
    }

    /// Adversarial courts corrupt the newest generation of an index in place to
    /// prove that index-backed reads still deny rather than answer from it.
    #[cfg(test)]
    pub(crate) fn corrupt_latest_generation(
        &self,
        index_id: DerivedIndexId,
        corrupt: impl FnOnce(&mut DerivedIndexGeneration),
    ) {
        let mut state = self.state.write();
        let generation = state
            .generations
            .get_mut(&index_id)
            .and_then(|generations| generations.last_mut())
            .expect("court installs the generation it corrupts");
        corrupt(Arc::make_mut(generation));
    }

    pub(crate) fn clear_unique_index(&self) {
        self.state.write().entity_unique_aspect_field_index.clear();
    }
}

impl RuntimeSubsystem for IndexingSubsystem {
    type Config = ();

    fn new(_: &Self::Config) -> Self {
        Self {
            state: RuntimeOwnedState::new(IndexingState::empty()),
        }
    }

    fn fork(&self) -> Self {
        Self {
            state: RuntimeOwnedState::new(self.snapshot()),
        }
    }
}
