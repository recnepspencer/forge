use crate::runtime::ForgeQueryRuntimePublicSupportMatrix;

use super::document::ForgeQuerySupportSnapshotDocument;
use super::error::{ForgeQuerySupportSnapshotError, ForgeQuerySupportSnapshotErrorKind};
use super::evidence::support_snapshot_document_identity;
use super::row::ForgeQuerySupportSnapshotRow;
use super::schema::ForgeQuerySupportSnapshotSchemaVersion;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQuerySupportSnapshot {
    schema_version: ForgeQuerySupportSnapshotSchemaVersion,
    schema_identity: String,
    backend_posture: String,
    source_matrix_digest: String,
    rows: Vec<ForgeQuerySupportSnapshotRow>,
    snapshot_digest: String,
}

impl ForgeQuerySupportSnapshot {
    pub(crate) fn from_public_support_matrix(
        matrix: &ForgeQueryRuntimePublicSupportMatrix,
    ) -> Self {
        let schema_version = ForgeQuerySupportSnapshotSchemaVersion::current();
        let schema_identity = schema_version.identity();
        let backend_posture = matrix.backend_posture().as_str().to_string();
        let source_matrix_digest = matrix
            .matrix_digest()
            .terminal_projection_for_reporting()
            .to_string();
        let rows = matrix
            .rows()
            .iter()
            .map(|row| ForgeQuerySupportSnapshotRow::from_runtime_row(schema_version, row))
            .collect::<Vec<_>>();
        let snapshot_digest = support_snapshot_document_identity(
            schema_version,
            &schema_identity,
            &backend_posture,
            &source_matrix_digest,
            rows.iter(),
        )
        .terminal_projection_for_reporting()
        .to_string();
        Self {
            schema_version,
            schema_identity: schema_identity
                .terminal_projection_for_reporting()
                .to_string(),
            backend_posture,
            source_matrix_digest,
            rows,
            snapshot_digest,
        }
    }

    pub(crate) fn from_document(
        document: ForgeQuerySupportSnapshotDocument,
        expected_schema_version: ForgeQuerySupportSnapshotSchemaVersion,
    ) -> Result<Self, ForgeQuerySupportSnapshotError> {
        document.validate(expected_schema_version)
    }

    pub fn schema_version(&self) -> ForgeQuerySupportSnapshotSchemaVersion {
        self.schema_version
    }

    pub fn schema_identity(&self) -> &str {
        &self.schema_identity
    }

    pub fn backend_posture(&self) -> &str {
        &self.backend_posture
    }

    pub fn source_matrix_digest(&self) -> &str {
        &self.source_matrix_digest
    }

    pub fn rows(&self) -> &[ForgeQuerySupportSnapshotRow] {
        &self.rows
    }

    pub fn snapshot_digest(&self) -> &str {
        &self.snapshot_digest
    }

    pub fn to_document(&self) -> ForgeQuerySupportSnapshotDocument {
        ForgeQuerySupportSnapshotDocument::from_snapshot(self)
    }

    pub fn to_stable_json(&self) -> Result<String, ForgeQuerySupportSnapshotError> {
        self.to_canonical_json()
    }

    pub fn to_canonical_json(&self) -> Result<String, ForgeQuerySupportSnapshotError> {
        self.to_document().to_stable_json()
    }

    pub(crate) fn rebuild_digest(&self) -> Result<String, ForgeQuerySupportSnapshotError> {
        let schema_identity = self.schema_version.identity();
        let actual_schema_identity = schema_identity.terminal_projection_for_reporting();
        if self.schema_identity != actual_schema_identity {
            return Err(ForgeQuerySupportSnapshotError::with_expected_found(
                ForgeQuerySupportSnapshotErrorKind::SchemaIdentityMismatch,
                format!(
                    "support snapshot schema identity mismatch: expected {actual_schema_identity}, found {}",
                    self.schema_identity
                ),
                actual_schema_identity,
                self.schema_identity.clone(),
            ));
        }
        Ok(support_snapshot_document_identity(
            self.schema_version,
            &schema_identity,
            &self.backend_posture,
            &self.source_matrix_digest,
            self.rows.iter(),
        )
        .terminal_projection_for_reporting()
        .to_string())
    }

    pub(crate) fn from_validated_parts(
        schema_version: ForgeQuerySupportSnapshotSchemaVersion,
        schema_identity: String,
        backend_posture: String,
        source_matrix_digest: String,
        rows: Vec<ForgeQuerySupportSnapshotRow>,
        snapshot_digest: String,
    ) -> Self {
        Self {
            schema_version,
            schema_identity,
            backend_posture,
            source_matrix_digest,
            rows,
            snapshot_digest,
        }
    }
}
