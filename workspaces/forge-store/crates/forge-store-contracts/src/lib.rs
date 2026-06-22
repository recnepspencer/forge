#![forbid(unsafe_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DurableArtifactClass {
    Authoritative,
    DerivedDurable,
    Ephemeral,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DerivedAccuracyClass {
    Exact,
    Conservative,
    Approximate,
    Heuristic,
    Advisory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RoadmapScope {
    pub roadmap: &'static str,
    pub sequence: &'static str,
}

impl RoadmapScope {
    pub const fn new(roadmap: &'static str, sequence: &'static str) -> Self {
        Self { roadmap, sequence }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StableArtifactId(String);

impl StableArtifactId {
    pub fn new(value: impl Into<String>) -> Result<Self, StoreContractError> {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StableDigest(String);

impl StableDigest {
    pub fn new(value: impl Into<String>) -> Result<Self, StoreContractError> {
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

pub type StoreContractResult<T> = Result<T, StoreContractError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreContractError {
    EmptyStableId,
    EmptyDigest,
    EmptyRequiredField,
    UnsupportedRoadmapClaim,
}
