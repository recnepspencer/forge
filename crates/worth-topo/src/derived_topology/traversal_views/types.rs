pub type WireInterpretation = schema::facade::WireInterpretationRecord;
pub type ShellInterpretation = schema::facade::ShellInterpretationRecord;
pub type TopologyInterpretationSet = schema::facade::TopologyInterpretationRecordSet;

use forge_relational::facade::identity::EntityId;
use serde::{Deserialize, Serialize};

use crate::derived_topology::materialized_graph::MaterializedTopologyView;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryInterpretationSummary {
    pub shell_id: EntityId,
    pub boundary_component_count: usize,
    pub boundary_half_edge_count: usize,
    pub closed_boundary: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RadialInterpretationSummary {
    pub shell_id: EntityId,
    pub boundary_half_edge_count: usize,
    pub non_manifold_edge_ids: Vec<EntityId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InterpretationReport {
    pub interpreted_wire_count: usize,
    pub interpreted_shell_count: usize,
    pub boundary_interpretation_count: usize,
    pub radial_interpretation_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InterpretedTopologyView {
    materialized: MaterializedTopologyView,
    interpretations: TopologyInterpretationSet,
    boundary_summaries: Vec<BoundaryInterpretationSummary>,
    radial_summaries: Vec<RadialInterpretationSummary>,
    report: InterpretationReport,
}

impl InterpretedTopologyView {
    pub(crate) fn new(
        materialized: MaterializedTopologyView,
        interpretations: TopologyInterpretationSet,
        boundary_summaries: Vec<BoundaryInterpretationSummary>,
        radial_summaries: Vec<RadialInterpretationSummary>,
        report: InterpretationReport,
    ) -> Self {
        Self {
            materialized,
            interpretations,
            boundary_summaries,
            radial_summaries,
            report,
        }
    }

    pub fn materialized(&self) -> &MaterializedTopologyView {
        &self.materialized
    }

    pub fn interpretations(&self) -> &TopologyInterpretationSet {
        &self.interpretations
    }

    pub fn boundary_summaries(&self) -> &[BoundaryInterpretationSummary] {
        &self.boundary_summaries
    }

    pub fn radial_summaries(&self) -> &[RadialInterpretationSummary] {
        &self.radial_summaries
    }

    pub fn report(&self) -> &InterpretationReport {
        &self.report
    }
}
