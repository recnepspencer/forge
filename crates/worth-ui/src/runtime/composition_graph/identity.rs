#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthUiCompositionNodeId(String);

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthUiCompositionRootId(String);

impl WorthUiCompositionNodeId {
    pub fn new(identity: impl Into<String>) -> Result<Self, String> {
        validated_identity(identity.into()).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl WorthUiCompositionRootId {
    pub fn new(identity: impl Into<String>) -> Result<Self, String> {
        validated_identity(identity.into()).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<WorthUiCompositionNodeId> for String {
    fn from(value: WorthUiCompositionNodeId) -> Self {
        value.0
    }
}

impl From<WorthUiCompositionRootId> for String {
    fn from(value: WorthUiCompositionRootId) -> Self {
        value.0
    }
}

fn validated_identity(identity: String) -> Result<String, String> {
    let trimmed = identity.trim();
    if trimmed.is_empty() {
        Err("composition identity must not be empty".to_owned())
    } else {
        Ok(trimmed.to_owned())
    }
}
