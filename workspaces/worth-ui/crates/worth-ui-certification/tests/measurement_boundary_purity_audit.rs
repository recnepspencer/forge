use std::path::Path;

use worth_ui_certification::topology::{
    audit_measurement_forbidden_host_authority_denial_surface,
    audit_measurement_host_request_surface, certify_measurement_host_boundary_purity,
};

fn workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crate parent")
        .parent()
        .expect("workspace root")
}

#[test]
fn measurement_host_boundary_purity_is_machine_checked() {
    certify_measurement_host_boundary_purity(workspace_root())
        .expect("measurement host boundary purity should stay certified");
}

#[test]
fn measurement_host_request_surface_stays_closed_and_forbidden_asks_stay_explicit() {
    let mut violations = audit_measurement_host_request_surface(workspace_root());
    violations.extend(audit_measurement_forbidden_host_authority_denial_surface());
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}
