//! Invariant validation errors for provenance payloads.

use serde::{Deserialize, Serialize};

use crate::tracing::EntityKind;

/// Invariant validation errors for serialized provenance payloads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProvenanceValidationError {
    InvalidSegmentEndpointKind {
        field: &'static str,
        kind: EntityKind,
    },
    InvalidSourceKind {
        field: &'static str,
        expected: EntityKind,
        actual: EntityKind,
    },
    TransportHashMismatch {
        expected: u64,
        actual: u64,
        directed: bool,
    },
}
