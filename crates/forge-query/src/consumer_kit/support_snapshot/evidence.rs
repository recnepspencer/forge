use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

use super::row::ForgeQuerySupportSnapshotRow;
use super::schema::ForgeQuerySupportSnapshotSchemaVersion;

pub(crate) fn support_snapshot_row_identity(
    schema_version: ForgeQuerySupportSnapshotSchemaVersion,
    row: &ForgeQuerySupportSnapshotRow,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::ConsumerSupportSnapshotRow)
        .field_shape(
            ForgeQueryEvidenceTag::new("schema_version"),
            schema_version.as_str(),
        )
        .field_shape(ForgeQueryEvidenceTag::new("surface"), row.surface())
        .optional_shape(
            ForgeQueryEvidenceTag::new("facade_family"),
            row.facade_family(),
        )
        .field_shape(ForgeQueryEvidenceTag::new("status"), row.status())
        .field_shape(
            ForgeQueryEvidenceTag::new("teaching_posture"),
            row.teaching_posture(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("owner_milestone"),
            row.owner_milestone(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("extension_rule"),
            row.extension_rule(),
        )
        .field_bool(
            ForgeQueryEvidenceTag::new("parallel_api_forbidden"),
            row.parallel_api_forbidden(),
        )
        .field_bool(
            ForgeQueryEvidenceTag::new("admission_fail_closed"),
            row.admission_fail_closed(),
        )
        .optional_value(
            ForgeQueryEvidenceTag::new("support_contract_digest"),
            row.support_contract_digest(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("live_row_digest"),
            row.live_row_digest(),
        )
        .seal()
}

pub(crate) fn support_snapshot_document_identity<'a>(
    schema_version: ForgeQuerySupportSnapshotSchemaVersion,
    schema_identity: &ForgeQueryEvidenceIdentity,
    backend_posture: &str,
    source_matrix_digest: &str,
    rows: impl IntoIterator<Item = &'a ForgeQuerySupportSnapshotRow>,
) -> ForgeQueryEvidenceIdentity {
    let rows = rows.into_iter().collect::<Vec<_>>();
    forge_query_evidence_identity(ForgeQueryEvidenceScope::ConsumerSupportSnapshotDocument)
        .field_shape(
            ForgeQueryEvidenceTag::new("schema_version"),
            schema_version.as_str(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("schema_identity"),
            schema_identity,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("backend_posture"),
            backend_posture,
        )
        .field_value(
            ForgeQueryEvidenceTag::new("source_matrix_digest"),
            source_matrix_digest,
        )
        .field_usize(ForgeQueryEvidenceTag::new("row_count"), rows.len())
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("snapshot_row_digest"),
            rows.iter().map(|row| row.snapshot_row_digest()),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("live_row_digest"),
            rows.iter().map(|row| row.live_row_digest()),
        )
        .seal()
}
