use worth_foundational::facade::{
    prepare_aspect_value_identity_basis, prepare_struct_aspect_value_identity_basis, AspectValue,
    ContractValidationInput, StructAspectValue,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryDesiredAspectOperation {
    Set,
    Clear,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDesiredAspectValue {
    operation: WorthQueryDesiredAspectOperation,
    value: Option<ContractValidationInput>,
}

impl WorthQueryDesiredAspectValue {
    pub(crate) fn set_native(value: ContractValidationInput) -> Self {
        Self {
            operation: WorthQueryDesiredAspectOperation::Set,
            value: Some(value),
        }
    }

    pub(crate) fn clear() -> Self {
        Self {
            operation: WorthQueryDesiredAspectOperation::Clear,
            value: None,
        }
    }

    pub fn value(&self) -> Option<&AspectValue> {
        match self.value.as_ref() {
            Some(ContractValidationInput::Scalar(value)) => Some(value),
            Some(ContractValidationInput::Struct(_)) | None => None,
        }
    }

    pub fn struct_value(&self) -> Option<&StructAspectValue> {
        match self.value.as_ref() {
            Some(ContractValidationInput::Struct(value)) => Some(value),
            Some(ContractValidationInput::Scalar(_)) | None => None,
        }
    }

    pub(crate) fn validation_input(&self) -> Option<&ContractValidationInput> {
        self.value.as_ref()
    }

    pub fn clears_existing_value(&self) -> bool {
        self.operation == WorthQueryDesiredAspectOperation::Clear
    }

    pub(crate) fn terminal_digest_material(&self) -> String {
        match (self.operation, self.value.as_ref()) {
            (WorthQueryDesiredAspectOperation::Clear, _) => "clear".to_string(),
            (
                WorthQueryDesiredAspectOperation::Set,
                Some(ContractValidationInput::Scalar(value)),
            ) => format!(
                "set:{}",
                prepare_aspect_value_identity_basis(value).as_str()
            ),
            (
                WorthQueryDesiredAspectOperation::Set,
                Some(ContractValidationInput::Struct(value)),
            ) => format!(
                "set:{}",
                prepare_struct_aspect_value_identity_basis(value).as_str()
            ),
            (WorthQueryDesiredAspectOperation::Set, None) => "set:<missing>".to_string(),
        }
    }
}
