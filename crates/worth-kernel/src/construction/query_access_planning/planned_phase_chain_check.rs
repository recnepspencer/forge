use forge_query::facade::ForgeQueryWorkspace;

use super::access_denial::PrimitiveConstructionQueryAccessError;
use super::access_receipt::PrimitiveConstructionPlannedQueryAccess;
use super::anchored_topology_read::plan_anchored_construction_topology_read;
use super::covered_surface::PrimitiveConstructionQueryAccessSurface;
use crate::construction::admitted_scaffold::PreparedPrimitiveConstructionAdmittedArtifact;

pub(crate) fn plan_phase_chain_topology_check(
    workspace: &mut ForgeQueryWorkspace,
    artifact: &PreparedPrimitiveConstructionAdmittedArtifact,
) -> Result<PrimitiveConstructionPlannedQueryAccess, PrimitiveConstructionQueryAccessError> {
    plan_anchored_construction_topology_read(
        workspace,
        artifact,
        PrimitiveConstructionQueryAccessSurface::PhaseChainTopologyCheck,
        2,
    )
}
