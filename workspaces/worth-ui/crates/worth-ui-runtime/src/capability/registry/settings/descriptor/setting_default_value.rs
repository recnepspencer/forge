#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SettingDefaultValue {
    Boolean(bool),
    Integer(i64),
    Decimal(String),
    Text(String),
    Enumeration(String),
}

impl SettingDefaultValue {
    pub fn boolean(value: bool) -> Self {
        Self::Boolean(value)
    }

    pub fn integer(value: i64) -> Self {
        Self::Integer(value)
    }

    pub fn decimal(value: impl Into<String>) -> Self {
        Self::Decimal(value.into())
    }

    pub fn text(value: impl Into<String>) -> Self {
        Self::Text(value.into())
    }

    pub fn enumeration(value: impl Into<String>) -> Self {
        Self::Enumeration(value.into())
    }

    pub(crate) fn digest_basis(&self) -> String {
        match self {
            Self::Boolean(value) => format!("boolean:{value}"),
            Self::Integer(value) => format!("integer:{value}"),
            Self::Decimal(value) => format!("decimal:{}", length_prefixed(value)),
            Self::Text(value) => format!("text:{}", length_prefixed(value)),
            Self::Enumeration(value) => format!("enum:{}", length_prefixed(value)),
        }
    }
}

fn length_prefixed(value: &str) -> String {
    format!("{}:{value}", value.len())
}
