use worth_spatial::facade::planar_projection_consumption::{
    ProjectionConsumedPlanarFacts, ProjectionConsumedPlanarFactsContracts,
    ProjectionConsumedPlanarFactsReceipt,
};
use worth_spatial::facade::planar_recovery::{
    PlanarRecoveryPosture, PlanarRecoveryPostureContracts, PlanarRecoveryPostureReceipt,
    PlanarRecoverySource,
};
use worth_spatial::facade::planar_retained_facts::RetainedPlanarFactsReceipt;

use crate::public_api_planar_projection_consumption::contract_subject::projection_consumed_planar_parts;
use crate::public_api_planar_projection_consumption::runtime_handles::projection_consumption_handle;

use super::diagnostic_fixture::diagnostic_receipt_for;
use super::neighborhood_fixture::local_neighborhood_receipt;
use super::runtime_handles::recovery_handle;

pub(crate) struct LocalRebuildParityParts {
    pub(crate) retained: RetainedPlanarFactsReceipt,
    pub(crate) projected: ProjectionConsumedPlanarFactsReceipt,
    pub(crate) recovery: PlanarRecoveryPostureReceipt,
    pub(crate) diagnostics:
        worth_spatial::facade::planar_diagnostics::PlanarDiagnosticBundleReceipt,
    pub(crate) neighborhood:
        worth_spatial::facade::neighborhood::TopologyNeighborhoodReplacementFactReceipt,
}

pub(crate) fn local_rebuild_parity_parts(world: &'static str) -> LocalRebuildParityParts {
    let projection_parts = projection_consumed_planar_parts(world);
    let projected = projection_consumed_planar_receipt(world, &projection_parts);
    let recovery =
        local_rebuild_recovery_receipt(world, projection_parts.retained.clone(), projected.clone());
    let diagnostics = diagnostic_receipt_for(
        world,
        worth_spatial::facade::planar_diagnostics::PlanarDiagnosticSubject::binding_failure(
            format!("local-rebuild-binding:{world}"),
        ),
    );
    LocalRebuildParityParts {
        retained: projection_parts.retained,
        projected,
        recovery,
        diagnostics,
        neighborhood: local_neighborhood_receipt(world),
    }
}

fn projection_consumed_planar_receipt(
    world: &'static str,
    projection_parts: &crate::public_api_planar_projection_consumption::contract_subject::ProjectionConsumedPlanarParts,
) -> ProjectionConsumedPlanarFactsReceipt {
    ProjectionConsumedPlanarFacts::from_retained_planar_facts(projection_parts.retained.clone())
        .consume_bundle_projection_receipts(projection_parts.projections.clone())
        .compile(&ProjectionConsumedPlanarFactsContracts::new(
            projection_consumption_handle(world),
        ))
        .expect("local rebuild projection-consumed plan")
        .consume()
        .expect("local rebuild projection-consumed receipt")
}

fn local_rebuild_recovery_receipt(
    world: &'static str,
    retained: RetainedPlanarFactsReceipt,
    projected: ProjectionConsumedPlanarFactsReceipt,
) -> PlanarRecoveryPostureReceipt {
    PlanarRecoveryPosture::from_blocked_planar_source(
        PlanarRecoverySource::from_retained_or_projection_basis_denial(format!(
            "local-rebuild-recovery:{world}"
        )),
    )
    .with_retained_planar_facts(retained)
    .with_projection_consumed_facts(projected)
    .prepare_next_step()
    .compile(&PlanarRecoveryPostureContracts::new(recovery_handle(world)))
    .expect("local rebuild recovery plan")
    .certify()
    .expect("local rebuild recovery receipt")
}
