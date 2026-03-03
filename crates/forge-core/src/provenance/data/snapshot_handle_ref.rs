//! Snapshot-scoped handle identity.

use serde::{Deserialize, Serialize};

use crate::tracing::EntityKind;

/// Serializable snapshot-scoped handle identity.
///
/// This is explicitly not a persistent identity. It captures a typed
/// `(kind, index, generation)` reference valid within a topology snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SnapshotHandleRef {
    pub kind: EntityKind,
    pub index: u32,
    pub generation: u32,
}

impl SnapshotHandleRef {
    pub fn new(kind: EntityKind, index: u32, generation: u32) -> Self {
        Self {
            kind,
            index,
            generation,
        }
    }

    pub fn packed_generational(self) -> u64 {
        ((self.generation as u64) << 32) | (self.index as u64)
    }
}
