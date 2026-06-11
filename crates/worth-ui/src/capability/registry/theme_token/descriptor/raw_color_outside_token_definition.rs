#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RawColorOutsideTokenDefinition {
    value: String,
}

impl RawColorOutsideTokenDefinition {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
