use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::runtime::{ForgeQueryAspectMutationOperation, ForgeQueryMutationMetadata};

pub(super) fn declared_aspect_operation_identities(
    operations: &[ForgeQueryAspectMutationOperation],
) -> Vec<ForgeQueryEvidenceIdentity> {
    operations
        .iter()
        .map(|operation| {
            forge_query_evidence_identity(
                ForgeQueryEvidenceScope::WriteReceiptDeclaredAspectOperation,
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("kind"),
                operation.kind().as_str(),
            )
            .field_value(
                ForgeQueryEvidenceTag::new("aspect_path"),
                operation.aspect_path(),
            )
            .seal()
        })
        .collect()
}

pub(super) fn mutation_metadata_entry_identities(
    mutation_metadata: &ForgeQueryMutationMetadata,
) -> Vec<ForgeQueryEvidenceIdentity> {
    mutation_metadata
        .entries()
        .iter()
        .map(|(key, value)| {
            let encoded_value = serde_json::to_string(value).unwrap_or_else(|_| value.to_string());
            forge_query_evidence_identity(
                ForgeQueryEvidenceScope::WriteReceiptMutationMetadataEntry,
            )
            .field_value(ForgeQueryEvidenceTag::new("key"), key)
            .field_value(ForgeQueryEvidenceTag::new("value"), encoded_value)
            .seal()
        })
        .collect()
}
