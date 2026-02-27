//! Explicit caching of near-coincident topological entities.
//!
//! DOMAIN: Record coplanar faces, collinear edges, and coincident vertices
//! found during the boolean assembly pre-pass.
//! DEPENDENCIES: BTreeMap.
//! INVARIANTS: Keys are strictly canonical `(min(A, B), max(A, B))` to
//! ensure deterministic querying and ordering.

use std::collections::BTreeMap;

/// Specifies the type of coincidence detected and its measured deviation.
#[derive(Debug, Clone)]
pub enum CoincidenceKind {
    /// Two faces are coplanar within the configured offset and angle epsilon.
    CoplanarFaces {
        /// The maximum gap measured between the two faces (mm).
        gap_mm: f64,
    },
    /// Two edges are collinear within the configured distance epsilon.
    CollinearEdges {
        /// The maximum gap measured between the two edges (mm).
        gap_mm: f64,
    },
    /// Two vertices are coincident within the configured distance epsilon.
    CoincidentVertices {
        /// The measured distance between the two vertices (mm).
        distance_mm: f64,
    },
}

/// Explicit graph recording near-coincident entity pairs.
///
/// Built once per boolean operation during the `assemble` pre-pass, then
/// consumed by the classification and gap closure phases to prevent
/// threshold flip-flops and ambiguous intersections.
///
/// Uses `BTreeMap` to guarantee completely deterministic iteration order
/// for tracing and reproducible replay. Keys are packed `u64` representing
/// arbitrary entity handles (usually `FaceId::into_raw()`).
#[derive(Debug, Clone, Default)]
pub struct CoincidenceGraph {
    /// Canonical map of `(Entity A, Entity B)` → `CoincidenceKind`.
    /// Guarantee: `key.0 < key.1`.
    edges: BTreeMap<(u64, u64), CoincidenceKind>,
}

impl CoincidenceGraph {
    /// Create a new empty coincidence graph.
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a coincidence relationship between two entities.
    ///
    /// Automatically canonicalizes the order of `a` and `b` to prevent
    /// duplicate edges `A → B` and `B → A`.
    ///
    /// # Panics
    /// Panics in debug builds if `a == b`. Self-coincidence is a meaningless logical error.
    pub fn insert_edge(&mut self, a: u64, b: u64, kind: CoincidenceKind) {
        debug_assert_ne!(a, b, "Cannot insert self-coincidence edge for entity {}", a);
        let key = if a < b { (a, b) } else { (b, a) };
        self.edges.insert(key, kind);
    }

    /// Query whether two entities are recorded as coincident.
    ///
    /// `a` and `b` can be provided in any order.
    pub fn query_edge(&self, a: u64, b: u64) -> Option<&CoincidenceKind> {
        let key = if a < b { (a, b) } else { (b, a) };
        self.edges.get(&key)
    }

    /// Total number of coincidence edges recorded.
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Iterate over all coincidence edges in deterministic order.
    pub fn iter(&self) -> impl Iterator<Item = (&(u64, u64), &CoincidenceKind)> {
        self.edges.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_canonicalizes_order() {
        let mut graph = CoincidenceGraph::new();
        graph.insert_edge(10, 5, CoincidenceKind::CoplanarFaces { gap_mm: 0.1 });

        // Querying with forward and reverse order should both find the edge.
        assert!(graph.query_edge(10, 5).is_some());
        assert!(graph.query_edge(5, 10).is_some());
        assert_eq!(graph.edge_count(), 1);

        // Under the hood, the key should be strictly (5, 10).
        let keys: Vec<_> = graph.edges.keys().copied().collect();
        assert_eq!(keys, vec![(5, 10)]);
    }

    #[test]
    fn query_missing_edge_returns_none() {
        let mut graph = CoincidenceGraph::new();
        graph.insert_edge(1, 2, CoincidenceKind::CoplanarFaces { gap_mm: 0.0 });

        assert!(graph.query_edge(2, 3).is_none());
    }
    #[test]
    fn canonical_key_ordering() {
        let mut graph = CoincidenceGraph::new();
        // Insert out of order
        graph.insert_edge(50, 10, CoincidenceKind::CoplanarFaces { gap_mm: 0.1 });
        // Insert in order
        graph.insert_edge(20, 30, CoincidenceKind::CoplanarFaces { gap_mm: 0.2 });
        // Update existing (will overwrite or keep canonical key depending on implementation)
        graph.insert_edge(10, 50, CoincidenceKind::CoplanarFaces { gap_mm: 0.3 });

        assert_eq!(graph.edge_count(), 2);

        let mut keys = graph.edges.keys();
        let k1 = keys.next().unwrap();
        assert_eq!(*k1, (10, 50)); // canonical ordering

        let k2 = keys.next().unwrap();
        assert_eq!(*k2, (20, 30)); // canonical ordering
    }
}
