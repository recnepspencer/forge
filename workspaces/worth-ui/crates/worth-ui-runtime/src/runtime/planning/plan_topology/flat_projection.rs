use crate::runtime::{WorthUiPlanLanePartition, WorthUiPlanLookupIndex, WorthUiPlanTopology};

/// Reconstructive compatibility projection over regional plan authority.
///
/// Initial activation may materialize this for cold inspection and legacy lane
/// certification. Regional successor construction is not required to rebuild
/// it, and ordinary execution must never depend on its presence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiPlanFlatProjection {
    topology: WorthUiPlanTopology,
    lane_partitions: Vec<WorthUiPlanLanePartition>,
    lookup_index: WorthUiPlanLookupIndex,
}

impl WorthUiPlanFlatProjection {
    pub(crate) fn new(
        topology: WorthUiPlanTopology,
        lane_partitions: Vec<WorthUiPlanLanePartition>,
        lookup_index: WorthUiPlanLookupIndex,
    ) -> Self {
        Self {
            topology,
            lane_partitions,
            lookup_index,
        }
    }

    pub(crate) fn topology(&self) -> &WorthUiPlanTopology {
        &self.topology
    }

    pub(crate) fn lane_partitions(&self) -> &[WorthUiPlanLanePartition] {
        &self.lane_partitions
    }

    pub(crate) fn lookup_index(&self) -> &WorthUiPlanLookupIndex {
        &self.lookup_index
    }
}
