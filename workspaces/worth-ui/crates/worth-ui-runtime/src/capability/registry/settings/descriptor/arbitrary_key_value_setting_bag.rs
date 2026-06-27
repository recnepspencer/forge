#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArbitraryKeyValueSettingBag {
    description: String,
}

impl ArbitraryKeyValueSettingBag {
    pub fn new(description: impl Into<String>) -> Self {
        Self {
            description: description.into(),
        }
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}
