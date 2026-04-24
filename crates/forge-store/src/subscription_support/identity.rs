use super::admission_error;
use crate::failure::StoreError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SubscriptionSupportFamilyKind {
    BasisBoundContinuationSupport,
    MaterializedNarrowingSupport,
    DegradedContinuationSupport,
    ExtensionDefinedSupport,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubscriptionSupportRole {
    ExactContinuation,
    NarrowingMaterialization,
    DegradedContinuation,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SubscriptionSupportFamilyId(pub(crate) String);

impl SubscriptionSupportFamilyId {
    pub fn new(value: impl Into<String>) -> Result<Self, StoreError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(admission_error(
                "subscription-support family ids must be non-empty",
            ));
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SubscriptionSupportArtifactId(pub(crate) String);

impl SubscriptionSupportArtifactId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportDeclarationDigest(pub(crate) String);

impl SubscriptionSupportDeclarationDigest {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
