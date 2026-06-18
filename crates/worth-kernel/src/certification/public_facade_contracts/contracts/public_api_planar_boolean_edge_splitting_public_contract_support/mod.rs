mod foreign_authority;

use super::edge_splitting_replay_parity_support::{
    build_edge_split_replay_parity_subject, replay_parity_report, EdgeSplitReplayParitySubject,
};
use super::metaboss_support::MetabossEventExtractionSubject;
use foreign_authority::assert_foreign_split_authorities_are_rejected;
use worth_kernel::workload_composition::WorthWorkload;
use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanDownstreamSplitConsumption, PlanarBooleanDownstreamSplitConsumptionDenialKind,
    PlanarBooleanDownstreamSplitConsumptionInput, PlanarBooleanEdgeSplitReplayParityReport,
    PlanarBooleanLoopReconstructionSplitConsumption,
    PlanarBooleanLoopReconstructionSplitConsumptionInput,
};
use worth_spatial::facade::workload_vocabulary::{WorkloadEvidenceLedger, WorkloadEvidenceRow};

pub(crate) fn assert_split_public_contract_requires_real_ledger_and_rejects_manual_evidence() {
    let subject =
        MetabossEventExtractionSubject::certify("phase7.3 public downstream split consumption");
    let replay_subject = build_edge_split_replay_parity_subject(&subject);
    let replay_report = replay_parity_report(&replay_subject);
    let completed_workload = completed_split_workload_for(&subject, &replay_subject);
    let consumption = admit_real_downstream_split_consumption(
        &replay_subject,
        &replay_report,
        &completed_workload,
    );

    assert_downstream_consumption_preserves_real_split_authority(
        &consumption,
        &replay_subject,
        &replay_report,
        &completed_workload,
    );
    assert_loop_reconstruction_consumes_downstream_split_product(&consumption);
    assert_foreign_split_authorities_are_rejected(
        &replay_subject,
        &replay_report,
        &completed_workload,
    );
    assert_manual_boolean_split_evidence_is_rejected(&replay_subject, &replay_report);
}

pub(crate) fn completed_split_workload_for(
    subject: &MetabossEventExtractionSubject,
    replay_subject: &EdgeSplitReplayParitySubject,
) -> WorthWorkload {
    let completed_workload = subject
        .pair()
        .left()
        .workload()
        .with_completed_boolean_split_ledger(replay_subject.original_ledger.receipt())
        .expect("real workload should admit the split ledger as BooleanSplit evidence");
    completed_workload
        .require_boolean_split(replay_subject.original_ledger.receipt())
        .expect("completed workload should require the exact split ledger receipt");
    completed_workload
}

fn admit_real_downstream_split_consumption(
    replay_subject: &EdgeSplitReplayParitySubject,
    replay_report: &PlanarBooleanEdgeSplitReplayParityReport,
    completed_workload: &WorthWorkload,
) -> PlanarBooleanDownstreamSplitConsumption {
    PlanarBooleanDownstreamSplitConsumption::admit(
        PlanarBooleanDownstreamSplitConsumptionInput::from_split_ledger_receipt(
            replay_subject.original_ledger.receipt(),
            replay_subject.original_decision_log.receipt(),
            &replay_subject.original_products.validation,
            &replay_subject.original_products.naming,
            replay_report.receipt(),
            completed_workload.evidence_ledger().stage_index(),
        ),
    )
    .expect("real split ledger receipt should admit downstream split consumption")
}

fn assert_downstream_consumption_preserves_real_split_authority(
    consumption: &PlanarBooleanDownstreamSplitConsumption,
    replay_subject: &EdgeSplitReplayParitySubject,
    replay_report: &PlanarBooleanEdgeSplitReplayParityReport,
    completed_workload: &WorthWorkload,
) {
    assert!(consumption.certifies_downstream_split_consumption());
    assert_eq!(
        consumption.split_ledger_receipt_identity(),
        replay_subject.original_ledger.receipt().receipt_identity()
    );
    assert_eq!(
        consumption.split_ledger_downstream_identity(),
        replay_subject
            .original_ledger
            .receipt()
            .downstream_consumption_identity()
    );
    assert_eq!(
        consumption.decision_log_receipt_identity(),
        replay_subject
            .original_decision_log
            .receipt()
            .receipt_identity()
    );
    assert_eq!(
        consumption.validation_receipt_identity(),
        replay_subject
            .original_products
            .validation
            .receipt_identity()
    );
    assert_eq!(
        consumption.persistent_naming_receipt_identity(),
        replay_subject.original_products.naming.receipt_identity()
    );
    assert_eq!(
        consumption.replay_parity_receipt_identity(),
        replay_report.receipt().receipt_identity()
    );
    assert_eq!(
        consumption.workload_stage_index_identity(),
        completed_workload
            .evidence_ledger()
            .stage_index()
            .index_identity()
    );
    assert_downstream_consumption_counters_match_real_split_authority(
        consumption,
        replay_subject,
        replay_report,
        completed_workload,
    );
}

fn assert_downstream_consumption_counters_match_real_split_authority(
    consumption: &PlanarBooleanDownstreamSplitConsumption,
    replay_subject: &EdgeSplitReplayParitySubject,
    replay_report: &PlanarBooleanEdgeSplitReplayParityReport,
    completed_workload: &WorthWorkload,
) {
    assert_eq!(
        consumption.counters().split_chains_consumed(),
        replay_subject
            .original_ledger
            .receipt()
            .chain_identities()
            .len()
    );
    assert_eq!(
        consumption.counters().fragment_rows_consumed(),
        replay_subject
            .original_products
            .validation
            .fragment_coverage_rows()
            .len()
    );
    assert_eq!(
        consumption.counters().vertex_rows_consumed(),
        replay_subject
            .original_decision_log
            .receipt()
            .counters()
            .coalescence_decisions_recorded()
    );
    assert_eq!(
        consumption.counters().persistent_name_rows_consumed(),
        replay_subject
            .original_products
            .naming
            .persistent_name_rows()
            .len()
    );
    assert_eq!(
        consumption.counters().replay_parity_rows_consumed(),
        replay_report.receipt().parity_rows().len()
    );
    assert_eq!(
        consumption.counters().stage_index_rows_consumed(),
        completed_workload
            .evidence_ledger()
            .stage_index()
            .rows()
            .len()
    );
    assert_eq!(consumption.counters().foreign_receipts_rejected(), 0);
    assert_eq!(consumption.counters().missing_receipts_rejected(), 0);
    assert_eq!(consumption.counters().non_receipt_evidence_rejected(), 0);
}

fn assert_loop_reconstruction_consumes_downstream_split_product(
    consumption: &PlanarBooleanDownstreamSplitConsumption,
) {
    let loop_consumption = PlanarBooleanLoopReconstructionSplitConsumption::admit(
        PlanarBooleanLoopReconstructionSplitConsumptionInput::from_downstream_split_consumption(
            consumption,
        ),
    )
    .expect("loop reconstruction should consume only the downstream split-consumption product");
    assert!(loop_consumption.certifies_loop_reconstruction_split_consumption());
    assert_eq!(
        loop_consumption.downstream_consumption_identity(),
        consumption.consumption_identity()
    );
    assert_eq!(
        loop_consumption.split_ledger_receipt_identity(),
        consumption.split_ledger_receipt_identity()
    );
    assert_eq!(
        loop_consumption.split_ledger_downstream_identity(),
        consumption.split_ledger_downstream_identity()
    );
    assert_eq!(
        loop_consumption.split_request_identity(),
        consumption.split_request_identity()
    );
    assert_eq!(
        loop_consumption.workload_stage_index_identity(),
        consumption.workload_stage_index_identity()
    );
    assert_eq!(loop_consumption.counters().downstream_gate_consumed(), 1);
}

fn assert_manual_boolean_split_evidence_is_rejected(
    replay_subject: &EdgeSplitReplayParitySubject,
    replay_report: &PlanarBooleanEdgeSplitReplayParityReport,
) {
    let evidence = WorkloadEvidenceLedger::from_rows(vec![WorkloadEvidenceRow::new(
        worth_spatial::facade::workload_vocabulary::WorkloadEvidenceStage::BooleanSplit,
        replay_subject.original_ledger.receipt().receipt_identity(),
    )])
    .expect("manual split row should stay indexable for downstream denial");

    let denial = PlanarBooleanDownstreamSplitConsumption::admit(
        PlanarBooleanDownstreamSplitConsumptionInput::from_split_ledger_receipt(
            replay_subject.original_ledger.receipt(),
            replay_subject.original_decision_log.receipt(),
            &replay_subject.original_products.validation,
            &replay_subject.original_products.naming,
            replay_report.receipt(),
            evidence.stage_index(),
        ),
    )
    .expect_err("manual BooleanSplit evidence must not certify downstream consumption");

    assert_eq!(
        denial.kind(),
        PlanarBooleanDownstreamSplitConsumptionDenialKind::NonReceiptBackedBooleanSplitEvidence
    );
    assert_eq!(denial.counters().non_receipt_evidence_rejected(), 1);
}
