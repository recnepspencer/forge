use std::collections::BTreeSet;

use forge_relational::facade::identity::{EntityId, RelationId};
use schema::facade::platform::aspects::{Aspect, DiagnosticsAspect, TopologyAspect};
use schema::facade::platform::authority::TopologyMutation;
use schema::facade::platform::relations::RelationKind;

use crate::topology_operators::declared_mutation_sequence_builder::TopologyDeclaredMutationSequenceBuilder;

use super::records::TopologyDeclaredMutationAction;
use super::{
    LoopEndpointKind, LoopSuccessorKind, TopologyDeclaredMutationRecord, TopologyDerivedRegion,
    TopologyMutationChangedScope, TopologyMutationDerivedFallbackPolicy, TopologyMutationFamily,
    TopologyMutationNamingScope,
};

impl TopologyDeclaredMutationSequenceBuilder {
    pub(crate) fn rewire_loop_successor(
        &mut self,
        relation_id: RelationId,
        kind: LoopSuccessorKind,
        half_edge_id: EntityId,
        successor_half_edge_id: EntityId,
    ) -> &mut Self {
        let touched_aspects = BTreeSet::from([
            Aspect::Topology(TopologyAspect::Boundary),
            Aspect::Diagnostics(DiagnosticsAspect::Decisions),
        ]);
        self.records.push(TopologyDeclaredMutationRecord {
            family: TopologyMutationFamily::RewireLoopSuccessor,
            action: TopologyDeclaredMutationAction::RewireLoopSuccessor {
                relation_id,
                kind,
                half_edge_id,
                successor_half_edge_id,
            },
            touched_aspects,
            changed_scopes: vec![
                TopologyMutationChangedScope::Relation,
                TopologyMutationChangedScope::Loop,
                TopologyMutationChangedScope::LocalNeighborhood,
            ],
            naming_scopes: vec![TopologyMutationNamingScope::AdjacentEntityNames],
            derived_regions: vec![
                TopologyDerivedRegion::LoopRegion,
                TopologyDerivedRegion::MutationLocalNeighborhoodRegion,
                TopologyDerivedRegion::NamingContinuityRegion,
            ],
            derived_fallback_policy: TopologyMutationDerivedFallbackPolicy::AllowExplicitFallback,
            lowered_mutations: vec![TopologyMutation::UpsertRelation {
                relation_id,
                kind: RelationKind::Topology(kind.relation_kind()),
                source: half_edge_id,
                target: successor_half_edge_id,
            }],
        });
        self
    }

    pub(crate) fn rewire_loop_endpoint(
        &mut self,
        relation_id: RelationId,
        endpoint: LoopEndpointKind,
        half_edge_id: EntityId,
        vertex_id: EntityId,
    ) -> &mut Self {
        let touched_aspects = BTreeSet::from([
            Aspect::Topology(TopologyAspect::Boundary),
            Aspect::Diagnostics(DiagnosticsAspect::Decisions),
        ]);
        self.records.push(TopologyDeclaredMutationRecord {
            family: TopologyMutationFamily::RewireLoopEndpoint,
            action: TopologyDeclaredMutationAction::RewireLoopEndpoint {
                relation_id,
                endpoint,
                half_edge_id,
                vertex_id,
            },
            touched_aspects,
            changed_scopes: vec![
                TopologyMutationChangedScope::Relation,
                TopologyMutationChangedScope::Loop,
                TopologyMutationChangedScope::LocalNeighborhood,
            ],
            naming_scopes: vec![TopologyMutationNamingScope::AdjacentEntityNames],
            derived_regions: vec![
                TopologyDerivedRegion::LoopRegion,
                TopologyDerivedRegion::MutationLocalNeighborhoodRegion,
                TopologyDerivedRegion::NamingContinuityRegion,
            ],
            derived_fallback_policy: TopologyMutationDerivedFallbackPolicy::AllowExplicitFallback,
            lowered_mutations: vec![TopologyMutation::UpsertRelation {
                relation_id,
                kind: RelationKind::Topology(endpoint.relation_kind()),
                source: half_edge_id,
                target: vertex_id,
            }],
        });
        self
    }
}
