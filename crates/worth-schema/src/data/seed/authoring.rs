use crate::data::authority::{
    CreateKey, EntityReference, MutationOrigin, RawTopologyIntent, TopologyMutation,
};
use crate::data::entities::{EntityKind, NamingEntityKind};
use crate::data::relations::{NamingRelationKind, RelationKind};

#[derive(Debug, Default, Clone)]
pub struct TopologyCreateBatchBuilder {
    mutations: Vec<TopologyMutation>,
}

impl TopologyCreateBatchBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn topology_entity(mut self, create_key: impl Into<String>, kind: EntityKind) -> Self {
        self.push_topology_entity(create_key, kind);
        self
    }

    pub fn push_topology_entity(&mut self, create_key: impl Into<String>, kind: EntityKind) {
        self.mutations.push(TopologyMutation::CreateEntity {
            create_key: CreateKey::new(create_key.into()),
            kind,
        });
    }

    pub fn relation(
        mut self,
        create_key: impl Into<String>,
        kind: RelationKind,
        source: EntityReference,
        target: EntityReference,
    ) -> Self {
        self.push_relation(create_key, kind, source, target);
        self
    }

    pub fn push_relation(
        &mut self,
        create_key: impl Into<String>,
        kind: RelationKind,
        source: EntityReference,
        target: EntityReference,
    ) {
        self.mutations.push(TopologyMutation::CreateRelation {
            create_key: CreateKey::new(create_key.into()),
            kind,
            source,
            target,
        });
    }

    pub fn persistent_name_for(mut self, topology_key: impl Into<String>) -> Self {
        self.push_persistent_name_for(topology_key);
        self
    }

    pub fn push_persistent_name_for(&mut self, topology_key: impl Into<String>) {
        let topology_key = topology_key.into();
        let persistent_name_key = format!("{topology_key}.persistent_name");
        self.mutations.push(TopologyMutation::CreateEntity {
            create_key: CreateKey::new(persistent_name_key.clone()),
            kind: EntityKind::Naming(NamingEntityKind::PersistentName),
        });
        self.mutations.push(TopologyMutation::CreateRelation {
            create_key: CreateKey::new(format!("{persistent_name_key}.targets")),
            kind: RelationKind::Naming(NamingRelationKind::PersistentNameTargetsEntity),
            source: created_ref(persistent_name_key),
            target: created_ref(topology_key),
        });
    }

    pub fn finish(self, mutation_origin: MutationOrigin) -> RawTopologyIntent {
        RawTopologyIntent::new(self.mutations, mutation_origin)
    }
}

pub fn created_ref(create_key: impl Into<String>) -> EntityReference {
    EntityReference::Created(CreateKey::new(create_key.into()))
}
