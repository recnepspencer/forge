use super::*;
use crate::workload_platform::evidence_ledger::{
    BooleanEvidenceReceipt, BooleanEvidenceReceiptSealed, BooleanEvidenceRowAuthority,
    BooleanEvidenceStageKind, CompleteWorkloadEvidenceLedger, WorkloadEvidenceLedger,
    WorkloadEvidenceLedgerError, WorkloadEvidenceRow, WorkloadEvidenceStage,
    WorkloadEvidenceStageCounters, WorkloadEvidenceSupport,
};

#[test]
fn admission_contract_requires_sealed_receipt_complete_ledger_and_stage_lookup() {
    let receipt = FakeSplitReceipt::admitted("split-authority");
    let complete = complete_ledger_with_receipt(&receipt);

    let authority = SpatialGeometryEvidenceTouchRequest::from_boolean_receipt(&receipt)
        .with_complete_ledger(&complete)
        .admit()
        .expect("receipt plus complete ledger should admit");

    assert_eq!(authority.boolean_stage(), BooleanEvidenceStageKind::Split);
    assert_eq!(
        authority.evidence_stage(),
        WorkloadEvidenceStage::BooleanSplit
    );
    assert_eq!(authority.evidence_identity(), "split-authority");
    assert_eq!(authority.support(), WorkloadEvidenceSupport::Admitted);
    assert_eq!(authority.lookup_counters().raw_row_scan_count(), 0);
    assert_eq!(
        authority.stage_index_identity(),
        complete.stage_index().index_identity()
    );
}

#[test]
fn partial_products_are_denied_before_authority_construction() {
    let receipt = FakeSplitReceipt::admitted("split-denial");
    let complete = complete_ledger_with_receipt(&receipt);
    let row = WorkloadEvidenceRow::new(WorkloadEvidenceStage::BooleanSplit, "split-denial");
    let lookup = complete
        .require_boolean_receipt_lookup(&receipt)
        .expect("lookup product should exist for lookup-only denial");

    let denied = [
        SpatialGeometryEvidenceTouchRejectedInput::receipt_only(&receipt).deny(),
        SpatialGeometryEvidenceTouchRejectedInput::workload_evidence_row(&row).deny(),
        SpatialGeometryEvidenceTouchRejectedInput::boolean_receipt_lookup_product(&lookup).deny(),
    ];

    for denial in denied {
        assert_eq!(
            denial.kind(),
            SpatialGeometryEvidenceTouchDenialKind::SourceSubstitution
        );
    }
}

#[test]
fn denial_precedence_prefers_source_then_support_then_counter_before_query_gap() {
    let source_first = SpatialGeometryEvidenceTouchDenialPrecedence::new()
        .with_source_substitution(SpatialGeometryEvidenceTouchRejectedInputKind::QueryDescriptor)
        .with_support_posture(
            WorkloadEvidenceStage::BooleanSplit,
            WorkloadEvidenceSupport::Unsupported,
        )
        .with_counter_honesty(WorkloadEvidenceLedgerError::CounterlessBooleanStage(
            WorkloadEvidenceStage::BooleanSplit,
        ))
        .with_query_gap("Query lowering is deferred")
        .deny()
        .expect("source substitution should dominate");
    assert_eq!(
        source_first.kind(),
        SpatialGeometryEvidenceTouchDenialKind::SourceSubstitution
    );

    let support_second = SpatialGeometryEvidenceTouchDenialPrecedence::new()
        .with_support_posture(
            WorkloadEvidenceStage::BooleanEventExtractionRequest,
            WorkloadEvidenceSupport::Blocked,
        )
        .with_counter_honesty(WorkloadEvidenceLedgerError::CounterlessBooleanStage(
            WorkloadEvidenceStage::BooleanSplit,
        ))
        .with_query_gap("Query lowering is deferred")
        .deny()
        .expect("support should dominate counters and Query");
    assert_eq!(
        support_second.kind(),
        SpatialGeometryEvidenceTouchDenialKind::SupportPosture
    );
    assert!(support_second
        .detail()
        .contains("boolean event extraction request"));

    let counter_third = SpatialGeometryEvidenceTouchDenialPrecedence::new()
        .with_counter_honesty(WorkloadEvidenceLedgerError::CounterlessBooleanStage(
            WorkloadEvidenceStage::BooleanSplit,
        ))
        .with_query_gap("Query lowering is deferred")
        .deny()
        .expect("counter should dominate Query");
    assert_eq!(
        counter_third.kind(),
        SpatialGeometryEvidenceTouchDenialKind::CounterHonesty
    );

    let stage_link_before_query = SpatialGeometryEvidenceTouchDenialPrecedence::new()
        .with_stage_link_failure(
            WorkloadEvidenceLedgerError::MismatchedAuthorityStageBinding(
                WorkloadEvidenceStage::RetainedReplay,
                WorkloadEvidenceStage::Transform,
            ),
        )
        .with_query_gap("Query lowering is deferred")
        .deny()
        .expect("stage-link failure should dominate Query");
    assert_eq!(
        stage_link_before_query.kind(),
        SpatialGeometryEvidenceTouchDenialKind::StageLinkFailure
    );
}

#[test]
fn admission_denials_keep_ledger_counter_and_stage_link_classes_distinct() {
    let missing_receipt = FakeSplitReceipt::admitted("missing-split");
    let complete_without_boolean_receipt = complete_ledger_without_boolean_receipt();
    let missing_denial =
        SpatialGeometryEvidenceTouchRequest::from_boolean_receipt(&missing_receipt)
            .with_complete_ledger(&complete_without_boolean_receipt)
            .admit()
            .expect_err("missing boolean receipt row should deny before authority");
    assert_eq!(
        missing_denial.kind(),
        SpatialGeometryEvidenceTouchDenialKind::LedgerIncompleteness
    );

    let counterless_receipt = FakeSplitReceipt::admitted("counterless-split");
    let counterless_complete = complete_ledger_with_counterless_receipt(&counterless_receipt);
    let counterless_denial =
        SpatialGeometryEvidenceTouchRequest::from_boolean_receipt(&counterless_receipt)
            .with_complete_ledger(&counterless_complete)
            .admit()
            .expect_err("counterless receipt row should deny before authority");
    assert_eq!(
        counterless_denial.kind(),
        SpatialGeometryEvidenceTouchDenialKind::CounterHonesty
    );

    let stage_link_denial = SpatialGeometryEvidenceTouchDenial::stage_link_failure(
        WorkloadEvidenceLedgerError::MismatchedAuthorityStageBinding(
            WorkloadEvidenceStage::RetainedReplay,
            WorkloadEvidenceStage::Transform,
        ),
    );
    assert_eq!(
        stage_link_denial.kind(),
        SpatialGeometryEvidenceTouchDenialKind::StageLinkFailure
    );
}

#[test]
fn receipt_only_preview_reports_status_but_cannot_act_as_authority() {
    let receipt = FakeSplitReceipt::admitted("split-preview");
    let preview =
        SpatialGeometryEvidenceTouchRequest::from_boolean_receipt(&receipt).receipt_only_preview();

    assert_eq!(
        preview.status(),
        SpatialGeometryEvidenceTouchDiagnosticStatus::ReceiptOnly
    );
    assert_eq!(preview.evidence_identity(), "split-preview");
    assert_eq!(
        preview
            .lower_to_query()
            .expect_err("no Query lowering")
            .kind(),
        SpatialGeometryEvidenceTouchDenialKind::DiagnosticOnly
    );
    assert_eq!(
        preview
            .build_lookup_authority()
            .expect_err("no lookup authority")
            .kind(),
        SpatialGeometryEvidenceTouchDenialKind::DiagnosticOnly
    );
    assert_eq!(
        preview.satisfy_replay().expect_err("no replay").kind(),
        SpatialGeometryEvidenceTouchDenialKind::DiagnosticOnly
    );
    assert_eq!(
        preview.pass_closeout().expect_err("no closeout").kind(),
        SpatialGeometryEvidenceTouchDenialKind::DiagnosticOnly
    );
}

fn complete_ledger_with_receipt(receipt: &FakeSplitReceipt) -> CompleteWorkloadEvidenceLedger {
    let mut rows = authority_rows();
    rows.push(WorkloadEvidenceRow::from_boolean_evidence_receipt(receipt));
    WorkloadEvidenceLedger::from_rows(rows)
        .expect("rows should index")
        .certify_complete()
        .expect("authority rows should complete")
}

fn complete_ledger_without_boolean_receipt() -> CompleteWorkloadEvidenceLedger {
    WorkloadEvidenceLedger::from_rows(authority_rows())
        .expect("authority rows should index")
        .certify_complete()
        .expect("authority rows should complete")
}

fn complete_ledger_with_counterless_receipt(
    receipt: &FakeSplitReceipt,
) -> CompleteWorkloadEvidenceLedger {
    let mut rows = authority_rows();
    rows.push(WorkloadEvidenceRow::receipt_backed_with_receipt_type::<
        FakeSplitReceipt,
    >(
        receipt.boolean_stage().evidence_stage(),
        receipt.evidence_identity(),
        receipt.evidence_support(),
        WorkloadEvidenceStageCounters::default(),
    ));
    WorkloadEvidenceLedger::from_rows(rows)
        .expect("counterless receipt row should still index")
        .certify_complete()
        .expect("authority rows should complete")
}

fn authority_rows() -> Vec<WorkloadEvidenceRow> {
    vec![
        WorkloadEvidenceRow::receipt_backed(
            WorkloadEvidenceStage::Topology,
            "topology",
            WorkloadEvidenceStageCounters::topology(1, 1, 1),
        ),
        WorkloadEvidenceRow::receipt_backed(
            WorkloadEvidenceStage::GeometryBinding,
            "geometry",
            WorkloadEvidenceStageCounters::binding(1),
        ),
        WorkloadEvidenceRow::receipt_backed(
            WorkloadEvidenceStage::SurfaceSupport,
            "surface",
            WorkloadEvidenceStageCounters::surface_support(1),
        ),
        WorkloadEvidenceRow::receipt_backed(
            WorkloadEvidenceStage::Projection,
            "projection",
            WorkloadEvidenceStageCounters::projection(1, 1),
        ),
        WorkloadEvidenceRow::receipt_backed(
            WorkloadEvidenceStage::Transform,
            "transform",
            WorkloadEvidenceStageCounters::transform(1, 1, 0),
        ),
        WorkloadEvidenceRow::receipt_backed(
            WorkloadEvidenceStage::RetainedReplay,
            "replay",
            WorkloadEvidenceStageCounters::retained_replay(1, 1),
        ),
        WorkloadEvidenceRow::receipt_backed(
            WorkloadEvidenceStage::Diagnostics,
            "diagnostics",
            WorkloadEvidenceStageCounters::diagnostics(1),
        ),
        WorkloadEvidenceRow::receipt_backed(
            WorkloadEvidenceStage::Response,
            "response",
            WorkloadEvidenceStageCounters::response(1),
        ),
    ]
}

struct FakeSplitReceipt {
    identity: &'static str,
}

impl FakeSplitReceipt {
    fn admitted(identity: &'static str) -> Self {
        Self { identity }
    }
}

impl BooleanEvidenceReceipt for FakeSplitReceipt {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        BooleanEvidenceStageKind::Split
    }

    fn evidence_identity(&self) -> &str {
        self.identity
    }

    fn evidence_support(&self) -> WorkloadEvidenceSupport {
        WorkloadEvidenceSupport::Admitted
    }

    fn evidence_counters(&self) -> WorkloadEvidenceStageCounters {
        WorkloadEvidenceStageCounters::boolean_split()
    }
}

impl BooleanEvidenceReceiptSealed for FakeSplitReceipt {}

impl BooleanEvidenceRowAuthority for FakeSplitReceipt {}
