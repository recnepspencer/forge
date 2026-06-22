use super::error::{ForgeQuerySupportSnapshotError, ForgeQuerySupportSnapshotErrorKind};
use super::schema::ForgeQuerySupportSnapshotSchemaVersion;
use super::semantic_admission::admit_support_snapshot_backend_posture;
use super::snapshot::ForgeQuerySupportSnapshot;
use super::ForgeQuerySupportSnapshotRow;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ForgeQuerySupportSnapshotDocument {
    schema_version: u16,
    schema_identity: String,
    backend_posture: String,
    source_matrix_digest: String,
    snapshot_digest: String,
    rows: Vec<ForgeQuerySupportSnapshotRow>,
}

impl ForgeQuerySupportSnapshotDocument {
    pub(crate) fn from_snapshot(snapshot: &ForgeQuerySupportSnapshot) -> Self {
        Self {
            schema_version: snapshot.schema_version().major(),
            schema_identity: snapshot.schema_identity().to_string(),
            backend_posture: snapshot.backend_posture().to_string(),
            source_matrix_digest: snapshot.source_matrix_digest().to_string(),
            snapshot_digest: snapshot.snapshot_digest().to_string(),
            rows: snapshot.rows().to_vec(),
        }
    }

    pub fn from_json(json: &str) -> Result<Self, ForgeQuerySupportSnapshotError> {
        serde_json::from_str(json).map_err(|error| {
            ForgeQuerySupportSnapshotError::new(
                ForgeQuerySupportSnapshotErrorKind::JsonDecodeFailed,
                format!("support snapshot document JSON decode failed: {error}"),
            )
        })
    }

    pub fn to_stable_json(&self) -> Result<String, ForgeQuerySupportSnapshotError> {
        self.to_canonical_json()
    }

    pub fn to_canonical_json(&self) -> Result<String, ForgeQuerySupportSnapshotError> {
        serde_json::to_string_pretty(self).map_err(|error| {
            ForgeQuerySupportSnapshotError::new(
                ForgeQuerySupportSnapshotErrorKind::JsonEncodeFailed,
                format!("support snapshot document JSON encode failed: {error}"),
            )
        })
    }

    pub(crate) fn validate(
        self,
        expected_schema_version: ForgeQuerySupportSnapshotSchemaVersion,
    ) -> Result<ForgeQuerySupportSnapshot, ForgeQuerySupportSnapshotError> {
        if self.schema_version != expected_schema_version.major() {
            return Err(ForgeQuerySupportSnapshotError::with_expected_found(
                ForgeQuerySupportSnapshotErrorKind::SchemaVersionMismatch,
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
        let snapshot = ForgeQuerySupportSnapshot::from_validated_parts(
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
            return Err(ForgeQuerySupportSnapshotError::new(
                ForgeQuerySupportSnapshotErrorKind::SnapshotDigestMismatch,
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
