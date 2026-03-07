use std::collections::{BTreeMap, BTreeSet};

use crate::data::error::SpecError;
use crate::data::graph::{NodeRecord, RelationRecord};
use crate::data::identity::{DeterministicIdAllocator, NamingAnchorId, SpecNodeId, SpecRelationId};
use crate::data::journal::MutationJournal;
use crate::data::lineage::LineageRecord;
use crate::data::naming::NamingAnchor;
use crate::data::payload::PayloadRecord;
use crate::data::replay::SpecReplayRecord;
use crate::data::snapshot::SpecState;

mod commands;
mod finalize;
mod queries;
mod validation;

pub struct SpecDraft {
    pub(super) base: SpecState,
    pub(super) allocator: DeterministicIdAllocator,
    pub(super) created_nodes: BTreeMap<SpecNodeId, NodeRecord>,
    pub(super) deleted_nodes: BTreeSet<SpecNodeId>,
    pub(super) created_relations: BTreeMap<SpecRelationId, RelationRecord>,
    pub(super) deleted_relations: BTreeSet<SpecRelationId>,
    pub(super) created_payloads: Vec<PayloadRecord>,
    pub(super) created_anchors: BTreeMap<NamingAnchorId, NamingAnchor>,
    pub(super) lineage_records: Vec<LineageRecord>,
    pub(super) replay_records: Vec<SpecReplayRecord>,
    pub(super) journal: MutationJournal,
    pub(super) next_operation_id: u64,
    pub(super) finished: bool,
}

impl SpecDraft {
    pub(crate) fn new(base: SpecState, allocator: DeterministicIdAllocator) -> Self {
        let next_operation_id = base.next_operation_id();
        Self {
            base,
            allocator,
            created_nodes: BTreeMap::new(),
            deleted_nodes: BTreeSet::new(),
            created_relations: BTreeMap::new(),
            deleted_relations: BTreeSet::new(),
            created_payloads: Vec::new(),
            created_anchors: BTreeMap::new(),
            lineage_records: Vec::new(),
            replay_records: Vec::new(),
            journal: MutationJournal::default(),
            next_operation_id,
            finished: false,
        }
    }

    pub(super) fn ensure_open(&self) -> Result<(), SpecError> {
        if self.finished {
            return Err(SpecError::TransactionFinished);
        }
        Ok(())
    }
}
