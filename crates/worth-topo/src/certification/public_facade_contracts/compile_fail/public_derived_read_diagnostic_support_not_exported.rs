use topology::derived_read_diagnostic_input::support::{
    current_topology_derived_read_diagnostic_input_with_selected_route_authority,
    TopologyDerivedReadDiagnosticInputCurrentError,
    TopologyDerivedReadDiagnosticSelectedRouteAuthority,
};

fn main() {
    let _ = current_topology_derived_read_diagnostic_input_with_selected_route_authority;
    let _ = TopologyDerivedReadDiagnosticInputCurrentError::detail;
    let _ = TopologyDerivedReadDiagnosticSelectedRouteAuthority::from_selected_route_identities;
}
