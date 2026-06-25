use crate::runtime::WorthUiRuntimeFactSet;

use super::{primitive_surface_edges::primitive_surface_edges, WorthUiGraphDependencyEdge};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiGraphFactRegistry {
    edges: Vec<WorthUiGraphDependencyEdge>,
}

impl WorthUiGraphFactRegistry {
    pub(crate) fn for_primitive_surface(surface_id: &str) -> Self {
        Self {
            edges: primitive_surface_edges(surface_id),
        }
    }

    pub fn edges(&self) -> &[WorthUiGraphDependencyEdge] {
        &self.edges
    }

    pub(crate) fn facts_reachable_from(
        &self,
        roots: &WorthUiRuntimeFactSet,
    ) -> WorthUiRuntimeFactSet {
        let mut reachable = roots.clone();
        loop {
            let before = reachable.len();
            for edge in &self.edges {
                if reachable.contains(edge.source()) {
                    reachable.insert(edge.target().clone());
                }
            }
            if reachable.len() == before {
                return reachable;
            }
        }
    }

    pub(crate) fn published_facts(&self) -> WorthUiRuntimeFactSet {
        let mut facts = WorthUiRuntimeFactSet::empty();
        for edge in &self.edges {
            facts.insert(edge.source().clone());
            facts.insert(edge.target().clone());
        }
        facts
    }
}
