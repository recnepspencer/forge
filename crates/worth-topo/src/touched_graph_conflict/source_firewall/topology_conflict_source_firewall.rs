use std::path::PathBuf;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthTopologyTouchedGraphConflictSourceFirewallRegion;

impl WorthTopologyTouchedGraphConflictSourceFirewallRegion {
    pub const fn region_label() -> &'static str {
        "topology_touched_graph_conflict"
    }

    pub const fn root_identity() -> &'static str {
        "worth-topo:touched-graph-conflict"
    }

    pub fn scan_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src")
    }
}
