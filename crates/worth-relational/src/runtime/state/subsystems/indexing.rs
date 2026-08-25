use std::collections::{BTreeMap, BTreeSet};

use worth_foundational::facade::AspectFieldLocator;

use crate::history::data::CommitId;
use crate::indexes::data::{
    DerivedIndexArtifacts, DerivedIndexDefinition, DerivedIndexGeneration, DerivedIndexId,
};
use crate::runtime::state::subsystems::RuntimeSubsystem;
use crate::storage::data::AuthoritativeFieldComparisonKey;

#[derive(Debug, Clone, Default)]
pub(crate) struct IndexingSubsystem {
    pub(crate) definitions: BTreeMap<DerivedIndexId, DerivedIndexDefinition>,
    pub(crate) generations: BTreeMap<DerivedIndexId, Vec<DerivedIndexGeneration>>,
    pub(crate) entity_unique_aspect_field_index: BTreeMap<
        AspectFieldLocator,
        BTreeMap<AuthoritativeFieldComparisonKey, BTreeSet<crate::identity::data::EntityId>>,
    >,
    pub(crate) next_index_id: u64,
    pub(crate) next_generation_id: u64,
}

impl IndexingSubsystem {
    fn empty() -> Self {
        Self {
            definitions: BTreeMap::new(),
            generations: BTreeMap::new(),
            entity_unique_aspect_field_index: BTreeMap::new(),
            next_index_id: 1,
            next_generation_id: 1,
        }
    }

    pub(crate) fn derived_artifacts_for_commit(
        &self,
        commit_id: CommitId,
    ) -> DerivedIndexArtifacts {
        DerivedIndexArtifacts::new(
            self.generations
                .values()
                .flat_map(|generations| generations.iter())
                .filter(|generation| generation.source_commit_id == commit_id)
                .cloned()
                .collect(),
        )
    }
}

impl RuntimeSubsystem for IndexingSubsystem {
    type Config = ();

    fn new(_: &Self::Config) -> Self {
        Self::empty()
    }

    fn fork(&self) -> Self {
        self.clone()
    }
}
