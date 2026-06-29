use std::path::PathBuf;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthSpatialTouchedGraphConflictSourceFirewallRegion;

impl WorthSpatialTouchedGraphConflictSourceFirewallRegion {
    pub const fn region_label() -> &'static str {
        "spatial_touched_graph_conflict"
    }

    pub const fn root_identity() -> &'static str {
        "worth-spatial:touched-graph-conflict"
    }

    pub fn scan_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src")
    }
}
