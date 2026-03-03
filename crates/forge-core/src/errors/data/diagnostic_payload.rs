//! Diagnostic replay payload type.

use serde::{Deserialize, Serialize};

/// Structured diagnostic context for replay and debugging.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticPayload {
    /// The operation that was executing when the failure occurred.
    pub operation: String,
    /// The topology state hash at the time of failure.
    pub state_hash: u128,
    /// The RNG seed at the time of failure.
    pub seed: u64,
    /// Additional human-readable context.
    pub context: String,
}
