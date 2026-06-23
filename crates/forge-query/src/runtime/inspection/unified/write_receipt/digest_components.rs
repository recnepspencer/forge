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
                ForgeQueryEvidenceTag::new("admitted_aspect_touch"),
                operation.aspect_touch().admitted_touch_digest_part(),
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
        .map(|(key, value)| {
            forge_query_evidence_identity(
                ForgeQueryEvidenceScope::WriteReceiptMutationMetadataEntry,
            )
            .field_value(ForgeQueryEvidenceTag::new("key"), key.as_str())
            .field_value(
                ForgeQueryEvidenceTag::new("value"),
                value.terminal_digest_text(),
            )
            .seal()
        })
        .collect()
}
