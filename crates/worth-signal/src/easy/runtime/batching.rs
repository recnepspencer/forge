use crate::facade::runtime::mark_dirty_batch;
use crate::facade::{BatchChange, SignalError};

use super::{SignalApp, DEFAULT_ASPECT};

impl SignalApp {
    pub fn batch<F>(&mut self, apply: F)
    where
        F: FnOnce(&mut Self),
    {
        self.try_batch(|graph| {
            apply(graph);
            Ok(())
        })
        .expect("easy path batch failed");
    }

    pub fn try_batch<F>(&mut self, apply: F) -> Result<(), SignalError>
    where
        F: FnOnce(&mut Self) -> Result<(), SignalError>,
    {
        self.batch_depth += 1;
        let apply_result = apply(self);
        self.batch_depth -= 1;
        if let Err(error) = apply_result {
            self.rollback_outer_batch();
            return Err(error);
        }
        if self.batch_depth == 0 {
            self.commit_outer_batch()?;
        }
        Ok(())
    }

    fn commit_outer_batch(&mut self) -> Result<(), SignalError> {
        let dirty_nodes = std::mem::take(&mut self.batched_dirty_nodes);
        let changed_nodes = dirty_nodes.clone();
        if let Err(error) = mark_dirty_batch(
            &mut self.graph,
            &BatchChange::from_sources(dirty_nodes.into_iter().map(|node| (node, DEFAULT_ASPECT))),
        ) {
            self.restore_batch_undo();
            return Err(error);
        }
        for node in &changed_nodes {
            self.ensure_evaluated(*node)?;
        }
        super::super::observation::deliver_observation_boundary(self, changed_nodes)?;
        self.clear_batch_undo();
        Ok(())
    }

    fn rollback_outer_batch(&mut self) {
        if self.batch_depth == 0 {
            self.batched_dirty_nodes.clear();
            self.restore_batch_undo();
        }
    }

    fn restore_batch_undo(&mut self) {
        for (node, entry) in std::mem::take(&mut self.batch_entry_undo) {
            if let Ok(mut slot) = self.graph.get_entry_mut(node) {
                *slot = entry;
            }
        }
        for (node, previous) in std::mem::take(&mut self.batch_value_undo) {
            match previous {
                Some(value) => {
                    self.values.insert(node, value);
                }
                None => {
                    self.values.remove(&node);
                }
            }
        }
        for (node, previous) in std::mem::take(&mut self.batch_pending_input_undo) {
            match previous {
                Some(version) => {
                    self.pending_input_versions.insert(node, version);
                }
                None => {
                    self.pending_input_versions.remove(&node);
                }
            }
        }
    }

    fn clear_batch_undo(&mut self) {
        self.batch_value_undo.clear();
        self.batch_entry_undo.clear();
        self.batch_pending_input_undo.clear();
    }
}
