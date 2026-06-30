use topology::facade::TopologyWorkload;
use worth_spatial::facade::workload_vocabulary::{
    DiagnosticWorkload, GeometryBindingWorkload, ProjectionWorkload, ResponseWorkload,
    RetainedReplayWorkload, SurfaceSupportWorkload, TransformWorkload,
    WorkloadEvidenceLedger, WorkloadEvidenceRow,
};

use super::super::closeout::current_replay_undo_inventory_report;
use super::super::declaration::{ReplayUndoDeclaredInputRole, ReplayUndoDeclaredSourceIdentity};
use crate::workload_composition::{WorthWorkload, WorthWorkloadParts};

#[test]
fn workload_lowering_preserves_authority_and_observability_split() {
    let closeout = current_replay_undo_inventory_report().expect("closeout");
    let workload = certified_workload();
    let retained = closeout
        .require_source(ReplayUndoDeclaredSourceIdentity::KernelWorthWorkloadRetainedReplay)
        .expect("retained");
    let diagnostics = closeout
        .require_source(ReplayUndoDeclaredSourceIdentity::KernelWorthWorkloadDiagnostics)
        .expect("diagnostics");

    assert!(!workload
        .retained_replay()
        .identity()
        .receipt_identity()
        .is_empty());
    assert!(!workload
        .diagnostics()
        .identity()
        .receipt_identity()
        .is_empty());
    assert!(retained
        .authority_roles()
        .contains(ReplayUndoDeclaredInputRole::RetainedReplayWorkloadReceipt));
    assert!(!retained
        .observability_roles()
        .contains(ReplayUndoDeclaredInputRole::DiagnosticsWorkloadReceipt));
    assert!(diagnostics
        .observability_roles()
        .contains(ReplayUndoDeclaredInputRole::DiagnosticsWorkloadReceipt));
}

fn certified_workload() -> WorthWorkload {
    let topology = TopologyWorkload::declared("replay inventory topology")
        .from_query_declaration(".topology.seed")
        .expect("topology");
    let geometry = GeometryBindingWorkload::for_topology_receipt(&topology)
        .admit()
        .expect("geometry");
    let support = SurfaceSupportWorkload::for_geometry_binding(&geometry)
        .admit()
        .expect("support");
    let projection = ProjectionWorkload::for_surface_support(&support)
        .admit()
        .expect("projection");
    let transform = TransformWorkload::for_projection(&projection)
        .admit()
        .expect("transform");
    let replay = RetainedReplayWorkload::for_transform(&transform)
        .admit()
        .expect("replay");
    let diagnostics = DiagnosticWorkload::for_retained_replay(&replay)
        .admit()
        .expect("diagnostics");
    let response = ResponseWorkload::for_diagnostics(&diagnostics)
        .admit()
        .expect("response");
    let ledger = WorkloadEvidenceLedger::from_rows(vec![
        WorkloadEvidenceRow::from_topology_receipt(&topology),
        WorkloadEvidenceRow::from_geometry_binding_receipt(&geometry),
        WorkloadEvidenceRow::from_surface_support_receipt(&support),
        WorkloadEvidenceRow::from_projection_receipt(&projection),
        WorkloadEvidenceRow::from_transform_receipt(&transform),
        WorkloadEvidenceRow::from_retained_replay_receipt(&replay),
        WorkloadEvidenceRow::from_diagnostic_receipt(&diagnostics),
        WorkloadEvidenceRow::from_response_receipt(&response),
    ])
    .expect("ledger rows")
    .certify_complete()
    .expect("complete ledger");

    WorthWorkload::compose(WorthWorkloadParts {
        topology,
        geometry_binding: geometry,
        surface_support: support,
        projection,
        transform,
        retained_replay: replay,
        batch_admission_execution: None,
        diagnostics,
        response,
        evidence_ledger: ledger,
    })
    .expect("workload")
}
