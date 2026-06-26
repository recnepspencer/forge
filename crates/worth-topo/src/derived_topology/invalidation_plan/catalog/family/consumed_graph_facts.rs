use schema::facade::platform::relations::TopologyRelationKind;
use serde::Serialize;

use crate::topology_operators::{TopologyTouchedAspect, TopologyTouchedGraphBasis};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedTopologyConsumedGraphFacts {
    relation_kinds: Vec<TopologyRelationKind>,
    aspects: Vec<TopologyTouchedAspect>,
}

impl DerivedTopologyConsumedGraphFacts {
    pub(crate) fn new(
        relation_kinds: Vec<TopologyRelationKind>,
        aspects: Vec<TopologyTouchedAspect>,
    ) -> Self {
        Self {
            relation_kinds: canonical_relation_kinds(relation_kinds),
            aspects: canonical_aspects(aspects),
        }
    }

    pub fn relation_kinds(&self) -> &[TopologyRelationKind] {
        &self.relation_kinds
    }

    pub fn aspects(&self) -> &[TopologyTouchedAspect] {
        &self.aspects
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.relation_kinds.is_empty() && self.aspects.is_empty()
    }

    pub(crate) fn intersects_touched_basis(&self, basis: &TopologyTouchedGraphBasis) -> bool {
        self.relation_kinds
            .iter()
            .any(|kind| basis.relation_kinds().contains(kind))
            || self
                .aspects
                .iter()
                .any(|aspect| basis.aspects().contains(aspect))
    }

    pub(crate) fn digest_parts(&self) -> Vec<String> {
        self.relation_kinds
            .iter()
            .map(|kind| format!("relation:{}", kind.kind_name()))
            .chain(
                self.aspects
                    .iter()
                    .map(|aspect| format!("aspect:{aspect:?}")),
            )
            .collect()
    }
}

fn canonical_relation_kinds(
    mut relation_kinds: Vec<TopologyRelationKind>,
) -> Vec<TopologyRelationKind> {
    relation_kinds.sort_by_key(|kind| kind.kind_name());
    relation_kinds.dedup();
    relation_kinds
}

fn canonical_aspects(mut aspects: Vec<TopologyTouchedAspect>) -> Vec<TopologyTouchedAspect> {
    aspects.sort();
    aspects.dedup();
    aspects
}
