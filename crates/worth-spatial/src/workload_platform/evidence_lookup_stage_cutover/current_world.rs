use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_platform::evidence_ledger::{
    current_complete_ledger_from_rows, current_workload_stage_rows, CompleteWorkloadEvidenceLedger,
    SpatialGeometryEvidenceTouchRowRequest, WorkloadEvidenceRow, WorkloadEvidenceStage,
    WorkloadEvidenceStageCounters, WorkloadEvidenceSupport,
};
use crate::workload_platform::vocabulary::{
    GeometryBindingWorkload, ProjectionWorkload, RetainedReplayWorkload,
    RetainedReplayWorkloadReceipt, SurfaceSupportWorkload, TransformWorkload,
};

use super::current_path::EvidenceLookupCurrentPathError;

pub(crate) fn current_spatial_touch_authority(
    stage: WorkloadEvidenceStage,
) -> Result<
    crate::workload_platform::evidence_ledger::SpatialGeometryEvidenceTouchAuthority,
    EvidenceLookupCurrentPathError,
> {
    let row = current_boolean_stage_row(stage);
    SpatialGeometryEvidenceTouchRowRequest::from_boolean_row(&row)
        .with_complete_ledger(&current_complete_ledger_for_row(&row))
        .admit()
        .map_err(|error| EvidenceLookupCurrentPathError::from_spatial_touch_denial(stage, error))
}

#[cfg(test)]
pub(crate) fn current_complete_ledger_for_authority(
    authority: &crate::workload_platform::evidence_ledger::SpatialGeometryEvidenceTouchAuthority,
) -> CompleteWorkloadEvidenceLedger {
    let mut rows = authority.authority_rows().to_vec();
    rows.push(authority.selected_receipt_row());
    rows.push(current_unrelated_boolean_row());
    current_complete_ledger_from_rows(rows)
}

fn current_complete_ledger_for_row(row: &WorkloadEvidenceRow) -> CompleteWorkloadEvidenceLedger {
    let mut rows = current_workload_stage_rows(current_world_label(row.stage()));
    rows.push(row.clone());
    current_complete_ledger_from_rows(rows)
}

fn current_boolean_stage_row(stage: WorkloadEvidenceStage) -> WorkloadEvidenceRow {
    WorkloadEvidenceRow::receipt_backed_with_support(
        stage,
        truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "worth-spatial:evidence-lookup-current-stage-receipt:v1".to_string(),
                format!("stage:{}", stage.human_name()),
                format!("authority-world:{}", current_world_label(stage)),
            ],
        ),
        WorkloadEvidenceSupport::Admitted,
        current_boolean_stage_counters(stage),
    )
}

#[cfg(test)]
fn current_unrelated_boolean_row() -> WorkloadEvidenceRow {
    WorkloadEvidenceRow::receipt_backed_with_support(
        WorkloadEvidenceStage::BooleanSharedPlaneIdentity,
        truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "worth-spatial:evidence-lookup-current-world:unrelated".to_string(),
                "stage:boolean-shared-plane-identity".to_string(),
            ],
        ),
        WorkloadEvidenceSupport::Admitted,
        WorkloadEvidenceStageCounters::boolean_shared_plane_identity(),
    )
}

fn current_world_label(stage: WorkloadEvidenceStage) -> &'static str {
    match stage {
        WorkloadEvidenceStage::BooleanEventLedger => {
            "phase-11 current event-ledger authority world"
        }
        WorkloadEvidenceStage::BooleanSplit => "phase-11 current split authority world",
        WorkloadEvidenceStage::BooleanOperandAProjectionConsumption => {
            "phase-11 current operand-a projection authority world"
        }
        WorkloadEvidenceStage::BooleanOperandBProjectionConsumption => {
            "phase-11 current operand-b projection authority world"
        }
        WorkloadEvidenceStage::BooleanSharedPlaneIdentity => {
            "phase-11 current shared-plane authority world"
        }
        WorkloadEvidenceStage::BooleanLocalFrameSelection => {
            "phase-11 current local-frame authority world"
        }
        other => panic!("unsupported current cutover-path stage: {other:?}"),
    }
}

pub(crate) fn current_retained_replay_receipt_for_stage(
    stage: WorkloadEvidenceStage,
) -> RetainedReplayWorkloadReceipt {
    let label = current_world_label(stage);
    let topology = topology::facade::TopologySeed::cube()
        .with_declaration(label)
        .build()
        .expect("current cutover-path retained replay should certify topology");
    let geometry = GeometryBindingWorkload::for_topology_receipt(
        topology.query_receipts().declaration_receipt(),
    )
    .declared(format!("{label} geometry binding"))
    .admit()
    .expect("current cutover-path retained replay should certify geometry binding");
    let support = SurfaceSupportWorkload::for_geometry_binding(&geometry)
        .declared(format!("{label} surface support"))
        .admit()
        .expect("current cutover-path retained replay should certify surface support");
    let projection = ProjectionWorkload::for_surface_support(&support)
        .declared(format!("{label} projection"))
        .admit()
        .expect("current cutover-path retained replay should certify projection");
    let transform = TransformWorkload::for_projection(&projection)
        .declared(format!("{label} transform"))
        .admit()
        .expect("current cutover-path retained replay should certify transform");
    RetainedReplayWorkload::for_transform(&transform)
        .declared(format!("{label} retained replay"))
        .admit()
        .expect("current cutover-path retained replay should certify retained replay")
}

fn current_boolean_stage_counters(stage: WorkloadEvidenceStage) -> WorkloadEvidenceStageCounters {
    match stage {
        WorkloadEvidenceStage::BooleanEventLedger => WorkloadEvidenceStageCounters::boolean_event_ledger(
            crate::workload_platform::planar_boolean_events::PlanarBooleanEventLedgerCounters::default(),
        ),
        WorkloadEvidenceStage::BooleanSplit => WorkloadEvidenceStageCounters::boolean_split(),
        WorkloadEvidenceStage::BooleanOperandAProjectionConsumption => {
            WorkloadEvidenceStageCounters::boolean_operand_a_projection_consumption()
        }
        WorkloadEvidenceStage::BooleanOperandBProjectionConsumption => {
            WorkloadEvidenceStageCounters::boolean_operand_b_projection_consumption()
        }
        WorkloadEvidenceStage::BooleanSharedPlaneIdentity => {
            WorkloadEvidenceStageCounters::boolean_shared_plane_identity()
        }
        WorkloadEvidenceStage::BooleanLocalFrameSelection => {
            WorkloadEvidenceStageCounters::boolean_local_frame_selection()
        }
        other => panic!("unsupported current cutover-path stage counters: {other:?}"),
    }
}
