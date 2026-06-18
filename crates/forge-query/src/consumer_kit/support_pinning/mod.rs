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

pub use contract::ForgeQuerySupportPinContract;
pub use declaration::{
    support_pinning_contract, ForgeQuerySupportPinContractBuilder, ForgeQuerySupportPinDeclaration,
};
pub use document::ForgeQuerySupportPinContractDocument;
pub use document::ForgeQuerySupportPinContractSchemaVersion;
pub use error::{ForgeQuerySupportPinningError, ForgeQuerySupportPinningErrorKind};
pub use evaluation::{
    ForgeQuerySupportPinFinding, ForgeQuerySupportPinFindingKind, ForgeQuerySupportPinReport,
};
pub use observed_row::ForgeQueryObservedSupportPin;
pub use requirement::{ForgeQuerySupportPinRequirement, ForgeQuerySupportPinRequirementDraft};
pub use status::{ForgeQueryPinnedSupportStatus, ForgeQueryPinnedTeachingPosture};

pub fn load_support_pin_contract_document(
    json: &str,
    expected_schema_version: ForgeQuerySupportPinContractSchemaVersion,
) -> Result<ForgeQuerySupportPinContract, ForgeQuerySupportPinningError> {
    ForgeQuerySupportPinContractDocument::from_json(json)?.validate(expected_schema_version)
}
