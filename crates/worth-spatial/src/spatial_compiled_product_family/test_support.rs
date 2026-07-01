use crate::facade::planar_projection_consumption::{
    ProjectionConsumedPlanarFacts, ProjectionConsumedPlanarFactsContracts,
    ProjectionConsumedPlanarFactsReceipt,
};
use crate::facade::planar_retained_facts::RetainedPlanarFactsReceipt;
use crate::public_api_planar_projection_consumption::contract_subject::projection_consumed_planar_parts;
use crate::public_api_planar_projection_consumption::runtime_handles::projection_consumption_handle;
use crate::workload_platform::evidence_ledger::{
    current_complete_ledger_from_rows, WorkloadEvidenceRow,
};
use crate::workload_platform::geometry_binding::{
    BoundGeometryWorkload, GeometryBindingWorkload, PlanarEdgeCarrierSet, PlanarFaceCarrierSet,
    PlanarLoopCarrierSet,
};
use crate::workload_platform::projection_workload::{
    LocalFrameBasis, ProjectedPlanarWorkload, ProjectionWorkload,
};
use crate::workload_platform::retained_cancellation_chain::{
    RetainedCancellationChainReceipt, RetainedCancellationChainWorkload,
    RetainedCancellationCheckpoint, RetainedReplaySampling,
};
use crate::workload_platform::retained_replay_workload::{ReplayWorkload, ReplayedWorkload};
use crate::workload_platform::spatial_compiled_product_consumer_cutover::admit_retained_replay_capture;
use crate::workload_platform::surface_support::{
    CertifiedSurfaceSupport, SurfaceFamily, SurfaceSupportWorkload,
};
use crate::workload_platform::transform_workload::{
    RotationTurn, TransformReorientation, TransformSequence, TransformWorkload,
    TransformedWorkload, VectorDelta,
};
use crate::workload_platform::vocabulary::{DiagnosticWorkload, ResponseWorkload};
use topology::facade::TopologySeed;

pub(crate) fn retained_and_projected_receipts(
    world: &'static str,
) -> (
    RetainedPlanarFactsReceipt,
    ProjectionConsumedPlanarFactsReceipt,
) {
    let parts = projection_consumed_planar_parts(world);
    let retained = parts.retained;
    let projected = ProjectionConsumedPlanarFacts::from_retained_planar_facts(retained.clone())
        .consume_bundle_projection_receipts(parts.projections)
        .compile(&ProjectionConsumedPlanarFactsContracts::new(
            projection_consumption_handle(world),
        ))
        .expect("projection-consumed plan")
        .consume()
        .expect("projection-consumed receipt");
    (retained, projected)
}

pub(crate) fn real_retained_cancellation_receipt(
    world: &'static str,
) -> RetainedCancellationChainReceipt {
    let projected = projected_cube_workload(world);
    let transformed = TransformWorkload::for_projected_workload(projected.projected.clone())
        .declared(format!("phase-four transform {world}"))
        .with_transform_sequence(acceptance_transform_sequence())
        .transform()
        .expect("transformed workload");
    let replayed = ReplayWorkload::for_transformed_workload(transformed.clone())
        .declared(format!("phase-four replay {world}"))
        .with_admitted_retained_replay_capture(admit_retained_replay_capture(
            crate::workload_platform::retained_replay_workload::canonical_retained_cancellation_chain_capture(world)
                .expect("retained cancellation capture"),
        ))
        .replay()
        .expect("replayed workload");
    let diagnostics = DiagnosticWorkload::for_retained_replay(replayed.receipts().stage_receipt())
        .declared(format!("phase-four diagnostics {world}"))
        .admit()
        .expect("diagnostics receipt");
    let response = ResponseWorkload::for_diagnostics(&diagnostics)
        .declared(format!("phase-four response {world}"))
        .admit()
        .expect("response receipt");
    let evidence_ledger = current_complete_ledger_from_rows(vec![
        WorkloadEvidenceRow::from_topology_seed_receipt(&projected.topology),
        WorkloadEvidenceRow::from_geometry_binding_receipt_set(projected.bound_geometry.receipts()),
        WorkloadEvidenceRow::from_surface_support_receipt_set(projected.surface_support.receipts()),
        WorkloadEvidenceRow::from_projection_receipt_set(projected.projected.receipts()),
        WorkloadEvidenceRow::from_transform_receipt_set(transformed.receipts()),
        WorkloadEvidenceRow::from_replay_receipt_set(replayed.receipts()),
        WorkloadEvidenceRow::from_diagnostic_receipt(&diagnostics),
        WorkloadEvidenceRow::from_response_receipt(&response),
    ]);

    RetainedCancellationChainWorkload::from_platform_evidence(&evidence_ledger)
        .declared(format!("phase-four retained cancellation {world}"))
        .with_required_checkpoints(32)
        .with_replay_sampling(RetainedReplaySampling::every_fourth_checkpoint_plus_trigger_steps())
        .with_checkpoints(retained_cancellation_checkpoints(
            transformed.clone(),
            replayed,
        ))
        .certify()
        .expect("retained cancellation receipt")
}

fn retained_cancellation_checkpoints(
    transformed: TransformedWorkload,
    replayed: ReplayedWorkload,
) -> Vec<RetainedCancellationCheckpoint> {
    (0..32)
        .map(|index| {
            let checkpoint = RetainedCancellationCheckpoint::from_receipts(
                index,
                transformed.receipts(),
                replayed.receipts(),
            );
            if index % 4 == 0 {
                checkpoint.sampled_for_replay()
            } else {
                checkpoint
            }
        })
        .collect()
}

fn projected_cube_workload(world: &'static str) -> ProjectedCubeWorkload {
    let topology = TopologySeed::cube()
        .with_declaration(world)
        .build()
        .expect("cube topology seed");
    let bound_geometry = GeometryBindingWorkload::for_topology_seed(&topology)
        .declared(format!("bind {world}"))
        .with_planar_faces(PlanarFaceCarrierSet::for_seed_faces(&topology))
        .with_planar_edges(PlanarEdgeCarrierSet::for_seed_edges(&topology))
        .with_planar_loops(PlanarLoopCarrierSet::for_seed_loops(&topology))
        .admit()
        .expect("bound geometry");
    let surface_support = SurfaceSupportWorkload::for_bound_geometry(bound_geometry.clone())
        .declared(format!("support {world}"))
        .with_surface_family(SurfaceFamily::Plane)
        .certify()
        .expect("surface support");
    let projected = ProjectionWorkload::for_certified_surface_support(surface_support.clone())
        .declared(format!("project {world}"))
        .with_local_frame(LocalFrameBasis::from_certified_plane())
        .project()
        .expect("projected workload");
    ProjectedCubeWorkload {
        topology,
        bound_geometry,
        surface_support,
        projected,
    }
}

fn acceptance_transform_sequence() -> TransformSequence {
    TransformSequence::new()
        .translate(VectorDelta::xy(10, 0))
        .rotate(RotationTurn::quarter_turn_clockwise())
        .reorient(TransformReorientation::preserves_handedness())
        .cancel_with_exact_replay(16)
}

struct ProjectedCubeWorkload {
    topology: topology::facade::TopologySeedReceipt,
    bound_geometry: BoundGeometryWorkload,
    surface_support: CertifiedSurfaceSupport,
    projected: ProjectedPlanarWorkload,
}
