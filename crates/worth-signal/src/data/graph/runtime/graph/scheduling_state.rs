use crate::data::handle::NodeId;

use super::SignalGraph;

impl SignalGraph {
    #[cfg(test)]
    pub(crate) fn record_repeated_invalidation_admission(&mut self, target: NodeId) {
        let count = self
            .pending_repeated_invalidation_admissions
            .entry(target)
            .or_default();
        *count = count.saturating_add(1);
    }

    pub(crate) fn take_repeated_invalidation_admissions(&mut self, target: NodeId) -> u64 {
        self.pending_repeated_invalidation_admissions
            .remove(&target)
            .unwrap_or(0)
    }
}
