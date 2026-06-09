use crate::platform_entry_closeout::docs_coverage_alignment_audit_from_audit;
use crate::public_doc_coverage::{
    ForgeQueryPublicDocCoverageAudit, ForgeQueryPublicDocCoverageInventory,
};

use super::support::{
    current_row, inventory_with_extra_row, inventory_with_replaced_row,
    inventory_without_golden_label, inventory_without_public_name, orphan_row,
    row_with_doc_reference, row_with_golden, row_without_golden, row_without_journey,
    row_without_readme,
};
use crate::public_doc_coverage::forge_query_public_doc_coverage_golden_transcripts;

const PUBLIC_DOC_COVERAGE_DOC: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/domain-capabilities/public-doc-coverage.md"
));
const PLATFORM_ENTRY_CLOSEOUT_DOC: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/domain-capabilities/platform-entry-closeout.md"
));

#[test]
fn public_doc_coverage_docs_name_every_runtime_audit_drift_class_exactly_once() {
    let expected_doc_lines = [
        "- live public surfaces with no valid feature-doc coverage",
        "- live public surfaces with no valid surface-coverage golden",
        "- coverage rows for surfaces that no longer exist",
        "- surface-coverage golden manifest rows that are no longer used by live rows",
        "- README discovery labels that are missing from the docs index",
        "- journey mismatches between a coverage row and its golden",
    ];

    for line in expected_doc_lines {
        assert!(
            PUBLIC_DOC_COVERAGE_DOC.contains(line),
            "public doc coverage doc must name drift class `{line}`"
        );
    }

    assert!(
        PLATFORM_ENTRY_CLOSEOUT_DOC.contains("- docs and golden breadth from public doc coverage")
    );
    assert!(PLATFORM_ENTRY_CLOSEOUT_DOC.contains("- `docs_coverage_alignment().gaps()`"));
}

#[test]
fn alignment_gap_prefixes_cover_every_public_doc_audit_failure_family() {
    let scenarios = [
        (
            ForgeQueryPublicDocCoverageAudit::from_inventory(&inventory_without_public_name(
                "orchestrate_signal_compatibility",
            )),
            "undocumented:orchestrate_signal_compatibility",
        ),
        (
            ForgeQueryPublicDocCoverageAudit::from_inventory(&inventory_with_replaced_row(
                row_without_golden(&current_row("prepare_continuation_from_target")),
            )),
            "missing_golden:prepare_continuation_from_target",
        ),
        (
            ForgeQueryPublicDocCoverageAudit::from_inventory(&inventory_with_extra_row(
                orphan_row(),
            )),
            "orphan_doc:fake_surface",
        ),
        (
            ForgeQueryPublicDocCoverageAudit::from_inventory(&inventory_without_golden_label(
                "grouped_authoring_surface_readout",
            )),
            "orphan_golden:grouped_authoring_surface_readout",
        ),
        (
            ForgeQueryPublicDocCoverageAudit::from_inventory(&inventory_with_replaced_row(
                row_without_readme(&current_row("orchestrate_declaration_entry")),
            )),
            "readme_gap:orchestrate_declaration_entry",
        ),
        (
            ForgeQueryPublicDocCoverageAudit::from_inventory(&inventory_with_replaced_row(
                row_without_journey(&current_row("orchestrate_declaration_entry")),
            )),
            "journey_gap:orchestrate_declaration_entry",
        ),
    ];

    for (audit, expected_gap) in scenarios {
        let alignment = docs_coverage_alignment_audit_from_audit(&audit);
        assert!(
            alignment.gaps().iter().any(|gap| gap == expected_gap),
            "expected `{expected_gap}` in {:?}",
            alignment.gaps()
        );
    }
}

#[test]
fn docs_alignment_stays_exact_for_doc_pointer_and_journey_mismatch_failures() {
    let broken_doc_pointer = ForgeQueryPublicDocCoverageAudit::from_inventory(
        &inventory_with_replaced_row(row_with_doc_reference(
            &current_row("orchestrate_declaration_with_contributions"),
            "crates/forge-query/docs/domain-capabilities/certification/certification-surface-and-closeout-bundle.md",
            "missing section",
        )),
    );
    let broken_doc_alignment = docs_coverage_alignment_audit_from_audit(&broken_doc_pointer);
    assert_eq!(
        broken_doc_alignment.gaps(),
        ["undocumented:orchestrate_declaration_with_contributions"]
    );

    let mismatched_golden = forge_query_public_doc_coverage_golden_transcripts()
        .iter()
        .find(|golden| golden.label() == "signal_compatibility_surface_readout")
        .copied()
        .expect("signal golden should exist");
    let broken_journey = ForgeQueryPublicDocCoverageAudit::from_inventory(
        &inventory_with_replaced_row(row_with_golden(
            &current_row("prepare_preview_for_active_face_selection"),
            mismatched_golden,
        )),
    );
    let broken_journey_alignment = docs_coverage_alignment_audit_from_audit(&broken_journey);
    assert_eq!(
        broken_journey_alignment.gaps(),
        [
            "journey_gap:prepare_preview_for_active_face_selection",
            "missing_golden:prepare_preview_for_active_face_selection",
        ]
    );
}

#[test]
fn current_inventory_audit_and_closeout_alignment_share_one_clean_truth() {
    let inventory = ForgeQueryPublicDocCoverageInventory::current();
    let audit = ForgeQueryPublicDocCoverageAudit::current();
    let alignment = docs_coverage_alignment_audit_from_audit(&audit);

    assert_eq!(
        audit.coverage_digest(),
        inventory.coverage_digest(),
        "audit digest should certify the current runtime-backed inventory"
    );
    assert_eq!(alignment.digest(), audit.coverage_digest());
    assert!(alignment.is_aligned());
    assert!(alignment.gaps().is_empty());
}
