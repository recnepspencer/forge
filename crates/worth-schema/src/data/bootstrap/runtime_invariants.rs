use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum BootstrapRuntimeInvariant {
    OwnershipSurface,
    LoopWiring,
    RadialSurface,
    WireConnectivity,
    VertexBranching,
    ShellClosureSurface,
    NamingCoverage,
}

impl BootstrapRuntimeInvariant {
    pub const ALL: [Self; 7] = [
        Self::OwnershipSurface,
        Self::LoopWiring,
        Self::RadialSurface,
        Self::WireConnectivity,
        Self::VertexBranching,
        Self::ShellClosureSurface,
        Self::NamingCoverage,
    ];
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapRuntimeInvariantPlan {
    pub topology: Vec<BootstrapRuntimeInvariant>,
}

impl BootstrapRuntimeInvariantPlan {
    pub fn all_invariants(&self) -> Vec<BootstrapRuntimeInvariant> {
        self.topology.clone()
    }
}

pub fn bootstrap_runtime_invariant_plan() -> BootstrapRuntimeInvariantPlan {
    BootstrapRuntimeInvariantPlan {
        topology: BootstrapRuntimeInvariant::ALL.to_vec(),
    }
}
