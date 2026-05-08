use crate::data::invariants::{
    DiagnosticsInvariantGroup, GeometryInvariantGroup, InvariantGroup, LineageInvariantGroup,
    NamingInvariantGroup, TopologyInvariantGroup,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapInvariantPlan {
    pub topology: Vec<TopologyInvariantGroup>,
    pub geometry: Vec<GeometryInvariantGroup>,
    pub lineage: Vec<LineageInvariantGroup>,
    pub naming: Vec<NamingInvariantGroup>,
    pub diagnostics: Vec<DiagnosticsInvariantGroup>,
}

impl BootstrapInvariantPlan {
    pub fn all_groups(&self) -> Vec<InvariantGroup> {
        self.topology
            .iter()
            .copied()
            .map(InvariantGroup::Topology)
            .chain(self.geometry.iter().copied().map(InvariantGroup::Geometry))
            .chain(self.lineage.iter().copied().map(InvariantGroup::Lineage))
            .chain(self.naming.iter().copied().map(InvariantGroup::Naming))
            .chain(
                self.diagnostics
                    .iter()
                    .copied()
                    .map(InvariantGroup::Diagnostics),
            )
            .collect()
    }
}

pub fn bootstrap_invariant_plan() -> BootstrapInvariantPlan {
    BootstrapInvariantPlan {
        topology: TopologyInvariantGroup::ALL.to_vec(),
        geometry: GeometryInvariantGroup::ALL.to_vec(),
        lineage: LineageInvariantGroup::ALL.to_vec(),
        naming: NamingInvariantGroup::ALL.to_vec(),
        diagnostics: DiagnosticsInvariantGroup::ALL.to_vec(),
    }
}
