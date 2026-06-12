use std::collections::BTreeSet;

use super::conflict_graph_edges::TilingConflictEdge;
use super::conflict_graph_errors::ConflictGraphError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ConflictGraphExtractionIndex {
    vertices: Vec<String>,
    edges: Vec<TilingConflictEdge>,
}

impl ConflictGraphExtractionIndex {
    pub(crate) fn from_edges(
        mut edges: Vec<TilingConflictEdge>,
    ) -> Result<Self, ConflictGraphError> {
        if edges.is_empty() {
            return Err(ConflictGraphError::EmptyConflictEdges);
        }
        edges.sort_by_key(TilingConflictEdge::stable_token);
        edges.dedup_by_key(|edge| edge.stable_token());
        let mut vertex_set = BTreeSet::new();
        for edge in &edges {
            vertex_set.insert(edge.left_vertex_label().to_string());
            vertex_set.insert(edge.right_vertex_label().to_string());
        }
        Ok(Self {
            vertices: vertex_set.into_iter().collect(),
            edges,
        })
    }

    pub(crate) fn vertices(&self) -> &[String] {
        &self.vertices
    }

    pub(crate) fn edges(&self) -> &[TilingConflictEdge] {
        &self.edges
    }

    pub(crate) fn stable_token(&self) -> String {
        let vertices = self.vertices.join("|");
        let edges = self
            .edges
            .iter()
            .map(TilingConflictEdge::stable_token)
            .collect::<Vec<_>>()
            .join("|");
        format!("vertices={vertices};edges={edges}")
    }
}
