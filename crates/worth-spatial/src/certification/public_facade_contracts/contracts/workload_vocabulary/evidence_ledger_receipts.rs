use topology::facade::{TopologySeed, TopologySeedReceipt};
use worth_spatial::facade::planar_diagnostics::PlanarDiagnosticBundleReceipt;
use worth_spatial::facade::projection_workload::{
    LocalFrameBasis, ProjectionReceiptSet, ProjectionWorkload as RichProjectionWorkload,
};
use worth_spatial::facade::retained_replay_workload::{
    admit_retained_replay_capture, ReplayReceiptSet, ReplayWorkload, ReplayedWorkload,
};
use worth_spatial::facade::surface_support::{
    CertifiedSurfaceSupport, SurfaceFamily, SurfaceSupportReceiptSet,
    SurfaceSupportWorkload as RichSurfaceSupportWorkload,
};
use worth_spatial::facade::transform_workload::{
    TransformReceiptSet, TransformSequence, TransformWorkload as RichTransformWorkload,
    TransformedWorkload,
};
use worth_spatial::facade::user_response::WorthUserResponseReceipt;
use worth_spatial::facade::workload_binding::{
    BoundGeometryWorkload, GeometryBindingReceiptSet,
    GeometryBindingWorkload as RichGeometryBindingWorkload, PlanarEdgeCarrierSet,
    PlanarFaceCarrierSet, PlanarLoopCarrierSet,
};
use worth_spatial::facade::workload_vocabulary::{
    DiagnosticWorkload, GeometryBindingWorkload, ProjectionWorkload, ResponseWorkload,
    RetainedReplayWorkload, SurfaceSupportWorkload, TransformWorkload, WorkloadEvidenceGuardError,
    WorkloadEvidenceLedger, WorkloadEvidenceLedgerError, WorkloadEvidenceRow,
    WorkloadEvidenceStage,
};

use crate::public_api_retained_replay_workload::contract_subject::{
    captured_retained_workload, retained_replay_parts,
};
use crate::public_api_transform_workload::contract_subject::acceptance_transform_sequence;
use crate::public_api_user_response::contract_subject::admitted_response;

#[test]
fn evidence_ledger_requires_source_receipts_for_every_completed_stage() {
    super::run_stack_heavy_test(|| {
        let receipts = admitted_receipts();
        let complete = WorkloadEvidenceLedger::from_rows(counter_backed_rows("complete ledger"))
            .expect("receipt-backed rows should form an inspectable ledger")
            .certify_complete()
            .expect("all authority stages should certify completion");

        assert_eq!(complete.counters().rows(), 8);
        complete
            .guards()
            .assert_counters_are_receipt_backed()
            .expect("complete ledger rows should be receipt-backed with counters");

        let simple_stage_receipts =
            WorkloadEvidenceLedger::from_rows(receipt_backed_rows(&receipts))
                .expect("simple stage receipts should remain inspectable")
                .guards()
                .assert_counters_are_receipt_backed()
                .expect_err("honesty guards must reject rows without receipt-backed counters");
        assert_eq!(
            simple_stage_receipts,
            WorkloadEvidenceGuardError::MissingReceiptBackedCounters(
                WorkloadEvidenceStage::GeometryBinding
            )
        );

        let mut mixed_rows = counter_backed_rows("mixed simple projection");
        mixed_rows[3] = WorkloadEvidenceRow::from_projection_receipt(&receipts.projection);
        let mixed_projection = WorkloadEvidenceLedger::from_rows(mixed_rows)
            .expect("mixed rows should remain inspectable")
            .guards()
            .assert_counters_are_receipt_backed()
            .expect_err("simple projection receipt must not count as structural proof");
        assert_eq!(
            mixed_projection,
            WorkloadEvidenceGuardError::MissingReceiptBackedCounters(
                WorkloadEvidenceStage::Projection
            )
        );

        let manual_projection = WorkloadEvidenceLedger::from_rows(vec![
            WorkloadEvidenceRow::from_topology_workload_and_seed_receipts(
                &receipts.topology,
                &receipts.topology_seed,
            ),
            WorkloadEvidenceRow::from_geometry_binding_receipt(&receipts.geometry),
            WorkloadEvidenceRow::from_surface_support_receipt(&receipts.support),
            WorkloadEvidenceRow::new(
                WorkloadEvidenceStage::Projection,
                "projection fixture label",
            ),
            WorkloadEvidenceRow::from_transform_receipt(&receipts.transform),
            WorkloadEvidenceRow::from_retained_replay_receipt(&receipts.replay),
            WorkloadEvidenceRow::from_diagnostic_receipt(&receipts.diagnostics),
            WorkloadEvidenceRow::from_response_receipt(&receipts.response),
        ])
        .expect("manual rows remain inspectable before complete certification")
        .certify_complete()
        .expect_err("complete ledger must reject hand-filled projection evidence");

        assert_eq!(
            manual_projection,
            WorkloadEvidenceLedgerError::ManualAuthorityStage(WorkloadEvidenceStage::Projection)
        );
        assert!(manual_projection
            .human_reason()
            .contains("hand-filled projection evidence"));
    });
}

pub(crate) struct VocabularyReceipts {
    pub(crate) topology_seed: TopologySeedReceipt,
    pub(crate) topology: topology::facade::TopologyWorkloadReceipt,
    pub(crate) geometry: worth_spatial::facade::workload_vocabulary::GeometryBindingWorkloadReceipt,
    pub(crate) support: worth_spatial::facade::workload_vocabulary::SurfaceSupportWorkloadReceipt,
    pub(crate) projection: worth_spatial::facade::workload_vocabulary::ProjectionWorkloadReceipt,
    pub(crate) transform: worth_spatial::facade::workload_vocabulary::TransformWorkloadReceipt,
    pub(crate) replay: worth_spatial::facade::workload_vocabulary::RetainedReplayWorkloadReceipt,
    pub(crate) diagnostics: worth_spatial::facade::workload_vocabulary::DiagnosticWorkloadReceipt,
    pub(crate) response: worth_spatial::facade::workload_vocabulary::ResponseWorkloadReceipt,
}

pub(crate) fn admitted_receipts() -> VocabularyReceipts {
    let topology_seed = TopologySeed::cube()
        .with_declaration("topology seed")
        .build()
        .expect("topology seed should certify");
    let topology = topology_seed.query_receipts().declaration_receipt().clone();
    let geometry = GeometryBindingWorkload::for_topology_receipt(&topology)
        .admit()
        .expect("geometry binding should certify");
    let support = SurfaceSupportWorkload::for_geometry_binding(&geometry)
        .admit()
        .expect("surface support should certify");
    let projection = ProjectionWorkload::for_surface_support(&support)
        .admit()
        .expect("projection should certify");
    let transform = TransformWorkload::for_projection(&projection)
        .admit()
        .expect("transform should certify");
    let replay = RetainedReplayWorkload::for_transform(&transform)
        .admit()
        .expect("retained replay should certify");
    let diagnostics = DiagnosticWorkload::for_retained_replay(&replay)
        .admit()
        .expect("diagnostics should certify");
    let response = ResponseWorkload::for_diagnostics(&diagnostics)
        .admit()
        .expect("response should certify");

    VocabularyReceipts {
        topology_seed,
        topology,
        geometry,
        support,
        projection,
        transform,
        replay,
        diagnostics,
        response,
    }
}

pub(crate) fn receipt_backed_rows(receipts: &VocabularyReceipts) -> Vec<WorkloadEvidenceRow> {
    vec![
        WorkloadEvidenceRow::from_topology_workload_and_seed_receipts(
            &receipts.topology,
            &receipts.topology_seed,
        ),
        WorkloadEvidenceRow::from_geometry_binding_receipt(&receipts.geometry),
        WorkloadEvidenceRow::from_surface_support_receipt(&receipts.support),
        WorkloadEvidenceRow::from_projection_receipt(&receipts.projection),
        WorkloadEvidenceRow::from_transform_receipt(&receipts.transform),
        WorkloadEvidenceRow::from_retained_replay_receipt(&receipts.replay),
        WorkloadEvidenceRow::from_diagnostic_receipt(&receipts.diagnostics),
        WorkloadEvidenceRow::from_response_receipt(&receipts.response),
    ]
}

pub(crate) fn counter_backed_rows(world: &'static str) -> Vec<WorkloadEvidenceRow> {
    let receipts = counter_backed_receipts(world);
    counter_backed_rows_from_receipts(&receipts)
}

pub(crate) fn counter_backed_rows_with_transform(
    world: &'static str,
    transform_sequence: TransformSequence,
) -> Vec<WorkloadEvidenceRow> {
    let receipts = counter_backed_receipts_with_transform(world, transform_sequence);
    counter_backed_rows_from_receipts(&receipts)
}

fn counter_backed_rows_from_receipts(receipts: &CounterBackedReceipts) -> Vec<WorkloadEvidenceRow> {
    vec![
        WorkloadEvidenceRow::from_topology_seed_receipt(&receipts.topology),
        WorkloadEvidenceRow::from_geometry_binding_receipt_set(&receipts.geometry),
        WorkloadEvidenceRow::from_surface_support_receipt_set(&receipts.support),
        WorkloadEvidenceRow::from_projection_receipt_set(&receipts.projection),
        WorkloadEvidenceRow::from_transform_receipt_set(&receipts.transform),
        WorkloadEvidenceRow::from_replay_receipt_set(&receipts.replay),
        WorkloadEvidenceRow::from_planar_diagnostic_receipt(&receipts.diagnostics),
        WorkloadEvidenceRow::from_user_response_receipt(&receipts.response),
    ]
}

pub(crate) struct CounterBackedReceipts {
    pub(crate) topology: TopologySeedReceipt,
    pub(crate) geometry: GeometryBindingReceiptSet,
    pub(crate) support: SurfaceSupportReceiptSet,
    pub(crate) projection: ProjectionReceiptSet,
    pub(crate) transform: TransformReceiptSet,
    pub(crate) replay: ReplayReceiptSet,
    pub(crate) diagnostics: PlanarDiagnosticBundleReceipt,
    pub(crate) response: WorthUserResponseReceipt,
}

pub(crate) fn counter_backed_receipts(world: &'static str) -> CounterBackedReceipts {
    counter_backed_receipts_with_transform(world, acceptance_transform_sequence())
}

pub(crate) fn counter_backed_receipts_with_transform(
    world: &'static str,
    transform_sequence: TransformSequence,
) -> CounterBackedReceipts {
    let topology = counter_backed_topology(world);
    let bound_geometry = counter_backed_geometry(world, &topology);
    let geometry = bound_geometry.receipts().clone();
    let surface_support = counter_backed_surface_support(world, bound_geometry);
    let support = surface_support.receipts().clone();
    let projected = counter_backed_projection(world, surface_support);
    let projection = projected.receipts().clone();
    let transformed = counter_backed_transform(world, projected, transform_sequence);
    let transform = transformed.receipts().clone();
    let replayed = counter_backed_replay(world, transformed);
    let replay = replayed.receipts().clone();
    let diagnostics = counter_backed_diagnostics(&replay);
    let response = admitted_response("complete-ledger-user-response");

    CounterBackedReceipts {
        topology,
        geometry,
        support,
        projection,
        transform,
        replay,
        diagnostics,
        response,
    }
}

fn counter_backed_topology(world: &'static str) -> TopologySeedReceipt {
    TopologySeed::cube()
        .with_declaration(world)
        .build()
        .expect("cube topology seed should certify")
}

fn counter_backed_geometry(
    world: &'static str,
    topology: &TopologySeedReceipt,
) -> BoundGeometryWorkload {
    RichGeometryBindingWorkload::for_topology_seed(topology)
        .declared(format!("bind {world}"))
        .with_planar_faces(PlanarFaceCarrierSet::for_seed_faces(&topology))
        .with_planar_edges(PlanarEdgeCarrierSet::for_seed_edges(&topology))
        .with_planar_loops(PlanarLoopCarrierSet::for_seed_loops(&topology))
        .admit()
        .expect("complete planar geometry binding should admit")
}

fn counter_backed_surface_support(
    world: &'static str,
    bound_geometry: BoundGeometryWorkload,
) -> CertifiedSurfaceSupport {
    RichSurfaceSupportWorkload::for_bound_geometry(bound_geometry)
        .declared(format!("certify plane support for {world}"))
        .with_surface_family(SurfaceFamily::Plane)
        .certify()
        .expect("plane support should certify")
}

fn counter_backed_projection(
    world: &'static str,
    surface_support: CertifiedSurfaceSupport,
) -> worth_spatial::facade::projection_workload::ProjectedPlanarWorkload {
    RichProjectionWorkload::for_certified_surface_support(surface_support)
        .declared(format!("project {world}"))
        .with_local_frame(LocalFrameBasis::from_certified_plane())
        .project()
        .expect("certified support should project")
}

fn counter_backed_transform(
    world: &'static str,
    projected: worth_spatial::facade::projection_workload::ProjectedPlanarWorkload,
    transform_sequence: TransformSequence,
) -> TransformedWorkload {
    RichTransformWorkload::for_projected_workload(projected)
        .declared(format!("transform {world}"))
        .with_transform_sequence(transform_sequence)
        .transform()
        .expect("transform evidence should admit")
}

fn counter_backed_replay(
    world: &'static str,
    transformed: TransformedWorkload,
) -> ReplayedWorkload {
    let retained_parts = retained_replay_parts("complete-ledger-retained-source");
    let captured = captured_retained_workload("complete-ledger-retained-source", &retained_parts);
    ReplayWorkload::for_transformed_workload(transformed)
        .declared(format!("replay retained artifacts for {world}"))
        .with_admitted_retained_replay_capture(admit_retained_replay_capture(captured))
        .replay()
        .expect("retained replay should admit")
}

fn counter_backed_diagnostics(replay: &ReplayReceiptSet) -> PlanarDiagnosticBundleReceipt {
    crate::public_api_planar_overlap::metaboss::diagnostics::certify_tiny_rotation_diagnostic(
        replay.stage_identity().receipt_identity().as_str(),
    )
}
