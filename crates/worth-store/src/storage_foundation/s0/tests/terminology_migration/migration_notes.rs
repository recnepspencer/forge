use super::super::support::{digest, metadata, semantic_cleanup_row};
use crate::storage_foundation::s0::{SemanticPhysicalClaimStatus, TestMigrationNotes};

#[test]
fn phase1_test_migration_notes_classify_named_suite_scope() {
    let report = TestMigrationNotes::from_milestone_rows(
        "source:rev:a",
        digest("roadmap:digest"),
        "worth-store-s0",
        metadata("migration"),
        &[semantic_cleanup_row()],
    )
    .unwrap();

    assert_eq!(report.rows().len(), 1);
    assert_eq!(
        report.rows()[0].evidence_scope(),
        SemanticPhysicalClaimStatus::PhysicalDebt
    );
}
