use crate::aspects::structs::StructAspectValue;
use crate::values::AspectValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractValidationInput {
    Scalar(AspectValue),
    Struct(StructAspectValue),
}

impl From<AspectValue> for ContractValidationInput {
    fn from(value: AspectValue) -> Self {
        Self::Scalar(value)
    }
}

impl From<StructAspectValue> for ContractValidationInput {
    fn from(value: StructAspectValue) -> Self {
        Self::Struct(value)
    }
}
