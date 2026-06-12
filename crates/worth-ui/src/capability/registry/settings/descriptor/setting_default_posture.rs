use super::SettingDefaultValue;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SettingDefaultPosture {
    SchemaDefault(SettingDefaultValue),
    RuntimeComputed,
}

impl SettingDefaultPosture {
    pub fn schema_default(value: SettingDefaultValue) -> Self {
        Self::SchemaDefault(value)
    }

    pub fn runtime_computed() -> Self {
        Self::RuntimeComputed
    }

    pub(crate) fn default_value(&self) -> Option<&SettingDefaultValue> {
        match self {
            Self::SchemaDefault(value) => Some(value),
            Self::RuntimeComputed => None,
        }
    }

    pub(crate) fn digest_basis(&self) -> String {
        match self {
            Self::SchemaDefault(value) => {
                format!("schema_default:{}", value.digest_basis())
            }
            Self::RuntimeComputed => "runtime_computed".to_string(),
        }
    }
}
