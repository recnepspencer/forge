use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthTopologyAspect {
    Structure,
    Ownership,
    Boundary,
    Radial,
}

impl WorthTopologyAspect {
    pub const ALL: [Self; 4] = [
        Self::Structure,
        Self::Ownership,
        Self::Boundary,
        Self::Radial,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Structure => "topology.structure",
            Self::Ownership => "topology.ownership",
            Self::Boundary => "topology.boundary",
            Self::Radial => "topology.radial",
        }
    }
}
