use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthBootstrapRuntimeInvariant {
    OwnershipSurface,
    LoopWiring,
    RadialSurface,
    WireConnectivity,
    VertexBranching,
    ShellClosureSurface,
    NamingCoverage,
}

impl WorthBootstrapRuntimeInvariant {
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
pub struct WorthBootstrapRuntimeInvariantPlan {
    pub topology: Vec<WorthBootstrapRuntimeInvariant>,
}

impl WorthBootstrapRuntimeInvariantPlan {
    pub fn all_invariants(&self) -> Vec<WorthBootstrapRuntimeInvariant> {
        self.topology.clone()
    }
}

pub fn worth_bootstrap_runtime_invariant_plan() -> WorthBootstrapRuntimeInvariantPlan {
    WorthBootstrapRuntimeInvariantPlan {
        topology: WorthBootstrapRuntimeInvariant::ALL.to_vec(),
    }
}
