use forge_query::facade::consumer_kit::{
    hard_prohibition_boundary_audit, query_boundary_source_inventory,
    ForgeQueryBoundaryAuditSourceSet, ForgeQueryProhibitedSeam,
};

#[test]
fn public_boundary_audit_dx_accepts_clean_source_set() {
    let report = hard_prohibition_boundary_audit()
        .covering_sources(
            ForgeQueryBoundaryAuditSourceSet::new("worth-kernel").source_file(
                "construction.authoring",
                "src/construction/authoring.rs",
                "fn clean() { admitted_lane(command); }",
            ),
        )
        .assert_clean();

    assert!(report.findings().is_empty());
    assert_eq!(report.crate_name(), "worth-kernel");
}

#[test]
fn public_boundary_audit_dx_reports_seeded_bypass() {
    let failure = hard_prohibition_boundary_audit()
        .covering_sources(real_worth_kernel_source_set().source_file(
            "construction.seeded-bypass",
            "src/construction/seeded_bypass.rs",
            "fn dirty(workspace: &mut ForgeQueryWorkspace) { workspace.write(command); }",
        ))
        .try_assert_clean()
        .expect_err("seeded bypass should fail with typed findings");

    assert_eq!(failure.findings().len(), 1);
    assert_eq!(
        failure.findings()[0].seam(),
        ForgeQueryProhibitedSeam::WorkspaceDirectWrite
    );
    assert_eq!(
        failure.findings()[0].site().source_path(),
        Some("src/construction/seeded_bypass.rs")
    );
    assert!(!failure.report().finding_identities().is_empty());
}

#[test]
fn public_boundary_audit_dx_accepts_real_worth_kernel_source_inventory() {
    let inventory = real_worth_kernel_source_inventory();
    let report = hard_prohibition_boundary_audit()
        .covering_sources(inventory.boundary_sources())
        .try_assert_clean()
        .expect("real worth-kernel source inventory should stay on sanctioned query path");

    assert!(report.findings().is_empty());
    assert!(report.source_labels().len() > 10);
    assert_eq!(report.source_labels().len(), inventory.source_count());
    assert!(report.source_paths().iter().any(|path| path
        .as_deref()
        .is_some_and(|path| path.ends_with("/src/construction/authoring.rs"))));
}

fn real_worth_kernel_source_set() -> ForgeQueryBoundaryAuditSourceSet {
    real_worth_kernel_source_inventory().boundary_sources()
}

fn real_worth_kernel_source_inventory(
) -> forge_query::facade::consumer_kit::ForgeQueryBoundaryAuditSourceInventory {
    query_boundary_source_inventory("worth-kernel")
        .required_root(format!(
            "{}/../worth-kernel/src",
            env!("CARGO_MANIFEST_DIR")
        ))
        .include_rs_files()
        .seal()
        .expect("worth-kernel source inventory should be discoverable")
}
