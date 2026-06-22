mod boolean_evidence_ledger;
pub(crate) mod evidence_ledger_receipts;
mod feature_gate_leakage;
mod honesty_guards;
mod naming_honesty;
mod public_facade_parity;
mod spatial_touch_admission;
mod surface_deletion_ledger;

use topology::facade::TopologyWorkload;
use worth_spatial::certification::workload_evidence::{
    certification_only_admitted_stage_row, complete_ledger_with_additional_rows,
};
use worth_spatial::facade::workload_vocabulary::{
    DiagnosticWorkload, GeometryBindingWorkload, ProjectionWorkload, ResponseWorkload,
    RetainedReplayWorkload, SpatialWorkloadStage, SurfaceSupportWorkload, TransformWorkload,
    WorkloadEvidenceLedger, WorkloadEvidenceLedgerError, WorkloadEvidenceRow,
    WorkloadEvidenceStage, WorkloadEvidenceStageCounters, WorkloadStageDenial,
    WorkloadStagePosture, WorkloadStageSupport,
};

use self::evidence_ledger_receipts::counter_backed_rows;

pub(crate) fn run_stack_heavy_test(test: impl FnOnce() + Send + 'static) {
    let result = std::thread::Builder::new()
        .stack_size(32 * 1024 * 1024)
        .spawn(test)
        .expect("stack-heavy public workload test should spawn")
        .join();
    if let Err(panic_payload) = result {
        std::panic::resume_unwind(panic_payload);
    }
}

#[test]
fn workload_vocabulary_preserves_authority_boundaries() {
    run_stack_heavy_test(|| {
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

        let rows = counter_backed_rows("vocabulary boundary");
        let ledger = WorkloadEvidenceLedger::from_rows(rows).expect("ledger should certify");

        assert_eq!(topology.envelope().counters().declaration_rows(), 1);
        assert_eq!(ledger.counters().rows(), 8);
        assert!(ledger.covers_authority_stages());
        ledger
            .guards()
            .assert_counters_are_receipt_backed()
            .expect("ledger rows should be receipt-backed with counters");
        assert!(response.envelope().posture().reason().contains("admitted"));
    });
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

#[test]
fn workload_evidence_ledger_exposes_stage_index_product_as_lookup_authority() {
    run_stack_heavy_test(|| {
        let ledger = WorkloadEvidenceLedger::from_rows(counter_backed_rows("stage index contract"))
            .expect("counter-backed rows should build an indexed ledger");
        let stage_index = ledger.stage_index();

        assert!(!stage_index.index_identity().is_empty());
        assert_eq!(stage_index.counters().row_count(), ledger.counters().rows());
        assert_eq!(stage_index.counters().indexed_stage_count(), 8);
        assert_eq!(stage_index.counters().duplicate_stage_count(), 0);
        assert_eq!(stage_index.counters().manual_row_count(), 0);
        assert_eq!(stage_index.counters().unadmitted_row_count(), 0);
        let projection_link = stage_index
            .link_required_stages(&[WorkloadEvidenceStage::Projection])
            .expect("projection stage link should be typed")
            .link_for_stage(WorkloadEvidenceStage::Projection)
            .cloned()
            .expect("projection link should exist");
        let ledger_projection_link = ledger
            .link_required_stages(&[WorkloadEvidenceStage::Projection])
            .expect("ledger projection stage link should be typed")
            .link_for_stage(WorkloadEvidenceStage::Projection)
            .cloned()
            .expect("ledger projection link should exist");
        assert_eq!(projection_link, ledger_projection_link);
    });
}

#[test]
fn certification_only_rows_cannot_satisfy_production_stage_links() {
    run_stack_heavy_test(|| {
        let complete = WorkloadEvidenceLedger::from_rows(counter_backed_rows("cert-only boundary"))
            .expect("counter-backed rows should build a ledger")
            .certify_complete()
            .expect("classical authority rows should certify");
        let with_certification_row = complete_ledger_with_additional_rows(
            &complete,
            vec![certification_only_admitted_stage_row(
                WorkloadEvidenceStage::BooleanSharedPlaneIdentity,
                "synthetic shared-plane identity",
                WorkloadEvidenceStageCounters::boolean_shared_plane_identity(),
            )],
        )
        .expect("certification-only row should remain inspectable");

        assert_eq!(
            with_certification_row
                .link_required_stages(&[WorkloadEvidenceStage::BooleanSharedPlaneIdentity])
                .expect_err("certification-only rows must not become production stage links"),
            WorkloadEvidenceLedgerError::ManualAuthorityStage(
                WorkloadEvidenceStage::BooleanSharedPlaneIdentity
            )
        );
    });
}
