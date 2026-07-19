use std::borrow::Cow;

mod terminal_json_codec;

use super::error::{WorthQuerySupportSnapshotError, WorthQuerySupportSnapshotErrorKind};
use super::schema::WorthQuerySupportSnapshotSchemaVersion;
use super::semantic_admission::admit_support_snapshot_backend_posture;
use super::snapshot::WorthQuerySupportSnapshot;
use super::WorthQuerySupportSnapshotRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryExternalSupportSnapshotTerminalJsonDocument {
    text: Cow<'static, str>,
}

impl WorthQueryExternalSupportSnapshotTerminalJsonDocument {
    pub fn from_external_terminal_json_document(text: impl Into<String>) -> Self {
        Self {
            text: Cow::Owned(text.into()),
        }
    }

    pub const fn from_static_external_terminal_json_document(text: &'static str) -> Self {
        Self {
            text: Cow::Borrowed(text),
        }
    }

    pub fn as_str(&self) -> &str {
        self.text.as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQuerySupportSnapshotTerminalJsonDocument {
    text: String,
}

impl WorthQuerySupportSnapshotTerminalJsonDocument {
    pub(crate) fn from_native_terminal_projection(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }

    pub fn as_str(&self) -> &str {
        &self.text
    }

    pub fn to_external_terminal_json_document(
        &self,
    ) -> WorthQueryExternalSupportSnapshotTerminalJsonDocument {
        WorthQueryExternalSupportSnapshotTerminalJsonDocument::from_external_terminal_json_document(
            self.text.clone(),
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub(crate) struct WorthQuerySupportSnapshotDocument {
    schema_version: u16,
    schema_identity: String,
    backend_posture: String,
    source_matrix_digest: String,
    snapshot_digest: String,
    rows: Vec<WorthQuerySupportSnapshotRow>,
}

impl WorthQuerySupportSnapshotDocument {
    pub(crate) fn from_snapshot(snapshot: &WorthQuerySupportSnapshot) -> Self {
        Self {
            schema_version: snapshot.schema_version().major(),
            schema_identity: snapshot.schema_identity().to_string(),
            backend_posture: snapshot.backend_posture().to_string(),
            source_matrix_digest: snapshot.source_matrix_digest().to_string(),
            snapshot_digest: snapshot.snapshot_digest().to_string(),
            rows: snapshot.rows().to_vec(),
        }
    }

    pub fn from_terminal_json_document(
        terminal_json_document: &WorthQueryExternalSupportSnapshotTerminalJsonDocument,
    ) -> Result<Self, WorthQuerySupportSnapshotError> {
        terminal_json_codec::decode_external_terminal_json_document(terminal_json_document)
    }

    pub fn to_stable_terminal_json_document(
        &self,
    ) -> Result<WorthQuerySupportSnapshotTerminalJsonDocument, WorthQuerySupportSnapshotError> {
        self.to_canonical_terminal_json_document()
    }

    pub fn to_canonical_terminal_json_document(
        &self,
    ) -> Result<WorthQuerySupportSnapshotTerminalJsonDocument, WorthQuerySupportSnapshotError> {
        terminal_json_codec::encode_native_terminal_json_document(self)
    }

    pub(crate) fn validate(
        self,
        expected_schema_version: WorthQuerySupportSnapshotSchemaVersion,
    ) -> Result<WorthQuerySupportSnapshot, WorthQuerySupportSnapshotError> {
        if self.schema_version != expected_schema_version.major() {
            return Err(WorthQuerySupportSnapshotError::with_expected_found(
                WorthQuerySupportSnapshotErrorKind::SchemaVersionMismatch,
                format!(
                    "support snapshot schema version mismatch: expected {}, found {}",
                    expected_schema_version.major(),
                    self.schema_version
                ),
                expected_schema_version.major().to_string(),
                self.schema_version.to_string(),
            ));
        }
        admit_support_snapshot_backend_posture(&self.backend_posture)?;
        let snapshot = WorthQuerySupportSnapshot::from_validated_parts(
            expected_schema_version,
            self.schema_identity,
            self.backend_posture,
            self.source_matrix_digest,
            self.rows,
            self.snapshot_digest,
        );
        for row in snapshot.rows() {
            row.admit_schema_v1_semantics()?;
            row.validate_snapshot_row_digest(expected_schema_version)?;
        }
        let rebuilt_digest = snapshot.rebuild_digest()?;
        if rebuilt_digest != snapshot.snapshot_digest() {
            return Err(WorthQuerySupportSnapshotError::new(
                WorthQuerySupportSnapshotErrorKind::SnapshotDigestMismatch,
                format!(
                    "support snapshot digest mismatch: expected {}, found {}",
                    rebuilt_digest,
                    snapshot.snapshot_digest()
                ),
            ));
        }
        Ok(snapshot)
    }
}
