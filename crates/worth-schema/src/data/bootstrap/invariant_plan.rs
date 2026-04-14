use crate::data::invariants::{
    WorthDiagnosticsInvariantGroup, WorthGeometryInvariantGroup, WorthInvariantGroup,
    WorthLineageInvariantGroup, WorthNamingInvariantGroup, WorthTopologyInvariantGroup,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorthBootstrapInvariantPlan {
    pub topology: Vec<WorthTopologyInvariantGroup>,
    pub geometry: Vec<WorthGeometryInvariantGroup>,
    pub lineage: Vec<WorthLineageInvariantGroup>,
    pub naming: Vec<WorthNamingInvariantGroup>,
    pub diagnostics: Vec<WorthDiagnosticsInvariantGroup>,
}

impl WorthBootstrapInvariantPlan {
    pub fn all_groups(&self) -> Vec<WorthInvariantGroup> {
        self.topology
            .iter()
            .copied()
            .map(WorthInvariantGroup::Topology)
            .chain(
                self.geometry
                    .iter()
                    .copied()
                    .map(WorthInvariantGroup::Geometry),
            )
            .chain(
                self.lineage
                    .iter()
                    .copied()
                    .map(WorthInvariantGroup::Lineage),
            )
            .chain(self.naming.iter().copied().map(WorthInvariantGroup::Naming))
            .chain(
                self.diagnostics
                    .iter()
                    .copied()
                    .map(WorthInvariantGroup::Diagnostics),
            )
            .collect()
    }
}

pub fn worth_bootstrap_invariant_plan() -> WorthBootstrapInvariantPlan {
    WorthBootstrapInvariantPlan {
        topology: WorthTopologyInvariantGroup::ALL.to_vec(),
        geometry: WorthGeometryInvariantGroup::ALL.to_vec(),
        lineage: WorthLineageInvariantGroup::ALL.to_vec(),
        naming: WorthNamingInvariantGroup::ALL.to_vec(),
        diagnostics: WorthDiagnosticsInvariantGroup::ALL.to_vec(),
    }
}
