use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLiveGraphReadMaintenanceBudget {
    max_touched_edges: usize,
    max_touched_frontiers: usize,
    max_requirement_rows: usize,
    admits_snapshot_refresh: bool,
    digest: String,
}

impl ForgeQueryLiveGraphReadMaintenanceBudget {
    pub fn bounded() -> Self {
        Self::new(64, 16, 16, false)
    }

    pub fn bounded_with_snapshot_refresh() -> Self {
        Self::new(256, 64, 64, true)
    }

    pub fn strict_incremental(
        max_touched_edges: usize,
        max_touched_frontiers: usize,
        max_requirement_rows: usize,
    ) -> Self {
        Self::new(
            max_touched_edges,
            max_touched_frontiers,
            max_requirement_rows,
            false,
        )
    }

    fn new(
        max_touched_edges: usize,
        max_touched_frontiers: usize,
        max_requirement_rows: usize,
        admits_snapshot_refresh: bool,
    ) -> Self {
        let digest = hash_parts(&[
            "forge_query_live_graph_read_maintenance_budget_v1".to_string(),
            format!("max_edges:{max_touched_edges}"),
            format!("max_frontiers:{max_touched_frontiers}"),
            format!("max_requirement_rows:{max_requirement_rows}"),
            format!("snapshot_refresh:{admits_snapshot_refresh}"),
        ]);
        Self {
            max_touched_edges,
            max_touched_frontiers,
            max_requirement_rows,
            admits_snapshot_refresh,
            digest,
        }
    }

    pub fn max_touched_edges(&self) -> usize {
        self.max_touched_edges
    }

    pub fn max_touched_frontiers(&self) -> usize {
        self.max_touched_frontiers
    }

    pub fn max_requirement_rows(&self) -> usize {
        self.max_requirement_rows
    }

    pub fn admits_snapshot_refresh(&self) -> bool {
        self.admits_snapshot_refresh
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}
