use crate::workload_platform::evidence_ledger::{
    WorkloadEvidenceLedger, WorkloadEvidenceRow, WorkloadEvidenceStage,
    WorkloadEvidenceStageCounters,
};
use crate::workload_platform::evidence_lookup_family_catalog::EvidenceLookupDiagnosticWitnessKind;
use crate::workload_platform::planar_boolean_edge_splitting::event_ledger_lookup_execution_subject;
use crate::workload_platform::planar_boolean_events::PlanarBooleanEventLedgerLookupExecutionWitness;

use super::test_support::{
    candidate_index_gate_for, event_ledger_for, event_ledger_with_identities, indexed_evidence,
    production_segment_pair_receipt, request_subject,
};
use super::{
    PlanarBooleanEdgeSplitRequest, PlanarBooleanEdgeSplitRequestDenial,
    PlanarBooleanEdgeSplitRequestDenialKind, PlanarBooleanEdgeSplitRequestInput,
};

#[test]
fn first_stage_lookup_slice_preserves_semantics_with_new_receipt() {
    let subject = request_subject();
    let event_ledger_lookup = event_ledger_lookup_execution_subject(
        "edge-split-request",
        &subject.event_ledger,
        vec![
            WorkloadEvidenceRow::from_boolean_evidence_receipt(&subject.segment_pairs),
            WorkloadEvidenceRow::from_boolean_evidence_receipt(&subject.event_ledger),
        ],
    );
    let request = PlanarBooleanEdgeSplitRequest::admit(PlanarBooleanEdgeSplitRequestInput::new(
        &subject.event_ledger,
        &subject.candidate_index_gate,
        &event_ledger_lookup.witness,
        None,
    ))
    .expect("split request should admit from receipt-backed event ledger and candidate-index gate");

    assert_eq!(
        request.event_ledger_identity(),
        subject.event_ledger.event_ledger_identity()
    );
    assert_eq!(
        request.downstream_consumption_identity(),
        subject.event_ledger.downstream_consumption_identity()
    );
    assert_eq!(
        request.reduced_pair_identity(),
        subject.event_ledger.reduced_pair_identity()
    );
    assert_eq!(
        request.segment_pair_enumeration_identity(),
        subject.event_ledger.segment_pair_enumeration_identity()
    );
    assert_eq!(
        request.candidate_index_consumption_gate_identity(),
        subject.candidate_index_gate.gate_identity()
    );
    assert_eq!(
        request.candidate_index_product_identity(),
        subject
            .candidate_index_gate
            .candidate_index_product_identity()
    );
    assert_eq!(
        request.query_index_plan_digest(),
        subject.candidate_index_gate.query_index_plan_digest()
    );
    assert_eq!(
        request.event_ledger_lookup_selected_plan_digest(),
        event_ledger_lookup
            .packet
            .selected_plan()
            .selected_plan_digest()
    );
    assert_eq!(
        request.event_ledger_lookup_execution_receipt_digest(),
        event_ledger_lookup
            .packet
            .execution_receipt()
            .execution_receipt_digest()
    );
    assert_eq!(
        request.event_ledger_lookup_product_output_digest(),
        event_ledger_lookup
            .packet
            .execution_receipt()
            .lookup_product_output_digest()
    );
    assert_eq!(request.counters().split_request_count(), 1);
    let selected_row = event_ledger_lookup
        .packet
        .selected_plan()
        .rows()
        .iter()
        .find(|row| {
            row.outcome()
                == crate::workload_platform::evidence_lookup_plan_selection::EvidenceLookupPlanRowOutcome::Selected
        })
        .expect("lookup packet should retain the selected family row");
    assert_eq!(
        event_ledger_lookup.packet.selected_family_identity(),
        selected_row.family_identity()
    );
    assert_eq!(
        event_ledger_lookup
            .packet
            .selected_family_diagnostic_witness_shape()
            .kind(),
        EvidenceLookupDiagnosticWitnessKind::SpatialTouchStageReceiptOnly
    );
}

#[test]
fn first_stage_lookup_slice_denies_wrong_touch_or_stage_receipt() {
    let subject = request_subject();
    let foreign_event_ledger = event_ledger_for("foreign-segment-pair-enumeration");
    let foreign_lookup = event_ledger_lookup_execution_subject(
        "foreign-touch",
        &foreign_event_ledger,
        vec![WorkloadEvidenceRow::from_boolean_evidence_receipt(
            &foreign_event_ledger,
        )],
    );

    let denial = PlanarBooleanEventLedgerLookupExecutionWitness::admit(
        &subject.event_ledger,
        &foreign_lookup.complete_ledger,
    )
    .expect_err("foreign complete ledger must deny before lookup execution");

    assert_eq!(
        denial.kind(),
        crate::workload_platform::planar_boolean_events::PlanarBooleanEventLedgerLookupExecutionDenialKind::SpatialTouchAuthority
    );
}

#[test]
fn first_stage_lookup_slice_scale_pressure_exposes_broad_scan() {
    let subject = request_subject();
    let baseline = execution_receipt_for_rows(
        &subject.event_ledger,
        vec![
            WorkloadEvidenceRow::from_boolean_evidence_receipt(&subject.segment_pairs),
            WorkloadEvidenceRow::from_boolean_evidence_receipt(&subject.event_ledger),
        ],
    );
    let noisy = execution_receipt_for_rows(
        &subject.event_ledger,
        std::iter::once(WorkloadEvidenceRow::from_boolean_evidence_receipt(
            &subject.segment_pairs,
        ))
        .chain(std::iter::once(
            WorkloadEvidenceRow::from_boolean_evidence_receipt(&subject.event_ledger),
        ))
        .chain([
            WorkloadEvidenceRow::receipt_backed(
                WorkloadEvidenceStage::BooleanSharedPlaneIdentity,
                "unrelated-shared-plane",
                WorkloadEvidenceStageCounters::boolean_shared_plane_identity(),
            ),
            WorkloadEvidenceRow::receipt_backed(
                WorkloadEvidenceStage::BooleanLocalFrameSelection,
                "unrelated-local-frame",
                WorkloadEvidenceStageCounters::boolean_local_frame_selection(),
            ),
            WorkloadEvidenceRow::receipt_backed(
                WorkloadEvidenceStage::BooleanOperandAProjectionConsumption,
                "unrelated-operand-a-projection",
                WorkloadEvidenceStageCounters::boolean_operand_a_projection_consumption(),
            ),
            WorkloadEvidenceRow::receipt_backed(
                WorkloadEvidenceStage::BooleanOperandBProjectionConsumption,
                "unrelated-operand-b-projection",
                WorkloadEvidenceStageCounters::boolean_operand_b_projection_consumption(),
            ),
            WorkloadEvidenceRow::receipt_backed(
                WorkloadEvidenceStage::BooleanReducedOperandPair,
                "unrelated-reduced-pair",
                WorkloadEvidenceStageCounters::boolean_reduced_operand_pair(),
            ),
        ])
        .collect(),
    );

    assert_eq!(baseline.outcome(), noisy.outcome());
    assert_eq!(
        baseline.counters().ledger_rows_touched_count(),
        noisy.counters().ledger_rows_touched_count()
    );
    assert_eq!(
        baseline.counters().evidence_candidate_count(),
        noisy.counters().evidence_candidate_count()
    );
}

#[test]
fn edge_split_request_rejects_missing_event_ledger_evidence() {
    let segment_pairs = production_segment_pair_receipt();
    let event_ledger = event_ledger_for(segment_pairs.segment_pair_enumeration_identity());
    let evidence = WorkloadEvidenceLedger::from_rows(vec![
        WorkloadEvidenceRow::from_boolean_evidence_receipt(&segment_pairs),
    ])
    .expect("segment-pair row should index without event ledger row");
    let _gate = candidate_index_gate_for(&event_ledger, &segment_pairs);

    let lookup_error = evidence
        .require_boolean_receipt_lookup(&event_ledger)
        .expect_err("split request input must require event-ledger evidence");
    let denial = PlanarBooleanEdgeSplitRequestDenial::from_event_ledger_evidence_error(
        lookup_error,
        event_ledger.event_ledger_identity(),
    );

    assert_eq!(
        denial.kind(),
        PlanarBooleanEdgeSplitRequestDenialKind::MissingEventLedgerEvidence
    );
}

#[test]
fn edge_split_request_rejects_manual_event_ledger_evidence() {
    let segment_pairs = production_segment_pair_receipt();
    let event_ledger = event_ledger_for(segment_pairs.segment_pair_enumeration_identity());
    let evidence = WorkloadEvidenceLedger::from_rows(vec![
        WorkloadEvidenceRow::from_boolean_evidence_receipt(&segment_pairs),
        WorkloadEvidenceRow::new(
            WorkloadEvidenceStage::BooleanEventLedger,
            event_ledger.event_ledger_identity(),
        ),
    ])
    .expect("manual event-ledger row should index before split request admission");
    let _gate = candidate_index_gate_for(&event_ledger, &segment_pairs);

    let lookup_error = evidence
        .require_boolean_receipt_lookup(&event_ledger)
        .expect_err("split request input must reject hand-filled event-ledger evidence");
    let denial = PlanarBooleanEdgeSplitRequestDenial::from_event_ledger_evidence_error(
        lookup_error,
        event_ledger.event_ledger_identity(),
    );

    assert_eq!(
        denial.kind(),
        PlanarBooleanEdgeSplitRequestDenialKind::ManualEventLedgerEvidence
    );
}

#[test]
fn edge_split_request_rejects_candidate_index_gate_from_foreign_event_ledger() {
    let subject = request_subject();
    let foreign_event_ledger = event_ledger_for("foreign-segment-pair-enumeration");
    let _evidence = WorkloadEvidenceLedger::from_rows(vec![
        WorkloadEvidenceRow::from_boolean_evidence_receipt(&subject.segment_pairs),
        WorkloadEvidenceRow::from_boolean_evidence_receipt(&foreign_event_ledger),
    ])
    .expect("foreign event-ledger row should index before split request binding denial");
    let event_ledger_lookup = event_ledger_lookup_execution_subject(
        "foreign-event-ledger",
        &foreign_event_ledger,
        vec![
            WorkloadEvidenceRow::from_boolean_evidence_receipt(&subject.segment_pairs),
            WorkloadEvidenceRow::from_boolean_evidence_receipt(&foreign_event_ledger),
        ],
    );

    let denial = PlanarBooleanEdgeSplitRequest::admit(PlanarBooleanEdgeSplitRequestInput::new(
        &foreign_event_ledger,
        &subject.candidate_index_gate,
        &event_ledger_lookup.witness,
        None,
    ))
    .expect_err("split request must reject a candidate-index gate from another event ledger");

    assert_eq!(
        denial.kind(),
        PlanarBooleanEdgeSplitRequestDenialKind::CandidateIndexGateEventLedgerMismatch
    );
}

#[test]
fn edge_split_request_rejects_candidate_index_gate_from_foreign_downstream_consumption() {
    let subject = request_subject();
    let foreign_event_ledger = event_ledger_with_identities(
        subject.event_ledger.segment_pair_enumeration_identity(),
        subject.event_ledger.event_ledger_identity(),
        "foreign-downstream-consumption",
        subject.event_ledger.reduced_pair_identity(),
    );
    let _evidence = indexed_evidence(&subject.segment_pairs, &foreign_event_ledger);
    let event_ledger_lookup = event_ledger_lookup_execution_subject(
        "foreign-downstream",
        &foreign_event_ledger,
        vec![
            WorkloadEvidenceRow::from_boolean_evidence_receipt(&subject.segment_pairs),
            WorkloadEvidenceRow::from_boolean_evidence_receipt(&foreign_event_ledger),
        ],
    );

    let denial = PlanarBooleanEdgeSplitRequest::admit(PlanarBooleanEdgeSplitRequestInput::new(
        &foreign_event_ledger,
        &subject.candidate_index_gate,
        &event_ledger_lookup.witness,
        None,
    ))
    .expect_err("split request must reject a foreign downstream-consumption lineage");

    assert_eq!(
        denial.kind(),
        PlanarBooleanEdgeSplitRequestDenialKind::CandidateIndexGateDownstreamMismatch
    );
}

#[test]
fn edge_split_request_rejects_candidate_index_gate_from_foreign_reduced_pair() {
    let subject = request_subject();
    let foreign_event_ledger = event_ledger_with_identities(
        subject.event_ledger.segment_pair_enumeration_identity(),
        subject.event_ledger.event_ledger_identity(),
        subject.event_ledger.downstream_consumption_identity(),
        "foreign-reduced-pair",
    );
    let _evidence = indexed_evidence(&subject.segment_pairs, &foreign_event_ledger);
    let event_ledger_lookup = event_ledger_lookup_execution_subject(
        "foreign-reduced-pair",
        &foreign_event_ledger,
        vec![
            WorkloadEvidenceRow::from_boolean_evidence_receipt(&subject.segment_pairs),
            WorkloadEvidenceRow::from_boolean_evidence_receipt(&foreign_event_ledger),
        ],
    );

    let denial = PlanarBooleanEdgeSplitRequest::admit(PlanarBooleanEdgeSplitRequestInput::new(
        &foreign_event_ledger,
        &subject.candidate_index_gate,
        &event_ledger_lookup.witness,
        None,
    ))
    .expect_err("split request must reject a foreign reduced-pair lineage");

    assert_eq!(
        denial.kind(),
        PlanarBooleanEdgeSplitRequestDenialKind::CandidateIndexGateReducedPairMismatch
    );
}

#[test]
fn edge_split_request_rejects_candidate_index_gate_from_foreign_segment_pair_enumeration() {
    let subject = request_subject();
    let foreign_event_ledger = event_ledger_with_identities(
        "foreign-segment-pair-enumeration",
        subject.event_ledger.event_ledger_identity(),
        subject.event_ledger.downstream_consumption_identity(),
        subject.event_ledger.reduced_pair_identity(),
    );
    let _evidence = WorkloadEvidenceLedger::from_rows(vec![
        WorkloadEvidenceRow::from_boolean_evidence_receipt(&foreign_event_ledger),
    ])
    .expect("foreign event-ledger row should index before split request binding denial");
    let event_ledger_lookup = event_ledger_lookup_execution_subject(
        "foreign-segment-pair",
        &foreign_event_ledger,
        vec![WorkloadEvidenceRow::from_boolean_evidence_receipt(
            &foreign_event_ledger,
        )],
    );

    let denial = PlanarBooleanEdgeSplitRequest::admit(PlanarBooleanEdgeSplitRequestInput::new(
        &foreign_event_ledger,
        &subject.candidate_index_gate,
        &event_ledger_lookup.witness,
        None,
    ))
    .expect_err("split request must reject a foreign segment-pair enumeration lineage");

    assert_eq!(
        denial.kind(),
        PlanarBooleanEdgeSplitRequestDenialKind::CandidateIndexGateSegmentPairMismatch
    );
}

fn execution_receipt_for_rows(
    event_ledger: &crate::workload_platform::planar_boolean_events::PlanarBooleanEventLedgerReceipt,
    evidence_rows: Vec<WorkloadEvidenceRow>,
) -> crate::workload_platform::evidence_lookup_execution::EvidenceLookupExecutionReceipt {
    event_ledger_lookup_execution_subject("edge-split-scale", event_ledger, evidence_rows)
        .packet
        .execution_receipt()
        .clone()
}
