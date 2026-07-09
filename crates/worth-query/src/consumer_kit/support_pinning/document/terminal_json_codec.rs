use super::{
    WorthQueryExternalSupportPinContractTerminalJsonDocument, WorthQuerySupportPinContractDocument,
    WorthQuerySupportPinContractTerminalJsonDocument,
};
use crate::consumer_kit::support_pinning::{
    WorthQuerySupportPinningError, WorthQuerySupportPinningErrorKind,
};

pub(super) fn decode_external_terminal_json_document(
    terminal_json_document: &WorthQueryExternalSupportPinContractTerminalJsonDocument,
) -> Result<WorthQuerySupportPinContractDocument, WorthQuerySupportPinningError> {
    serde_json::from_str(terminal_json_document.as_str()).map_err(|error| {
        WorthQuerySupportPinningError::new(
            WorthQuerySupportPinningErrorKind::JsonDecodeFailed,
            format!("support pin contract document JSON decode failed: {error}"),
        )
    })
}

pub(super) fn encode_native_terminal_json_document(
    document: &WorthQuerySupportPinContractDocument,
) -> Result<WorthQuerySupportPinContractTerminalJsonDocument, WorthQuerySupportPinningError> {
    serde_json::to_string_pretty(document)
        .map(WorthQuerySupportPinContractTerminalJsonDocument::from_native_terminal_projection)
        .map_err(|error| {
            WorthQuerySupportPinningError::new(
                WorthQuerySupportPinningErrorKind::JsonEncodeFailed,
                format!("support pin contract document JSON encode failed: {error}"),
            )
        })
}
