//! Disjoint-set (Union-Find) data structure with path compression and union-by-rank.
//!
//! DOMAIN: Generic set partitioning for vertex clustering.
//!
//! INVARIANTS:
//! - `find` uses path compression for amortized O(α(n)) lookups
//! - `union` uses rank-based merging to keep trees flat
//! - No allocation after construction

/// Disjoint-set forest with path compression and union-by-rank.
pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<u32>,
}

impl UnionFind {
    /// Create a new Union-Find with `n` singleton sets.
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    /// Find the root representative of the set containing `x`.
    ///
    /// Uses path compression: every node on the path to root
    /// gets re-parented directly to root.
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    /// Merge the sets containing `x` and `y`.
    ///
    /// Uses union-by-rank: the shorter tree becomes a child
    /// of the taller tree, keeping depth logarithmic.
    pub fn union(&mut self, x: usize, y: usize) {
        let rx = self.find(x);
        let ry = self.find(y);
        if rx == ry {
            return;
        }
        match self.rank[rx].cmp(&self.rank[ry]) {
            std::cmp::Ordering::Less => self.parent[rx] = ry,
            std::cmp::Ordering::Greater => self.parent[ry] = rx,
            std::cmp::Ordering::Equal => {
                self.parent[ry] = rx;
                self.rank[rx] += 1;
            }
        }
    }

    /// Check if `x` and `y` belong to the same set.
    pub fn same_set(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    /// Grow the structure by one element (a new singleton set).
    pub fn push(&mut self) -> usize {
        let id = self.parent.len();
        self.parent.push(id);
        self.rank.push(0);
        id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn singleton_sets() {
        let mut uf = UnionFind::new(5);
        for i in 0..5 {
            assert_eq!(uf.find(i), i);
        }
    }

    #[test]
    fn union_merges_sets() {
        let mut uf = UnionFind::new(5);
        uf.union(0, 1);
        uf.union(2, 3);
        assert!(uf.same_set(0, 1));
        assert!(uf.same_set(2, 3));
        assert!(!uf.same_set(0, 2));
    }

    #[test]
    fn transitive_union() {
        let mut uf = UnionFind::new(5);
        uf.union(0, 1);
        uf.union(1, 2);
        assert!(uf.same_set(0, 2));
    }

    #[test]
    fn push_adds_singleton() {
        let mut uf = UnionFind::new(3);
        let id = uf.push();
        assert_eq!(id, 3);
        assert!(!uf.same_set(0, id));
        uf.union(0, id);
        assert!(uf.same_set(0, id));
    }
}
