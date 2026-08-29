use crate::runtime::state::subsystems::RuntimeInstrumentation;

/// The lane a partition-map copy is charged to.
///
/// Copying the authoritative map is legitimate work in more than one lane, and
/// the lanes must never share a counter: an ordinary-lane zero is only
/// interpretable when reconstructive copies are accounted somewhere else
/// instead of being laundered into silence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PartitionEditionCopyLane {
    /// Ordinary reads, writes, and settlement. A copy here means a write
    /// observed an outstanding reader edition.
    Ordinary,
    /// Runtime forks, checkpoint capture, and recovery rebuilds, which
    /// materialize whole state by contract.
    Reconstructive,
}

impl PartitionEditionCopyLane {
    /// Charge one whole-map spine copy to this lane.
    pub(crate) fn charge_spine_copy(self, instrumentation: &RuntimeInstrumentation) {
        instrumentation.count(|counters| match self {
            Self::Ordinary => counters.full_state_clones += 1,
            Self::Reconstructive => counters.reconstructive_state_clones += 1,
        });
    }

    /// Charge the deep materialization of `partitions` partition states.
    ///
    /// Only the reconstructive lane can reach this: an ordinary edit copies the
    /// map spine, never every partition behind it.
    pub(crate) fn charge_partition_materialization(
        self,
        instrumentation: &RuntimeInstrumentation,
        partitions: usize,
    ) {
        debug_assert_eq!(
            self,
            Self::Reconstructive,
            "whole-state materialization belongs to the reconstructive lane"
        );
        instrumentation.count(|counters| {
            counters.reconstructive_state_clones += 1;
            counters.reconstructive_partitions_materialized += partitions;
        });
    }
}
