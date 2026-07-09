use crate::{StoreContractError, StoreContractResult};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StableArtifactId(String);

impl StableArtifactId {
    pub fn new(value: impl Into<String>) -> StoreContractResult<Self> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(StoreContractError::EmptyStableId);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StableDigest(String);

impl StableDigest {
    pub fn new(value: impl Into<String>) -> StoreContractResult<Self> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(StoreContractError::EmptyDigest);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
