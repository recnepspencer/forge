use crate::workload_platform::evidence_ledger::{
    CompleteWorkloadEvidenceLedger, WorkloadEvidenceLedger, WorkloadEvidenceRow,
    WorkloadEvidenceStage, WorkloadEvidenceStageCounters,
};
use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanEventLedgerLookupExecutionPacket, PlanarBooleanEventLedgerLookupExecutionWitness,
    PlanarBooleanEventLedgerReceipt,
};
use crate::workload_platform::vocabulary::{
    DiagnosticWorkload, GeometryBindingWorkload, ProjectionWorkload, ResponseWorkload,
    RetainedReplayWorkload, SurfaceSupportWorkload, TransformWorkload,
};
use topology::facade::TopologySeed;

pub(crate) struct EventLedgerLookupExecutionTestSubject {
    pub(crate) complete_ledger: CompleteWorkloadEvidenceLedger,
    pub(crate) packet: PlanarBooleanEventLedgerLookupExecutionPacket,
    pub(crate) witness: PlanarBooleanEventLedgerLookupExecutionWitness,
}

pub(crate) fn event_ledger_lookup_execution_subject(
    tag: &str,
    event_ledger: &PlanarBooleanEventLedgerReceipt,
    evidence_rows: Vec<WorkloadEvidenceRow>,
) -> EventLedgerLookupExecutionTestSubject {
    let complete_ledger = complete_lookup_test_ledger(tag, evidence_rows);
    let packet =
        PlanarBooleanEventLedgerLookupExecutionPacket::admit(event_ledger, &complete_ledger)
            .expect("event-ledger lookup execution packet should admit for test subject");
    let witness = packet.witness().clone();
    EventLedgerLookupExecutionTestSubject {
        complete_ledger,
        packet,
        witness,
    }
}

fn complete_lookup_test_ledger(
    tag: &str,
    mut evidence_rows: Vec<WorkloadEvidenceRow>,
) -> CompleteWorkloadEvidenceLedger {
    let label = format!("{tag} lookup execution");
    let topology = TopologySeed::cube()
        .with_declaration(label.clone())
        .build()
        .expect("topology seed should certify");
    let topology_workload = topology.query_receipts().declaration_receipt();
    let geometry = GeometryBindingWorkload::for_topology_receipt(topology_workload)
        .declared(format!("{label} geometry binding"))
        .admit()
        .expect("geometry binding should certify");
    let support = SurfaceSupportWorkload::for_geometry_binding(&geometry)
        .declared(format!("{label} surface support"))
        .admit()
        .expect("surface support should certify");
    let projection = ProjectionWorkload::for_surface_support(&support)
        .declared(format!("{label} projection"))
        .admit()
        .expect("projection should certify");
    let transform = TransformWorkload::for_projection(&projection)
        .declared(format!("{label} transform"))
        .admit()
        .expect("transform should certify");
    let replay = RetainedReplayWorkload::for_transform(&transform)
        .declared(format!("{label} retained replay"))
        .admit()
        .expect("retained replay should certify");
    let diagnostics = DiagnosticWorkload::for_retained_replay(&replay)
        .declared(format!("{label} diagnostics"))
        .admit()
        .expect("diagnostics should certify");
    let response = ResponseWorkload::for_diagnostics(&diagnostics)
        .declared(format!("{label} response"))
        .admit()
        .expect("response should certify");

    let mut rows = vec![
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
    ];
    rows.append(&mut evidence_rows);
    WorkloadEvidenceLedger::from_rows(rows)
        .expect("lookup execution test rows should index")
        .certify_complete()
        .expect("lookup execution test rows should certify complete")
}
