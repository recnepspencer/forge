mod boolean_evidence_ledger;
pub(crate) mod evidence_ledger_receipts;
mod honesty_guards;

use topology::facade::TopologyWorkload;
use worth_spatial::facade::workload_vocabulary::{
    DiagnosticWorkload, GeometryBindingWorkload, ProjectionWorkload, ResponseWorkload,
    RetainedReplayWorkload, SpatialWorkloadStage, SurfaceSupportWorkload, TransformWorkload,
    WorkloadEvidenceLedger, WorkloadEvidenceLedgerError, WorkloadEvidenceRow,
    WorkloadEvidenceStage, WorkloadStageDenial, WorkloadStagePosture, WorkloadStageSupport,
};

use self::evidence_ledger_receipts::counter_backed_rows;

#[test]
fn workload_vocabulary_preserves_authority_boundaries() {
    let topology = TopologyWorkload::declared("topology seed")
        .from_query_declaration(".topology.seed")
        .expect("topology workload should certify");
    let geometry = GeometryBindingWorkload::for_topology_receipt(&topology)
        .declared("bind cube geometry")
        .admit()
        .expect("geometry binding should certify");
    let support = SurfaceSupportWorkload::for_geometry_binding(&geometry)
        .declared("support planar faces")
        .admit()
        .expect("surface support should certify");
    let projection = ProjectionWorkload::for_surface_support(&support)
        .declared("project planar loops")
        .admit()
        .expect("projection should certify");
    let transform = TransformWorkload::for_projection(&projection)
        .declared("apply rigid transform")
        .admit()
        .expect("transform should certify");
    let replay = RetainedReplayWorkload::for_transform(&transform)
        .declared("retain replay trail")
        .admit()
        .expect("retained replay should certify");
    let diagnostics = DiagnosticWorkload::for_retained_replay(&replay)
        .declared("explain workload outcome")
        .admit()
        .expect("diagnostics should certify");
    let response = ResponseWorkload::for_diagnostics(&diagnostics)
        .declared("user-facing response")
        .admit()
        .expect("response should certify");

    assert_eq!(
        geometry.identity().stage(),
        SpatialWorkloadStage::GeometryBinding
    );
    assert_eq!(
        support.identity().stage(),
        SpatialWorkloadStage::SurfaceSupport
    );
    assert_eq!(
        projection.identity().stage(),
        SpatialWorkloadStage::Projection
    );
    assert_eq!(
        transform.identity().stage(),
        SpatialWorkloadStage::Transform
    );
    assert_eq!(
        replay.identity().stage(),
        SpatialWorkloadStage::RetainedReplay
    );
    assert_eq!(
        diagnostics.identity().stage(),
        SpatialWorkloadStage::Diagnostics
    );
    assert_eq!(response.identity().stage(), SpatialWorkloadStage::Response);

    let ledger = WorkloadEvidenceLedger::from_rows(counter_backed_rows("vocabulary boundary"))
        .expect("ledger should certify");

    assert_eq!(topology.envelope().counters().declaration_rows(), 1);
    assert_eq!(ledger.counters().rows(), 8);
    assert!(ledger.covers_authority_stages());
    assert!(ledger
        .rows()
        .iter()
        .all(WorkloadEvidenceRow::is_receipt_backed));
    assert!(response.envelope().posture().reason().contains("admitted"));
}

#[test]
fn workload_vocabulary_rejects_unproven_stage_construction() {
    let topology = TopologyWorkload::declared("topology seed")
        .from_query_declaration(".topology.seed")
        .expect("topology workload should certify");
    let denial = GeometryBindingWorkload::for_topology_receipt(&topology)
        .declared("")
        .admit()
        .expect_err("geometry binding cannot certify unnamed work");

    assert_eq!(denial, WorkloadStageDenial::MissingDeclaration);
    assert_eq!(
        denial.human_reason(),
        "workload stage requires a declaration"
    );
}

#[test]
fn workload_vocabulary_names_every_support_posture_state() {
    let admitted = WorkloadStagePosture::admitted(
        SpatialWorkloadStage::Projection,
        "projection workload is admitted",
    );
    let unsupported = WorkloadStagePosture::unsupported(
        SpatialWorkloadStage::Projection,
        "projection workload family is not supported by this runtime",
    );
    let blocked = WorkloadStagePosture::blocked(
        SpatialWorkloadStage::Projection,
        "projection workload is blocked until topology evidence is complete",
    );

    assert_eq!(admitted.support(), WorkloadStageSupport::Admitted);
    assert_eq!(unsupported.support(), WorkloadStageSupport::Unsupported);
    assert_eq!(blocked.support(), WorkloadStageSupport::Blocked);
    assert!(unsupported.reason().contains("not supported"));
    assert!(blocked.reason().contains("blocked"));

    let missing_reason = WorkloadStagePosture::blocked(SpatialWorkloadStage::Projection, "");
    assert!(missing_reason.reason().contains("human-readable reason"));
}

#[test]
fn workload_evidence_ledger_rejects_duplicate_and_reports_missing_authority_stage() {
    let duplicate = WorkloadEvidenceLedger::from_rows(vec![
        WorkloadEvidenceRow::new(WorkloadEvidenceStage::Topology, "topology seed"),
        WorkloadEvidenceRow::new(WorkloadEvidenceStage::Topology, "topology seed again"),
    ])
    .expect_err("ledger must reject duplicate authority rows");

    assert_eq!(
        duplicate,
        WorkloadEvidenceLedgerError::DuplicateEvidenceStage(WorkloadEvidenceStage::Topology)
    );
    assert!(duplicate.human_reason().contains("duplicate topology"));

    let partial = WorkloadEvidenceLedger::from_rows(vec![WorkloadEvidenceRow::new(
        WorkloadEvidenceStage::Topology,
        "topology seed",
    )])
    .expect("single unique row is a valid partial ledger");

    assert_eq!(
        partial.missing_authority_stage(),
        Some(WorkloadEvidenceStage::GeometryBinding)
    );
    assert!(!partial.covers_authority_stages());
}
