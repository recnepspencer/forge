use super::{
    WorthQueryExternalSupportSnapshotTerminalJsonDocument, WorthQuerySupportSnapshotDocument,
    WorthQuerySupportSnapshotTerminalJsonDocument,
};
use crate::consumer_kit::support_snapshot::{
    WorthQuerySupportSnapshotError, WorthQuerySupportSnapshotErrorKind,
};

pub(super) fn decode_external_terminal_json_document(
    terminal_json_document: &WorthQueryExternalSupportSnapshotTerminalJsonDocument,
) -> Result<WorthQuerySupportSnapshotDocument, WorthQuerySupportSnapshotError> {
    serde_json::from_str(terminal_json_document.as_str()).map_err(|error| {
        WorthQuerySupportSnapshotError::new(
            WorthQuerySupportSnapshotErrorKind::JsonDecodeFailed,
            format!("support snapshot document JSON decode failed: {error}"),
        )
    })
}

pub(super) fn encode_native_terminal_json_document(
    document: &WorthQuerySupportSnapshotDocument,
) -> Result<WorthQuerySupportSnapshotTerminalJsonDocument, WorthQuerySupportSnapshotError> {
    serde_json::to_string_pretty(document)
        .map(WorthQuerySupportSnapshotTerminalJsonDocument::from_native_terminal_projection)
        .map_err(|error| {
            WorthQuerySupportSnapshotError::new(
                WorthQuerySupportSnapshotErrorKind::JsonEncodeFailed,
                format!("support snapshot document JSON encode failed: {error}"),
            )
        })
}
