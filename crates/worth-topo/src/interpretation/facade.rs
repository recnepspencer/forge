use crate::data::topology_view::WorthTopologyView;
use crate::interpretation::shell::interpret_shells;
use crate::interpretation::types::WorthTopologyInterpretationSet;
use crate::interpretation::wire::interpret_wires;
use worth_schema::facade::{
    CertifiedTopologyInterpretation, DerivedTopologyReadBasis, WorthTopologyReadArtifact,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct WorthTopologyInterpreter;

impl WorthTopologyInterpreter {
    pub fn interpret(view: &WorthTopologyView) -> WorthTopologyInterpretationSet {
        WorthTopologyInterpretationSet {
            wires: interpret_wires(view),
            shells: interpret_shells(view),
        }
    }
}

pub fn interpret_topology_view(view: &WorthTopologyView) -> WorthTopologyInterpretationSet {
    WorthTopologyInterpreter::interpret(view)
}

pub fn build_topology_read_artifact(
    read_basis: &DerivedTopologyReadBasis,
    view: &WorthTopologyView,
) -> WorthTopologyReadArtifact {
    WorthTopologyReadArtifact::from_read_basis_and_interpretation(
        read_basis,
        interpret_topology_view(view),
    )
}

pub fn certify_topology_view(
    read_basis: DerivedTopologyReadBasis,
    view: &WorthTopologyView,
) -> CertifiedTopologyInterpretation {
    CertifiedTopologyInterpretation::from_read_basis_and_interpretation(
        read_basis,
        interpret_topology_view(view),
    )
}
