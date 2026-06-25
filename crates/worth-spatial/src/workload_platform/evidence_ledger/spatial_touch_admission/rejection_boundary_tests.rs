use std::marker::PhantomData;

use super::admission_test_support::{
    complete_ledger_from_rows, rows_without_topology, synthetic_authority_rows,
    synthetic_authority_rows_with_synthetic_topology, with_receipt_row,
};
use super::*;
use crate::workload_platform::evidence_ledger::{
    BooleanEvidenceReceipt, BooleanEvidenceStageKind, WorkloadEvidenceLedger,
    WorkloadEvidenceLedgerError, WorkloadEvidenceRow, WorkloadEvidenceStage,
    WorkloadEvidenceSupport,
};
use crate::workload_platform::planar_boolean_edge_splitting::{
    source_carriers_for_tests, split_pair_receipt_for_tests,
};
use crate::workload_platform::planar_boolean_events::PlanarBooleanSegmentPairEnumerationReceipt;

#[test]
fn manual_incomplete_unsupported_guard_and_copied_sources_deny_before_query() {
    let carriers = source_carriers_for_tests();
    let receipt = split_pair_receipt_for_tests(&carriers);

    let manual_rows = with_receipt_row(
        synthetic_authority_rows(),
        WorkloadEvidenceRow::new(
            receipt.boolean_stage().evidence_stage(),
            receipt.evidence_identity(),
        ),
    );
    let manual_ledger = WorkloadEvidenceLedger::from_rows(manual_rows)
        .expect("manual row should still index")
        .certify_complete()
        .expect("manual boolean row does not satisfy authority construction");
    let manual_error = SpatialGeometryEvidenceTouchRequest::from_boolean_receipt(&receipt)
        .with_complete_ledger(&manual_ledger)
        .admit()
        .expect_err("manual boolean row must deny before authority");
    assert_eq!(
        manual_error.kind(),
        SpatialGeometryEvidenceTouchDenialKind::SourceSubstitution
    );

    let missing_error = WorkloadEvidenceLedger::from_rows(rows_without_topology())
        .expect("incomplete authority rows should index")
        .certify_complete()
        .expect_err("missing authority stage must not certify complete");
    assert_eq!(
        missing_error,
        WorkloadEvidenceLedgerError::MissingAuthorityStage(WorkloadEvidenceStage::Topology)
    );

    let unsupported_ledger = complete_ledger_from_rows(with_receipt_row(
        synthetic_authority_rows(),
        WorkloadEvidenceRow::receipt_backed_with_receipt_type::<
            PlanarBooleanSegmentPairEnumerationReceipt,
        >(
            receipt.boolean_stage().evidence_stage(),
            receipt.evidence_identity(),
            WorkloadEvidenceSupport::Unsupported,
            receipt.evidence_counters(),
        ),
    ));
    let unsupported = SpatialGeometryEvidenceTouchRequest::from_boolean_receipt(&receipt)
        .with_complete_ledger(&unsupported_ledger)
        .admit()
        .expect_err("unsupported receipt row must deny before authority");
    assert_eq!(
        unsupported.kind(),
        SpatialGeometryEvidenceTouchDenialKind::SupportPosture
    );

    let synthetic_topology_ledger = complete_ledger_from_rows(with_receipt_row(
        synthetic_authority_rows_with_synthetic_topology(),
        WorkloadEvidenceRow::from_boolean_evidence_receipt(&receipt),
    ));
    let guard_failure = SpatialGeometryEvidenceTouchRequest::from_boolean_receipt(&receipt)
        .with_complete_ledger(&synthetic_topology_ledger)
        .admit()
        .expect_err("synthetic topology guard failure must deny before authority");
    assert_eq!(
        guard_failure.kind(),
        SpatialGeometryEvidenceTouchDenialKind::GuardFailure
    );
    assert!(guard_failure.detail().contains("real topology"));

    struct CopiedReceiptShape {
        stage: BooleanEvidenceStageKind,
        identity: String,
    }
    let copied = CopiedReceiptShape {
        stage: receipt.boolean_stage(),
        identity: receipt.evidence_identity().to_string(),
    };
    let copied_denial =
        SpatialGeometryEvidenceTouchRejectedInput::copied_receipt_fields(&copied).deny();
    assert_eq!(
        copied_denial.kind(),
        SpatialGeometryEvidenceTouchDenialKind::SourceSubstitution
    );
    assert_eq!(copied.stage, receipt.boolean_stage());
    assert_eq!(copied.identity, receipt.evidence_identity());

    let query_denial =
        SpatialGeometryEvidenceTouchRejectedInput::query_descriptor(&PhantomData::<()>).deny();
    assert_eq!(
        query_denial.kind(),
        SpatialGeometryEvidenceTouchDenialKind::SourceSubstitution
    );
}
