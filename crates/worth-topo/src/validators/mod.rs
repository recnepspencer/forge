mod error;
mod facade;
mod loop_wiring;
mod naming;
mod reference_integrity;
mod shell_closure;
mod shared;
mod tests;
mod vertex_branching;
mod radial;

pub use error::WorthTopologyValidationError;
pub use facade::{
    topology_validation_report, validate_named_topology_truth, validate_topology_view,
    WorthTopologyValidationReport, WorthTopologyValidationRow, WorthTopologyValidator,
};
