use worth_query::facade::runtime::WorthQueryEvidenceIdentity;

use super::{UiNativeTextValue, UiProjectionAvailability, UiProjectionFactReceipt};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiCollectionCompleteness {
    Complete,
    Partial,
}

#[derive(Debug, Eq, PartialEq)]
pub struct UiCollectionContinuation {
    query_continuation_identity: WorthQueryEvidenceIdentity,
}

impl UiCollectionContinuation {
    pub fn identity_for_reporting(&self) -> &str {
        self.query_continuation_identity
            .terminal_projection_for_reporting()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct UiCollectionProjectionRowReference {
    query_row_identity: WorthQueryEvidenceIdentity,
}

impl UiCollectionProjectionRowReference {
    pub fn identity_for_reporting(&self) -> &str {
        self.query_row_identity.terminal_projection_for_reporting()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct UiCollectionProjectionTextRow {
    row: UiCollectionProjectionRowReference,
    selected_values: Box<[UiNativeTextValue]>,
}

impl UiCollectionProjectionTextRow {
    pub fn row(&self) -> &UiCollectionProjectionRowReference {
        &self.row
    }

    pub fn selected_values(&self) -> &[UiNativeTextValue] {
        &self.selected_values
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct UiCollectionProjectionValue {
    rows: Box<[UiCollectionProjectionTextRow]>,
    completeness: UiCollectionCompleteness,
    continuation: Option<UiCollectionContinuation>,
}

impl UiCollectionProjectionValue {
    pub fn rows(&self) -> &[UiCollectionProjectionTextRow] {
        &self.rows
    }

    pub fn completeness(&self) -> UiCollectionCompleteness {
        self.completeness
    }

    pub fn continuation(&self) -> Option<&UiCollectionContinuation> {
        self.continuation.as_ref()
    }
}

#[must_use]
#[derive(Debug, Eq, PartialEq)]
pub struct UiCollectionProjectionFactReceipt {
    core: UiProjectionFactReceipt,
    availability: UiProjectionAvailability<UiCollectionProjectionValue>,
}

impl UiCollectionProjectionFactReceipt {
    pub fn core(&self) -> &UiProjectionFactReceipt {
        &self.core
    }

    pub fn availability(&self) -> &UiProjectionAvailability<UiCollectionProjectionValue> {
        &self.availability
    }
}
