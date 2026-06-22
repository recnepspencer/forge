use worth_spatial::facade::planar_projection_consumption::{
    ProjectionConsumedPlanarFacts, ProjectionConsumedPlanarFactsContracts,
    ProjectionConsumedPlanarFactsReceipt,
};
use worth_spatial::facade::planar_recovery::PlanarRecoverySource;
use worth_spatial::facade::planar_retained_facts::RetainedPlanarFactsReceipt;

use crate::public_api_planar_projection_consumption::contract_subject::projection_consumed_planar_parts;
use crate::public_api_planar_projection_consumption::runtime_handles::projection_consumption_handle;

pub(crate) struct PlanarRecoveryParts {
    pub(crate) retained: RetainedPlanarFactsReceipt,
    pub(crate) projected: ProjectionConsumedPlanarFactsReceipt,
}

pub(crate) fn planar_recovery_parts(world: &'static str) -> PlanarRecoveryParts {
    let parts = projection_consumed_planar_parts(world);
    let projected =
        ProjectionConsumedPlanarFacts::from_retained_planar_facts(parts.retained.clone())
            .consume_bundle_projection_receipts(parts.projections.clone())
            .compile(&ProjectionConsumedPlanarFactsContracts::new(
                projection_consumption_handle(world),
            ))
            .expect("projection-consumed plan for recovery")
            .consume()
            .expect("projection-consumed receipt for recovery");
    PlanarRecoveryParts {
        retained: parts.retained,
        projected,
    }
}

pub(crate) fn projection_basis_source() -> PlanarRecoverySource {
    PlanarRecoverySource::from_projection_denial("denial:projection-basis")
}

pub(crate) fn retained_projection_basis_source() -> PlanarRecoverySource {
    PlanarRecoverySource::from_retained_or_projection_basis_denial(
        "denial:retained-projection-basis",
    )
}
