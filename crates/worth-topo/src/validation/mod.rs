mod error;
mod facade;
mod loop_wiring;
mod naming;
mod ownership;
mod radial_rings;
pub(crate) mod reference_integrity;
mod shared;
mod shell_closure;
mod tests;
mod vertex_disks;

pub use error::TopologyValidationError;
pub use facade::{
    topology_validation_report, validate_interpreted_topology, validate_materialized_topology,
    validate_named_topology_truth, validate_topology_view, DerivedTopologyValidationReport,
    TopologyValidationInputClass, TopologyValidationPhase, TopologyValidationReport,
    TopologyValidationRow, TopologyValidator,
};
