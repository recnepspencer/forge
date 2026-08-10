use crate::data::error::SignalError;
use crate::data::graph::signal_graph::{stale_error, SignalGraph};
use crate::data::graph::storage::Slot;
use crate::data::handle::NodeId;
use crate::data::node::{NodeEntry, NodeWarmData};

const NODE_ARENA_RESERVE_CHUNK: usize = 1024;

impl SignalGraph {
    pub(in crate::data::graph) fn allocate_node(&mut self, entry: NodeEntry) -> NodeId {
        let (hot, warm, cold) = entry.into_storage_parts();
        while let Some(index) = self.arena.free_list.pop() {
            if index as usize >= self.arena.nodes.len() {
                continue;
            }
            self.arena.free_slots.clear(index as usize);
            let slot = &mut self.arena.nodes[index as usize];
            if slot.is_retired() {
                continue;
            }
            self.arena.hot[index as usize] = Some(hot.clone());
            self.arena.warm[index as usize] = warm.clone();
            self.arena.cold[index as usize] = cold.clone();
            let generation = slot.occupy();
            self.arena.active_nodes += 1;
            let node = NodeId::new(index, generation);
            self.record_branch_mutation_introduced(node);
            return node;
        }

        let index = self.arena.nodes.len() as u32;
        if self.arena.nodes.len() == self.arena.nodes.capacity() {
            self.arena.nodes.reserve(NODE_ARENA_RESERVE_CHUNK);
            self.arena.hot.reserve(NODE_ARENA_RESERVE_CHUNK);
            self.arena.warm.reserve(NODE_ARENA_RESERVE_CHUNK);
            self.arena.cold.reserve(NODE_ARENA_RESERVE_CHUNK);
        }
        let mut slot = Slot::vacant();
        let generation = slot.occupy();
        self.arena.nodes.push(slot);
        self.arena.hot.push(Some(hot));
        self.arena.warm.push(warm);
        self.arena.cold.push(cold);
        self.arena.active_nodes += 1;
        let node = NodeId::new(index, generation);
        self.record_branch_mutation_introduced(node);
        node
    }

    pub(crate) fn rollback_created_nodes(&mut self, created_nodes: &[NodeId]) {
        for node in created_nodes {
            self.conditional_dependency_versions.remove(node);
        }
        let mut indices = created_nodes
            .iter()
            .map(|node| node.index() as usize)
            .collect::<Vec<_>>();
        indices.sort_unstable();
        indices.dedup();
        self.observation
            .telemetry
            .storage
            .rolled_back_created_node_count += indices.len() as u64;

        for index in indices.iter().rev().copied() {
            let Some(slot) = self.arena.nodes.get_mut(index) else {
                continue;
            };
            if slot.is_occupied() {
                slot.vacate();
                self.arena.hot[index] = None;
                self.arena.warm[index] = NodeWarmData::default();
                self.arena.cold[index] = None;
                self.arena.active_nodes = self.arena.active_nodes.saturating_sub(1);
            }
            if !slot.is_retired() && !self.arena.free_slots.contains(index) {
                self.arena.free_list.push(index as u32);
                self.arena.free_slots.mark(index);
            }
        }

        while self
            .arena
            .nodes
            .last()
            .is_some_and(|slot| !slot.is_occupied())
        {
            self.arena.free_slots.clear(self.arena.nodes.len() - 1);
            self.arena.nodes.pop();
            self.arena.hot.pop();
            self.arena.warm.pop();
            self.arena.cold.pop();
        }
        self.arena
            .free_list
            .retain(|index| (*index as usize) < self.arena.nodes.len());
    }

    pub(crate) fn node_allocator_state(&self) -> u32 {
        self.arena.nodes.len() as u32
    }

    pub(crate) fn synchronize_node_allocator(&mut self, next_node_index: u32) {
        if self.arena.nodes.len() as u32 >= next_node_index {
            return;
        }
        let missing = next_node_index as usize - self.arena.nodes.len();
        self.arena.nodes.reserve(missing);
        for _ in 0..missing {
            self.arena.nodes.push(Slot::retired_placeholder());
            self.arena.hot.push(None);
            self.arena.warm.push(NodeWarmData::default());
            self.arena.cold.push(None);
        }
    }

    #[cfg(test)]
    pub(crate) fn reserve_node_capacity(&mut self, additional: usize) {
        self.arena.nodes.reserve(additional);
        self.arena.hot.reserve(additional);
        self.arena.warm.reserve(additional);
        self.arena.cold.reserve(additional);
    }

    pub(in crate::data::graph) fn validate_handle(&self, id: NodeId) -> Result<(), SignalError> {
        let idx = id.index() as usize;
        if idx >= self.arena.nodes.len() {
            return Err(stale_error(id, id.generation()));
        }
        let slot = &self.arena.nodes[idx];
        if slot.generation != id.generation() || !slot.is_occupied() {
            return Err(stale_error(id, slot.generation));
        }
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn free_list_snapshot(&self) -> Vec<u32> {
        self.arena.free_list.clone()
    }

    #[cfg(test)]
    pub(crate) fn force_slot_generation_for_test(
        &mut self,
        index: u32,
        generation: u32,
    ) -> Result<(), SignalError> {
        let slot = self
            .arena
            .nodes
            .get_mut(index as usize)
            .ok_or_else(|| SignalError::invalid_input(format!("unknown slot `{index}`")))?;
        slot.generation = generation;
        slot.retired = false;
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn is_slot_retired_for_test(&self, index: u32) -> Result<bool, SignalError> {
        let slot = self
            .arena
            .nodes
            .get(index as usize)
            .ok_or_else(|| SignalError::invalid_input(format!("unknown slot `{index}`")))?;
        Ok(slot.is_retired())
    }
}
