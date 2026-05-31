use std::collections::BTreeSet;

use forge_relational::facade::identity::{EntityId, RelationId};
use schema::facade::platform::aspects::{Aspect, DiagnosticsAspect, TopologyAspect};
use schema::facade::platform::authority::TopologyMutation;
use schema::facade::platform::relations::{RelationKind, TopologyRelationKind};

use crate::topology_operators::declared_mutation_sequence_builder::TopologyDeclaredMutationSequenceBuilder;

use super::records::TopologyDeclaredMutationAction;
use super::{
    TopologyDeclaredMutationRecord, TopologyDerivedRegion, TopologyMutationChangedScope,
    TopologyMutationDerivedFallbackPolicy, TopologyMutationFamily, TopologyMutationNamingScope,
};

impl TopologyDeclaredMutationSequenceBuilder {
    pub(crate) fn splice_radial_adjacency(
        &mut self,
        relation_id: RelationId,
        half_edge_id: EntityId,
        radial_next_half_edge_id: EntityId,
    ) -> &mut Self {
        self.records.push(TopologyDeclaredMutationRecord {
            family: TopologyMutationFamily::SpliceRadialAdjacency,
            action: TopologyDeclaredMutationAction::SpliceRadialAdjacency {
                relation_id,
                half_edge_id,
                radial_next_half_edge_id,
            },
            touched_aspects: BTreeSet::from([
                Aspect::Topology(TopologyAspect::Radial),
                Aspect::Diagnostics(DiagnosticsAspect::Decisions),
            ]),
            changed_scopes: vec![
                TopologyMutationChangedScope::Relation,
                TopologyMutationChangedScope::RadialNeighborhood,
                TopologyMutationChangedScope::LocalNeighborhood,
            ],
            naming_scopes: vec![TopologyMutationNamingScope::AdjacentEntityNames],
            derived_regions: vec![
                TopologyDerivedRegion::RadialNeighborhoodRegion,
                TopologyDerivedRegion::MutationLocalNeighborhoodRegion,
                TopologyDerivedRegion::NamingContinuityRegion,
            ],
            derived_fallback_policy: TopologyMutationDerivedFallbackPolicy::AllowExplicitFallback,
            lowered_mutations: vec![TopologyMutation::UpsertRelation {
                relation_id,
                kind: RelationKind::Topology(TopologyRelationKind::HalfEdgeRadialNext),
                source: half_edge_id,
                target: radial_next_half_edge_id,
            }],
        });
        self
    }

    pub(crate) fn detach_radial_adjacency(&mut self, relation_id: RelationId) -> &mut Self {
        self.records.push(TopologyDeclaredMutationRecord {
            family: TopologyMutationFamily::DetachRadialAdjacency,
            action: TopologyDeclaredMutationAction::DetachRadialAdjacency { relation_id },
            touched_aspects: BTreeSet::from([
                Aspect::Topology(TopologyAspect::Radial),
                Aspect::Diagnostics(DiagnosticsAspect::Decisions),
            ]),
            changed_scopes: vec![
                TopologyMutationChangedScope::Relation,
                TopologyMutationChangedScope::RadialNeighborhood,
                TopologyMutationChangedScope::LocalNeighborhood,
            ],
            naming_scopes: vec![TopologyMutationNamingScope::AdjacentEntityNames],
            derived_regions: vec![
                TopologyDerivedRegion::RadialNeighborhoodRegion,
                TopologyDerivedRegion::MutationLocalNeighborhoodRegion,
                TopologyDerivedRegion::NamingContinuityRegion,
            ],
            derived_fallback_policy: TopologyMutationDerivedFallbackPolicy::AllowExplicitFallback,
            lowered_mutations: vec![TopologyMutation::RemoveRelation { relation_id }],
        });
        self
    }
}
