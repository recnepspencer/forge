use serde::{Deserialize, Serialize};

use super::PortableAspectContractBasis;
use crate::aspects::{ContractValidationInput, FieldKey};
use crate::values::AspectValue;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortableRecordAspectPatch {
    operations: Vec<PortableAspectPatchOperation>,
}

impl PortableRecordAspectPatch {
    pub fn new(operations: impl IntoIterator<Item = PortableAspectPatchOperation>) -> Self {
        Self {
            operations: operations.into_iter().collect(),
        }
    }

    pub fn operations(&self) -> &[PortableAspectPatchOperation] {
        &self.operations
    }

    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }

    pub(crate) fn into_operations(self) -> Vec<PortableAspectPatchOperation> {
        self.operations
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PortableAspectPatchOperation {
    SetWhole {
        basis: PortableAspectContractBasis,
        value: ContractValidationInput,
    },
    ClearWhole {
        basis: PortableAspectContractBasis,
    },
    PatchFields {
        basis: PortableAspectContractBasis,
        selected_fields: Vec<FieldKey>,
        field_sets: Vec<PortableAspectFieldSet>,
        field_clears: Vec<FieldKey>,
    },
}

impl PortableAspectPatchOperation {
    pub fn basis(&self) -> &PortableAspectContractBasis {
        match self {
            Self::SetWhole { basis, .. }
            | Self::ClearWhole { basis }
            | Self::PatchFields { basis, .. } => basis,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortableAspectFieldSet {
    field: FieldKey,
    value: AspectValue,
}

impl PortableAspectFieldSet {
    pub fn new(field: FieldKey, value: AspectValue) -> Self {
        Self { field, value }
    }

    pub fn field(&self) -> &FieldKey {
        &self.field
    }

    pub fn value(&self) -> &AspectValue {
        &self.value
    }

    pub(crate) fn into_parts(self) -> (FieldKey, AspectValue) {
        (self.field, self.value)
    }
}
