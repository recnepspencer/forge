mod boundary;
mod facade;
mod radial;
mod shells;
#[cfg(test)]
mod tests;
mod types;
mod vertex_branching;
mod wires;

pub use facade::{
    build_topology_read_artifact, certify_topology_view, interpret_topology_view,
    TopologyInterpreter,
};
pub use types::{
    BoundaryInterpretationSummary, InterpretationReport, InterpretedTopologyView,
    RadialInterpretationSummary, ShellInterpretation, TopologyInterpretationSet,
    WireInterpretation,
};
