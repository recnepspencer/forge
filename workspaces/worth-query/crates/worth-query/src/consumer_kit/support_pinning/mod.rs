mod contract;
mod declaration;
mod document;
mod error;
mod evaluation;
mod evidence;
mod observed_row;
mod requirement;
mod snapshot_index;
mod status;

#[cfg(test)]
mod tests;

pub use contract::WorthQuerySupportPinContract;
pub use declaration::{
    support_pinning_contract, WorthQuerySupportPinContractBuilder, WorthQuerySupportPinDeclaration,
};
pub use document::{
    WorthQueryExternalSupportPinContractTerminalJsonDocument,
    WorthQuerySupportPinContractSchemaVersion, WorthQuerySupportPinContractTerminalJsonDocument,
};
pub use error::{WorthQuerySupportPinningError, WorthQuerySupportPinningErrorKind};
pub use evaluation::{
    WorthQuerySupportPinFinding, WorthQuerySupportPinFindingKind, WorthQuerySupportPinReport,
};
pub use observed_row::WorthQueryObservedSupportPin;
pub use requirement::{WorthQuerySupportPinRequirement, WorthQuerySupportPinRequirementDraft};
pub use status::{WorthQueryPinnedSupportStatus, WorthQueryPinnedTeachingPosture};

use document::WorthQuerySupportPinContractDocument;

pub fn load_support_pin_contract_terminal_json_document(
    terminal_json_document: &WorthQueryExternalSupportPinContractTerminalJsonDocument,
    expected_schema_version: WorthQuerySupportPinContractSchemaVersion,
) -> Result<WorthQuerySupportPinContract, WorthQuerySupportPinningError> {
    WorthQuerySupportPinContractDocument::from_terminal_json_document(terminal_json_document)?
        .validate(expected_schema_version)
}
