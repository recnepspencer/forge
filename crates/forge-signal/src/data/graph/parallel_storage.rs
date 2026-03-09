use crate::data::dependency::{DependencyEdge, DependencySnapshot, DependencySnapshotId};
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::data::node::NodeEntry;

use super::signal_graph::SignalGraph;
use super::slot::Slot;

impl SignalGraph {
    pub(crate) fn store_dependency_snapshot(
        &mut self,
        snapshot: DependencySnapshot,
    ) -> DependencySnapshotId {
        self.dependency_snapshots.insert(snapshot)
    }

    pub(crate) fn store_dependency_edges(
        &mut self,
        edges: &[DependencyEdge],
    ) -> super::DependencySetId {
        self.dependency_edges.insert_from_slice(edges)
    }

    pub(crate) fn store_subscribers(&mut self, subscribers: &[NodeId]) -> super::SubscriberSetId {
        self.subscriber_edges.insert_from_slice(subscribers)
    }

    pub(crate) fn replace_entries_parallel(
        &mut self,
        updates: &[(NodeId, NodeEntry)],
    ) -> Result<(), SignalError> {
        for &(node, _) in updates {
            self.validate_handle(node)?;
        }

        let mut sorted = updates.to_vec();
        sorted.sort_by_key(|(node, _)| (node.index(), node.generation()));
        for window in sorted.windows(2) {
            if let [left, right] = window {
                if left.0 == right.0 {
                    return Err(SignalError::internal(
                        "parallel entry replacement encountered duplicate node target",
                    ));
                }
            }
        }

        replace_entries_recursive(&mut self.nodes, 0, &sorted)
    }
}

fn replace_entries_recursive(
    slots: &mut [Slot],
    base_index: usize,
    updates: &[(NodeId, NodeEntry)],
) -> Result<(), SignalError> {
    if updates.is_empty() {
        return Ok(());
    }
    if updates.len() <= 1 || slots.len() <= 1 {
        for (node, entry) in updates {
            let relative_index =
                (node.index() as usize)
                    .checked_sub(base_index)
                    .ok_or_else(|| {
                        SignalError::internal("parallel entry replacement index underflow")
                    })?;
            let slot = slots.get_mut(relative_index).ok_or_else(|| {
                SignalError::internal("parallel entry replacement index overflow")
            })?;
            let target = slot
                .data
                .as_mut()
                .ok_or_else(|| SignalError::internal("parallel apply targeted vacant slot"))?;
            *target = entry.clone();
        }
        return Ok(());
    }

    let mid = slots.len() / 2;
    let split_index =
        updates.partition_point(|(node, _)| (node.index() as usize) < base_index + mid);
    let (left_slots, right_slots) = slots.split_at_mut(mid);
    let (left_updates, right_updates) = updates.split_at(split_index);
    let (left_result, right_result) = rayon::join(
        || replace_entries_recursive(left_slots, base_index, left_updates),
        || replace_entries_recursive(right_slots, base_index + mid, right_updates),
    );
    left_result?;
    right_result?;
    Ok(())
}
