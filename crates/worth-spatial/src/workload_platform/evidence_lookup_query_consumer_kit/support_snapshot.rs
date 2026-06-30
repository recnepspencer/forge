use forge_query::facade::consumer_kit::{project_support_snapshot, ForgeQuerySupportSnapshot};
use forge_query::facade::ForgeQueryApplicationFacade;

use crate::workload_platform::evidence_lookup_query_surface_matrix::EvidenceLookupQuerySurfaceMatrixCloseout;

pub(crate) fn project_evidence_lookup_query_support_snapshot(
    _matrix: &EvidenceLookupQuerySurfaceMatrixCloseout,
) -> Result<ForgeQuerySupportSnapshot, forge_query::facade::consumer_kit::ForgeQueryTestBackendError>
{
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    Ok(project_support_snapshot(
        facade
            .domain_entry_support_snapshot()
            .runtime_support_matrix(),
    ))
}
