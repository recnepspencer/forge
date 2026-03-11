mod dependency_edges;
mod snapshots;
mod subscribers;

use std::collections::HashMap;
use std::hash::Hash;

use crate::data::node::NodeEntry;

use super::schedule::CompactionFamily;
use super::super::signal_graph::SignalGraph;

impl SignalGraph {
    #[cfg(test)]
    pub(crate) fn compact_graph_storage(&mut self) {
        self.compact_storage_family(CompactionFamily::DependencyEdges);
        self.compact_storage_family(CompactionFamily::Subscribers);
        self.compact_storage_family(CompactionFamily::Snapshots);
    }

    pub(in crate::data::graph) fn compact_storage_family(&mut self, family: CompactionFamily) {
        match family {
            CompactionFamily::DependencyEdges => self.compact_dependency_edge_storage(),
            CompactionFamily::Subscribers => self.compact_subscriber_edge_storage(),
            CompactionFamily::Snapshots => self.compact_dependency_snapshot_storage(),
        }
    }
}

fn remap_live_entry_handles<OldId, NewId>(
    graph: &mut SignalGraph,
    mut read_id: impl FnMut(&NodeEntry) -> OldId,
    mut remap_id: impl FnMut(OldId) -> NewId,
    mut write_id: impl FnMut(&mut NodeEntry, NewId),
) where
    OldId: Eq + Hash + Copy,
    NewId: Copy,
{
    let mut id_map = HashMap::new();

    for index in 0..graph.nodes.len() {
        let Some(node) = graph.live_node_id_at(index) else {
            continue;
        };
        let old_id = match graph.get_entry(node) {
            Ok(entry) => read_id(entry),
            Err(_) => continue,
        };
        let new_id = *id_map.entry(old_id).or_insert_with(|| remap_id(old_id));
        if let Ok(live_entry) = graph.get_entry_mut(node) {
            write_id(live_entry, new_id);
        }
    }
}
