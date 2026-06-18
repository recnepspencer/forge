mod foreign_authority;

use super::edge_splitting_replay_parity_support::{
    build_edge_split_replay_parity_subject, replay_parity_report, EdgeSplitReplayParitySubject,
};
use super::metaboss_support::MetabossEventExtractionSubject;
use foreign_authority::assert_foreign_split_authorities_are_rejected;
use worth_kernel::workload_composition::CompletedBooleanSplitHandoff;
use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanDownstreamSplitConsumption, PlanarBooleanEdgeSplitReplayParityReport,
    PlanarBooleanLoopReconstructionSplitConsumption,
    PlanarBooleanLoopReconstructionSplitConsumptionInput,
};

pub(crate) fn assert_split_public_contract_requires_real_ledger_and_preserves_authority_boundaries()
{
    let subject =
        MetabossEventExtractionSubject::certify("phase7.3 public downstream split consumption");
    let replay_subject = build_edge_split_replay_parity_subject(&subject);
    let replay_report = replay_parity_report(&replay_subject);
    let completed_split_handoff = completed_split_handoff_for(&subject, &replay_subject);
    let consumption = admit_real_downstream_split_consumption(
        &replay_subject,
        &replay_report,
        &completed_split_handoff,
    );

    assert_downstream_consumption_preserves_real_split_authority(
        &consumption,
        &replay_subject,
        &replay_report,
        &completed_split_handoff,
    );
    assert_loop_reconstruction_consumes_downstream_split_product(&consumption);
    assert_foreign_split_authorities_are_rejected(
        &replay_subject,
        &replay_report,
        &completed_split_handoff,
    );
}

pub(crate) fn completed_split_handoff_for(
    subject: &MetabossEventExtractionSubject,
    replay_subject: &EdgeSplitReplayParitySubject,
) -> CompletedBooleanSplitHandoff {
    let completed_split_handoff = subject
        .pair()
        .left()
        .workload()
        .complete_boolean_split_handoff(replay_subject.original_ledger.receipt())
        .expect("real workload should produce a proof-bearing split completion handoff");
    completed_split_handoff
        .require_boolean_split()
        .expect("completed split handoff should require the exact split ledger receipt");
    completed_split_handoff
}

fn admit_real_downstream_split_consumption(
    replay_subject: &EdgeSplitReplayParitySubject,
    replay_report: &PlanarBooleanEdgeSplitReplayParityReport,
    completed_split_handoff: &CompletedBooleanSplitHandoff,
) -> PlanarBooleanDownstreamSplitConsumption {
    completed_split_handoff
        .admit_downstream_split_consumption(
            replay_subject.original_decision_log.receipt(),
            &replay_subject.original_products.validation,
            &replay_subject.original_products.naming,
            replay_report.receipt(),
        )
        .expect("real split ledger receipt should admit downstream split consumption")
}

fn assert_downstream_consumption_preserves_real_split_authority(
    consumption: &PlanarBooleanDownstreamSplitConsumption,
    replay_subject: &EdgeSplitReplayParitySubject,
    replay_report: &PlanarBooleanEdgeSplitReplayParityReport,
    completed_split_handoff: &CompletedBooleanSplitHandoff,
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
        completed_split_handoff.workload_stage_index_identity()
    );
    assert_downstream_consumption_counters_match_real_split_authority(
        consumption,
        replay_subject,
        replay_report,
        completed_split_handoff,
    );
}

fn assert_downstream_consumption_counters_match_real_split_authority(
    consumption: &PlanarBooleanDownstreamSplitConsumption,
    replay_subject: &EdgeSplitReplayParitySubject,
    replay_report: &PlanarBooleanEdgeSplitReplayParityReport,
    completed_split_handoff: &CompletedBooleanSplitHandoff,
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
        completed_split_handoff
            .completed_workload()
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
