use crate::data::bitset::DenseBitset;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::node::NodeEntry;

#[derive(Debug, Clone)]
struct NodePatch {
    original: NodeEntry,
}

/// Sparse patch storage with O(touched) rollback/clear semantics.
#[derive(Debug, Clone, Default)]
pub(super) struct SparsePatchBuffer {
    patches: Vec<Option<NodePatch>>,
    dirty_bits: DenseBitset,
    touched_indices: Vec<usize>,
}

impl SparsePatchBuffer {
    pub(super) fn new() -> Self {
        Self {
            patches: Vec::new(),
            dirty_bits: DenseBitset::new(),
            touched_indices: Vec::new(),
        }
    }

    fn ensure_capacity(&mut self, index: usize) {
        if index >= self.patches.len() {
            self.patches.resize_with(index + 1, || None);
        }
        self.dirty_bits.ensure_len(index + 1);
    }

    pub(super) fn stage_original(&mut self, graph: &SignalGraph, node: NodeId) -> Result<(), SignalError> {
        let index = node.index() as usize;
        self.ensure_capacity(index);
        if self.patches[index].is_none() {
            let original = graph.get_entry(node)?.clone();
            self.patches[index] = Some(NodePatch { original });
            if self.dirty_bits.mark(index) {
                self.touched_indices.push(index);
            }
        }
        Ok(())
    }

    pub(super) fn touched_count(&self) -> usize {
        self.touched_indices.len()
    }

    /// Commit path: graph already contains staged changes, so clear patches only.
    pub(super) fn commit_and_clear(&mut self) {
        let mut touched = std::mem::take(&mut self.touched_indices);
        touched.sort_unstable();
        for index in touched {
            if let Some(slot) = self.patches.get_mut(index) {
                *slot = None;
            }
            self.dirty_bits.clear(index);
        }
    }

    /// Roll back graph to staged originals, then clear patch set.
    pub(super) fn rollback_and_clear(&mut self, graph: &mut SignalGraph) -> Result<(), SignalError> {
        let mut touched = std::mem::take(&mut self.touched_indices);
        touched.sort_unstable();
        for index in touched {
            let Some(slot) = self.patches.get_mut(index) else {
                self.dirty_bits.clear(index);
                continue;
            };
            let Some(patch) = slot.take() else {
                self.dirty_bits.clear(index);
                continue;
            };

            let node = graph
                .live_node_id_at(index)
                .ok_or_else(|| SignalError::internal("rollback encountered stale patch node"))?;
            graph.replace_entry(node, patch.original)?;
            self.dirty_bits.clear(index);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::SparsePatchBuffer;
    use crate::data::aspect::{Aspect, AspectVersion};
    use crate::data::error::SignalError;
    use crate::data::graph::SignalGraph;
    use crate::data::node::NodeState;
    use crate::logic::invalidation::mark_dirty;

    #[test]
    fn rollback_clears_only_touched_entries_and_preserves_untouched() -> Result<(), SignalError> {
        let mut graph = SignalGraph::new();
        let a = graph.create_node();
        let b = graph.create_node();
        graph.add_dependency(b, a, Aspect::Geometry)?;

        let before_a = graph.get_state(a)?;
        let before_b = graph.get_state(b)?;

        let mut patches = SparsePatchBuffer::new();
        patches.stage_original(&graph, a)?;
        patches.stage_original(&graph, b)?;
        assert_eq!(patches.touched_count(), 2);

        mark_dirty(&mut graph, a, Aspect::Geometry)?;
        assert_eq!(graph.get_state(a)?, NodeState::Dirty);

        patches.rollback_and_clear(&mut graph)?;
        assert_eq!(graph.get_state(a)?, before_a);
        assert_eq!(graph.get_state(b)?, before_b);
        assert_eq!(patches.touched_count(), 0);
        Ok(())
    }

    #[test]
    fn stage_high_index_node_grows_capacity_safely() -> Result<(), SignalError> {
        let mut graph = SignalGraph::new();
        let mut target = graph.create_node();
        for _ in 0..10_000 {
            target = graph.create_node();
        }

        let mut patches = SparsePatchBuffer::new();
        patches.stage_original(&graph, target)?;
        assert_eq!(patches.touched_count(), 1);
        Ok(())
    }

    #[test]
    fn rollback_then_restage_has_no_ghost_data() -> Result<(), SignalError> {
        let mut graph = SignalGraph::new();
        let a = graph.create_node();
        let baseline = graph.get_entry(a)?.clone();
        let mut patches = SparsePatchBuffer::new();

        patches.stage_original(&graph, a)?;
        graph
            .get_entry_mut(a)?
            .set_aspect_version(AspectVersion::new(0, 10));
        patches.rollback_and_clear(&mut graph)?;
        assert_eq!(*graph.get_entry(a)?, baseline);
        assert_eq!(patches.touched_count(), 0);

        patches.stage_original(&graph, a)?;
        graph
            .get_entry_mut(a)?
            .set_aspect_version(AspectVersion::new(0, 11));
        patches.commit_and_clear();
        assert_eq!(graph.get_entry(a)?.get_aspect_version(), AspectVersion::new(0, 11));
        assert_eq!(patches.touched_count(), 0);
        Ok(())
    }
}
