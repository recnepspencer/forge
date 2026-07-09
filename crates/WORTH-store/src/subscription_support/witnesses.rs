use super::classification_error;
use crate::failure::StoreError;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportBasisWitness {
    pub(crate) stable_basis_digest: String,
}

impl SubscriptionSupportBasisWitness {
    pub(crate) fn new(stable_basis_digest: impl Into<String>) -> Result<Self, StoreError> {
        let stable_basis_digest = stable_basis_digest.into();
        if stable_basis_digest.trim().is_empty() {
            return Err(classification_error(
                "subscription-support basis witnesses require stable-basis truth",
            ));
        }
        Ok(Self {
            stable_basis_digest,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportCursorWitness {
    pub(crate) cursor_digest: String,
}

impl SubscriptionSupportCursorWitness {
    pub(crate) fn new(cursor_digest: impl Into<String>) -> Result<Self, StoreError> {
        let cursor_digest = cursor_digest.into();
        if cursor_digest.trim().is_empty() {
            return Err(classification_error(
                "subscription-support cursor witnesses require cursor truth",
            ));
        }
        Ok(Self { cursor_digest })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportCheckpointWitness {
    pub(crate) checkpoint_digest: String,
}

impl SubscriptionSupportCheckpointWitness {
    pub(crate) fn new(checkpoint_digest: impl Into<String>) -> Result<Self, StoreError> {
        let checkpoint_digest = checkpoint_digest.into();
        if checkpoint_digest.trim().is_empty() {
            return Err(classification_error(
                "subscription-support checkpoint witnesses require checkpoint truth",
            ));
        }
        Ok(Self { checkpoint_digest })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportSchemaWitness {
    pub(crate) schema_digest: String,
}

impl SubscriptionSupportSchemaWitness {
    pub(crate) fn new(schema_digest: impl Into<String>) -> Result<Self, StoreError> {
        let schema_digest = schema_digest.into();
        if schema_digest.trim().is_empty() {
            return Err(classification_error(
                "subscription-support schema witnesses require schema truth",
            ));
        }
        Ok(Self { schema_digest })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportCompatibilityWitness {
    pub(crate) compatibility_digest: String,
}

impl SubscriptionSupportCompatibilityWitness {
    pub(crate) fn new(compatibility_digest: impl Into<String>) -> Result<Self, StoreError> {
        let compatibility_digest = compatibility_digest.into();
        if compatibility_digest.trim().is_empty() {
            return Err(classification_error(
                "subscription-support compatibility witnesses require compatibility truth",
            ));
        }
        Ok(Self {
            compatibility_digest,
        })
    }
}
