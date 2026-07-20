use worth_foundational::facade::{prepare_aspect_value_identity_basis, AspectValue};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthQueryPortableConditionParameterValue {
    Bool(bool),
    U64(u64),
    I64(i64),
    Text(String),
    NativeValue(AspectValue),
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthQueryPortableConditionParameter {
    name: String,
    value: WorthQueryPortableConditionParameterValue,
}

impl WorthQueryPortableConditionParameter {
    pub fn bool(name: impl Into<String>, value: bool) -> Result<Self, &'static str> {
        Self::new(name, WorthQueryPortableConditionParameterValue::Bool(value))
    }

    pub fn u64(name: impl Into<String>, value: u64) -> Result<Self, &'static str> {
        Self::new(name, WorthQueryPortableConditionParameterValue::U64(value))
    }

    pub fn i64(name: impl Into<String>, value: i64) -> Result<Self, &'static str> {
        Self::new(name, WorthQueryPortableConditionParameterValue::I64(value))
    }

    pub fn text(name: impl Into<String>, value: impl Into<String>) -> Result<Self, &'static str> {
        Self::new(
            name,
            WorthQueryPortableConditionParameterValue::Text(value.into()),
        )
    }

    pub fn native_value(name: impl Into<String>, value: AspectValue) -> Result<Self, &'static str> {
        Self::new(
            name,
            WorthQueryPortableConditionParameterValue::NativeValue(value),
        )
    }

    fn new(
        name: impl Into<String>,
        value: WorthQueryPortableConditionParameterValue,
    ) -> Result<Self, &'static str> {
        let name = name.into();
        if name.is_empty() || name.trim() != name || name.chars().any(char::is_whitespace) {
            return Err("invalid-portable-condition-parameter-name");
        }
        Ok(Self { name, value })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &WorthQueryPortableConditionParameterValue {
        &self.value
    }
}

pub(crate) fn parameter_token(parameter: &WorthQueryPortableConditionParameter) -> String {
    let value = match parameter.value() {
        WorthQueryPortableConditionParameterValue::Bool(value) => format!("bool:{value}"),
        WorthQueryPortableConditionParameterValue::U64(value) => format!("u64:{value}"),
        WorthQueryPortableConditionParameterValue::I64(value) => format!("i64:{value}"),
        WorthQueryPortableConditionParameterValue::Text(value) => {
            format!("text#{}:{value}", value.len())
        }
        WorthQueryPortableConditionParameterValue::NativeValue(value) => format!(
            "native#{}",
            prepare_aspect_value_identity_basis(value).as_str()
        ),
    };
    format!(
        "name#{}:{};{value}",
        parameter.name().len(),
        parameter.name()
    )
}
