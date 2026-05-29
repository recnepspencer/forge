use std::collections::{BTreeMap, BTreeSet};

use forge_foundational::facade::FieldKey;

use crate::indexes::data::{DerivedIndexDefinition, DerivedIndexGeneration, DerivedIndexId};
use crate::logic::runtime::state::subsystems::RuntimeSubsystem;
use crate::storage::data::AuthoritativeFieldComparisonKey;

#[derive(Debug, Clone, Default)]
pub(crate) struct IndexingSubsystem {
    pub(crate) definitions: BTreeMap<DerivedIndexId, DerivedIndexDefinition>,
    pub(crate) generations: BTreeMap<DerivedIndexId, Vec<DerivedIndexGeneration>>,
    pub(crate) entity_unique_field_index: BTreeMap<
        FieldKey,
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
            entity_unique_field_index: BTreeMap::new(),
            next_index_id: 1,
            next_generation_id: 1,
        }
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
