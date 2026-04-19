pub type WorthWireInterpretation = worth_schema::facade::WorthWireInterpretationRecord;
pub type WorthShellInterpretation = worth_schema::facade::WorthShellInterpretationRecord;
pub type WorthTopologyInterpretationSet =
    worth_schema::facade::WorthTopologyInterpretationRecordSet;

use forge_relational::facade::identity::EntityId;
use serde::{Deserialize, Serialize};

use crate::materialization::MaterializedTopologyView;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthBoundaryInterpretationSummary {
    pub shell_id: EntityId,
    pub boundary_component_count: usize,
    pub boundary_half_edge_count: usize,
    pub closed_boundary: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthRadialInterpretationSummary {
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
    interpretations: WorthTopologyInterpretationSet,
    boundary_summaries: Vec<WorthBoundaryInterpretationSummary>,
    radial_summaries: Vec<WorthRadialInterpretationSummary>,
    report: InterpretationReport,
}

impl InterpretedTopologyView {
    pub(crate) fn new(
        materialized: MaterializedTopologyView,
        interpretations: WorthTopologyInterpretationSet,
        boundary_summaries: Vec<WorthBoundaryInterpretationSummary>,
        radial_summaries: Vec<WorthRadialInterpretationSummary>,
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

    pub fn interpretations(&self) -> &WorthTopologyInterpretationSet {
        &self.interpretations
    }

    pub fn boundary_summaries(&self) -> &[WorthBoundaryInterpretationSummary] {
        &self.boundary_summaries
    }

    pub fn radial_summaries(&self) -> &[WorthRadialInterpretationSummary] {
        &self.radial_summaries
    }

    pub fn report(&self) -> &InterpretationReport {
        &self.report
    }
}
