use crate::derived_topology::loop_cycles::interpret_boundaries;
use crate::derived_topology::materialized_graph::MaterializedTopologyView;
use crate::derived_topology::shell_views::interpret_shells;
use crate::derived_topology::traversal_views::types::{
    InterpretationReport, InterpretedTopologyView, TopologyInterpretationSet,
};
use crate::derived_topology::wire_views::interpret_wires;
use schema::facade::{
    CertifiedTopologyInterpretation, DerivedTopologyReadBasis, TopologyReadArtifact,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct TopologyInterpreter;

impl TopologyInterpreter {
    pub fn interpret(view: &MaterializedTopologyView) -> InterpretedTopologyView {
        let wires = interpret_wires(view);
        let (shells, radial_summaries) = interpret_shells(view);
        let interpretations = TopologyInterpretationSet { wires, shells };
        let boundary_summaries = interpret_boundaries(&interpretations);
        let report = InterpretationReport {
            interpreted_wire_count: interpretations.wires.len(),
            interpreted_shell_count: interpretations.shells.len(),
            boundary_interpretation_count: boundary_summaries.len(),
            radial_interpretation_count: radial_summaries.len(),
        };

        InterpretedTopologyView::new(
            view.clone(),
            interpretations,
            boundary_summaries,
            radial_summaries,
            report,
        )
    }
}

pub fn interpret_topology_view(view: &MaterializedTopologyView) -> InterpretedTopologyView {
    TopologyInterpreter::interpret(view)
}

pub fn build_topology_read_artifact(
    read_basis: &DerivedTopologyReadBasis,
    view: &InterpretedTopologyView,
) -> TopologyReadArtifact {
    TopologyReadArtifact::from_read_basis_and_interpretation(
        read_basis,
        view.interpretations().clone(),
    )
}

pub fn certify_topology_view(
    read_basis: DerivedTopologyReadBasis,
    view: &InterpretedTopologyView,
) -> CertifiedTopologyInterpretation {
    CertifiedTopologyInterpretation::from_read_basis_and_interpretation(
        read_basis,
        view.interpretations().clone(),
    )
}
