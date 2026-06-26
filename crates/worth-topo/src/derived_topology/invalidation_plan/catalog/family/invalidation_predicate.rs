use serde::{Deserialize, Serialize};

use crate::topology_operators::TopologyTouchedGraphBasis;

use super::DerivedTopologyConsumedGraphFacts;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DerivedTopologyInvalidationPredicate {
    ConsumedGraphFactsIntersectTouchedClosure,
}

impl DerivedTopologyInvalidationPredicate {
    pub(crate) fn matches_touched_basis(
        self,
        consumed_graph_facts: &DerivedTopologyConsumedGraphFacts,
        basis: &TopologyTouchedGraphBasis,
    ) -> bool {
        match self {
            Self::ConsumedGraphFactsIntersectTouchedClosure => {
                consumed_graph_facts.intersects_touched_basis(basis)
            }
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumedGraphFactsIntersectTouchedClosure => {
                "consumed_graph_facts_intersect_touched_closure"
            }
        }
    }
}
