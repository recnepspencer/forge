use worth_ui_certification::topology::{
    audit_legacy_crate_dispositions, audit_legacy_public_surface_narrowing,
    audit_legacy_shim_honesty, audit_no_parallel_legacy_authority,
};

fn workspace_root() -> &'static worth_ui_certification::topology::WorkspaceSourceInventory {
    super::workspace_source_inventory()
}

#[test]
fn every_legacy_ui_crate_has_an_explicit_disposition() {
    let violations = audit_legacy_crate_dispositions(workspace_root());
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

#[test]
fn no_parallel_legacy_authority_paths_survive() {
    let violations = audit_no_parallel_legacy_authority(workspace_root());
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

#[test]
fn legacy_phase_2_uses_no_semantic_shims() {
    let violations = audit_legacy_shim_honesty(workspace_root());
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

#[test]
fn legacy_public_surfaces_narrow_or_retire_instead_of_duplicating() {
    let violations = audit_legacy_public_surface_narrowing(workspace_root());
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}
