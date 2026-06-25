use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum DerivedTopologyProductFamilyIdentity {
    MaterializedGraph,
    TraversalViews,
    LoopCycles,
    RadialRings,
    ShellViews,
    VertexDisks,
    WireViews,
}

impl DerivedTopologyProductFamilyIdentity {
    pub const REQUIRED: [Self; 7] = [
        Self::MaterializedGraph,
        Self::TraversalViews,
        Self::LoopCycles,
        Self::RadialRings,
        Self::ShellViews,
        Self::VertexDisks,
        Self::WireViews,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MaterializedGraph => "materialized_graph",
            Self::TraversalViews => "traversal_views",
            Self::LoopCycles => "loop_cycles",
            Self::RadialRings => "radial_rings",
            Self::ShellViews => "shell_views",
            Self::VertexDisks => "vertex_disks",
            Self::WireViews => "wire_views",
        }
    }
}
