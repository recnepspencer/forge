use serde::{Deserialize, Serialize};

use super::PortableAspectContractBasis;
use crate::aspects::ContractValidationInput;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortableRecordAspectState {
    entries: Vec<PortableRecordAspectStateEntry>,
}

impl PortableRecordAspectState {
    pub fn new(entries: impl IntoIterator<Item = PortableRecordAspectStateEntry>) -> Self {
        Self {
            entries: entries.into_iter().collect(),
        }
    }

    pub fn entries(&self) -> &[PortableRecordAspectStateEntry] {
        &self.entries
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub(crate) fn into_entries(self) -> Vec<PortableRecordAspectStateEntry> {
        self.entries
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortableRecordAspectStateEntry {
    basis: PortableAspectContractBasis,
    value: ContractValidationInput,
}

impl PortableRecordAspectStateEntry {
    pub fn new(basis: PortableAspectContractBasis, value: ContractValidationInput) -> Self {
        Self { basis, value }
    }

    pub fn basis(&self) -> &PortableAspectContractBasis {
        &self.basis
    }

    pub fn value(&self) -> &ContractValidationInput {
        &self.value
    }

    pub(crate) fn into_parts(self) -> (PortableAspectContractBasis, ContractValidationInput) {
        (self.basis, self.value)
    }
}
