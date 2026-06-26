use std::path::{Path, PathBuf};

use super::super::closeout::closeout_for_workspace_root;
use super::super::errors::WorthGraphReadAccessSpatialDensePostureErrorKind;
use super::super::source_firewall::reject_spatial_dense_local_graph_read_residue;
use super::{production_phase_five_closeout, production_phase_five_seed};

#[test]
fn source_firewall_rejects_spatial_dense_local_fallbacks() {
    let violation = reject_spatial_dense_local_graph_read_residue(
        "crates/worth-spatial/src/planar_boolean.rs",
        "fn bad() { local_spatial_evidence_graph_read_fallback(); }",
    )
    .expect_err("local spatial fallback must be rejected");

    assert_eq!(
        violation.forbidden_pattern(),
        "local_spatial_evidence_graph_read_fallback"
    );
}

#[test]
fn closeout_exports_clean_source_firewall_report() {
    let closeout = production_phase_five_closeout();

    assert!(closeout.source_firewall_report().scanned_region_count() > 0);
    assert!(closeout.source_firewall_report().scanned_source_count() > 0);
    assert_eq!(closeout.source_firewall_report().violation_count(), 0);
    assert!(closeout.source_firewall_report().forbidden_pattern_count() >= 8);
}

#[test]
fn closeout_rejects_workspace_source_residue() {
    let workspace_root = temp_workspace_with_spatial_dense_residue();
    let error = closeout_for_workspace_root(&production_phase_five_seed(), &workspace_root)
        .expect_err("Phase 5 closeout must fail when scanned workspace contains residue");

    assert_eq!(
        error.kind(),
        WorthGraphReadAccessSpatialDensePostureErrorKind::SourceFirewallViolation
    );
}

fn temp_workspace_with_spatial_dense_residue() -> PathBuf {
    let workspace_root = std::env::temp_dir().join(format!(
        "worth_phase_five_source_firewall_{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&workspace_root);
    let residue_root = workspace_root
        .join("crates")
        .join("worth-spatial")
        .join("src");
    std::fs::create_dir_all(&residue_root).expect("temp spatial source root should be creatable");
    std::fs::write(
        residue_root.join("residue.rs"),
        "fn residue() { local_spatial_evidence_graph_read_fallback(); }",
    )
    .expect("temp residue source should be writable");
    write_empty_source_root(
        &workspace_root
            .join("crates")
            .join("worth-kernel")
            .join("src")
            .join("graph_read_access_plan_adoption"),
    );
    write_empty_source_root(
        &workspace_root
            .join("crates")
            .join("worth-topo")
            .join("src")
            .join("projection"),
    );
    workspace_root
}

fn write_empty_source_root(root: &Path) {
    std::fs::create_dir_all(root).expect("temp source root should be creatable");
    std::fs::write(root.join("clean.rs"), "").expect("temp clean source should be writable");
}
