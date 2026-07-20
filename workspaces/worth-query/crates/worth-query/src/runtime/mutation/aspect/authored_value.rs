use worth_foundational::facade::{AspectValue, ContractValidationInput, StructAspectValue};

/// Raw consumer-authored native value. This carrier adds no validation authority.
#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryAuthoredAspectValue {
    value: ContractValidationInput,
}

impl WorthQueryAuthoredAspectValue {
    pub fn native(value: AspectValue) -> Self {
        Self {
            value: value.into(),
        }
    }

    pub fn struct_value(value: StructAspectValue) -> Self {
        Self {
            value: value.into(),
        }
    }

    pub fn string(value: impl Into<String>) -> Self {
        Self::native(AspectValue::String(value.into().into()))
    }

    pub fn int64(value: i64) -> Self {
        Self::native(AspectValue::Int64(value))
    }

    pub fn bool(value: bool) -> Self {
        Self::native(AspectValue::Bool(value))
    }

    pub fn null() -> Self {
        Self::native(AspectValue::Null)
    }

    #[cfg(test)]
    pub(crate) fn from_foundational_value(value: AspectValue) -> Self {
        Self::native(value)
    }

    pub(crate) fn into_validation_input(self) -> ContractValidationInput {
        self.value
    }
}

impl From<AspectValue> for WorthQueryAuthoredAspectValue {
    fn from(value: AspectValue) -> Self {
        Self::native(value)
    }
}

impl From<StructAspectValue> for WorthQueryAuthoredAspectValue {
    fn from(value: StructAspectValue) -> Self {
        Self::struct_value(value)
    }
}

impl From<String> for WorthQueryAuthoredAspectValue {
    fn from(value: String) -> Self {
        Self::string(value)
    }
}

impl From<&str> for WorthQueryAuthoredAspectValue {
    fn from(value: &str) -> Self {
        Self::string(value)
    }
}

impl From<bool> for WorthQueryAuthoredAspectValue {
    fn from(value: bool) -> Self {
        Self::bool(value)
    }
}

impl From<i64> for WorthQueryAuthoredAspectValue {
    fn from(value: i64) -> Self {
        Self::int64(value)
    }
}
