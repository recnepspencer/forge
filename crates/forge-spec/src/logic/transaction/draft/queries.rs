use crate::data::error::SpecError;
use crate::data::graph::{NodeRecord, RelationRecord};
use crate::data::identity::{SpecNodeId, SpecRelationId};
use crate::data::payload::{PayloadRecord, ShellPayload, SpecShellKind};
use crate::data::schema::{RelationKind, SpecNodeKind};

use super::SpecDraft;

impl SpecDraft {
    pub fn outgoing_targets_of_kind(
        &self,
        source: SpecNodeId,
        kind: RelationKind,
    ) -> Vec<SpecNodeId> {
        self.current_outgoing_of_kind(source, kind)
            .into_iter()
            .map(|relation| relation.target)
            .collect()
    }

    pub fn incoming_sources_of_kind(
        &self,
        target: SpecNodeId,
        kind: RelationKind,
    ) -> Vec<SpecNodeId> {
        self.current_incoming_of_kind(target, kind)
            .into_iter()
            .map(|relation| relation.source)
            .collect()
    }

    pub fn outgoing_relations(&self, source: SpecNodeId) -> Vec<RelationRecord> {
        let mut relations: Vec<RelationRecord> = self
            .base
            .graph()
            .outgoing_relations(source)
            .into_iter()
            .filter(|relation| !self.deleted_relations.contains(&relation.id))
            .cloned()
            .collect();
        relations.extend(
            self.created_relations
                .values()
                .filter(|relation| relation.source == source)
                .cloned(),
        );
        relations.sort_by_key(|relation| {
            (
                relation.kind,
                relation.ordinal,
                relation.source,
                relation.target,
                relation.id,
            )
        });
        relations
    }

    pub fn single_outgoing_target(
        &self,
        source: SpecNodeId,
        kind: RelationKind,
    ) -> Result<SpecNodeId, SpecError> {
        let mut relations = self.current_outgoing_of_kind(source, kind);
        match relations.len() {
            1 => Ok(relations.pop().unwrap().target),
            0 => Err(SpecError::not_found(format!(
                "missing outgoing {:?} relation for node {}",
                kind, source
            ))),
            _ => Err(SpecError::invalid(format!(
                "expected exactly one outgoing {:?} relation for node {}",
                kind, source
            ))),
        }
    }

    pub fn single_incoming_source(
        &self,
        target: SpecNodeId,
        kind: RelationKind,
    ) -> Result<SpecNodeId, SpecError> {
        let mut relations = self.current_incoming_of_kind(target, kind);
        match relations.len() {
            1 => Ok(relations.pop().unwrap().source),
            0 => Err(SpecError::not_found(format!(
                "missing incoming {:?} relation for node {}",
                kind, target
            ))),
            _ => Err(SpecError::invalid(format!(
                "expected exactly one incoming {:?} relation for node {}",
                kind, target
            ))),
        }
    }

    pub fn node_kind(&self, id: SpecNodeId) -> Result<SpecNodeKind, SpecError> {
        Ok(self.current_node(id)?.kind)
    }

    pub fn shell_kind(&self, id: SpecNodeId) -> Result<SpecShellKind, SpecError> {
        let node = self.current_node(id)?;
        if node.kind != SpecNodeKind::Shell {
            return Err(SpecError::invalid(format!(
                "node {} is not a shell; found {:?}",
                id, node.kind
            )));
        }
        let payload = self.payload_record(
            node.payload
                .ok_or_else(|| SpecError::not_found(format!("shell {} has no payload", id)))?,
        )?;
        Ok(ShellPayload::decode(&payload.bytes)?.kind())
    }

    pub(super) fn current_node(&self, id: SpecNodeId) -> Result<&NodeRecord, SpecError> {
        if self.deleted_nodes.contains(&id) {
            return Err(SpecError::not_found(format!(
                "node {} is deleted in draft",
                id
            )));
        }
        self.created_nodes
            .get(&id)
            .or_else(|| self.base.graph().node(id))
            .ok_or_else(|| SpecError::not_found(format!("node {} not found", id)))
    }

    pub(super) fn current_relation(
        &self,
        id: SpecRelationId,
    ) -> Result<&RelationRecord, SpecError> {
        if self.deleted_relations.contains(&id) {
            return Err(SpecError::not_found(format!(
                "relation {} is deleted in draft",
                id
            )));
        }
        self.created_relations
            .get(&id)
            .or_else(|| self.base.graph().relation(id))
            .ok_or_else(|| SpecError::not_found(format!("relation {} not found", id)))
    }

    pub(super) fn current_outbound_count(&self, node: SpecNodeId) -> usize {
        let mut count = self
            .base
            .graph()
            .outgoing_relations(node)
            .into_iter()
            .filter(|relation| !self.deleted_relations.contains(&relation.id))
            .count();
        count += self
            .created_relations
            .values()
            .filter(|relation| relation.source == node)
            .count();
        count
    }

    pub(super) fn current_inbound_count(&self, node: SpecNodeId) -> usize {
        let mut count = self
            .base
            .graph()
            .incoming_relations(node)
            .into_iter()
            .filter(|relation| !self.deleted_relations.contains(&relation.id))
            .count();
        count += self
            .created_relations
            .values()
            .filter(|relation| relation.target == node)
            .count();
        count
    }

    pub(crate) fn current_node_count(&self) -> usize {
        let base_count = self
            .base
            .graph()
            .iter_nodes()
            .filter(|node| !self.deleted_nodes.contains(&node.id))
            .count();
        base_count + self.created_nodes.len()
    }

    pub(super) fn current_outgoing_of_kind(
        &self,
        source: SpecNodeId,
        kind: RelationKind,
    ) -> Vec<RelationRecord> {
        let mut relations: Vec<RelationRecord> = self
            .base
            .graph()
            .outgoing_of_kind(source, kind)
            .into_iter()
            .filter(|relation| !self.deleted_relations.contains(&relation.id))
            .cloned()
            .collect();
        relations.extend(
            self.created_relations
                .values()
                .filter(|relation| relation.source == source && relation.kind == kind)
                .cloned(),
        );
        relations.sort_by_key(|relation| (relation.ordinal, relation.id));
        relations
    }

    pub(super) fn current_incoming_of_kind(
        &self,
        target: SpecNodeId,
        kind: RelationKind,
    ) -> Vec<RelationRecord> {
        let mut relations: Vec<RelationRecord> = self
            .base
            .graph()
            .incoming_relations(target)
            .into_iter()
            .filter(|relation| {
                relation.kind == kind && !self.deleted_relations.contains(&relation.id)
            })
            .cloned()
            .collect();
        relations.extend(
            self.created_relations
                .values()
                .filter(|relation| relation.target == target && relation.kind == kind)
                .cloned(),
        );
        relations.sort_by_key(|relation| (relation.ordinal, relation.id));
        relations
    }

    fn payload_record(
        &self,
        key: crate::data::payload::PayloadKey,
    ) -> Result<&PayloadRecord, SpecError> {
        self.created_payloads
            .iter()
            .find(|record| record.key == key)
            .or_else(|| self.base.payloads().get(key))
            .ok_or_else(|| SpecError::not_found(format!("payload {} not found", key)))
    }
}
