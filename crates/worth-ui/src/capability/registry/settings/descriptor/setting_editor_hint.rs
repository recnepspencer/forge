#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SettingEditorHint {
    Toggle,
    TextInput,
    NumberInput,
    Select,
    Hidden,
    Custom(String),
}

impl SettingEditorHint {
    pub fn toggle() -> Self {
        Self::Toggle
    }

    pub fn text_input() -> Self {
        Self::TextInput
    }

    pub fn number_input() -> Self {
        Self::NumberInput
    }

    pub fn select() -> Self {
        Self::Select
    }

    pub fn hidden() -> Self {
        Self::Hidden
    }

    pub fn custom(value: impl Into<String>) -> Self {
        Self::Custom(value.into())
    }

    pub(crate) fn digest_basis(&self) -> String {
        match self {
            Self::Toggle => "toggle".to_string(),
            Self::TextInput => "text_input".to_string(),
            Self::NumberInput => "number_input".to_string(),
            Self::Select => "select".to_string(),
            Self::Hidden => "hidden".to_string(),
            Self::Custom(value) => format!("custom:{}:{value}", value.len()),
        }
    }
}
