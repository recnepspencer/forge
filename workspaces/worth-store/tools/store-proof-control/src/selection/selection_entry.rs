use std::path::Path;

use crate::ValidatedProofInventory;

use super::{
    select_with_observed_source_edit, source_edit, ProofProductUnavailable,
    SelectedProofExecutionPlan, StoreProofRequest, StructuralPreflightReference,
};

pub fn select(
    workspace_root: &Path,
    inventory: &ValidatedProofInventory,
    request: StoreProofRequest,
    structural_preflight: StructuralPreflightReference,
) -> Result<SelectedProofExecutionPlan, ProofProductUnavailable> {
    let source_edit = source_edit::observe(workspace_root, request.source_edit())
        .map_err(ProofProductUnavailable::RepositoryObservation)?;
    select_with_observed_source_edit(
        workspace_root,
        inventory,
        request,
        structural_preflight,
        source_edit,
    )
}
