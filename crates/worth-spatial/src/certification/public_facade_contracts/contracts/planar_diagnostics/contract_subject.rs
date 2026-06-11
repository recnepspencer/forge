use worth_spatial::facade::planar_diagnostics::PlanarDiagnosticTopologyEvidence;
use worth_spatial::facade::planar_projection_consumption::{
    ProjectionConsumedPlanarFacts, ProjectionConsumedPlanarFactsContracts,
    ProjectionConsumedPlanarFactsReceipt,
};
use worth_spatial::facade::planar_retained_facts::RetainedPlanarFactsReceipt;

use crate::public_api_planar_projection_consumption::contract_subject::projection_consumed_planar_parts;
use crate::public_api_planar_projection_consumption::runtime_handles::projection_consumption_handle;

pub(crate) use super::causal_reference::causal_reference;

pub(crate) struct DiagnosticPlanarParts {
    pub(crate) retained: RetainedPlanarFactsReceipt,
    pub(crate) projected: ProjectionConsumedPlanarFactsReceipt,
}

pub(crate) fn diagnostic_planar_parts(world: &'static str) -> DiagnosticPlanarParts {
    let parts = projection_consumed_planar_parts(world);
    let projected =
        ProjectionConsumedPlanarFacts::from_retained_planar_facts(parts.retained.clone())
            .consume_bundle_projection_receipts(parts.projections)
            .compile(&ProjectionConsumedPlanarFactsContracts::new(
                projection_consumption_handle(world),
            ))
            .expect("diagnostic projection-consumed plan")
            .consume()
            .expect("diagnostic projection-consumed receipt");
    DiagnosticPlanarParts {
        retained: parts.retained,
        projected,
    }
}

pub(crate) fn topology_surface(world: &'static str) -> PlanarDiagnosticTopologyEvidence {
    PlanarDiagnosticTopologyEvidence::declared_surface(format!("topology-declared-surface:{world}"))
}
