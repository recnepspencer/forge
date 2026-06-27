use super::{SpatialGeometryEvidenceTouchAuthority, SpatialGeometryEvidenceTouchRequest};
use crate::workload_platform::evidence_ledger::{
    BooleanEvidenceReceipt, BooleanEvidenceRowAuthority, BooleanEvidenceStageKind,
    CompleteWorkloadEvidenceLedger, WorkloadEvidenceLedger, WorkloadEvidenceRow,
    WorkloadEvidenceStage, WorkloadEvidenceStageCounters, WorkloadEvidenceSupport,
};
use crate::workload_platform::vocabulary::{
    DiagnosticWorkload, GeometryBindingWorkload, ProjectionWorkload, ResponseWorkload,
    RetainedReplayWorkload, SurfaceSupportWorkload, TransformWorkload,
};
use topology::facade::TopologySeed;

pub(crate) fn receipt_backed_event_ledger_touch_authority_for_admission_tests(
) -> SpatialGeometryEvidenceTouchAuthority {
    receipt_backed_touch_authority_for_admission_tests_with_declared_world(
        BooleanEvidenceStageKind::EventLedger,
        "phase-11-event-ledger-receipt",
        "phase-3 lookup input admission receipt-backed fixture",
    )
}

pub(crate) fn receipt_backed_touch_authority_for_admission_tests(
    boolean_stage: BooleanEvidenceStageKind,
    evidence_identity: &'static str,
) -> SpatialGeometryEvidenceTouchAuthority {
    receipt_backed_touch_authority_for_admission_tests_with_declared_world(
        boolean_stage,
        evidence_identity,
        "phase-3 lookup input admission receipt-backed fixture",
    )
}

pub(crate) fn receipt_backed_touch_authority_for_admission_tests_with_declared_world(
    boolean_stage: BooleanEvidenceStageKind,
    evidence_identity: &'static str,
    declared_world: &'static str,
) -> SpatialGeometryEvidenceTouchAuthority {
    let receipt = ReceiptBackedAdmissionTestReceipt {
        boolean_stage,
        evidence_identity,
    };
    let complete = complete_ledger_from_rows(with_receipt_row(
        certified_workload_stage_rows(declared_world),
        WorkloadEvidenceRow::from_boolean_evidence_receipt(&receipt),
    ));
    SpatialGeometryEvidenceTouchRequest::from_boolean_receipt(&receipt)
        .with_complete_ledger(&complete)
        .admit()
        .expect("test receipt fixture should admit through the production touch boundary")
}

fn certified_workload_stage_rows(label: &'static str) -> Vec<WorkloadEvidenceRow> {
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

fn complete_ledger_from_rows(rows: Vec<WorkloadEvidenceRow>) -> CompleteWorkloadEvidenceLedger {
    WorkloadEvidenceLedger::from_rows(rows)
        .expect("rows should index")
        .certify_complete()
        .expect("authority rows should complete")
}

fn with_receipt_row(
    mut rows: Vec<WorkloadEvidenceRow>,
    receipt_row: WorkloadEvidenceRow,
) -> Vec<WorkloadEvidenceRow> {
    rows.push(receipt_row);
    rows
}

struct ReceiptBackedAdmissionTestReceipt {
    boolean_stage: BooleanEvidenceStageKind,
    evidence_identity: &'static str,
}

impl BooleanEvidenceReceipt for ReceiptBackedAdmissionTestReceipt {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        self.boolean_stage
    }

    fn evidence_identity(&self) -> &str {
        self.evidence_identity
    }

    fn evidence_support(&self) -> WorkloadEvidenceSupport {
        WorkloadEvidenceSupport::Admitted
    }

    fn evidence_counters(&self) -> WorkloadEvidenceStageCounters {
        match self.boolean_stage {
            BooleanEvidenceStageKind::SharedPlaneIdentity => {
                WorkloadEvidenceStageCounters::boolean_shared_plane_identity()
            }
            BooleanEvidenceStageKind::LocalFrameSelection => {
                WorkloadEvidenceStageCounters::boolean_local_frame_selection()
            }
            BooleanEvidenceStageKind::OperandAProjectionConsumption => {
                WorkloadEvidenceStageCounters::boolean_operand_a_projection_consumption()
            }
            BooleanEvidenceStageKind::OperandBProjectionConsumption => {
                WorkloadEvidenceStageCounters::boolean_operand_b_projection_consumption()
            }
            BooleanEvidenceStageKind::EventLedger => {
                WorkloadEvidenceStageCounters::boolean_event_ledger(
                    crate::workload_platform::planar_boolean_events::PlanarBooleanEventLedgerCounters::default(),
                )
            }
            BooleanEvidenceStageKind::Split => WorkloadEvidenceStageCounters::boolean_split(),
            _ => WorkloadEvidenceStageCounters::boolean_declaration(),
        }
    }
}

impl crate::trusted_boolean_evidence_authority::Seal for ReceiptBackedAdmissionTestReceipt {}

impl BooleanEvidenceRowAuthority for ReceiptBackedAdmissionTestReceipt {}
