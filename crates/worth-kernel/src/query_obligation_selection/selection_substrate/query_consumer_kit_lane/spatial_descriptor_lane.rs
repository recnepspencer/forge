use forge_query::facade::consumer_kit::ForgeQueryGraphObligationExecutionBackedAdoptionProof;
use worth_spatial::facade::query_adoption::{
    spatial_query_graph_obligation_adoption_proof_for_descriptor,
    WorthSpatialQueryConsumerKitAdoptionError,
};

use crate::query_obligation_selection::selection_substrate::{
    QueryObligationSelectionError, QueryObligationSelectionInput,
};

use super::local_ceremony_audit::selection_substrate_local_ceremony_audit;

pub fn prove_spatial_descriptor_query_selection(
    input: &QueryObligationSelectionInput,
) -> Result<ForgeQueryGraphObligationExecutionBackedAdoptionProof, QueryObligationSelectionError> {
    require_selection_substrate_local_ceremony_is_clean()?;
    let descriptor = input
        .spatial_descriptor()
        .ok_or_else(QueryObligationSelectionError::missing_spatial_descriptor_authority)?;
    spatial_query_graph_obligation_adoption_proof_for_descriptor(descriptor)
        .map_err(map_spatial_consumer_kit_error)
}

fn require_selection_substrate_local_ceremony_is_clean() -> Result<(), QueryObligationSelectionError>
{
    let audit = selection_substrate_local_ceremony_audit();
    if audit.is_clean() {
        return Ok(());
    }
    Err(
        QueryObligationSelectionError::local_selector_authority_denied(&format!(
            "selection substrate audit {} found {} local ceremony finding(s)",
            audit.audit_digest(),
            audit.findings().len()
        )),
    )
}

fn map_spatial_consumer_kit_error(
    error: WorthSpatialQueryConsumerKitAdoptionError,
) -> QueryObligationSelectionError {
    QueryObligationSelectionError::spatial_consumer_kit(format!("{error:?}"))
}
