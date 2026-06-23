use forge_query::facade::consumer_kit::{
    hard_prohibition_boundary_audit, ForgeQueryBoundaryAuditFindingKind,
};
use forge_query::facade::ForgeQueryEvidenceScope;
use topology::certification::milestone_one_runtime_builder;
use topology::runtime_support::{topology_runtime, TopologyRuntimeAdapters};

use super::super::query_enforcement_adoption::{
    assert_no_hand_assembled_test_backend_residue, assert_no_query_enforcement_folklore_residue,
    evaluate_reference_support_pins, evaluate_test_backend_residue_audit,
    reference_consumer_enforcement_adoption_report, seeded_query_bypass_source_sets,
    test_backend_adoption_posture, worth_domain_hygiene_classification_report,
    worth_kernel_query_boundary_inventory, worth_kernel_query_boundary_source_count,
    worth_kernel_query_boundary_sources, ReferenceConsumerAdoptionResidueReport,
    TestBackendAdoptionPosture,
};

#[test]
fn reference_consumer_uses_shipped_query_boundary_audit() {
    let inventory =
        worth_kernel_query_boundary_inventory().expect("worth-kernel source inventory seals");
    let report = hard_prohibition_boundary_audit()
        .covering_sources(inventory.boundary_sources())
        .evaluate()
        .expect("worth-kernel query boundary sources must parse");

    report.assert_clean();
    assert_eq!(
        report.source_labels().len(),
        worth_kernel_query_boundary_source_count()
    );
    assert!(
        inventory.source_count() > 10,
        "Phase 8 must audit discovered worth-kernel source files, not only anchors"
    );
    assert!(inventory
        .source_paths()
        .iter()
        .any(|path| path.ends_with("/src/construction/authoring.rs")));
    assert!(inventory
        .source_paths()
        .iter()
        .any(|path| path.ends_with("/src/lib.rs")));
    assert!(
        !inventory
            .inventory_identity()
            .terminal_projection_for_reporting()
            .is_empty(),
        "source inventory must carry its own digest"
    );
    assert_eq!(
        report.report_identity().scope(),
        ForgeQueryEvidenceScope::ConsumerBoundaryAuditReport
    );
    assert!(
        !report.coverage_rows().is_empty(),
        "shipped audit must carry registry-derived coverage rows"
    );
}

#[test]
fn seeded_query_bypasses_fail_through_shipped_audit_artifact() {
    let seeded_sets = seeded_query_bypass_source_sets();
    assert!(
        seeded_sets.len() > 1,
        "seeded bypass coverage must be registry-derived, not one direct-write fixture"
    );

    for seeded_set in seeded_sets {
        let report = hard_prohibition_boundary_audit()
            .covering_sources(seeded_set.sources())
            .evaluate()
            .expect("seeded worth-kernel source inventory must parse");

        let failure = report
            .try_assert_clean()
            .expect_err("seeded Query seam must fail the shipped audit");
        let finding = failure
            .findings()
            .iter()
            .find(|finding| finding.source_label() == seeded_set.source_label())
            .expect("seeded finding should be localized to seeded source");

        assert_eq!(
            finding.kind(),
            ForgeQueryBoundaryAuditFindingKind::ProhibitedSeamUsage
        );
        assert_eq!(finding.seam(), seeded_set.seam());
        assert_eq!(finding.site().source_path(), Some(seeded_set.source_path()));
        assert!(finding.line() > 0);
        assert!(finding.column() > 0);
    }
}

#[test]
fn support_pinning_and_adoption_inventory_are_query_owned_evidence() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.phase-eight.enforcement-adoption".to_string(),
    )
    .expect("workspace");
    let audit_report = hard_prohibition_boundary_audit()
        .covering_sources(worth_kernel_query_boundary_sources())
        .evaluate()
        .expect("worth-kernel query boundary sources must parse");
    audit_report.assert_clean();

    let support_pin_report =
        evaluate_reference_support_pins(&workspace).expect("support pins evaluate");
    support_pin_report
        .assert_satisfied()
        .expect("worth-kernel support pins remain satisfied");
    let backend_residue_report =
        evaluate_test_backend_residue_audit().expect("backend residue audit evaluates");
    backend_residue_report.assert_clean();

    let adoption_report = reference_consumer_enforcement_adoption_report(
        &audit_report,
        &support_pin_report,
        &backend_residue_report,
    )
    .expect("adoption report seals");
    let hygiene_report =
        worth_domain_hygiene_classification_report().expect("hygiene classification seals");

    assert_eq!(
        adoption_report.report_identity().scope(),
        ForgeQueryEvidenceScope::ConsumerEvidenceReport
    );
    assert_eq!(
        hygiene_report.report_identity().scope(),
        ForgeQueryEvidenceScope::ConsumerEvidenceReport
    );
    assert!(
        adoption_report
            .report_identity()
            .terminal_projection_for_reporting()
            .starts_with("forge.query.evidence-identity.v1:"),
        "adoption inventory must be canonical Query evidence"
    );
    assert!(
        hygiene_report
            .report_identity()
            .terminal_projection_for_reporting()
            .starts_with("forge.query.evidence-identity.v1:"),
        "hygiene classification must be canonical Query evidence"
    );
    assert_eq!(hygiene_report.indexed_field_count(), 4);
    assert_eq!(
        adoption_report
            .field("query-boundary-finding-count")
            .expect("query boundary finding count field")
            .as_usize(),
        Some(0)
    );
    assert_eq!(
        adoption_report
            .field("support-pin-finding-count")
            .expect("support pin finding count field")
            .as_usize(),
        Some(0)
    );
    assert_eq!(
        adoption_report
            .field("test-backend-residue-finding-count")
            .expect("test backend residue finding count field")
            .as_usize(),
        Some(0)
    );
    assert_eq!(adoption_report.indexed_field_count(), 13);
}

#[test]
fn query_enforcement_folklore_and_backend_hand_assembly_are_absent() {
    assert_no_query_enforcement_folklore_residue();
    assert_no_hand_assembled_test_backend_residue();
    let residue_report =
        ReferenceConsumerAdoptionResidueReport::current().expect("residue report seals");
    assert_eq!(residue_report.query_owned_residue_count(), 0);
    assert_eq!(residue_report.report_digest_residue_count(), 0);
    assert_eq!(residue_report.prohibition_audit_residue_count(), 0);
    assert_eq!(residue_report.support_pinning_residue_count(), 0);
    assert_eq!(residue_report.test_backend_residue_count(), 0);
    assert_eq!(residue_report.defended_worth_domain_residue_count(), 3);
    assert!(!residue_report
        .evidence_report()
        .report_identity()
        .terminal_projection_for_reporting()
        .is_empty());
    assert_eq!(
        test_backend_adoption_posture(),
        TestBackendAdoptionPosture::NotApplicableNoHandAssemblyResidue
    );
}
