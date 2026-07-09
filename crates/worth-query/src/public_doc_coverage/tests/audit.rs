use crate::public_doc_coverage::WorthQueryPublicDocCoverageAudit;

use super::support::{
    current_row, inventory_with_extra_row, inventory_with_replaced_row,
    inventory_without_golden_label, inventory_without_public_name, orphan_row,
    row_with_doc_reference, row_with_golden, row_without_golden, row_without_journey,
    row_without_readme,
};
use crate::public_doc_coverage::worth_query_public_doc_coverage_golden_transcripts;

#[test]
fn current_audit_is_clean() {
    let audit = WorthQueryPublicDocCoverageAudit::current();

    assert!(audit.undocumented_public_surfaces().is_empty());
    assert!(audit.surfaces_missing_goldens().is_empty());
    assert!(audit.orphan_doc_rows().is_empty());
    assert!(audit.orphan_golden_rows().is_empty());
    assert!(audit.readme_discovery_gaps().is_empty());
    assert!(audit.journey_coverage_gaps().is_empty());
    assert!(!audit.coverage_digest().is_empty());
}

#[test]
fn audit_flags_dropped_real_surface_coverage_as_undocumented() {
    let coverage = inventory_without_public_name("orchestrate_signal_compatibility");
    let audit = WorthQueryPublicDocCoverageAudit::from_inventory(&coverage);

    assert_eq!(
        audit.undocumented_public_surfaces(),
        ["orchestrate_signal_compatibility"]
    );
}

#[test]
fn audit_flags_missing_golden_transcripts() {
    let coverage = inventory_with_replaced_row(row_without_golden(&current_row(
        "prepare_continuation_from_target",
    )));
    let audit = WorthQueryPublicDocCoverageAudit::from_inventory(&coverage);

    assert_eq!(
        audit.surfaces_missing_goldens(),
        ["prepare_continuation_from_target"]
    );
}

#[test]
fn audit_flags_orphan_doc_and_golden_rows() {
    let coverage = inventory_with_extra_row(orphan_row());
    let audit = WorthQueryPublicDocCoverageAudit::from_inventory(&coverage);

    assert_eq!(audit.orphan_doc_rows(), ["fake_surface"]);
    assert!(audit.orphan_golden_rows().is_empty());
}

#[test]
fn audit_flags_readme_and_journey_gaps() {
    let without_readme = inventory_with_replaced_row(row_without_readme(&current_row(
        "orchestrate_declaration_entry",
    )));
    let readme_audit = WorthQueryPublicDocCoverageAudit::from_inventory(&without_readme);
    assert_eq!(
        readme_audit.readme_discovery_gaps(),
        ["orchestrate_declaration_entry"]
    );

    let without_journey = inventory_with_replaced_row(row_without_journey(&current_row(
        "orchestrate_declaration_entry",
    )));
    let journey_audit = WorthQueryPublicDocCoverageAudit::from_inventory(&without_journey);
    assert_eq!(
        journey_audit.journey_coverage_gaps(),
        ["orchestrate_declaration_entry"]
    );

    let mismatched_golden = worth_query_public_doc_coverage_golden_transcripts()
        .iter()
        .find(|golden| golden.label() == "signal_compatibility_surface_readout")
        .copied()
        .expect("signal golden should exist");
    let mismatched_coverage = inventory_with_replaced_row(row_with_golden(
        &current_row("prepare_preview_for_active_face_selection"),
        mismatched_golden,
    ));
    let mismatch_audit = WorthQueryPublicDocCoverageAudit::from_inventory(&mismatched_coverage);
    assert_eq!(
        mismatch_audit.journey_coverage_gaps(),
        ["prepare_preview_for_active_face_selection"]
    );
}

#[test]
fn audit_flags_doc_rows_that_no_longer_point_to_real_teaching_content() {
    let coverage = inventory_with_replaced_row(row_with_doc_reference(
        &current_row("orchestrate_declaration_with_contributions"),
        "crates/worth-query/docs/domain-capabilities/certification/certification-surface-and-closeout-bundle.md",
        "missing section",
    ));
    let audit = WorthQueryPublicDocCoverageAudit::from_inventory(&coverage);

    assert_eq!(
        audit.undocumented_public_surfaces(),
        ["orchestrate_declaration_with_contributions"]
    );
}

#[test]
fn audit_flags_unused_surface_golden_manifest_rows() {
    let coverage = inventory_without_golden_label("grouped_authoring_surface_readout");
    let audit = WorthQueryPublicDocCoverageAudit::from_inventory(&coverage);

    assert_eq!(
        audit.orphan_golden_rows(),
        ["grouped_authoring_surface_readout"]
    );
}
