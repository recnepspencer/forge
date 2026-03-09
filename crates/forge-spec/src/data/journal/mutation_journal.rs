use serde::{Deserialize, Serialize};

use crate::data::identity::{NamingAnchorId, SpecNodeId, SpecRelationId};
use crate::data::schema::{RelationKind, SpecNodeKind};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MutationJournalEntry {
    NodeCreated {
        id: SpecNodeId,
        kind: SpecNodeKind,
    },
    NodeDeleted {
        id: SpecNodeId,
        kind: SpecNodeKind,
    },
    NodePayloadChanged {
        id: SpecNodeId,
    },
    RelationAdded {
        id: SpecRelationId,
        kind: RelationKind,
    },
    RelationRemoved {
        id: SpecRelationId,
        kind: RelationKind,
    },
    AnchorCreated {
        id: NamingAnchorId,
        target: SpecNodeId,
    },
    AnchorRetargeted {
        id: NamingAnchorId,
        old_target: SpecNodeId,
        new_target: SpecNodeId,
    },
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationJournal {
    entries: Vec<MutationJournalEntry>,
}

impl MutationJournal {
    pub fn record(&mut self, entry: MutationJournalEntry) {
        self.entries.push(entry);
    }

    pub fn entries(&self) -> &[MutationJournalEntry] {
        &self.entries
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
