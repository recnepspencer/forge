use crate::runtime::{ForgeQueryRuntimePublicSupportMatrix, ForgeQueryWorkspace};

use super::document::ForgeQuerySupportSnapshotDocument;
use super::error::ForgeQuerySupportSnapshotError;
use super::schema::ForgeQuerySupportSnapshotSchemaVersion;
use super::snapshot::ForgeQuerySupportSnapshot;

pub fn project_support_snapshot(
    matrix: &ForgeQueryRuntimePublicSupportMatrix,
) -> ForgeQuerySupportSnapshot {
    ForgeQuerySupportSnapshot::from_public_support_matrix(matrix)
}

pub fn project_workspace_support_snapshot(
    workspace: &ForgeQueryWorkspace,
) -> ForgeQuerySupportSnapshot {
    let matrix = workspace.public_support_matrix();
    project_support_snapshot(&matrix)
}

pub fn load_support_snapshot_document(
    json: &str,
    expected_schema_version: ForgeQuerySupportSnapshotSchemaVersion,
) -> Result<ForgeQuerySupportSnapshot, ForgeQuerySupportSnapshotError> {
    ForgeQuerySupportSnapshot::from_document(
        ForgeQuerySupportSnapshotDocument::from_json(json)?,
        expected_schema_version,
    )
}
