use super::super::edge_splitting_ledger_support::build_split_edge_chain_ledger_with_manifest_for_metaboss;
use super::super::metaboss_support::MetabossEventExtractionSubject;
use super::super::reduced_pair_support;
use worth_kernel::workload_composition::WorkloadCompositionError;
use worth_spatial::certification::workload_evidence::{
    certification_only_admitted_stage_row, matched_boolean_receipt_snapshot,
};
use worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanSplitEdgeChainLedgerReceipt;
use worth_spatial::facade::workload_vocabulary::{
    WorkloadEvidenceRow, WorkloadEvidenceStage, WorkloadEvidenceStageCounters,
};

pub(crate) fn assert_split_ledger_satisfies_workload_requirement_for_7_4_consumption(
    subject: &MetabossEventExtractionSubject,
) {
    let (result, _, _, _) = build_split_edge_chain_ledger_with_manifest_for_metaboss(subject);
    let receipt = result.receipt();
    let workload = reduced_pair_support::rebuild_left_workload(subject.pair(), vec![])
        .complete_boolean_split_handoff(receipt)
        .expect(
            "completed split ledger handoff must compose into a proof-bearing workload handoff",
        );

    workload
        .require_boolean_split()
        .expect("completed split edge-chain ledger receipt must satisfy BooleanSplit closeout");
    assert_eq!(
        matched_boolean_receipt_snapshot(workload.completed_workload().evidence_ledger(), receipt)
            .expect("split evidence row must match the concrete split ledger receipt")
            .evidence_identity(),
        receipt.receipt_identity()
    );
}

pub(crate) fn assert_split_ledger_rejects_manual_or_counterless_evidence(
    subject: &MetabossEventExtractionSubject,
) {
    let (result, _, _, _) = build_split_edge_chain_ledger_with_manifest_for_metaboss(subject);
    let receipt = result.receipt();
    let manual_workload = reduced_pair_support::rebuild_left_workload(
        subject.pair(),
        vec![WorkloadEvidenceRow::new(
            WorkloadEvidenceStage::BooleanSplit,
            receipt.receipt_identity(),
        )],
    );
    let manual_denial = manual_workload
        .require_boolean_split(receipt)
        .expect_err("manual split evidence must not satisfy completed split closeout");
    assert_eq!(
        manual_denial,
        WorkloadCompositionError::ManualEvidenceStage(WorkloadEvidenceStage::BooleanSplit)
    );

    let counterless_workload = reduced_pair_support::rebuild_left_workload(
        subject.pair(),
        vec![certification_only_admitted_stage_row(
            WorkloadEvidenceStage::BooleanSplit,
            receipt.receipt_identity(),
            WorkloadEvidenceStageCounters::default(),
        )],
    );
    let counterless_denial = counterless_workload
        .require_boolean_split(receipt)
        .expect_err("counterless split evidence must not satisfy completed split closeout");
    assert_eq!(
        counterless_denial,
        WorkloadCompositionError::CounterlessEvidenceStage(WorkloadEvidenceStage::BooleanSplit)
    );
}

pub(crate) fn assert_split_stage_requirement_maps_only_to_split_ledger_receipts(
    subject: &MetabossEventExtractionSubject,
) {
    let (result, _, _, _) = build_split_edge_chain_ledger_with_manifest_for_metaboss(subject);
    let receipt = result.receipt();
    assert_split_request_identity_does_not_satisfy_split_ledger(subject, receipt);
    assert_same_identity_substitution_does_not_satisfy_split_ledger(subject, receipt);
    assert_foreign_split_identity_does_not_satisfy_split_ledger(subject, receipt);
}

fn assert_split_request_identity_does_not_satisfy_split_ledger(
    subject: &MetabossEventExtractionSubject,
    receipt: &PlanarBooleanSplitEdgeChainLedgerReceipt,
) {
    let workload = reduced_pair_support::rebuild_left_workload(
        subject.pair(),
        vec![certification_only_admitted_stage_row(
            WorkloadEvidenceStage::BooleanSplit,
            receipt.split_request_identity(),
            WorkloadEvidenceStageCounters::boolean_split(),
        )],
    );

    let denial = workload
        .require_boolean_split(receipt)
        .expect_err("edge split request identity must not satisfy completed split ledger closeout");
    assert_eq!(
        denial,
        WorkloadCompositionError::MismatchedEvidenceStage(WorkloadEvidenceStage::BooleanSplit)
    );
}

fn assert_same_identity_substitution_does_not_satisfy_split_ledger(
    subject: &MetabossEventExtractionSubject,
    receipt: &PlanarBooleanSplitEdgeChainLedgerReceipt,
) {
    let workload = reduced_pair_support::rebuild_left_workload(
        subject.pair(),
        vec![certification_only_admitted_stage_row(
            WorkloadEvidenceStage::BooleanSplit,
            receipt.receipt_identity(),
            WorkloadEvidenceStageCounters::boolean_split(),
        )],
    );

    let denial = workload.require_boolean_split(receipt).expect_err(
        "copied split ledger identity must not spoof the concrete split ledger receipt",
    );
    assert_eq!(
        denial,
        WorkloadCompositionError::MismatchedEvidenceStage(WorkloadEvidenceStage::BooleanSplit)
    );
}

fn assert_foreign_split_identity_does_not_satisfy_split_ledger(
    subject: &MetabossEventExtractionSubject,
    receipt: &PlanarBooleanSplitEdgeChainLedgerReceipt,
) {
    let workload = reduced_pair_support::rebuild_left_workload(
        subject.pair(),
        vec![certification_only_admitted_stage_row(
            WorkloadEvidenceStage::BooleanSplit,
            "foreign split ledger receipt",
            WorkloadEvidenceStageCounters::boolean_split(),
        )],
    );
    let denial = workload
        .require_boolean_split(receipt)
        .expect_err("foreign split ledger evidence must not satisfy completed split closeout");
    assert_eq!(
        denial,
        WorkloadCompositionError::MismatchedEvidenceStage(WorkloadEvidenceStage::BooleanSplit)
    );
}
