pub(crate) use super::query_authority::{
    require_primitive_construction_query_authority, PrimitiveConstructionOperatingContext,
    PrimitiveConstructionQueryAuthorityReceipt, PrimitiveConstructionQueryAuthorityRequest,
    PrimitiveConstructionQueryDeclarationInput, PrimitiveConstructionQueryDomain,
};
use forge_query::facade::ForgeQueryWorkspace;

use super::query_authority::{
    default_primitive_construction_query_authority_receipt,
    PrimitiveConstructionQueryAuthorityError,
};

#[derive(Debug)]
pub enum PrimitiveConstructionQueryEntryError {
    Authority(PrimitiveConstructionQueryAuthorityError),
}

impl From<PrimitiveConstructionQueryAuthorityError> for PrimitiveConstructionQueryEntryError {
    fn from(value: PrimitiveConstructionQueryAuthorityError) -> Self {
        Self::Authority(value)
    }
}

impl std::fmt::Display for PrimitiveConstructionQueryEntryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authority(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionQueryEntryError {}

pub(crate) fn require_default_primitive_construction_query_authority(
    workspace: &ForgeQueryWorkspace,
) -> Result<PrimitiveConstructionQueryAuthorityReceipt, PrimitiveConstructionQueryEntryError> {
    let workspace_support_matrix = workspace.public_support_matrix();
    let workspace_support_matrix_digest = workspace_support_matrix
        .matrix_digest()
        .terminal_projection_for_reporting();
    let request = PrimitiveConstructionQueryAuthorityRequest::projection_consumption_surface(
        format!(
            "worth-kernel.primitive-construction.current-head:{workspace_support_matrix_digest}"
        ),
        workspace_support_matrix_digest,
    );
    default_primitive_construction_query_authority_receipt(workspace, request).map_err(Into::into)
}
