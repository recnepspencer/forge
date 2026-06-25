mod local_ceremony_audit;
mod primitive_construction_lane;
mod spatial_descriptor_lane;
mod topology_touched_basis_lane;

use forge_query::facade::consumer_kit::ForgeQueryGraphObligationExecutionBackedAdoptionProof;

use super::selection_error::QueryObligationSelectionError;
use super::selection_request::{
    QueryObligationSelectionAuthorityKind, QueryObligationSelectionInput,
};

pub fn prove_execution_backed_query_selection(
    input: &QueryObligationSelectionInput,
) -> Result<ForgeQueryGraphObligationExecutionBackedAdoptionProof, QueryObligationSelectionError> {
    match input.authority_kind() {
        QueryObligationSelectionAuthorityKind::TopologyTouchedBasis => {
            topology_touched_basis_lane::prove_topology_touched_basis_query_selection(input)
        }
        QueryObligationSelectionAuthorityKind::SpatialQueryDescriptor => {
            spatial_descriptor_lane::prove_spatial_descriptor_query_selection(input)
        }
    }
}
