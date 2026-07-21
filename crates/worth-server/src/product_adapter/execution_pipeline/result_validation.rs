use crate::{
    WorthServerProductOperationDeclaration, WorthServerProductOperationExecutionBoundary,
    WorthServerProductOperationSuccess, WorthServerProductOperationSurfaceDenial,
    WorthServerProductOperationSurfaceDenialCode, WorthServerProductOperationSurfaceDenialFacts,
};

pub(in crate::product_adapter) fn validate_success_result(
    declaration: &WorthServerProductOperationDeclaration,
    success: &WorthServerProductOperationSuccess,
) -> Result<(), WorthServerProductOperationSurfaceDenial> {
    if crate::product_result::artifact_matches_contract(
        success.result_artifact(),
        declaration.result_contract(),
    ) {
        return Ok(());
    }
    Err(WorthServerProductOperationSurfaceDenial::new(
        WorthServerProductOperationSurfaceDenialCode::InvalidResultArtifact,
        format!(
            "product operation `{}` returned result contract `{}` but declaration requires `{}`",
            declaration.operation_name(),
            success.result_artifact().contract().canonical_digest(),
            declaration.result_contract().canonical_digest(),
        ),
    )
    .with_facts(
        WorthServerProductOperationSurfaceDenialFacts::default().with_execution_boundary(
            WorthServerProductOperationExecutionBoundary::AdapterExecutionAttempted,
        ),
    ))
}
