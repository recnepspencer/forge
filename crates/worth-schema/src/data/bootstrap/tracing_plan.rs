use crate::data::aspects::{
    WorthAspect, WorthDiagnosticsAspect, WorthGeometryAspect, WorthLineageAspect,
    WorthNamingAspect, WorthTopologyAspect,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorthBootstrapTracingPlan {
    pub topology: Vec<WorthTopologyAspect>,
    pub geometry: Vec<WorthGeometryAspect>,
    pub lineage: Vec<WorthLineageAspect>,
    pub naming: Vec<WorthNamingAspect>,
    pub diagnostics: Vec<WorthDiagnosticsAspect>,
}

impl WorthBootstrapTracingPlan {
    pub fn all_aspects(&self) -> Vec<WorthAspect> {
        self.topology
            .iter()
            .copied()
            .map(WorthAspect::Topology)
            .chain(self.geometry.iter().copied().map(WorthAspect::Geometry))
            .chain(self.lineage.iter().copied().map(WorthAspect::Lineage))
            .chain(self.naming.iter().copied().map(WorthAspect::Naming))
            .chain(
                self.diagnostics
                    .iter()
                    .copied()
                    .map(WorthAspect::Diagnostics),
            )
            .collect()
    }
}

pub fn worth_bootstrap_tracing_plan() -> WorthBootstrapTracingPlan {
    WorthBootstrapTracingPlan {
        topology: WorthTopologyAspect::ALL.to_vec(),
        geometry: WorthGeometryAspect::ALL.to_vec(),
        lineage: WorthLineageAspect::ALL.to_vec(),
        naming: WorthNamingAspect::ALL.to_vec(),
        diagnostics: WorthDiagnosticsAspect::ALL.to_vec(),
    }
}
