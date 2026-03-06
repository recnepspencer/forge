//! Epsilon-tolerance vertex welder using spatial hashing and Union-Find.
//!
//! DOMAIN: Cluster vertices within a tolerance distance using a spatial
//! hash grid for O(1) neighbor lookup and Union-Find for transitive merging.
//!
//! INVARIANTS:
//! - All vertices within ε of each other end up in the same cluster
//! - Transitivity is guaranteed: if A≈B and B≈C then A,B,C share a root
//! - Cluster root position is the first vertex added to that cluster
//! - Grid cell size = 2·tolerance so all ε-neighbors are in adjacent cells

use crate::spatial::union_find::UnionFind;
use std::collections::BTreeMap;

/// Spatial hash grid + Union-Find for epsilon-tolerance vertex welding.
pub struct EpsilonWelder {
    grid: BTreeMap<[i64; 3], Vec<usize>>,
    positions: Vec<[f64; 3]>,
    uf: UnionFind,
    cell_size: f64,
    tolerance_sq: f64,
}

impl EpsilonWelder {
    /// Create a new welder with the given linear tolerance.
    ///
    /// Cell size is set to `2 * tolerance` so that vertices within ε
    /// are guaranteed to reside in the same or adjacent grid cells.
    pub fn new(tolerance: f64) -> Self {
        let tol = tolerance.max(1e-30);
        Self {
            grid: BTreeMap::new(),
            positions: Vec::new(),
            uf: UnionFind::new(0),
            cell_size: tol * 2.0,
            tolerance_sq: tol * tol,
        }
    }

    /// Add a vertex and immediately cluster it with neighbors.
    ///
    /// Returns the internal index for this vertex.
    pub fn add_vertex(&mut self, pos: [f64; 3]) -> usize {
        let idx = self.uf.push();
        self.positions.push(pos);

        let center = self.cell_coords(&pos);
        for dx in -1i64..=1 {
            for dy in -1i64..=1 {
                for dz in -1i64..=1 {
                    let cell = [center[0] + dx, center[1] + dy, center[2] + dz];
                    if let Some(neighbors) = self.grid.get(&cell) {
                        for &n in neighbors {
                            if self.squared_distance(idx, n) <= self.tolerance_sq {
                                self.uf.union(idx, n);
                            }
                        }
                    }
                }
            }
        }

        self.grid.entry(center).or_default().push(idx);
        idx
    }

    /// Get the cluster root index for a vertex.
    pub fn root_id(&mut self, idx: usize) -> usize {
        self.uf.find(idx)
    }

    /// Get the canonical position for a cluster (the root's position).
    pub fn canonical_position(&mut self, idx: usize) -> [f64; 3] {
        let root = self.uf.find(idx);
        self.positions[root]
    }

    /// Check if two vertices are in the same cluster.
    pub fn same_cluster(&mut self, a: usize, b: usize) -> bool {
        self.uf.same_set(a, b)
    }

    /// Total number of vertices added.
    pub fn len(&self) -> usize {
        self.positions.len()
    }

    /// Whether no vertices have been added.
    pub fn is_empty(&self) -> bool {
        self.positions.is_empty()
    }

    /// Find the nearest registered vertex within tolerance (read-only).
    ///
    /// Returns the root index of the cluster containing the nearest
    /// existing vertex, or None if no vertex is within tolerance.
    /// Does NOT add the query point to the welder.
    pub fn find_nearest(&mut self, pos: &[f64; 3]) -> Option<usize> {
        let center = self.cell_coords(pos);
        let mut best: Option<(usize, f64)> = None;

        for dx in -1i64..=1 {
            for dy in -1i64..=1 {
                for dz in -1i64..=1 {
                    let cell = [center[0] + dx, center[1] + dy, center[2] + dz];
                    if let Some(neighbors) = self.grid.get(&cell) {
                        for &n in neighbors {
                            let dist_sq = self.squared_distance_to(pos, n);
                            if dist_sq <= self.tolerance_sq {
                                match best {
                                    None => best = Some((n, dist_sq)),
                                    Some((_, bd)) if dist_sq < bd => best = Some((n, dist_sq)),
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
        }

        best.map(|(n, _)| self.uf.find(n))
    }

    fn cell_coords(&self, pos: &[f64; 3]) -> [i64; 3] {
        [
            (pos[0] / self.cell_size).floor() as i64,
            (pos[1] / self.cell_size).floor() as i64,
            (pos[2] / self.cell_size).floor() as i64,
        ]
    }

    fn squared_distance(&self, a: usize, b: usize) -> f64 {
        let pa = &self.positions[a];
        let pb = &self.positions[b];
        let dx = pa[0] - pb[0];
        let dy = pa[1] - pb[1];
        let dz = pa[2] - pb[2];
        dx * dx + dy * dy + dz * dz
    }

    fn squared_distance_to(&self, pos: &[f64; 3], idx: usize) -> f64 {
        let pb = &self.positions[idx];
        let dx = pos[0] - pb[0];
        let dy = pos[1] - pb[1];
        let dz = pos[2] - pb[2];
        dx * dx + dy * dy + dz * dz
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distinct_vertices_stay_separate() {
        let mut w = EpsilonWelder::new(1e-8);
        let a = w.add_vertex([0.0, 0.0, 0.0]);
        let b = w.add_vertex([1.0, 0.0, 0.0]);
        assert!(!w.same_cluster(a, b));
    }

    #[test]
    fn close_vertices_merge() {
        let mut w = EpsilonWelder::new(1e-8);
        let a = w.add_vertex([0.0, 0.0, 0.0]);
        let b = w.add_vertex([1e-14, 0.0, 0.0]);
        assert!(w.same_cluster(a, b));
    }

    #[test]
    fn transitive_chain_merges() {
        let mut w = EpsilonWelder::new(1e-8);
        let a = w.add_vertex([0.0, 0.0, 0.0]);
        let b = w.add_vertex([5e-9, 0.0, 0.0]);
        let c = w.add_vertex([1e-8, 0.0, 0.0]);
        assert!(w.same_cluster(a, b));
        assert!(w.same_cluster(b, c));
        assert!(
            w.same_cluster(a, c),
            "Transitive chain: A≈B and B≈C should imply A≈C"
        );
    }

    #[test]
    fn dense_cluster_single_root() {
        let mut w = EpsilonWelder::new(1e-8);
        let mut ids = Vec::new();
        for i in 0..100 {
            let drift = i as f64 * 1e-14;
            ids.push(w.add_vertex([drift, 0.0, 0.0]));
        }
        let root = w.root_id(ids[0]);
        for &id in &ids {
            assert_eq!(
                w.root_id(id),
                root,
                "All 100 vertices within 10^-12 should share one root"
            );
        }
    }

    #[test]
    fn canonical_position_is_roots() {
        let mut w = EpsilonWelder::new(1e-8);
        let a = w.add_vertex([1.0, 2.0, 3.0]);
        let b = w.add_vertex([1.0 + 1e-14, 2.0, 3.0]);
        let root = w.root_id(a);
        let pos = w.canonical_position(b);
        assert_eq!(pos, w.positions[root]);
    }
}
