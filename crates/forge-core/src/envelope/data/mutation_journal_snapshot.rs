//! Immutable per-operation mutation snapshot.

use serde::{Deserialize, Serialize};

use crate::EntityRef;

/// Frozen mutation journal emitted by every executed operation.
///
/// Entity lists are always canonicalized to `(kind, index, generation)` order.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MutationJournalSnapshot {
    /// Snapshot schema version for forward compatibility.
    #[serde(default = "MutationJournalSnapshot::default_schema_version")]
    pub schema_version: u32,
    /// Entities created during the operation.
    pub created: Vec<EntityRef>,
    /// Entities destroyed during the operation.
    pub destroyed: Vec<EntityRef>,
}

impl MutationJournalSnapshot {
    /// Current snapshot encoding schema version.
    pub const SCHEMA_VERSION: u32 = 1;

    fn default_schema_version() -> u32 {
        Self::SCHEMA_VERSION
    }

    /// Empty snapshot for no-op operations.
    pub fn empty() -> Self {
        Self {
            schema_version: Self::SCHEMA_VERSION,
            created: Vec::new(),
            destroyed: Vec::new(),
        }
    }

    /// Canonicalize both vectors in-place.
    pub fn canonicalize(&mut self) {
        self.created.sort_unstable_by_key(entity_sort_key);
        self.destroyed.sort_unstable_by_key(entity_sort_key);
    }

    /// Merge another snapshot into this one and preserve canonical ordering.
    pub fn absorb(&mut self, mut other: MutationJournalSnapshot) {
        self.created.append(&mut other.created);
        self.destroyed.append(&mut other.destroyed);
        self.canonicalize();
    }
}

fn entity_sort_key(entity: &EntityRef) -> (u8, u32, u32) {
    (entity.kind() as u8, entity.index(), entity.generation())
}
