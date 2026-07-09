#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerDirectMaterializationDigest(String);

impl WorthServerDirectMaterializationDigest {
    pub(crate) fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
