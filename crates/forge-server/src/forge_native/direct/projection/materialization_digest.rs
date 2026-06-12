#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerDirectMaterializationDigest(String);

impl ForgeServerDirectMaterializationDigest {
    pub(crate) fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
