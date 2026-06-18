use super::super::edge_splitting_replay_parity_support::{
    build_edge_split_replay_parity_subject, replay_parity_report, EdgeSplitReplayParitySubject,
};
use super::super::metaboss_support::MetabossEventExtractionSubject;
use super::completed_split_workload_for;
use worth_kernel::workload_composition::WorthWorkload;
use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanDownstreamSplitConsumption, PlanarBooleanDownstreamSplitConsumptionDenialKind,
    PlanarBooleanDownstreamSplitConsumptionInput, PlanarBooleanEdgeSplitReplayParityReport,
};

pub(crate) fn assert_foreign_split_authorities_are_rejected(
    replay_subject: &EdgeSplitReplayParitySubject,
    replay_report: &PlanarBooleanEdgeSplitReplayParityReport,
    completed_workload: &WorthWorkload,
) {
    let foreign_subject =
        MetabossEventExtractionSubject::certify("phase7.3 foreign downstream split consumption");
    let foreign_replay_subject = build_edge_split_replay_parity_subject(&foreign_subject);
    let foreign_replay_report = replay_parity_report(&foreign_replay_subject);
    let foreign_completed_workload =
        completed_split_workload_for(&foreign_subject, &foreign_replay_subject);

    assert_foreign_decision_log_is_rejected(
        replay_subject,
        replay_report,
        completed_workload,
        &foreign_replay_subject,
    );
    assert_foreign_validation_receipt_is_rejected(
        replay_subject,
        replay_report,
        completed_workload,
        &foreign_replay_subject,
    );
    assert_foreign_persistent_naming_receipt_is_rejected(
        replay_subject,
        replay_report,
        completed_workload,
        &foreign_replay_subject,
    );
    assert_foreign_replay_parity_receipt_is_rejected(
        replay_subject,
        &foreign_replay_report,
        completed_workload,
    );
    assert_foreign_workload_stage_index_is_rejected(
        replay_subject,
        replay_report,
        &foreign_completed_workload,
    );
}

fn assert_foreign_decision_log_is_rejected(
    replay_subject: &EdgeSplitReplayParitySubject,
    replay_report: &PlanarBooleanEdgeSplitReplayParityReport,
    completed_workload: &WorthWorkload,
    foreign_replay_subject: &EdgeSplitReplayParitySubject,
) {
    let denial = PlanarBooleanDownstreamSplitConsumption::admit(
        PlanarBooleanDownstreamSplitConsumptionInput::from_split_ledger_receipt(
            replay_subject.original_ledger.receipt(),
            foreign_replay_subject.original_decision_log.receipt(),
            &replay_subject.original_products.validation,
            &replay_subject.original_products.naming,
            replay_report.receipt(),
            completed_workload.evidence_ledger().stage_index(),
        ),
    )
    .expect_err("foreign decision log must not certify downstream split consumption");
    assert_eq!(
        denial.kind(),
        PlanarBooleanDownstreamSplitConsumptionDenialKind::ForeignDecisionLogReceipt
    );
    assert_eq!(denial.counters().foreign_receipts_rejected(), 1);
}

fn assert_foreign_validation_receipt_is_rejected(
    replay_subject: &EdgeSplitReplayParitySubject,
    replay_report: &PlanarBooleanEdgeSplitReplayParityReport,
    completed_workload: &WorthWorkload,
    foreign_replay_subject: &EdgeSplitReplayParitySubject,
) {
    let denial = PlanarBooleanDownstreamSplitConsumption::admit(
        PlanarBooleanDownstreamSplitConsumptionInput::from_split_ledger_receipt(
            replay_subject.original_ledger.receipt(),
            replay_subject.original_decision_log.receipt(),
            &foreign_replay_subject.original_products.validation,
            &replay_subject.original_products.naming,
            replay_report.receipt(),
            completed_workload.evidence_ledger().stage_index(),
        ),
    )
    .expect_err("foreign validation receipt must not certify downstream split consumption");
    assert_eq!(
        denial.kind(),
        PlanarBooleanDownstreamSplitConsumptionDenialKind::ForeignValidationReceipt
    );
}

fn assert_foreign_persistent_naming_receipt_is_rejected(
    replay_subject: &EdgeSplitReplayParitySubject,
    replay_report: &PlanarBooleanEdgeSplitReplayParityReport,
    completed_workload: &WorthWorkload,
    foreign_replay_subject: &EdgeSplitReplayParitySubject,
) {
    let denial = PlanarBooleanDownstreamSplitConsumption::admit(
        PlanarBooleanDownstreamSplitConsumptionInput::from_split_ledger_receipt(
            replay_subject.original_ledger.receipt(),
            replay_subject.original_decision_log.receipt(),
            &replay_subject.original_products.validation,
            &foreign_replay_subject.original_products.naming,
            replay_report.receipt(),
            completed_workload.evidence_ledger().stage_index(),
        ),
    )
    .expect_err("foreign persistent naming receipt must not certify downstream consumption");
    assert_eq!(
        denial.kind(),
        PlanarBooleanDownstreamSplitConsumptionDenialKind::ForeignPersistentNamingReceipt
    );
}

fn assert_foreign_replay_parity_receipt_is_rejected(
    replay_subject: &EdgeSplitReplayParitySubject,
    foreign_replay_report: &PlanarBooleanEdgeSplitReplayParityReport,
    completed_workload: &WorthWorkload,
) {
    let denial = PlanarBooleanDownstreamSplitConsumption::admit(
        PlanarBooleanDownstreamSplitConsumptionInput::from_split_ledger_receipt(
            replay_subject.original_ledger.receipt(),
            replay_subject.original_decision_log.receipt(),
            &replay_subject.original_products.validation,
            &replay_subject.original_products.naming,
            foreign_replay_report.receipt(),
            completed_workload.evidence_ledger().stage_index(),
        ),
    )
    .expect_err("foreign replay parity receipt must not certify downstream split consumption");
    assert_eq!(
        denial.kind(),
        PlanarBooleanDownstreamSplitConsumptionDenialKind::ForeignReplayParityReceipt
    );
}

fn assert_foreign_workload_stage_index_is_rejected(
    replay_subject: &EdgeSplitReplayParitySubject,
    replay_report: &PlanarBooleanEdgeSplitReplayParityReport,
    foreign_completed_workload: &WorthWorkload,
) {
    let denial = PlanarBooleanDownstreamSplitConsumption::admit(
        PlanarBooleanDownstreamSplitConsumptionInput::from_split_ledger_receipt(
            replay_subject.original_ledger.receipt(),
            replay_subject.original_decision_log.receipt(),
            &replay_subject.original_products.validation,
            &replay_subject.original_products.naming,
            replay_report.receipt(),
            foreign_completed_workload.evidence_ledger().stage_index(),
        ),
    )
    .expect_err("foreign stage index must not certify downstream split consumption");
    assert_eq!(
        denial.kind(),
        PlanarBooleanDownstreamSplitConsumptionDenialKind::ForeignWorkloadStageIndex
    );
}
