mod error;
mod facade;
mod loop_wiring;
mod naming;
mod ownership;
mod radial_rings;
pub mod reference_integrity;
mod shared;
mod shell_closure;
mod tests;
mod vertex_disks;

pub use error::TopologyValidationError;
pub use facade::{
    validate_interpreted_topology, validate_named_topology_truth,
    DerivedTopologyValidationReport, TopologyValidationPhase, TopologyValidationReport,
};
