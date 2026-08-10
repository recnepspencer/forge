use super::super::support::{
    digest, evidence_ref, metadata, semantic_cleanup_row, verified_complexity_report,
};
use crate::storage_foundation::s0::{
    BackendForbiddenClaim, BackendForbiddenClaimKind, DeferredPhysicalGuaranteeCategory,
    DeferredPhysicalGuaranteeMap, DeferredPhysicalGuaranteeRow, Roadmap2SequenceId,
    S0ArtifactRowId, S0ArtifactRowStatus, S0ArtifactSubjectKind, S0CounterSnapshot,
    S0DeferredGuaranteeBuildRejection, S0PhysicalStatus, S0RequiredArtifactSet,
};

#[test]
fn phase1_deferred_guarantee_map_extracts_required_s_sequence_rows() {
    let row = semantic_cleanup_row();
    let map = DeferredPhysicalGuaranteeMap::from_milestone_rows(
        "source:rev:a",
        digest("roadmap:digest"),
        "worth-store-s0",
        metadata("deferred"),
        &[row],
    )
    .unwrap();
    let counters = S0CounterSnapshot::from_artifact_and_complexity_reports(
        &S0RequiredArtifactSet::canonical().validate_present_artifacts([]),
        &verified_complexity_report(),
    )
    .with_deferred_guarantee_map(&map);

    assert_eq!(map.rows().len(), 3);
    assert!(map
        .rows()
        .iter()
        .any(|row| row.row_id().as_str().contains("PageSegmentExtentSubstrate")));
    assert!(map
        .rows()
        .iter()
        .any(|row| row.row_id().as_str().contains("PageFrameChunkIntegrity")));
    assert!(map.rows().iter().any(|row| row
        .row_id()
        .as_str()
        .contains("PhysicalDatabaseCertification")));
    assert_eq!(counters.unmapped_deferred_guarantee_count(), 0);
}

#[test]
fn phase1_deferred_guarantee_map_rejects_category_without_required_anchor_sequence() {
    let error = DeferredPhysicalGuaranteeRow::new(
        S0ArtifactRowId::new("Milestone13_3PageSegmentExtentSubstrate").unwrap(),
        S0ArtifactSubjectKind::Milestone,
        "13.3",
        "deferred-physical-guarantee",
        vec![evidence_ref("13.3")],
        vec![
            BackendForbiddenClaim::new(BackendForbiddenClaimKind::PhysicalPersistence, "S2")
                .unwrap(),
        ],
        vec![Roadmap2SequenceId::new("S2").unwrap()],
        S0ArtifactRowStatus::Deferred,
        "deferred guarantee row",
        DeferredPhysicalGuaranteeCategory::PageSegmentExtentSubstrate,
        S0PhysicalStatus::PhysicalDebt,
        "page substrate proof remains unearned",
        "Shipped store capability reclassification test",
        vec!["subscription-support trust".to_string()],
    )
    .expect_err("S1 anchor must be required for page substrate debt");

    assert_eq!(
        error,
        S0DeferredGuaranteeBuildRejection::GuaranteeCategorySequenceMismatch
    );
}
