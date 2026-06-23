use super::{
    ForgeQueryExternalSupportSnapshotTerminalJsonDocument, ForgeQuerySupportSnapshotDocument,
    ForgeQuerySupportSnapshotTerminalJsonDocument,
};
use crate::consumer_kit::support_snapshot::{
    ForgeQuerySupportSnapshotError, ForgeQuerySupportSnapshotErrorKind,
};

pub(super) fn decode_external_terminal_json_document(
    terminal_json_document: &ForgeQueryExternalSupportSnapshotTerminalJsonDocument,
) -> Result<ForgeQuerySupportSnapshotDocument, ForgeQuerySupportSnapshotError> {
    serde_json::from_str(terminal_json_document.as_str()).map_err(|error| {
        ForgeQuerySupportSnapshotError::new(
            ForgeQuerySupportSnapshotErrorKind::JsonDecodeFailed,
            format!("support snapshot document JSON decode failed: {error}"),
        )
    })
}

pub(super) fn encode_native_terminal_json_document(
    document: &ForgeQuerySupportSnapshotDocument,
) -> Result<ForgeQuerySupportSnapshotTerminalJsonDocument, ForgeQuerySupportSnapshotError> {
    serde_json::to_string_pretty(document)
        .map(ForgeQuerySupportSnapshotTerminalJsonDocument::from_native_terminal_projection)
        .map_err(|error| {
            ForgeQuerySupportSnapshotError::new(
                ForgeQuerySupportSnapshotErrorKind::JsonEncodeFailed,
                format!("support snapshot document JSON encode failed: {error}"),
            )
        })
}
