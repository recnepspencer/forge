use crate::data::graph::SubscriberEdgeStore;

use super::{remap_live_entry_handles, SignalGraph};

impl SignalGraph {
    pub(super) fn compact_subscriber_edge_storage(&mut self) {
        let old_subscriber_edges = std::mem::take(&mut self.topology.subscriber_edges);
        self.observation
            .telemetry
            .storage
            .graph_storage_compaction_count += 1;
        self.observation
            .telemetry
            .storage
            .graph_storage_subscriber_segments_rewritten +=
            old_subscriber_edges.live_segment_count() as u64;

        let mut compacted_subscriber_edges = SubscriberEdgeStore::default();
        remap_live_entry_handles(
            self,
            |entry| entry.get_subscribers_id(),
            |subscribers_id| {
                compacted_subscriber_edges
                    .insert_from_slice(old_subscriber_edges.get(subscribers_id))
            },
            |entry, subscribers_id| entry.set_subscribers_id(subscribers_id),
        );

        self.topology.subscriber_edges = compacted_subscriber_edges;
    }
}
