use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FoundationalBranchIdConstructionDenial {
    EmptyName,
}

/// The stable descriptive identity of one mutable branch reference.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct FoundationalBranchId(String);

impl FoundationalBranchId {
    pub fn new(name: impl Into<String>) -> Result<Self, FoundationalBranchIdConstructionDenial> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(FoundationalBranchIdConstructionDenial::EmptyName);
        }

        Ok(Self(name))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for FoundationalBranchId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let name = String::deserialize(deserializer)?;
        Self::new(name).map_err(|denial| D::Error::custom(format!("invalid branch id: {denial:?}")))
    }
}
