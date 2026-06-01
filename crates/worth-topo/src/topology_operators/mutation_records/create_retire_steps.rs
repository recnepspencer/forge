use std::collections::BTreeSet;

use forge_relational::facade::identity::EntityId;
use schema::facade::platform::aspects::{Aspect, DiagnosticsAspect, NamingAspect, TopologyAspect};
use schema::facade::platform::authority::{CreateKey, EntityReference, TopologyMutation};
use schema::facade::platform::entities::TopologyEntityKind;
use schema::facade::platform::relations::RelationKind;

use crate::topology_operators::declared_mutation_sequence_builder::TopologyDeclaredMutationSequenceBuilder;

use super::records::TopologyDeclaredMutationAction;
use super::{
    TopologyDeclaredMutationRecord, TopologyDerivedRegion, TopologyMutationChangedScope,
    TopologyMutationDerivedFallbackPolicy, TopologyMutationFamily, TopologyMutationNamingScope,
};

impl TopologyDeclaredMutationSequenceBuilder {
    pub(crate) fn create_topology_entity(
        &mut self,
        create_key: impl Into<String>,
        kind: TopologyEntityKind,
    ) -> &mut Self {
        let create_key = CreateKey::new(create_key.into());
        let persistent_name_key =
            CreateKey::new(format!("{}.persistent_name", create_key.as_str()));
        let persistent_name_relation_key =
            CreateKey::new(format!("{}.targets", persistent_name_key.as_str()));
        let touched_aspects = BTreeSet::from([
            Aspect::Topology(TopologyAspect::Structure),
            Aspect::Naming(NamingAspect::PersistentName),
            Aspect::Diagnostics(DiagnosticsAspect::Decisions),
        ]);
        let lowered_mutations = vec![
            TopologyMutation::CreateEntity {
                create_key: create_key.clone(),
                kind: schema::facade::platform::entities::EntityKind::Topology(kind),
            },
            TopologyMutation::CreateEntity {
                create_key: persistent_name_key.clone(),
                kind: schema::facade::platform::entities::EntityKind::Naming(
                    schema::facade::platform::entities::NamingEntityKind::PersistentName,
                ),
            },
            TopologyMutation::CreateRelation {
                create_key: persistent_name_relation_key.clone(),
                kind: RelationKind::Naming(
                    schema::facade::platform::relations::NamingRelationKind::PersistentNameTargetsEntity,
                ),
                source: EntityReference::Created(persistent_name_key.clone()),
                target: EntityReference::Created(create_key.clone()),
            },
        ];
        self.records.push(TopologyDeclaredMutationRecord {
            family: TopologyMutationFamily::CreateTopologyEntity,
            action: TopologyDeclaredMutationAction::CreateTopologyEntity {
                create_key,
                kind,
                persistent_name_key,
                persistent_name_relation_key,
            },
            touched_aspects,
            changed_scopes: vec![
                TopologyMutationChangedScope::Entity,
                TopologyMutationChangedScope::Naming,
            ],
            naming_scopes: vec![TopologyMutationNamingScope::EditedEntityNames],
            derived_regions: vec![
                TopologyDerivedRegion::MutationLocalNeighborhoodRegion,
                TopologyDerivedRegion::NamingContinuityRegion,
            ],
            derived_fallback_policy: TopologyMutationDerivedFallbackPolicy::AllowExplicitFallback,
            lowered_mutations,
        });
        self
    }

    pub(crate) fn retire_topology_entity(
        &mut self,
        entity_id: EntityId,
        kind: TopologyEntityKind,
    ) -> &mut Self {
        let touched_aspects = BTreeSet::from([
            Aspect::Topology(TopologyAspect::Structure),
            Aspect::Diagnostics(DiagnosticsAspect::Decisions),
        ]);
        self.records.push(TopologyDeclaredMutationRecord {
            family: TopologyMutationFamily::RetireTopologyEntity,
            action: TopologyDeclaredMutationAction::RetireTopologyEntity { entity_id, kind },
            touched_aspects,
            changed_scopes: vec![
                TopologyMutationChangedScope::Entity,
                TopologyMutationChangedScope::LocalNeighborhood,
            ],
            naming_scopes: vec![TopologyMutationNamingScope::EditedEntityNames],
            derived_regions: vec![
                TopologyDerivedRegion::MutationLocalNeighborhoodRegion,
                TopologyDerivedRegion::NamingContinuityRegion,
            ],
            derived_fallback_policy: TopologyMutationDerivedFallbackPolicy::AllowExplicitFallback,
            lowered_mutations: vec![TopologyMutation::RemoveEntity { entity_id }],
        });
        self
    }
}
