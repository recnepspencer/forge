use worth_spatial::facade::planar_projection_consumption::{
    ProjectionConsumedPlanarFacts, ProjectionConsumedPlanarFactsContracts,
    ProjectionConsumedPlanarFactsReceipt,
};
use worth_spatial::facade::retained_replay_workload::{CapturedRetainedWorkload, RetainedWorkload};
use worth_spatial::facade::transform_workload::{TransformWorkload, TransformedWorkload};

use crate::public_api_planar_projection_consumption::contract_subject::{
    projection_consumed_planar_parts, ProjectionConsumedPlanarParts,
};
use crate::public_api_planar_projection_consumption::runtime_handles::projection_consumption_handle;
use crate::public_api_transform_workload::contract_subject::{
    acceptance_transform_sequence, projected_cube_workload,
};

pub(crate) struct RetainedReplayParts {
    pub(crate) transformed: TransformedWorkload,
    pub(crate) retained_parts: ProjectionConsumedPlanarParts,
    pub(crate) projection_consumed: ProjectionConsumedPlanarFactsReceipt,
}

pub(crate) fn retained_replay_parts(world: &'static str) -> RetainedReplayParts {
    let retained_parts = projection_consumed_planar_parts(world);
    let projection_consumed = projection_consumed_receipt(world, &retained_parts);
    let transformed = TransformWorkload::for_projected_workload(projected_cube_workload(world))
        .declared(format!("transform {world} before retained replay"))
        .with_transform_sequence(acceptance_transform_sequence())
        .transform()
        .expect("retained replay transform workload");
    RetainedReplayParts {
        transformed,
        retained_parts,
        projection_consumed,
    }
}

pub(crate) fn projection_consumed_receipt(
    world: &'static str,
    parts: &ProjectionConsumedPlanarParts,
) -> ProjectionConsumedPlanarFactsReceipt {
    ProjectionConsumedPlanarFacts::from_retained_planar_facts(parts.retained.clone())
        .consume_bundle_projection_receipts(parts.projections.clone())
        .materialize_as(format!("materialization:retained-replay:{world}"))
        .compile(&ProjectionConsumedPlanarFactsContracts::new(
            projection_consumption_handle(world),
        ))
        .expect("retained replay projection-consumption plan")
        .consume()
        .expect("retained replay projection-consumed receipt")
}

pub(crate) fn captured_retained_workload(
    world: &'static str,
    parts: &RetainedReplayParts,
) -> CapturedRetainedWorkload {
    RetainedWorkload::from_retained_planar_facts(parts.retained_parts.retained.clone())
        .declared(format!("capture retained artifacts for {world}"))
        .with_projection_consumed_facts(parts.projection_consumed.clone())
        .capture()
        .expect("retained workload capture")
}
