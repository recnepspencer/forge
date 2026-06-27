use super::ThemeColorValue;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ThemeTokenValue {
    Color(ThemeColorValue),
}

impl ThemeTokenValue {
    pub fn color(value: ThemeColorValue) -> Self {
        Self::Color(value)
    }

    pub(crate) fn is_valid(&self) -> bool {
        match self {
            Self::Color(value) => value.is_valid(),
        }
    }

    pub(crate) fn digest_basis(&self) -> String {
        match self {
            Self::Color(value) => format!("color({})", value.digest_basis()),
        }
    }
}
