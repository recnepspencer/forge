use super::fixtures::{
    complete_ledger_for_plan, selected_lookup_slice_for_plan,
    selected_lookup_slice_scope_error_for_plan, IndexProductSubject,
};
use crate::workload_platform::evidence_ledger::{
    BooleanEvidenceReceipt, WorkloadEvidenceLedger, WorkloadEvidenceRow, WorkloadEvidenceStage,
};
use crate::workload_platform::evidence_lookup_index_product::{
    audit_evidence_lookup_index_product_basis, EvidenceLookupIndexBasisAuditScope,
    EvidenceLookupIndexProductErrorKind,
};
use crate::workload_platform::planar_boolean_common_plane::{
    common_plane_readiness_receipt_for_tests, common_plane_shared_plane_identity_receipt_for_tests,
    PlanarBooleanCommonPlaneLocalFrameSelectionReceipt, PlanarBooleanCommonPlaneOperandSide,
};
use crate::workload_platform::planar_boolean_edge_splitting::{
    event_ledger_lookup_execution_subject, source_carriers_for_tests, split_carrier_for_tests,
    split_event_ledger_for_tests, split_pair_receipt_for_tests,
};
use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanEventLedgerLookupExecutionDenialKind,
    PlanarBooleanEventLedgerLookupExecutionPacket,
};
use crate::workload_platform::spatial_compiled_product_consumer_cutover::lower_evidence_lookup_index_product;

#[test]
fn hidden_all_evidence_index_fails_index_contract() {
    let selected_plan = IndexProductSubject::sparse_event_ledger().select_plan();
    let ledger = complete_ledger_for_plan(&selected_plan);
    let bounded = lower_evidence_lookup_index_product(
        &selected_plan,
        &selected_lookup_slice_for_plan(&selected_plan),
    )
    .expect("bounded selected-scope basis should admit");

    let error = audit_evidence_lookup_index_product_basis(
        &selected_plan,
        &ledger,
        EvidenceLookupIndexBasisAuditScope::CompleteLedgerUnbounded,
    )
    .expect_err("all-ledger basis must deny on the production audit boundary");

    assert_eq!(
        error.kind(),
        EvidenceLookupIndexProductErrorKind::LedgerBasisExceedsSelectedScope
    );
    assert!(
        error.counters().selected_basis_row_count() > bounded.counters().selected_basis_row_count()
    );
    assert!(error.counters().resident_byte_count() > bounded.counters().resident_byte_count());
}

#[test]
fn ordinary_index_path_uses_selected_slice_only_even_with_surrounding_broad_scan_residue() {
    let carriers = source_carriers_for_tests();
    let segment_pairs = split_pair_receipt_for_tests(&carriers);
    let event_ledger = split_event_ledger_for_tests(
        segment_pairs.segment_pair_enumeration_identity(),
        carriers,
        Vec::new(),
        "phase-7-ordinary-index-broad-scan",
    );
    let baseline = event_ledger_lookup_execution_subject(
        "phase-7-ordinary-index-broad-scan",
        &event_ledger,
        vec![
            WorkloadEvidenceRow::from_boolean_evidence_receipt(&segment_pairs),
            WorkloadEvidenceRow::from_boolean_evidence_receipt(&event_ledger),
        ],
    );
    let foreign_segment_pairs = split_pair_receipt_for_tests(&[
        split_carrier_for_tests(
            PlanarBooleanCommonPlaneOperandSide::Left,
            "foreign-left-edge-a",
        ),
        split_carrier_for_tests(
            PlanarBooleanCommonPlaneOperandSide::Left,
            "foreign-left-edge-b",
        ),
        split_carrier_for_tests(
            PlanarBooleanCommonPlaneOperandSide::Right,
            "foreign-right-edge-a",
        ),
        split_carrier_for_tests(
            PlanarBooleanCommonPlaneOperandSide::Right,
            "foreign-right-edge-b",
        ),
    ]);
    let foreign_segment_pair_ledger = WorkloadEvidenceLedger::from_rows(
        baseline
            .complete_ledger
            .rows()
            .iter()
            .filter(|row| row.stage() != WorkloadEvidenceStage::BooleanSegmentPairEnumeration)
            .cloned()
            .chain(std::iter::once(
                WorkloadEvidenceRow::from_boolean_evidence_receipt(&foreign_segment_pairs),
            ))
            .collect(),
    )
    .expect("foreign segment-pair substitution should still index")
    .certify_complete()
    .expect("foreign segment-pair substitution should still certify complete");
    let foreign_segment_pair_denial =
        PlanarBooleanEventLedgerLookupExecutionPacket::admit(&event_ledger, &foreign_segment_pair_ledger)
            .expect_err(
                "ordinary event-ledger lookup must deny foreign segment-pair residue before family selection",
            );
    let shared_plane = common_plane_shared_plane_identity_receipt_for_tests(
        "phase-7-ordinary-index-broad-scan-unrelated-shared-plane",
    );
    let local_frame = PlanarBooleanCommonPlaneLocalFrameSelectionReceipt::from_shared_plane_identity_and_m7_readiness(
        &shared_plane,
        &common_plane_readiness_receipt_for_tests(),
    )
    .expect("real local-frame selection receipt should certify from shared-plane identity");
    let shared_plane_ledger = baseline
        .complete_ledger
        .with_boolean_evidence_receipt(&shared_plane)
        .expect("real unrelated boolean receipt should still certify complete");
    let shared_plane_denial =
        PlanarBooleanEventLedgerLookupExecutionPacket::admit(&event_ledger, &shared_plane_ledger)
            .expect_err("ordinary event-ledger lookup must deny unrelated shared-plane residue before family selection");
    let local_frame_ledger = baseline
        .complete_ledger
        .with_boolean_evidence_receipt(&local_frame)
        .expect("real local-frame selection receipt should still certify complete");
    let local_frame_denial =
        PlanarBooleanEventLedgerLookupExecutionPacket::admit(&event_ledger, &local_frame_ledger)
            .expect_err("ordinary event-ledger lookup must deny unrelated local-frame residue before family selection");

    assert_eq!(
        selected_lookup_slice_scope_error_for_plan(baseline.packet.selected_plan()),
        crate::workload_platform::evidence_ledger::WorkloadEvidenceLedgerError::SelectedLookupSliceExceedsScope(
            WorkloadEvidenceStage::BooleanSharedPlaneIdentity
        )
    );
    assert_eq!(
        foreign_segment_pair_denial.kind(),
        PlanarBooleanEventLedgerLookupExecutionDenialKind::BroadBooleanResidue
    );
    assert!(
        foreign_segment_pair_denial
            .detail()
            .contains("before family selection"),
        "ordinary event-ledger lookup denial must name the pre-selection broad-residue boundary"
    );
    assert!(
        foreign_segment_pair_denial
            .detail()
            .contains("boolean segment-pair enumeration evidence"),
        "ordinary event-ledger lookup denial must reject foreign segment-pair residue by stage"
    );
    assert_eq!(
        shared_plane_denial.kind(),
        PlanarBooleanEventLedgerLookupExecutionDenialKind::BroadBooleanResidue
    );
    assert!(
        shared_plane_denial
            .detail()
            .contains("before family selection"),
        "ordinary event-ledger lookup denial must name the pre-selection broad-residue boundary"
    );
    assert!(
        shared_plane_denial
            .detail()
            .contains("shared plane identity"),
        "ordinary event-ledger lookup denial must name the unrelated boolean stage"
    );
    assert!(!shared_plane_denial
        .detail()
        .contains(shared_plane.evidence_identity()));
    assert_eq!(
        local_frame_denial.kind(),
        PlanarBooleanEventLedgerLookupExecutionDenialKind::BroadBooleanResidue
    );
    assert!(
        local_frame_denial
            .detail()
            .contains("before family selection"),
        "ordinary event-ledger lookup denial must name the pre-selection broad-residue boundary"
    );
    assert!(
        local_frame_denial
            .detail()
            .contains("local-frame selection"),
        "ordinary event-ledger lookup denial must name the unrelated local-frame stage"
    );
    assert!(!local_frame_denial
        .detail()
        .contains(local_frame.evidence_identity()));
    assert_eq!(
        baseline
            .packet
            .selected_plan()
            .counters()
            .broad_receipt_scan_count(),
        0,
        "baseline ordinary path must still prove there is no broad receipt fallback"
    );
}
