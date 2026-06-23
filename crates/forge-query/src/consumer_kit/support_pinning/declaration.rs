use crate::runtime::ForgeQueryRuntimeFacadeFamily;
use std::collections::BTreeSet;

use super::super::support_snapshot::ForgeQuerySupportSnapshot;
use super::contract::ForgeQuerySupportPinContract;
use super::document::schema::{
    support_pin_vocabulary_identity, ForgeQuerySupportPinContractSchemaVersion,
};
use super::error::ForgeQuerySupportPinningError;
use super::error::ForgeQuerySupportPinningErrorKind;
use super::evidence::{
    derive_support_pin_contract_identity, derive_support_pin_observed_row_identity,
    derive_support_pin_requirement_identity,
};
use super::observed_row::ForgeQueryObservedSupportPin;
use super::requirement::{ForgeQuerySupportPinRequirement, ForgeQuerySupportPinRequirementDraft};
use super::snapshot_index::SupportPinSnapshotIndex;

pub fn support_pinning_contract(
    consumer_name: impl Into<String>,
) -> ForgeQuerySupportPinDeclaration {
    ForgeQuerySupportPinDeclaration {
        consumer_name: consumer_name.into(),
    }
}

#[derive(Debug)]
pub struct ForgeQuerySupportPinDeclaration {
    consumer_name: String,
}

impl ForgeQuerySupportPinDeclaration {
    pub fn against_snapshot(
        self,
        snapshot: &ForgeQuerySupportSnapshot,
    ) -> Result<ForgeQuerySupportPinContractBuilder<'_>, ForgeQuerySupportPinningError> {
        Ok(ForgeQuerySupportPinContractBuilder {
            consumer_name: self.consumer_name,
            contract_schema_version: ForgeQuerySupportPinContractSchemaVersion::current(),
            contract_schema_identity: ForgeQuerySupportPinContractSchemaVersion::current()
                .identity()
                .terminal_projection_for_reporting()
                .to_string(),
            pinned_vocabulary_identity: support_pin_vocabulary_identity()
                .terminal_projection_for_reporting()
                .to_string(),
            support_snapshot_schema_identity: snapshot.schema_identity().to_string(),
            source_matrix_digest: snapshot.source_matrix_digest().to_string(),
            basis_index: SupportPinSnapshotIndex::new(snapshot)?,
            requirements: Vec::new(),
            observed_rows: Vec::new(),
            required_families: BTreeSet::new(),
            observed_families: BTreeSet::new(),
        })
    }
}

#[derive(Debug)]
pub struct ForgeQuerySupportPinContractBuilder<'a> {
    consumer_name: String,
    contract_schema_version: ForgeQuerySupportPinContractSchemaVersion,
    contract_schema_identity: String,
    pinned_vocabulary_identity: String,
    support_snapshot_schema_identity: String,
    source_matrix_digest: String,
    basis_index: SupportPinSnapshotIndex<'a>,
    requirements: Vec<ForgeQuerySupportPinRequirement>,
    observed_rows: Vec<ForgeQueryObservedSupportPin>,
    required_families: BTreeSet<ForgeQueryRuntimeFacadeFamily>,
    observed_families: BTreeSet<ForgeQueryRuntimeFacadeFamily>,
}

impl ForgeQuerySupportPinContractBuilder<'_> {
    pub fn require_family(
        mut self,
        family: ForgeQueryRuntimeFacadeFamily,
        declare: impl FnOnce(
            ForgeQuerySupportPinRequirementDraft,
        ) -> ForgeQuerySupportPinRequirementDraft,
    ) -> Result<Self, ForgeQuerySupportPinningError> {
        self.reject_duplicate_required_family(family)?;
        let row = self.basis_index.required_row(family)?;
        let draft = declare(ForgeQuerySupportPinRequirementDraft::from_snapshot_row(
            family, row,
        ));
        self.requirements
            .push(ForgeQuerySupportPinRequirement::from_draft(draft)?);
        self.required_families.insert(family);
        Ok(self)
    }

    pub fn observe_family(
        mut self,
        family: ForgeQueryRuntimeFacadeFamily,
    ) -> Result<Self, ForgeQuerySupportPinningError> {
        self.reject_duplicate_observed_family(family)?;
        let observed = match self.basis_index.optional_row(family) {
            Some(row) => ForgeQueryObservedSupportPin::present(family, row),
            None => ForgeQueryObservedSupportPin::missing(family),
        };
        self.observed_rows.push(observed);
        self.observed_families.insert(family);
        Ok(self)
    }

    pub fn seal(self) -> Result<ForgeQuerySupportPinContract, ForgeQuerySupportPinningError> {
        let requirement_identities = self
            .requirements
            .iter()
            .map(derive_support_pin_requirement_identity)
            .collect::<Vec<_>>();
        let observed_identities = self
            .observed_rows
            .iter()
            .map(derive_support_pin_observed_row_identity)
            .collect::<Vec<_>>();
        let contract_digest = derive_support_pin_contract_identity(
            &self.consumer_name,
            &self.contract_schema_identity,
            &self.pinned_vocabulary_identity,
            &self.support_snapshot_schema_identity,
            &self.source_matrix_digest,
            &requirement_identities,
            &observed_identities,
        )
        .terminal_projection_for_reporting()
        .to_string();
        ForgeQuerySupportPinContract::new(
            self.consumer_name,
            self.contract_schema_version,
            self.contract_schema_identity,
            self.pinned_vocabulary_identity,
            self.support_snapshot_schema_identity,
            self.source_matrix_digest,
            self.requirements,
            self.observed_rows,
            contract_digest,
        )
    }

    fn reject_duplicate_required_family(
        &self,
        family: ForgeQueryRuntimeFacadeFamily,
    ) -> Result<(), ForgeQuerySupportPinningError> {
        if self.required_families.contains(&family) {
            return Err(ForgeQuerySupportPinningError::with_family(
                ForgeQuerySupportPinningErrorKind::DuplicateRequiredFamily,
                "support pin required family is declared more than once",
                family.as_str(),
            ));
        }
        if self.observed_families.contains(&family) {
            return Err(ForgeQuerySupportPinningError::with_family(
                ForgeQuerySupportPinningErrorKind::RequiredObservedFamilyConflict,
                "support pin family cannot be both required and observed",
                family.as_str(),
            ));
        }
        Ok(())
    }

    fn reject_duplicate_observed_family(
        &self,
        family: ForgeQueryRuntimeFacadeFamily,
    ) -> Result<(), ForgeQuerySupportPinningError> {
        if self.observed_families.contains(&family) {
            return Err(ForgeQuerySupportPinningError::with_family(
                ForgeQuerySupportPinningErrorKind::DuplicateObservedFamily,
                "support pin observed family is declared more than once",
                family.as_str(),
            ));
        }
        if self.required_families.contains(&family) {
            return Err(ForgeQuerySupportPinningError::with_family(
                ForgeQuerySupportPinningErrorKind::RequiredObservedFamilyConflict,
                "support pin family cannot be both required and observed",
                family.as_str(),
            ));
        }
        Ok(())
    }
}
