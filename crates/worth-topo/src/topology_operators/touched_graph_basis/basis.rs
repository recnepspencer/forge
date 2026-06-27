use schema::facade::platform::authority::{
    worth_topology_touched_graph_digest, WorthTopologyTouchedGraphCounters,
};
use schema::facade::platform::relations::TopologyRelationKind;
use serde::Serialize;

use super::digest::{canonical_digest_parts, canonical_values};
use super::{
    BasisDigestPart, TopologyGraphLifecyclePosture, TopologyTouchedAspect, TopologyTouchedEntity,
    TopologyTouchedOperatingWorld, TopologyTouchedRelation, TopologyTouchedScope,
};

pub type TopologyTouchedGraphCounters = WorthTopologyTouchedGraphCounters;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TopologyTouchedGraphBasis {
    entities: Vec<TopologyTouchedEntity>,
    relations: Vec<TopologyTouchedRelation>,
    relation_kinds: Vec<TopologyRelationKind>,
    aspects: Vec<TopologyTouchedAspect>,
    topology_scopes: Vec<TopologyTouchedScope>,
    lifecycle_posture: TopologyGraphLifecyclePosture,
    operating_world: TopologyTouchedOperatingWorld,
    counters: TopologyTouchedGraphCounters,
    digest: String,
}

pub(crate) struct TopologyTouchedGraphBasisInput {
    pub(crate) entities: Vec<TopologyTouchedEntity>,
    pub(crate) relations: Vec<TopologyTouchedRelation>,
    pub(crate) relation_kinds: Vec<TopologyRelationKind>,
    pub(crate) aspects: Vec<TopologyTouchedAspect>,
    pub(crate) topology_scopes: Vec<TopologyTouchedScope>,
    pub(crate) lifecycle_posture: TopologyGraphLifecyclePosture,
    pub(crate) operating_world: TopologyTouchedOperatingWorld,
}

impl TopologyTouchedGraphBasis {
    pub(crate) fn from_input(input: TopologyTouchedGraphBasisInput) -> Self {
        let entities = canonical_values(input.entities);
        let relations = canonical_values(input.relations);
        let relation_kinds = canonical_values(input.relation_kinds);
        let aspects = canonical_values(input.aspects);
        let topology_scopes = canonical_values(input.topology_scopes);
        let counters = TopologyTouchedGraphCounters::from_topology_breadth(
            entities.len(),
            relations.len(),
            relation_kinds.len(),
            aspects.len(),
            topology_scopes.len(),
        );
        let digest = basis_digest(
            &entities,
            &relations,
            &relation_kinds,
            &aspects,
            &topology_scopes,
            input.lifecycle_posture,
            &input.operating_world,
            counters,
        );
        Self {
            entities,
            relations,
            relation_kinds,
            aspects,
            topology_scopes,
            lifecycle_posture: input.lifecycle_posture,
            operating_world: input.operating_world,
            counters,
            digest,
        }
    }

    pub fn entities(&self) -> &[TopologyTouchedEntity] {
        &self.entities
    }

    pub fn relations(&self) -> &[TopologyTouchedRelation] {
        &self.relations
    }

    pub fn relation_kinds(&self) -> &[TopologyRelationKind] {
        &self.relation_kinds
    }

    pub fn aspects(&self) -> &[TopologyTouchedAspect] {
        &self.aspects
    }

    pub fn topology_scopes(&self) -> &[TopologyTouchedScope] {
        &self.topology_scopes
    }

    pub const fn lifecycle_posture(&self) -> TopologyGraphLifecyclePosture {
        self.lifecycle_posture
    }

    pub const fn operating_world(&self) -> &TopologyTouchedOperatingWorld {
        &self.operating_world
    }

    pub const fn counters(&self) -> TopologyTouchedGraphCounters {
        self.counters
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }

    #[cfg(any(test, feature = "test-support-lowering"))]
    pub(crate) fn with_operating_world_for_tests(
        self,
        operating_world: TopologyTouchedOperatingWorld,
    ) -> Self {
        Self::from_input(TopologyTouchedGraphBasisInput {
            entities: self.entities,
            relations: self.relations,
            relation_kinds: self.relation_kinds,
            aspects: self.aspects,
            topology_scopes: self.topology_scopes,
            lifecycle_posture: self.lifecycle_posture,
            operating_world,
        })
    }
}

fn basis_digest(
    entities: &[TopologyTouchedEntity],
    relations: &[TopologyTouchedRelation],
    relation_kinds: &[TopologyRelationKind],
    aspects: &[TopologyTouchedAspect],
    topology_scopes: &[TopologyTouchedScope],
    lifecycle_posture: TopologyGraphLifecyclePosture,
    operating_world: &TopologyTouchedOperatingWorld,
    counters: TopologyTouchedGraphCounters,
) -> String {
    let mut parts = vec![
        "worth-topo:touched-graph-basis:v1".to_string(),
        lifecycle_posture.digest_part(),
        operating_world.digest_part(),
        format!("counter:entity:{}", counters.entity_count()),
        format!("counter:relation:{}", counters.relation_count()),
        format!("counter:relation-kind:{}", counters.relation_kind_count()),
        format!("counter:aspect:{}", counters.touched_aspect_count()),
        format!("counter:topology-scope:{}", counters.topology_scope_count()),
    ];
    parts.extend(canonical_digest_parts(entities));
    parts.extend(canonical_digest_parts(relations));
    parts.extend(canonical_digest_parts(relation_kinds));
    parts.extend(canonical_digest_parts(aspects));
    parts.extend(canonical_digest_parts(topology_scopes));
    worth_topology_touched_graph_digest(&parts)
}
