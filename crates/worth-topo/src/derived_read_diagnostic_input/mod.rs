pub use crate::projection::planner_owned_routing::diagnostic_projection_input::{
    TopologyDerivedReadDiagnosticInput, TopologyDerivedReadDiagnosticInputAdmissionError,
};

#[cfg(any(feature = "kernel-diagnostic-support", test))]
#[doc(hidden)]
pub mod support {
    pub use crate::projection::planner_owned_routing::diagnostic_projection_input::{
        current_topology_derived_read_diagnostic_input_with_selected_route_authority,
        TopologyDerivedReadDiagnosticInputCurrentError,
        TopologyDerivedReadDiagnosticSelectedRouteAuthority,
    };
}
