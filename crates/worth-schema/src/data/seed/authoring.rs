use crate::data::authority::{
    RawWorthTopologyIntent, WorthCreateKey, WorthEntityReference, WorthMutationOrigin,
    WorthTopologyMutation,
};
use crate::data::entities::{WorthEntityKind, WorthNamingEntityKind};
use crate::data::relations::{WorthNamingRelationKind, WorthRelationKind};

#[derive(Debug, Default, Clone)]
pub struct WorthTopologyCreateBatchBuilder {
    mutations: Vec<WorthTopologyMutation>,
}

impl WorthTopologyCreateBatchBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn topology_entity(
        mut self,
        create_key: impl Into<String>,
        kind: WorthEntityKind,
    ) -> Self {
        self.push_topology_entity(create_key, kind);
        self
    }

    pub fn push_topology_entity(
        &mut self,
        create_key: impl Into<String>,
        kind: WorthEntityKind,
    ) {
        self.mutations.push(WorthTopologyMutation::CreateEntity {
            create_key: WorthCreateKey::new(create_key.into()),
            kind,
        });
    }

    pub fn relation(
        mut self,
        create_key: impl Into<String>,
        kind: WorthRelationKind,
        source: WorthEntityReference,
        target: WorthEntityReference,
    ) -> Self {
        self.push_relation(create_key, kind, source, target);
        self
    }

    pub fn push_relation(
        &mut self,
        create_key: impl Into<String>,
        kind: WorthRelationKind,
        source: WorthEntityReference,
        target: WorthEntityReference,
    ) {
        self.mutations.push(WorthTopologyMutation::CreateRelation {
            create_key: WorthCreateKey::new(create_key.into()),
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
        self.mutations.push(WorthTopologyMutation::CreateEntity {
            create_key: WorthCreateKey::new(persistent_name_key.clone()),
            kind: WorthEntityKind::Naming(WorthNamingEntityKind::PersistentName),
        });
        self.mutations.push(WorthTopologyMutation::CreateRelation {
            create_key: WorthCreateKey::new(format!("{persistent_name_key}.targets")),
            kind: WorthRelationKind::Naming(WorthNamingRelationKind::PersistentNameTargetsEntity),
            source: created_ref(persistent_name_key),
            target: created_ref(topology_key),
        });
    }

    pub fn finish(self, mutation_origin: WorthMutationOrigin) -> RawWorthTopologyIntent {
        RawWorthTopologyIntent::new(self.mutations, mutation_origin)
    }
}

pub fn created_ref(create_key: impl Into<String>) -> WorthEntityReference {
    WorthEntityReference::Created(WorthCreateKey::new(create_key.into()))
}
