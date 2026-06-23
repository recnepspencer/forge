use crate::consumer_kit::support_pinning::{
    ForgeQueryExternalSupportPinContractTerminalJsonDocument, ForgeQuerySupportPinContract,
};

pub(super) struct HostileSupportPinContractTerminalDocument {
    text: String,
}

impl HostileSupportPinContractTerminalDocument {
    pub(super) fn from_contract(contract: &ForgeQuerySupportPinContract) -> Self {
        Self {
            text: contract
                .to_canonical_terminal_json_document()
                .expect("support pin contract should encode as terminal document")
                .as_str()
                .to_string(),
        }
    }

    pub(super) fn replace_top_level_string(
        &mut self,
        field: &'static str,
        value: impl Into<String>,
    ) {
        replace_string_field(&mut self.text, field, value, 0);
    }

    pub(super) fn replace_first_requirement_string(
        &mut self,
        field: &'static str,
        value: impl Into<String>,
    ) {
        replace_string_field(&mut self.text, field, value, 0);
    }

    pub(super) fn into_external_terminal_json_document(
        self,
    ) -> ForgeQueryExternalSupportPinContractTerminalJsonDocument {
        ForgeQueryExternalSupportPinContractTerminalJsonDocument::from_external_terminal_json_document(
            self.text,
        )
    }
}

fn replace_string_field(
    document: &mut String,
    field: &'static str,
    value: impl Into<String>,
    occurrence: usize,
) {
    let field_marker = format!("\"{field}\": \"");
    let start = nth_match(document, &field_marker, occurrence) + field_marker.len();
    let end = document[start..]
        .find('"')
        .map(|offset| start + offset)
        .expect("hostile terminal string field should have a closing quote");
    document.replace_range(start..end, &value.into());
}

fn nth_match(document: &str, pattern: &str, occurrence: usize) -> usize {
    document
        .match_indices(pattern)
        .nth(occurrence)
        .map(|(index, _)| index)
        .expect("hostile terminal mutation field should exist")
}
