#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RawIconAssetReference {
    value: String,
}

impl RawIconAssetReference {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
