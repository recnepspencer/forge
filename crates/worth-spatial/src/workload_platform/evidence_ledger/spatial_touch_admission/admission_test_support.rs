use super::*;
use crate::workload_platform::evidence_ledger::{
    BooleanEvidenceRowAuthority, CompleteWorkloadEvidenceLedger, WorkloadEvidenceLedger,
    WorkloadEvidenceRow, WorkloadEvidenceStage, WorkloadEvidenceStageCounters,
};
use crate::workload_platform::planar_boolean_edge_splitting::{
    source_carriers_for_tests, split_event_ledger_for_tests, split_pair_receipt_for_tests,
    PlanarBooleanSplitEdgeChainLedgerReceipt,
};
use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanEventLedgerReceipt, PlanarBooleanSegmentPairEnumerationReceipt,
};
use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::{
    prepared_loop_reconstruction_subject, prepared_phase_fourteen_subject, LoopFixtureEntryOrder,
};
use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanLoopDecisionLog, PlanarBooleanLoopReconstructionLedger,
    PlanarBooleanLoopReconstructionLedgerReceipt,
};
use crate::workload_platform::vocabulary::{
    DiagnosticWorkload, GeometryBindingWorkload, ProjectionWorkload, ResponseWorkload,
    RetainedReplayWorkload, SurfaceSupportWorkload, TransformWorkload,
};
use topology::facade::TopologySeed;

pub(super) struct SpatialReceiptAdmissionSubject<T> {
    pub(super) complete: CompleteWorkloadEvidenceLedger,
    pub(super) receipt: T,
}

pub(super) fn segment_pair_request_subject(
) -> SpatialReceiptAdmissionSubject<PlanarBooleanSegmentPairEnumerationReceipt> {
    let carriers = source_carriers_for_tests();
    let receipt = split_pair_receipt_for_tests(&carriers);
    let complete = request_boundary_complete_ledger_with_receipt(
        "phase4 segment-pair request-boundary admission",
        &receipt,
    );
    SpatialReceiptAdmissionSubject { complete, receipt }
}

pub(super) fn event_ledger_request_subject(
) -> SpatialReceiptAdmissionSubject<PlanarBooleanEventLedgerReceipt> {
    let carriers = source_carriers_for_tests();
    let segment_pairs = split_pair_receipt_for_tests(&carriers);
    let receipt = split_event_ledger_for_tests(
        segment_pairs.segment_pair_enumeration_identity(),
        carriers,
        Vec::new(),
        "phase4-production-event-ledger",
    );
    let complete = request_boundary_complete_ledger_with_receipt(
        "phase4 event-ledger request-boundary admission",
        &receipt,
    );
    SpatialReceiptAdmissionSubject { complete, receipt }
}

pub(super) fn split_request_subject(
    order: LoopFixtureEntryOrder,
) -> SpatialReceiptAdmissionSubject<PlanarBooleanSplitEdgeChainLedgerReceipt> {
    let subject = prepared_loop_reconstruction_subject(order);
    let receipt = subject.split_ledger_result.receipt().clone();
    let complete = request_boundary_complete_ledger_with_receipt(
        "phase4 split request-boundary admission",
        &receipt,
    );
    SpatialReceiptAdmissionSubject { complete, receipt }
}

pub(super) fn loop_reconstruction_request_subject(
    order: LoopFixtureEntryOrder,
) -> SpatialReceiptAdmissionSubject<PlanarBooleanLoopReconstructionLedgerReceipt> {
    let subject = prepared_phase_fourteen_subject(order);
    let decision_log = PlanarBooleanLoopDecisionLog::record(subject.decision_log_input())
        .expect("loop decision log should record from production reconstruction subject");
    let (_, receipt) =
        PlanarBooleanLoopReconstructionLedger::assemble(subject.ledger_input(&decision_log))
            .expect(
                "loop reconstruction ledger should assemble from production reconstruction subject",
            );
    let complete = request_boundary_complete_ledger_with_receipt(
        "phase4 loop request-boundary admission",
        &receipt,
    );
    SpatialReceiptAdmissionSubject { complete, receipt }
}

fn request_boundary_complete_ledger_with_receipt<T>(
    label: &'static str,
    receipt: &T,
) -> CompleteWorkloadEvidenceLedger
where
    T: BooleanEvidenceRowAuthority + 'static,
{
    complete_ledger_from_rows(with_receipt_row(
        certified_workload_stage_rows(label),
        WorkloadEvidenceRow::from_boolean_evidence_receipt(receipt),
    ))
}

pub(super) fn certified_workload_stage_rows(label: &'static str) -> Vec<WorkloadEvidenceRow> {
    let topology = TopologySeed::cube()
        .with_declaration(label)
        .build()
        .expect("topology seed should certify");
    let topology_workload = topology.query_receipts().declaration_receipt();
    let geometry = GeometryBindingWorkload::for_topology_receipt(topology_workload)
        .declared(format!("{label} geometry binding"))
        .admit()
        .expect("geometry binding should certify from topology receipt");
    let support = SurfaceSupportWorkload::for_geometry_binding(&geometry)
        .declared(format!("{label} surface support"))
        .admit()
        .expect("surface support should certify from geometry binding");
    let projection = ProjectionWorkload::for_surface_support(&support)
        .declared(format!("{label} projection"))
        .admit()
        .expect("projection should certify from surface support");
    let transform = TransformWorkload::for_projection(&projection)
        .declared(format!("{label} transform"))
        .admit()
        .expect("transform should certify from projection");
    let replay = RetainedReplayWorkload::for_transform(&transform)
        .declared(format!("{label} retained replay"))
        .admit()
        .expect("retained replay should certify from transform");
    let diagnostics = DiagnosticWorkload::for_retained_replay(&replay)
        .declared(format!("{label} diagnostics"))
        .admit()
        .expect("diagnostics should certify from retained replay");
    let response = ResponseWorkload::for_diagnostics(&diagnostics)
        .declared(format!("{label} response"))
        .admit()
        .expect("response should certify from diagnostics");

    vec![
        WorkloadEvidenceRow::from_topology_seed_receipt(&topology),
        WorkloadEvidenceRow::receipt_backed(
            WorkloadEvidenceStage::GeometryBinding,
            geometry.identity().receipt_identity(),
            WorkloadEvidenceStageCounters::binding(1),
        ),
        WorkloadEvidenceRow::receipt_backed(
            WorkloadEvidenceStage::SurfaceSupport,
            support.identity().receipt_identity(),
            WorkloadEvidenceStageCounters::surface_support(1),
        ),
        WorkloadEvidenceRow::receipt_backed(
            WorkloadEvidenceStage::Projection,
            projection.identity().receipt_identity(),
            WorkloadEvidenceStageCounters::projection(1, 1),
        ),
        WorkloadEvidenceRow::receipt_backed(
            WorkloadEvidenceStage::Transform,
            transform.identity().receipt_identity(),
            WorkloadEvidenceStageCounters::transform(1, 1, 0),
        ),
        WorkloadEvidenceRow::receipt_backed(
            WorkloadEvidenceStage::RetainedReplay,
            replay.identity().receipt_identity(),
            WorkloadEvidenceStageCounters::retained_replay(1, 1),
        ),
        WorkloadEvidenceRow::from_diagnostic_receipt(&diagnostics),
        WorkloadEvidenceRow::from_response_receipt(&response),
    ]
}

pub(super) fn complete_ledger_from_rows(
    rows: Vec<WorkloadEvidenceRow>,
) -> CompleteWorkloadEvidenceLedger {
    WorkloadEvidenceLedger::from_rows(rows)
        .expect("rows should index")
        .certify_complete()
        .expect("authority rows should complete")
}

pub(super) fn with_receipt_row(
    mut rows: Vec<WorkloadEvidenceRow>,
    receipt_row: WorkloadEvidenceRow,
) -> Vec<WorkloadEvidenceRow> {
    rows.push(receipt_row);
    rows
}

pub(super) fn rows_without_topology() -> Vec<WorkloadEvidenceRow> {
    synthetic_authority_rows()
        .into_iter()
        .filter(|row| row.stage() != WorkloadEvidenceStage::Topology)
        .collect()
}

pub(super) fn synthetic_authority_rows() -> Vec<WorkloadEvidenceRow> {
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

pub(super) fn synthetic_authority_rows_with_synthetic_topology() -> Vec<WorkloadEvidenceRow> {
    let mut rows = synthetic_authority_rows();
    rows[0] = WorkloadEvidenceRow::receipt_backed(
        WorkloadEvidenceStage::Topology,
        "synthetic-topology",
        WorkloadEvidenceStageCounters::topology(0, 0, 0),
    );
    rows
}

pub(super) fn synthetic_authority_rows_with_label_only_transform() -> Vec<WorkloadEvidenceRow> {
    let mut rows = synthetic_authority_rows();
    rows[4] = WorkloadEvidenceRow::receipt_backed(
        WorkloadEvidenceStage::Transform,
        "label-only-transform",
        WorkloadEvidenceStageCounters::transform(1, 0, 0),
    );
    rows
}

pub(super) fn synthetic_authority_rows_with_synthetic_replay() -> Vec<WorkloadEvidenceRow> {
    let mut rows = synthetic_authority_rows();
    rows[5] = WorkloadEvidenceRow::receipt_backed(
        WorkloadEvidenceStage::RetainedReplay,
        "synthetic-replay",
        WorkloadEvidenceStageCounters::retained_replay(0, 0),
    );
    rows
}

pub(super) fn synthetic_authority_rows_with_missing_receipt_backed_counters(
) -> Vec<WorkloadEvidenceRow> {
    let mut rows = synthetic_authority_rows();
    rows[6] = WorkloadEvidenceRow::receipt_backed(
        WorkloadEvidenceStage::Diagnostics,
        "counterless-diagnostics",
        WorkloadEvidenceStageCounters::default(),
    );
    rows
}

pub(super) fn assert_indexed_single_receipt_lookup(
    authority: &SpatialGeometryEvidenceTouchAuthority,
) {
    assert_eq!(authority.lookup_counters().required_stage_count(), 1);
    assert_eq!(authority.lookup_counters().indexed_lookup_count(), 1);
    assert_eq!(authority.lookup_counters().raw_row_scan_count(), 0);
    assert_eq!(authority.lookup_counters().rejected_raw_row_scan_count(), 0);
    assert_eq!(
        authority
            .lookup_counters()
            .rejected_string_prefix_stage_link_count(),
        0
    );
}
