use worth_spatial::facade::surface_support::{
    SurfaceFamily, SurfaceSupportWorkload as RichSurfaceSupportWorkload,
};
use worth_spatial::facade::workload_binding::{
    GeometryBindingWorkload as RichGeometryBindingWorkload, PlanarEdgeCarrierSet,
    PlanarFaceCarrierSet, PlanarLoopCarrierSet,
};
use worth_spatial::facade::workload_vocabulary::{
    WorkloadEvidenceGuardError, WorkloadEvidenceLedger, WorkloadEvidenceLedgerError,
    WorkloadEvidenceRow, WorkloadEvidenceStage,
};

use super::evidence_ledger_receipts::{admitted_receipts, receipt_backed_rows};

#[test]
fn honesty_guards_reject_synthetic_replay_motion_and_fixture_arithmetic() {
    let receipts = admitted_receipts();

    let label_only_motion = WorkloadEvidenceLedger::from_rows(receipt_backed_rows(&receipts))
        .expect("simple receipt-backed ledger should remain inspectable")
        .guards()
        .assert_transform_changed_geometry()
        .expect_err("stage receipt alone is not changed-geometry proof");
    assert_eq!(
        label_only_motion,
        WorkloadEvidenceGuardError::LabelOnlyMotion
    );

    let synthetic_replay = WorkloadEvidenceLedger::from_rows(receipt_backed_rows(&receipts))
        .expect("simple receipt-backed ledger should remain inspectable")
        .guards()
        .assert_replay_consumed_retained_artifact()
        .expect_err("stage receipt alone is not retained replay proof");
    assert_eq!(
        synthetic_replay,
        WorkloadEvidenceGuardError::SyntheticReplay
    );

    let mut fixture_rows = receipt_backed_rows(&receipts);
    fixture_rows[3] =
        WorkloadEvidenceRow::new(WorkloadEvidenceStage::Projection, "4 projected rows");
    let fixture_arithmetic = WorkloadEvidenceLedger::from_rows(fixture_rows)
        .expect("manual fixture arithmetic should remain inspectable")
        .guards()
        .assert_no_fixture_arithmetic_as_truth()
        .expect_err("manual fixture arithmetic must not become truth evidence");
    assert_eq!(
        fixture_arithmetic,
        WorkloadEvidenceGuardError::FixtureArithmeticAsTruth(WorkloadEvidenceStage::Projection)
    );

    let unsupported = unsupported_surface_support_row("ledger-unsupported-freeform");
    let mut unsupported_rows = receipt_backed_rows(&receipts);
    unsupported_rows[2] = unsupported;
    let unsupported_complete = WorkloadEvidenceLedger::from_rows(unsupported_rows)
        .expect("unsupported receipt-backed rows should remain inspectable")
        .certify_complete()
        .expect_err("unsupported support cannot complete an admitted E2E ledger");
    assert_eq!(
        unsupported_complete,
        WorkloadEvidenceLedgerError::UnadmittedAuthorityStage(
            WorkloadEvidenceStage::SurfaceSupport
        )
    );
}

fn unsupported_surface_support_row(world: &str) -> WorkloadEvidenceRow {
    let topology = topology::facade::TopologySeed::cube()
        .with_declaration(world)
        .build()
        .expect("cube topology seed should admit");
    let bound_geometry = RichGeometryBindingWorkload::for_topology_seed(&topology)
        .declared(format!("bind {world}"))
        .with_planar_faces(PlanarFaceCarrierSet::for_seed_faces(&topology))
        .with_planar_edges(PlanarEdgeCarrierSet::for_seed_edges(&topology))
        .with_planar_loops(PlanarLoopCarrierSet::for_seed_loops(&topology))
        .admit()
        .expect("complete planar geometry binding should admit");

    let unsupported = RichSurfaceSupportWorkload::for_bound_geometry(bound_geometry)
        .declared(format!("reject unsupported freeform support for {world}"))
        .with_surface_family(SurfaceFamily::Freeform)
        .certify()
        .expect_err("freeform support must remain unsupported");
    let receipt = unsupported
        .receipt()
        .expect("unsupported support should expose posture receipt");
    WorkloadEvidenceRow::from_unsupported_surface_support_receipt(receipt)
}
