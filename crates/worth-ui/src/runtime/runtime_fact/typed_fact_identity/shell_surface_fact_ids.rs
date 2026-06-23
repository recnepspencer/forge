use super::authoring_fact_ids::{RuntimeFactIdentityText, WorthUiRuntimeFactIdentityError};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthUiShellSurfaceId {
    identity: RuntimeFactIdentityText,
}

impl WorthUiShellSurfaceId {
    pub fn new(raw_identity: impl AsRef<str>) -> Result<Self, WorthUiRuntimeFactIdentityError> {
        Ok(Self {
            identity: RuntimeFactIdentityText::new(raw_identity.as_ref())?,
        })
    }

    pub fn as_str(&self) -> &str {
        self.identity.as_str()
    }
}
