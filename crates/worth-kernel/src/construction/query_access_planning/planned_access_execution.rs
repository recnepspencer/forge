use forge_query::facade::ForgeQueryWorkspace;

use super::access_denial::PrimitiveConstructionQueryAccessError;
use super::access_receipt::{
    PrimitiveConstructionConsumedQueryAccess, PrimitiveConstructionPlannedQueryAccess,
};

pub(crate) fn execute_planned_construction_query_access(
    workspace: &mut ForgeQueryWorkspace,
    planned: PrimitiveConstructionPlannedQueryAccess,
) -> Result<PrimitiveConstructionConsumedQueryAccess, PrimitiveConstructionQueryAccessError> {
    let result =
        workspace.execute_read_family_with_access_plan(planned.family(), planned.plan().clone())?;
    PrimitiveConstructionConsumedQueryAccess::from_planned_result(&planned, result)
}
