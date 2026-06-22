use forge_query::facade::{ForgeQueryReadFamily, ForgeQueryWorkspace};

use super::access_receipt_requirements::{
    require_graph_access_receipt, require_no_caller_owned_graph_access,
};
use super::basis_context::TopologyReadExecutionTarget;
use super::family_execution::ExecutedTopologyReadFamily;
use crate::projection::read_views::domain::error::TopologyReadError;
use crate::projection::read_views::domain::read_proof::report::TopologyReadRequestReport;
use crate::projection::runtime_boundary::read_lowering::TopologyReadLoweringArtifact;

pub(crate) fn execute_access_planned_topology_read_family(
    workspace: &mut ForgeQueryWorkspace,
    execution_target: &TopologyReadExecutionTarget,
    family: &ForgeQueryReadFamily,
    lowering_artifact: TopologyReadLoweringArtifact,
    read_surface: &str,
) -> Result<ExecutedTopologyReadFamily, TopologyReadError> {
    let lowering_artifact =
        lowering_artifact.with_query_read_graph_relationship_proof(family.read_graph());
    let result =
        execution_target.execute_family_with_explicit_graph_access_plan(workspace, family)?;
    let (_, counters) = require_graph_access_receipt(result.receipt(), read_surface)?;
    require_no_caller_owned_graph_access(result.receipt(), counters, read_surface)?;
    Ok(ExecutedTopologyReadFamily {
        report: TopologyReadRequestReport::query_execution_without_fallback_debt(
            lowering_artifact,
            result.receipt(),
        ),
        result,
    })
}
