use crate::data::error::SpecError;
use crate::data::graph::{NodeRecord, RelationRecord};
use crate::data::identity::{NamingAnchorId, SpecNodeId, SpecRelationId};
use crate::data::journal::MutationJournalEntry;
use crate::data::lineage::LineageRecord;
use crate::data::naming::NamingAnchor;
use crate::data::payload::{PayloadKey, PayloadRecord};
use crate::data::replay::SpecReplayRecord;
use crate::data::schema::{RelationKind, SpecNodeKind};

use super::SpecDraft;

impl SpecDraft {
    pub fn create_node(
        &mut self,
        kind: SpecNodeKind,
        payload_bytes: Option<Vec<u8>>,
        role: &str,
    ) -> Result<SpecNodeId, SpecError> {
        self.ensure_open()?;
        let payload = payload_bytes.map(|bytes| self.insert_payload(bytes));
        let id = self.allocator.mint_node_id(kind, role);
        let node = NodeRecord { id, kind, payload };
        self.created_nodes.insert(id, node.clone());
        self.deleted_nodes.remove(&id);
        self.journal
            .record(MutationJournalEntry::NodeCreated { id, kind });
        Ok(id)
    }

    pub fn delete_node(&mut self, id: SpecNodeId) -> Result<(), SpecError> {
        self.ensure_open()?;
        let kind = self.current_node(id)?.kind;
        if self.current_outbound_count(id) > 0 || self.current_inbound_count(id) > 0 {
            return Err(SpecError::invalid(format!(
                "cannot delete node {} while relations still reference it",
                id
            )));
        }
        self.created_nodes.remove(&id);
        self.deleted_nodes.insert(id);
        self.journal
            .record(MutationJournalEntry::NodeDeleted { id, kind });
        Ok(())
    }

    pub fn add_relation(
        &mut self,
        kind: RelationKind,
        source: SpecNodeId,
        target: SpecNodeId,
        ordinal: u32,
        role: &str,
    ) -> Result<SpecRelationId, SpecError> {
        self.ensure_open()?;
        let source_kind = self.current_node(source)?.kind;
        let target_kind = self.current_node(target)?.kind;
        self.validate_relation_preconditions(kind, source, target, source_kind, target_kind, ordinal)?;
        let id = self.allocator.mint_relation_id(kind, role);
        let relation = RelationRecord {
            id,
            kind,
            source,
            target,
            ordinal,
        };
        self.created_relations.insert(id, relation.clone());
        self.deleted_relations.remove(&id);
        self.journal
            .record(MutationJournalEntry::RelationAdded { id, kind });
        Ok(id)
    }

    pub fn replace_single_relation(
        &mut self,
        kind: RelationKind,
        source: SpecNodeId,
        target: SpecNodeId,
        role: &str,
    ) -> Result<SpecRelationId, SpecError> {
        self.ensure_open()?;
        let relations = self.current_outgoing_of_kind(source, kind);
        if relations.len() != 1 {
            return Err(SpecError::invalid(format!(
                "expected exactly one outgoing {:?} relation for node {}",
                kind, source
            )));
        }
        let current_id = relations[0].id;
        self.remove_relation(current_id)?;
        self.add_relation(kind, source, target, 0, role)
    }

    pub fn remove_single_outgoing_relation(
        &mut self,
        kind: RelationKind,
        source: SpecNodeId,
    ) -> Result<SpecNodeId, SpecError> {
        self.ensure_open()?;
        let relations = self.current_outgoing_of_kind(source, kind);
        if relations.len() != 1 {
            return Err(SpecError::invalid(format!(
                "expected exactly one outgoing {:?} relation for node {}",
                kind, source
            )));
        }
        let relation = relations[0].clone();
        self.remove_relation(relation.id)?;
        Ok(relation.target)
    }

    pub fn remove_relation_between(
        &mut self,
        kind: RelationKind,
        source: SpecNodeId,
        target: SpecNodeId,
    ) -> Result<(), SpecError> {
        self.ensure_open()?;
        let relations = self.current_outgoing_of_kind(source, kind);
        let matching = relations
            .into_iter()
            .filter(|relation| relation.target == target)
            .collect::<Vec<_>>();
        if matching.len() != 1 {
            return Err(SpecError::invalid(format!(
                "expected exactly one {:?} relation from node {} to node {}",
                kind, source, target
            )));
        }
        self.remove_relation(matching[0].id)
    }

    pub fn remove_relation(&mut self, id: SpecRelationId) -> Result<(), SpecError> {
        self.ensure_open()?;
        let relation = self.current_relation(id)?.clone();
        self.created_relations.remove(&id);
        self.deleted_relations.insert(id);
        self.journal.record(MutationJournalEntry::RelationRemoved {
            id,
            kind: relation.kind,
        });
        Ok(())
    }

    pub fn create_naming_anchor(
        &mut self,
        target: SpecNodeId,
        target_kind: SpecNodeKind,
        semantic_role: impl Into<String>,
        ordinal: u32,
        origin_feature: Option<SpecNodeId>,
        origin_operation: u64,
    ) -> Result<NamingAnchorId, SpecError> {
        self.ensure_open()?;
        let semantic_role = semantic_role.into();
        let id = self.allocator.mint_anchor_id(&semantic_role);
        let anchor = NamingAnchor {
            id,
            target,
            target_kind,
            semantic_role,
            ordinal,
            origin_feature,
            origin_operation,
            retarget_history: Vec::new(),
        };
        self.created_anchors.insert(id, anchor.clone());
        self.journal
            .record(MutationJournalEntry::AnchorCreated { id, target });
        Ok(id)
    }

    pub fn record_lineage(&mut self, record: LineageRecord) -> Result<(), SpecError> {
        self.ensure_open()?;
        self.lineage_records.push(record);
        Ok(())
    }

    pub fn record_replay(&mut self, record: SpecReplayRecord) -> Result<(), SpecError> {
        self.ensure_open()?;
        self.replay_records.push(record);
        Ok(())
    }

    pub fn next_operation_id(&mut self) -> Result<u64, SpecError> {
        self.ensure_open()?;
        let current = self.next_operation_id;
        self.next_operation_id += 1;
        Ok(current)
    }

    fn insert_payload(&mut self, bytes: Vec<u8>) -> PayloadKey {
        let key = PayloadKey::new(
            self.created_payloads.len() as u64 + self.base.payloads().records().len() as u64,
        );
        self.created_payloads.push(PayloadRecord { key, bytes });
        key
    }
}
