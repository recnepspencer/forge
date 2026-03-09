use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap};

use crate::data::error::SpecError;
use crate::data::graph::{NodeRecord, RelationRecord};
use crate::data::identity::{SpecNodeId, SpecRelationId};
use crate::data::schema::{RelationKind, SpecNodeKind};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecGraph {
    nodes: Vec<NodeRecord>,
    relations: Vec<RelationRecord>,
    #[serde(skip)]
    node_index: HashMap<SpecNodeId, usize>,
    #[serde(skip)]
    relation_index: HashMap<SpecRelationId, usize>,
    #[serde(skip)]
    outgoing: HashMap<SpecNodeId, Vec<SpecRelationId>>,
    #[serde(skip)]
    incoming: HashMap<SpecNodeId, Vec<SpecRelationId>>,
}

impl SpecGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn rebuild_indexes(&mut self) {
        self.nodes.sort_by_key(|node| node.id);
        self.relations
            .sort_by_key(|rel| (rel.kind, rel.source, rel.target, rel.ordinal, rel.id));
        self.node_index.clear();
        self.relation_index.clear();
        self.outgoing.clear();
        self.incoming.clear();

        for (idx, node) in self.nodes.iter().enumerate() {
            self.node_index.insert(node.id, idx);
        }
        for (idx, relation) in self.relations.iter().enumerate() {
            self.relation_index.insert(relation.id, idx);
            self.outgoing
                .entry(relation.source)
                .or_default()
                .push(relation.id);
            self.incoming
                .entry(relation.target)
                .or_default()
                .push(relation.id);
        }
    }

    pub fn insert_node(&mut self, node: NodeRecord) -> Result<(), SpecError> {
        if self.node_index.contains_key(&node.id) {
            return Err(SpecError::invalid(format!("duplicate node id {}", node.id)));
        }
        self.nodes.push(node);
        self.rebuild_indexes();
        Ok(())
    }

    pub fn replace_node(&mut self, node: NodeRecord) -> Result<(), SpecError> {
        let Some(index) = self.node_index.get(&node.id).copied() else {
            return Err(SpecError::not_found(format!("missing node {}", node.id)));
        };
        self.nodes[index] = node;
        self.rebuild_indexes();
        Ok(())
    }

    pub fn insert_relation(&mut self, relation: RelationRecord) -> Result<(), SpecError> {
        if self.relation_index.contains_key(&relation.id) {
            return Err(SpecError::invalid(format!(
                "duplicate relation id {}",
                relation.id
            )));
        }
        if !self.node_index.contains_key(&relation.source) {
            return Err(SpecError::not_found(format!(
                "missing source node {}",
                relation.source
            )));
        }
        if !self.node_index.contains_key(&relation.target) {
            return Err(SpecError::not_found(format!(
                "missing target node {}",
                relation.target
            )));
        }
        self.relations.push(relation);
        self.rebuild_indexes();
        Ok(())
    }

    pub fn remove_node(&mut self, id: SpecNodeId) -> Result<NodeRecord, SpecError> {
        let Some(index) = self.node_index.get(&id).copied() else {
            return Err(SpecError::not_found(format!("missing node {}", id)));
        };
        let inbound = self.incoming.get(&id).map_or(0, Vec::len);
        let outbound = self.outgoing.get(&id).map_or(0, Vec::len);
        if inbound > 0 || outbound > 0 {
            return Err(SpecError::invalid(format!(
                "cannot remove node {} while {} inbound and {} outbound relations remain",
                id, inbound, outbound
            )));
        }
        let node = self.nodes.remove(index);
        self.rebuild_indexes();
        Ok(node)
    }

    pub fn remove_relation(&mut self, id: SpecRelationId) -> Result<RelationRecord, SpecError> {
        let Some(index) = self.relation_index.get(&id).copied() else {
            return Err(SpecError::not_found(format!("missing relation {}", id)));
        };
        let relation = self.relations.remove(index);
        self.rebuild_indexes();
        Ok(relation)
    }

    pub fn node(&self, id: SpecNodeId) -> Option<&NodeRecord> {
        self.node_index.get(&id).map(|&index| &self.nodes[index])
    }

    pub fn relation(&self, id: SpecRelationId) -> Option<&RelationRecord> {
        self.relation_index
            .get(&id)
            .map(|&index| &self.relations[index])
    }

    pub fn contains_node(&self, id: SpecNodeId) -> bool {
        self.node_index.contains_key(&id)
    }

    pub fn node_kind(&self, id: SpecNodeId) -> Option<SpecNodeKind> {
        self.node(id).map(|node| node.kind)
    }

    pub fn iter_nodes(&self) -> impl Iterator<Item = &NodeRecord> {
        self.nodes.iter()
    }

    pub fn iter_relations(&self) -> impl Iterator<Item = &RelationRecord> {
        self.relations.iter()
    }

    pub fn outgoing_relations(&self, source: SpecNodeId) -> Vec<&RelationRecord> {
        self.outgoing
            .get(&source)
            .into_iter()
            .flatten()
            .filter_map(|id| self.relation(*id))
            .collect()
    }

    pub fn outgoing_of_kind(&self, source: SpecNodeId, kind: RelationKind) -> Vec<&RelationRecord> {
        self.outgoing_relations(source)
            .into_iter()
            .filter(|rel| rel.kind == kind)
            .collect()
    }

    pub fn incoming_relations(&self, target: SpecNodeId) -> Vec<&RelationRecord> {
        self.incoming
            .get(&target)
            .into_iter()
            .flatten()
            .filter_map(|id| self.relation(*id))
            .collect()
    }

    pub fn relation_ordinals(&self, source: SpecNodeId, kind: RelationKind) -> BTreeSet<u32> {
        self.outgoing_of_kind(source, kind)
            .into_iter()
            .map(|rel| rel.ordinal)
            .collect()
    }
}
