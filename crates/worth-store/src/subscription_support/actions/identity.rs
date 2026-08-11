use super::super::classification_error;
use crate::failure::StoreError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SupportActionId(String);

impl SupportActionId {
    pub fn new(value: impl Into<String>) -> Result<Self, StoreError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(classification_error(
                "subscription-support action ids must be non-empty",
            ));
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SupportActionRecoveryDisposition {
    NotInterrupted,
    InterruptedBeforePublication,
    PublishedConsequenceRecovered,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SupportActionPublicationState {
    PendingPublication,
    InterruptedBeforePublication,
    PublishedConsequence,
}
