use crate::data::dependency::DependencySnapshot;
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::data::node::{NodeEntry, NodeEvaluationConfig, NodeState};
use crate::data::trace::CausalityMetadata;

use super::super::node_builder::NodeBuilder;
use super::super::signal_graph::SignalGraph;

impl SignalGraph {
    #[doc(hidden)]
    pub fn create_node(&mut self) -> NodeId {
        let entry = NodeEntry::new();
        self.allocate_node(entry)
    }

    pub fn node(&mut self) -> NodeBuilder<'_> {
        NodeBuilder::new(self)
    }

    #[doc(hidden)]
    pub fn create_node_with_config(&mut self, config: NodeEvaluationConfig) -> NodeId {
        let mut entry = NodeEntry::new();
        entry.set_eval_config(config);
        self.allocate_node(entry)
    }

    pub fn get_state(&self, id: NodeId) -> Result<NodeState, SignalError> {
        Ok(*self.get_entry(id)?.get_state())
    }

    pub fn get_entry(&self, id: NodeId) -> Result<&NodeEntry, SignalError> {
        self.validate_handle(id)?;
        let slot = &self.nodes[id.index() as usize];
        slot.data.as_ref().ok_or_else(|| stale_error(id))
    }

    pub fn get_entry_mut(&mut self, id: NodeId) -> Result<&mut NodeEntry, SignalError> {
        self.validate_handle(id)?;
        let slot = &mut self.nodes[id.index() as usize];
        slot.data.as_mut().ok_or_else(|| stale_error(id))
    }

    pub(crate) fn get_dep_snapshot(&self, id: NodeId) -> Result<&DependencySnapshot, SignalError> {
        let entry = self.get_entry(id)?;
        Ok(self.dependency_snapshots.get(entry.get_dep_snapshot_id()))
    }

    pub(crate) fn set_dep_snapshot(
        &mut self,
        id: NodeId,
        snapshot: DependencySnapshot,
    ) -> Result<(), SignalError> {
        let snapshot_id = self.dependency_snapshots.insert(snapshot);
        self.get_entry_mut(id)?.set_dep_snapshot_id(snapshot_id);
        self.record_graph_storage_pressure();
        Ok(())
    }

    pub fn is_alive(&self, id: NodeId) -> bool {
        let idx = id.index() as usize;
        if idx >= self.nodes.len() {
            return false;
        }
        let slot = &self.nodes[idx];
        slot.generation == id.generation() && slot.is_occupied()
    }

    pub fn active_node_count(&self) -> usize {
        self.active_nodes as usize
    }

    pub fn arena_capacity(&self) -> usize {
        self.nodes.len()
    }

    pub(crate) fn live_node_id_at(&self, index: usize) -> Option<NodeId> {
        let slot = self.nodes.get(index)?;
        if !slot.is_occupied() {
            return None;
        }
        Some(NodeId::new(index as u32, slot.generation))
    }

    pub(crate) fn replace_entry(
        &mut self,
        id: NodeId,
        entry: NodeEntry,
    ) -> Result<(), SignalError> {
        let target = self.get_entry_mut(id)?;
        *target = entry;
        Ok(())
    }

    pub fn causality_of(&self, node: NodeId) -> Result<Option<&CausalityMetadata>, SignalError> {
        Ok(self.get_entry(node)?.get_causality())
    }

    pub fn set_causality(
        &mut self,
        node: NodeId,
        causality: Option<CausalityMetadata>,
    ) -> Result<(), SignalError> {
        self.get_entry_mut(node)?.set_causality(causality);
        Ok(())
    }
}

pub(super) fn stale_error(id: NodeId) -> SignalError {
    SignalError::invalid_input(format!("stale NodeId: {id}"))
}
