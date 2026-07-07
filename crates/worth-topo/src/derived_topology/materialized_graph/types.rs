use serde::{Deserialize, Serialize};

use crate::brep::topology_graph::TopologyView;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaterializationFallbackClass {
    WholeViewRebuild,
    CompleteTopologyBootstrap,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaterializationBreadthReport {
    pub entity_count: usize,
    pub relation_count: usize,
    pub topology_entity_count: usize,
    pub topology_relation_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaterializationReport {
    pub breadth: MaterializationBreadthReport,
    pub whole_view_materialization: bool,
    pub fallback_class: Option<MaterializationFallbackClass>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaterializedTopologyView {
    topology: TopologyView,
    report: MaterializationReport,
}

impl MaterializedTopologyView {
    pub(crate) fn new(topology: TopologyView, report: MaterializationReport) -> Self {
        Self { topology, report }
    }

    pub(crate) fn from_complete_topology_view(topology: TopologyView) -> Self {
        let topology_entity_count = topology.models.len()
            + topology.bodies.len()
            + topology.lumps.len()
            + topology.regions.len()
            + topology.shells.len()
            + topology.faces.len()
            + topology.loops.len()
            + topology.wires.len()
            + topology.half_edges.len()
            + topology.edges.len()
            + topology.vertices.len();
        Self {
            topology,
            report: MaterializationReport {
                breadth: MaterializationBreadthReport {
                    entity_count: topology_entity_count,
                    relation_count: 0,
                    topology_entity_count,
                    topology_relation_count: 0,
                },
                whole_view_materialization: true,
                fallback_class: Some(MaterializationFallbackClass::CompleteTopologyBootstrap),
            },
        }
    }

    #[cfg(test)]
    pub(crate) fn whole_view(topology: TopologyView) -> Self {
        Self::from_complete_topology_view(topology)
    }

    pub fn topology(&self) -> &TopologyView {
        &self.topology
    }

    pub fn report(&self) -> &MaterializationReport {
        &self.report
    }

    #[cfg(test)]
    pub(crate) fn topology_mut(&mut self) -> &mut TopologyView {
        &mut self.topology
    }
}
