//! Serializable summary for `DiagnosticPayload`.

use serde::{Deserialize, Serialize};

use crate::errors::data::DiagnosticPayload;

/// Serializable summary of diagnostic replay payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticPayloadSummary {
    pub operation: String,
    pub state_hash: u128,
    pub seed: u64,
    pub context: String,
}

impl From<&DiagnosticPayload> for DiagnosticPayloadSummary {
    fn from(value: &DiagnosticPayload) -> Self {
        Self {
            operation: value.operation.clone(),
            state_hash: value.state_hash,
            seed: value.seed,
            context: value.context.clone(),
        }
    }
}
