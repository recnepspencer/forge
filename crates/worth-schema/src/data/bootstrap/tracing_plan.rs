use crate::data::aspects::{
    Aspect, DiagnosticsAspect, GeometryAspect, LineageAspect, NamingAspect, TopologyAspect,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapTracingPlan {
    pub topology: Vec<TopologyAspect>,
    pub geometry: Vec<GeometryAspect>,
    pub lineage: Vec<LineageAspect>,
    pub naming: Vec<NamingAspect>,
    pub diagnostics: Vec<DiagnosticsAspect>,
}

impl BootstrapTracingPlan {
    pub fn all_aspects(&self) -> Vec<Aspect> {
        self.topology
            .iter()
            .copied()
            .map(Aspect::Topology)
            .chain(self.geometry.iter().copied().map(Aspect::Geometry))
            .chain(self.lineage.iter().copied().map(Aspect::Lineage))
            .chain(self.naming.iter().copied().map(Aspect::Naming))
            .chain(self.diagnostics.iter().copied().map(Aspect::Diagnostics))
            .collect()
    }
}

pub fn bootstrap_tracing_plan() -> BootstrapTracingPlan {
    BootstrapTracingPlan {
        topology: TopologyAspect::ALL.to_vec(),
        geometry: GeometryAspect::ALL.to_vec(),
        lineage: LineageAspect::ALL.to_vec(),
        naming: NamingAspect::ALL.to_vec(),
        diagnostics: DiagnosticsAspect::ALL.to_vec(),
    }
}
