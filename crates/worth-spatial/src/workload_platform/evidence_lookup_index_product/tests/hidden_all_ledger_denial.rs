use super::fixtures::{
    complete_ledger_for_plan, selected_lookup_slice_for_plan,
    selected_lookup_slice_scope_error_for_plan, IndexProductSubject,
};
use crate::trusted_boolean_evidence_authority::Seal;
use crate::workload_platform::evidence_ledger::{
    receipt_backed_touch_authority_for_admission_tests, BooleanEvidenceReceipt,
    BooleanEvidenceRowAuthority, BooleanEvidenceStageKind, WorkloadEvidenceLedger,
    WorkloadEvidenceRow, WorkloadEvidenceStage, WorkloadEvidenceStageCounters,
    WorkloadEvidenceSupport,
};
use crate::workload_platform::evidence_lookup_index_product::{
    admit_evidence_lookup_index_product, audit_evidence_lookup_index_product_basis,
    EvidenceLookupIndexBasisAuditScope, EvidenceLookupIndexProductErrorKind,
};
use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide;
use crate::workload_platform::planar_boolean_edge_splitting::{
    event_ledger_lookup_execution_subject, source_carriers_for_tests, split_carrier_for_tests,
    split_event_ledger_for_tests, split_pair_receipt_for_tests,
};
use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanEventLedgerLookupExecutionDenialKind,
    PlanarBooleanEventLedgerLookupExecutionPacket,
};

#[test]
fn hidden_all_evidence_index_fails_index_contract() {
    let selected_plan = IndexProductSubject::sparse_event_ledger().select_plan();
    let ledger = complete_ledger_for_plan(&selected_plan);
    let bounded = admit_evidence_lookup_index_product(
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
    let unrelated = AuthorityBackedBooleanReceipt::from_touch_authority(
        &receipt_backed_touch_authority_for_admission_tests(
            BooleanEvidenceStageKind::SharedPlaneIdentity,
            "phase-7-ordinary-index-broad-scan-unrelated-shared-plane",
        ),
    );
    let broad_scan_ledger = baseline
        .complete_ledger
        .with_boolean_evidence_receipt(&unrelated)
        .expect("real unrelated boolean receipt should still certify complete");
    let noisy = PlanarBooleanEventLedgerLookupExecutionPacket::admit(&event_ledger, &broad_scan_ledger)
        .expect_err("ordinary event-ledger lookup must deny unrelated broad boolean residue before family selection");

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
        noisy.kind(),
        PlanarBooleanEventLedgerLookupExecutionDenialKind::BroadBooleanResidue
    );
    assert!(
        noisy.detail().contains("before family selection"),
        "ordinary event-ledger lookup denial must name the pre-selection broad-residue boundary"
    );
    assert!(
        noisy.detail().contains("shared plane identity"),
        "ordinary event-ledger lookup denial must name the unrelated boolean stage"
    );
    assert!(!noisy.detail().contains(unrelated.evidence_identity()));
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

struct AuthorityBackedBooleanReceipt {
    boolean_stage: BooleanEvidenceStageKind,
    evidence_identity: String,
    support: WorkloadEvidenceSupport,
    counters: WorkloadEvidenceStageCounters,
}

impl AuthorityBackedBooleanReceipt {
    fn from_touch_authority(
        authority: &crate::workload_platform::evidence_ledger::SpatialGeometryEvidenceTouchAuthority,
    ) -> Self {
        Self {
            boolean_stage: authority.boolean_stage(),
            evidence_identity: authority.evidence_identity().to_string(),
            support: authority.support(),
            counters: authority.evidence_counters(),
        }
    }
}

impl BooleanEvidenceReceipt for AuthorityBackedBooleanReceipt {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        self.boolean_stage
    }

    fn evidence_identity(&self) -> &str {
        &self.evidence_identity
    }

    fn evidence_support(&self) -> WorkloadEvidenceSupport {
        self.support
    }

    fn evidence_counters(&self) -> WorkloadEvidenceStageCounters {
        self.counters
    }
}

impl Seal for AuthorityBackedBooleanReceipt {}

impl BooleanEvidenceRowAuthority for AuthorityBackedBooleanReceipt {}
