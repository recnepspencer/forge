use std::collections::BTreeSet;

use super::super::support_snapshot::ForgeQuerySupportSnapshot;
use super::document::schema::ForgeQuerySupportPinContractSchemaVersion;
use super::document::{
    ForgeQuerySupportPinContractDocument, ForgeQuerySupportPinContractTerminalJsonDocument,
};
use super::error::{ForgeQuerySupportPinningError, ForgeQuerySupportPinningErrorKind};
use super::evaluation::{evaluate_support_pin_contract, ForgeQuerySupportPinReport};
use super::observed_row::ForgeQueryObservedSupportPin;
use super::requirement::ForgeQuerySupportPinRequirement;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQuerySupportPinContract {
    consumer_name: String,
    contract_schema_version: ForgeQuerySupportPinContractSchemaVersion,
    contract_schema_identity: String,
    pinned_vocabulary_identity: String,
    support_snapshot_schema_identity: String,
    source_matrix_digest: String,
    requirements: Vec<ForgeQuerySupportPinRequirement>,
    observed_rows: Vec<ForgeQueryObservedSupportPin>,
    contract_digest: String,
}

impl ForgeQuerySupportPinContract {
    pub(crate) fn new(
        consumer_name: String,
        contract_schema_version: ForgeQuerySupportPinContractSchemaVersion,
        contract_schema_identity: String,
        pinned_vocabulary_identity: String,
        support_snapshot_schema_identity: String,
        source_matrix_digest: String,
        requirements: Vec<ForgeQuerySupportPinRequirement>,
        observed_rows: Vec<ForgeQueryObservedSupportPin>,
        contract_digest: String,
    ) -> Result<Self, ForgeQuerySupportPinningError> {
        if consumer_name.trim().is_empty() {
            return Err(ForgeQuerySupportPinningError::new(
                ForgeQuerySupportPinningErrorKind::BlankConsumerName,
                "support pin consumer name must not be blank",
            ));
        }
        reject_duplicate_contract_families(&requirements, &observed_rows)?;
        Ok(Self {
            consumer_name,
            contract_schema_version,
            contract_schema_identity,
            pinned_vocabulary_identity,
            support_snapshot_schema_identity,
            source_matrix_digest,
            requirements,
            observed_rows,
            contract_digest,
        })
    }

    pub fn evaluate_snapshot(
        &self,
        snapshot: &ForgeQuerySupportSnapshot,
    ) -> Result<ForgeQuerySupportPinReport, ForgeQuerySupportPinningError> {
        evaluate_support_pin_contract(self, snapshot)
    }

    pub fn to_canonical_terminal_json_document(
        &self,
    ) -> Result<ForgeQuerySupportPinContractTerminalJsonDocument, ForgeQuerySupportPinningError>
    {
        self.to_document().to_canonical_terminal_json_document()
    }

    pub fn to_stable_terminal_json_document(
        &self,
    ) -> Result<ForgeQuerySupportPinContractTerminalJsonDocument, ForgeQuerySupportPinningError>
    {
        self.to_canonical_terminal_json_document()
    }

    pub(crate) fn to_document(&self) -> ForgeQuerySupportPinContractDocument {
        ForgeQuerySupportPinContractDocument::from_contract(self)
    }

    pub fn consumer_name(&self) -> &str {
        &self.consumer_name
    }

    pub fn contract_schema_version(&self) -> ForgeQuerySupportPinContractSchemaVersion {
        self.contract_schema_version
    }

    pub fn contract_schema_identity(&self) -> &str {
        &self.contract_schema_identity
    }

    pub fn pinned_vocabulary_identity(&self) -> &str {
        &self.pinned_vocabulary_identity
    }

    pub fn schema_identity(&self) -> &str {
        &self.support_snapshot_schema_identity
    }

    pub fn support_snapshot_schema_identity(&self) -> &str {
        &self.support_snapshot_schema_identity
    }

    pub fn source_matrix_digest(&self) -> &str {
        &self.source_matrix_digest
    }

    pub fn requirements(&self) -> &[ForgeQuerySupportPinRequirement] {
        &self.requirements
    }

    pub fn observed_rows(&self) -> &[ForgeQueryObservedSupportPin] {
        &self.observed_rows
    }

    pub fn contract_digest(&self) -> &str {
        &self.contract_digest
    }
}

fn reject_duplicate_contract_families(
    requirements: &[ForgeQuerySupportPinRequirement],
    observed_rows: &[ForgeQueryObservedSupportPin],
) -> Result<(), ForgeQuerySupportPinningError> {
    let mut required = BTreeSet::new();
    let mut observed = BTreeSet::new();
    for requirement in requirements {
        if !required.insert(requirement.family()) {
            return Err(ForgeQuerySupportPinningError::with_family(
                ForgeQuerySupportPinningErrorKind::DuplicateRequiredFamily,
                "support pin required family is declared more than once",
                requirement.family().as_str(),
            ));
        }
    }
    for observed_row in observed_rows {
        if !observed.insert(observed_row.family()) {
            return Err(ForgeQuerySupportPinningError::with_family(
                ForgeQuerySupportPinningErrorKind::DuplicateObservedFamily,
                "support pin observed family is declared more than once",
                observed_row.family().as_str(),
            ));
        }
        if required.contains(&observed_row.family()) {
            return Err(ForgeQuerySupportPinningError::with_family(
                ForgeQuerySupportPinningErrorKind::RequiredObservedFamilyConflict,
                "support pin family cannot be both required and observed",
                observed_row.family().as_str(),
            ));
        }
    }
    Ok(())
}
