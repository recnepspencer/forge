use crate::workload_platform::evidence_ledger::{
    CompleteWorkloadEvidenceLedger, WorkloadEvidenceLedger, WorkloadEvidenceRow,
    WorkloadEvidenceStage, WorkloadEvidenceStageCounters,
};
use crate::workload_platform::vocabulary::{
    DiagnosticWorkload, GeometryBindingWorkload, ProjectionWorkload, ResponseWorkload,
    RetainedReplayWorkload, SurfaceSupportWorkload, TransformWorkload,
};
use topology::facade::TopologySeed;

pub(crate) fn current_workload_stage_rows(label: &'static str) -> Vec<WorkloadEvidenceRow> {
    let topology = TopologySeed::cube()
        .with_declaration(label)
        .build()
        .expect("current authority world topology seed should certify");
    let topology_workload = topology.query_receipts().declaration_receipt();
    let geometry = GeometryBindingWorkload::for_topology_receipt(topology_workload)
        .declared(format!("{label} geometry binding"))
        .admit()
        .expect("current authority world geometry binding should certify");
    let support = SurfaceSupportWorkload::for_geometry_binding(&geometry)
        .declared(format!("{label} surface support"))
        .admit()
        .expect("current authority world surface support should certify");
    let projection = ProjectionWorkload::for_surface_support(&support)
        .declared(format!("{label} projection"))
        .admit()
        .expect("current authority world projection should certify");
    let transform = TransformWorkload::for_projection(&projection)
        .declared(format!("{label} transform"))
        .admit()
        .expect("current authority world transform should certify");
    let replay = RetainedReplayWorkload::for_transform(&transform)
        .declared(format!("{label} retained replay"))
        .admit()
        .expect("current authority world retained replay should certify");
    let diagnostics = DiagnosticWorkload::for_retained_replay(&replay)
        .declared(format!("{label} diagnostics"))
        .admit()
        .expect("current authority world diagnostics should certify");
    let response = ResponseWorkload::for_diagnostics(&diagnostics)
        .declared(format!("{label} response"))
        .admit()
        .expect("current authority world response should certify");

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

pub(crate) fn current_complete_ledger_from_rows(
    rows: Vec<WorkloadEvidenceRow>,
) -> CompleteWorkloadEvidenceLedger {
    WorkloadEvidenceLedger::from_rows(rows)
        .expect("current authority world rows should index")
        .certify_complete()
        .expect("current authority world rows should complete")
}
