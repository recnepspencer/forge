use super::{
    document_error, ContractDocument, ProjectionAuthorityContractDocumentError,
    ProjectionAuthorityContractDocumentErrorKind,
};

pub(super) fn encode(
    document: &ContractDocument,
) -> Result<String, ProjectionAuthorityContractDocumentError> {
    serde_json::to_string(document).map_err(invalid_json)
}

pub(super) fn decode(
    document: &str,
) -> Result<ContractDocument, ProjectionAuthorityContractDocumentError> {
    serde_json::from_str(document).map_err(invalid_json)
}

fn invalid_json(error: serde_json::Error) -> ProjectionAuthorityContractDocumentError {
    document_error(
        ProjectionAuthorityContractDocumentErrorKind::InvalidJson,
        error.to_string(),
    )
}
