//! Explicit coincidence framework for clustering coincident entities.
//!
//! DOMAIN: Math-level coincidence classification and merge logic.
//! INVARIANTS: Cluster representatives are always the smallest ID (D1 determinism).
//! DEPENDENCIES: `error` (MathError for undeclared entity lookups).
//!
//! When geometric predicates return [`Zero`](crate::sign::TriSign::Zero),
//! the kernel must decide how to handle the coincidence. This module provides:
//!
//! - [`Coincidence`] — what kind of coincidence was detected
//! - [`CoincidenceGraph`] — union-find clustering with deterministic tie-breaking
//! - [`MergeAction`] — what the kernel should do when entities merge

use crate::error::MathError;

pub mod sos;
pub use sos::{orient2d_sos, orient3d_sos, SosPoint};

/// A detected geometric coincidence between two entities.
///
/// Entity IDs are `u64` stable identifiers. The invariant `a < b` is
/// enforced at construction to guarantee deterministic ordering (D1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Coincidence {
    /// Two faces lie on the same plane.
    CoplanarFaces { a: u64, b: u64 },
    /// Two edges are collinear (share the same supporting line).
    CollinearEdges { a: u64, b: u64 },
    /// Two vertices occupy the same position.
    CoincidentVertices { a: u64, b: u64 },
}

impl Coincidence {
    /// Create a coplanar-faces coincidence with canonical ordering.
    pub fn coplanar_faces(id_a: u64, id_b: u64) -> Self {
        let (a, b) = if id_a <= id_b {
            (id_a, id_b)
        } else {
            (id_b, id_a)
        };
        Self::CoplanarFaces { a, b }
    }

    /// Create a collinear-edges coincidence with canonical ordering.
    pub fn collinear_edges(id_a: u64, id_b: u64) -> Self {
        let (a, b) = if id_a <= id_b {
            (id_a, id_b)
        } else {
            (id_b, id_a)
        };
        Self::CollinearEdges { a, b }
    }

    /// Create a coincident-vertices coincidence with canonical ordering.
    pub fn coincident_vertices(id_a: u64, id_b: u64) -> Self {
        let (a, b) = if id_a <= id_b {
            (id_a, id_b)
        } else {
            (id_b, id_a)
        };
        Self::CoincidentVertices { a, b }
    }

    /// The two entity IDs involved in this coincidence (always `a < b`).
    pub fn ids(&self) -> (u64, u64) {
        match *self {
            Self::CoplanarFaces { a, b }
            | Self::CollinearEdges { a, b }
            | Self::CoincidentVertices { a, b } => (a, b),
        }
    }
}

/// What the kernel should do when coincident entities are merged.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MergeAction {
    /// Keep the representative (smaller-ID entity), discard the other.
    RetainRepresentative,
    /// Two coplanar faces form a flush boundary — remove the internal face.
    FlushFaces { face_a: u64, face_b: u64 },
}

/// Union-find graph for clustering coincident entities.
///
/// Implements union-by-rank with path compression for O(α(n)) amortized
/// operations. Cluster representatives are deterministic: always the
/// smallest ID in the cluster (Doctrine D1).
pub struct CoincidenceGraph {
    nodes: Vec<Node>,
    id_to_index: std::collections::HashMap<u64, usize>,
}

/// Internal node in the union-find forest.
struct Node {
    entity_id: u64,
    parent: usize,
    rank: u32,
}

impl CoincidenceGraph {
    /// Create an empty coincidence graph.
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            id_to_index: std::collections::HashMap::new(),
        }
    }

    /// Register an entity in the graph. Idempotent.
    pub fn declare(&mut self, id: u64) {
        if !self.id_to_index.contains_key(&id) {
            let index = self.nodes.len();
            self.nodes.push(Node {
                entity_id: id,
                parent: index,
                rank: 0,
            });
            self.id_to_index.insert(id, index);
        }
    }

    /// Merge two entities as coincident. Declares them if not already present.
    ///
    /// The representative of the merged cluster is always the smallest ID,
    /// ensuring deterministic tie-breaking (D1).
    pub fn merge(&mut self, a: u64, b: u64) {
        self.declare(a);
        self.declare(b);

        let index_a = self.id_to_index[&a];
        let index_b = self.id_to_index[&b];
        let root_a = self.find(index_a);
        let root_b = self.find(index_b);

        if root_a == root_b {
            return;
        }

        let (winner, loser) = if self.nodes[root_a].entity_id < self.nodes[root_b].entity_id {
            (root_a, root_b)
        } else {
            (root_b, root_a)
        };

        self.nodes[loser].parent = winner;

        if self.nodes[winner].rank == self.nodes[loser].rank {
            self.nodes[winner].rank += 1;
        }
    }

    /// Find the cluster representative for an entity.
    ///
    /// Returns the smallest entity ID in the cluster (D1 determinism).
    /// Returns `MathError::InvalidInput` if the entity was never declared.
    pub fn representative(&mut self, id: u64) -> Result<u64, MathError> {
        let index = self.resolve_index(id)?;
        let root = self.find(index);
        Ok(self.nodes[root].entity_id)
    }

    /// Check whether two entities are in the same cluster.
    ///
    /// Returns `false` if either entity was never declared.
    pub fn same_cluster(&mut self, a: u64, b: u64) -> bool {
        let index_a = match self.resolve_index(a) {
            Ok(i) => i,
            Err(_) => return false,
        };
        let index_b = match self.resolve_index(b) {
            Ok(i) => i,
            Err(_) => return false,
        };
        let root_a = self.find(index_a);
        let root_b = self.find(index_b);
        root_a == root_b
    }

    /// Resolve an entity ID to its internal index, returning an error if undeclared.
    fn resolve_index(&self, id: u64) -> Result<usize, MathError> {
        self.id_to_index.get(&id).copied().ok_or_else(|| {
            MathError::InvalidInput(format!(
                "Entity {} was never declared in CoincidenceGraph",
                id
            ))
        })
    }

    /// The number of distinct clusters in the graph.
    pub fn cluster_count(&mut self) -> usize {
        let indices: Vec<usize> = (0..self.nodes.len()).collect();
        let mut roots: Vec<usize> = indices.iter().map(|&i| self.find(i)).collect();
        roots.sort_unstable();
        roots.dedup();
        roots.len()
    }

    /// All clusters and their members, sorted deterministically.
    ///
    /// Returns a `Vec` of `(representative_id, member_ids)` pairs.
    /// Both the outer vec and inner member vecs are sorted by ID.
    pub fn clusters(&mut self) -> Vec<(u64, Vec<u64>)> {
        let mut cluster_map: std::collections::BTreeMap<u64, Vec<u64>> =
            std::collections::BTreeMap::new();

        let entity_ids: Vec<u64> = self.id_to_index.keys().copied().collect();
        for id in entity_ids {
            let rep = self.representative(id).unwrap_or(id);
            cluster_map.entry(rep).or_default().push(id);
        }

        for members in cluster_map.values_mut() {
            members.sort_unstable();
        }

        cluster_map.into_iter().collect()
    }

    /// Determine the merge action for a coincidence.
    pub fn merge_action(coincidence: &Coincidence) -> MergeAction {
        match *coincidence {
            Coincidence::CoplanarFaces { a, b } => MergeAction::FlushFaces {
                face_a: a,
                face_b: b,
            },
            Coincidence::CollinearEdges { .. } | Coincidence::CoincidentVertices { .. } => {
                MergeAction::RetainRepresentative
            }
        }
    }

    /// Find with path compression.
    fn find(&mut self, mut index: usize) -> usize {
        while self.nodes[index].parent != index {
            let grandparent = self.nodes[self.nodes[index].parent].parent;
            self.nodes[index].parent = grandparent;
            index = grandparent;
        }
        index
    }
}

impl Default for CoincidenceGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for CoincidenceGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(CoincidenceGraph))
            .field("entity_count", &self.nodes.len())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coincidence_canonical_ordering() {
        let c = Coincidence::coplanar_faces(10, 5);
        assert_eq!(c.ids(), (5, 10));
    }

    #[test]
    fn single_entity_is_own_representative() {
        let mut g = CoincidenceGraph::new();
        g.declare(42);
        assert_eq!(g.representative(42).unwrap(), 42);
    }

    #[test]
    fn undeclared_entity_returns_error() {
        let mut g = CoincidenceGraph::new();
        assert!(g.representative(999).is_err());
    }

    #[test]
    fn merge_picks_smallest_id_as_representative() {
        let mut g = CoincidenceGraph::new();
        g.merge(10, 5);
        assert_eq!(g.representative(10).unwrap(), 5);
        assert_eq!(g.representative(5).unwrap(), 5);
    }

    #[test]
    fn transitive_merge() {
        let mut g = CoincidenceGraph::new();
        g.merge(10, 20);
        g.merge(20, 30);
        assert_eq!(g.representative(30).unwrap(), 10);
        assert!(g.same_cluster(10, 30));
    }

    #[test]
    fn separate_clusters_remain_separate() {
        let mut g = CoincidenceGraph::new();
        g.merge(1, 2);
        g.merge(3, 4);
        assert!(!g.same_cluster(1, 3));
        assert_eq!(g.cluster_count(), 2);
    }

    #[test]
    fn clusters_returns_sorted_results() {
        let mut g = CoincidenceGraph::new();
        g.merge(5, 10);
        g.merge(10, 15);
        g.merge(100, 200);

        let clusters = g.clusters();
        assert_eq!(clusters.len(), 2);
        assert_eq!(clusters[0], (5, vec![5, 10, 15]));
        assert_eq!(clusters[1], (100, vec![100, 200]));
    }

    #[test]
    fn coplanar_faces_produce_flush_action() {
        let c = Coincidence::coplanar_faces(1, 2);
        assert!(matches!(
            CoincidenceGraph::merge_action(&c),
            MergeAction::FlushFaces {
                face_a: 1,
                face_b: 2
            }
        ));
    }

    #[test]
    fn coincident_vertices_produce_retain_action() {
        let c = Coincidence::coincident_vertices(1, 2);
        assert!(matches!(
            CoincidenceGraph::merge_action(&c),
            MergeAction::RetainRepresentative
        ));
    }

    #[test]
    fn merge_is_idempotent() {
        let mut g = CoincidenceGraph::new();
        g.merge(1, 2);
        g.merge(1, 2);
        g.merge(2, 1);
        assert_eq!(g.cluster_count(), 1);
        assert_eq!(g.representative(1).unwrap(), 1);
        assert_eq!(g.representative(2).unwrap(), 1);
    }

    #[test]
    fn declare_is_idempotent() {
        let mut g = CoincidenceGraph::new();
        g.declare(7);
        g.declare(7);
        assert_eq!(g.cluster_count(), 1);
    }
}
