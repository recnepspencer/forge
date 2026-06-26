use crate::runtime::{ForgeQueryRuntimePublicSupportMatrix, ForgeQueryWorkspace};

use super::document::{
    ForgeQueryExternalSupportSnapshotTerminalJsonDocument, ForgeQuerySupportSnapshotDocument,
};
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

pub fn load_support_snapshot_terminal_json_document(
    terminal_json_document: &ForgeQueryExternalSupportSnapshotTerminalJsonDocument,
    expected_schema_version: ForgeQuerySupportSnapshotSchemaVersion,
) -> Result<ForgeQuerySupportSnapshot, ForgeQuerySupportSnapshotError> {
    ForgeQuerySupportSnapshot::from_document(
        ForgeQuerySupportSnapshotDocument::from_terminal_json_document(terminal_json_document)?,
        expected_schema_version,
    )
}
