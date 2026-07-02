mod admission_error;
mod current_input;
mod diagnostic_input;
mod selected_route_authority;
mod source;

#[cfg(test)]
mod tests;

pub use admission_error::TopologyDerivedReadDiagnosticInputAdmissionError;
pub(crate) use diagnostic_input::admit_topology_derived_read_diagnostic_input;
pub use diagnostic_input::TopologyDerivedReadDiagnosticInput;
pub(crate) use source::{
    build_derived_fallback_report, build_derived_fallback_report_from_counts,
    build_derived_invalidation_report, build_derived_invalidation_report_from_aspects,
    build_derived_read_diagnostics, build_derived_rebuild_report,
    derive_topology_validation_report, derived_validation_execution_report,
    topology_derived_diagnostic_projection_source, TopologyDerivedDiagnosticProjectionSource,
};

#[cfg(any(feature = "kernel-diagnostic-support", test))]
pub use current_input::{
    current_topology_derived_read_diagnostic_input_with_selected_route_authority,
    TopologyDerivedReadDiagnosticInputCurrentError,
};

#[cfg(any(feature = "kernel-diagnostic-support", test))]
pub use selected_route_authority::TopologyDerivedReadDiagnosticSelectedRouteAuthority;
