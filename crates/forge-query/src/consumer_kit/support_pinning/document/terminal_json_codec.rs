use super::{
    ForgeQueryExternalSupportPinContractTerminalJsonDocument, ForgeQuerySupportPinContractDocument,
    ForgeQuerySupportPinContractTerminalJsonDocument,
};
use crate::consumer_kit::support_pinning::{
    ForgeQuerySupportPinningError, ForgeQuerySupportPinningErrorKind,
};

pub(super) fn decode_external_terminal_json_document(
    terminal_json_document: &ForgeQueryExternalSupportPinContractTerminalJsonDocument,
) -> Result<ForgeQuerySupportPinContractDocument, ForgeQuerySupportPinningError> {
    serde_json::from_str(terminal_json_document.as_str()).map_err(|error| {
        ForgeQuerySupportPinningError::new(
            ForgeQuerySupportPinningErrorKind::JsonDecodeFailed,
            format!("support pin contract document JSON decode failed: {error}"),
        )
    })
}

pub(super) fn encode_native_terminal_json_document(
    document: &ForgeQuerySupportPinContractDocument,
) -> Result<ForgeQuerySupportPinContractTerminalJsonDocument, ForgeQuerySupportPinningError> {
    serde_json::to_string_pretty(document)
        .map(ForgeQuerySupportPinContractTerminalJsonDocument::from_native_terminal_projection)
        .map_err(|error| {
            ForgeQuerySupportPinningError::new(
                ForgeQuerySupportPinningErrorKind::JsonEncodeFailed,
                format!("support pin contract document JSON encode failed: {error}"),
            )
        })
}
