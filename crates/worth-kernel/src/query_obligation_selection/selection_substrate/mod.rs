mod local_ceremony_closeout;
mod local_selector_denial;
mod primitive_construction_contract;
mod primitive_construction_residue_baseline;
mod query_consumer_kit_lane;
mod selected_obligations;
mod selection_error;
mod selection_request;
mod selector_precision;
mod spatial_descriptor_input;
mod topology_touched_basis_input;

pub use local_ceremony_closeout::{
    QuerySelectionForbiddenAuthorityKind, QuerySelectionLocalCeremonyCloseout,
};
pub use local_selector_denial::{
    deny_broad_collection_query_obligation_selector_authority,
    deny_copied_query_obligation_selection_parts,
    deny_in_memory_query_obligation_selection_authority,
    deny_lifecycle_only_query_obligation_selector_authority,
    deny_local_query_obligation_selector_authority,
    deny_local_support_row_query_obligation_authority,
    deny_raw_descriptor_query_obligation_selection_authority,
    deny_source_grep_query_obligation_audit_authority,
    deny_topology_spatial_substitution_query_obligation_authority,
};
pub use primitive_construction_contract::{
    query_primitive_construction_family_cardinality_closeout,
    query_primitive_construction_residue_contract,
    QueryPrimitiveConstructionFamilyCardinalityCloseout, QueryPrimitiveConstructionResidueContract,
    QueryPrimitiveConstructionResidueContractRow,
};
pub use primitive_construction_residue_baseline::{
    query_primitive_construction_residue_baseline_v1, QueryPrimitiveConstructionResidueBaseline,
    QueryPrimitiveConstructionResidueBaselineRow,
};
pub use selected_obligations::{
    QuerySelectedGraphObligationCloseout, QuerySelectedGraphObligations,
};
pub use selection_error::{QueryObligationSelectionError, QueryObligationSelectionErrorKind};
pub use selection_request::{QueryObligationSelectionAuthorityKind, QueryObligationSelectionInput};
pub use selector_precision::{
    QueryBroadSelectorResidueRow, QueryBroadSelectorResidueRows,
    QuerySelectorExpressivenessGapKind, QuerySelectorExpressivenessGapRow,
    QuerySelectorExpressivenessGaps, QuerySelectorPrecisionPosture, QuerySelectorPrecisionReport,
};

use forge_query::facade::consumer_kit::{
    ForgeQueryGraphObligationConsumerKitError,
    ForgeQueryGraphObligationExecutionBackedAdoptionProof,
};

pub struct QueryObligationSelectionSubstrate;

impl QueryObligationSelectionSubstrate {
    pub fn select_execution_backed_obligations(
        input: QueryObligationSelectionInput,
    ) -> Result<QuerySelectedGraphObligations, QueryObligationSelectionError> {
        let proof = query_consumer_kit_lane::prove_execution_backed_query_selection(&input)?;
        require_execution_backed_proof_has_real_rows(&proof)?;
        Ok(QuerySelectedGraphObligations::from_query_proof(
            input, proof,
        ))
    }
}

fn require_execution_backed_proof_has_real_rows(
    proof: &ForgeQueryGraphObligationExecutionBackedAdoptionProof,
) -> Result<(), QueryObligationSelectionError> {
    if !proof.execution_proof().has_real_executor_rows() {
        return Err(QueryObligationSelectionError::empty_execution_proof());
    }
    Ok(())
}

impl From<ForgeQueryGraphObligationConsumerKitError> for QueryObligationSelectionError {
    fn from(error: ForgeQueryGraphObligationConsumerKitError) -> Self {
        Self::query_consumer_kit(error)
    }
}
