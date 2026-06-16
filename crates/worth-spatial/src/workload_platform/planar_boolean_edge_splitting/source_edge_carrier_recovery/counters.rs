#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanSplitSourceEdgeCarrierCounters {
    recovered_carrier_count: usize,
    distinct_source_edge_count: usize,
    point_carrier_references_inspected: usize,
    interval_carrier_references_inspected: usize,
    group_carrier_references_inspected: usize,
    duplicate_carrier_references_collapsed: usize,
    topology_bound_carrier_count: usize,
}

impl PlanarBooleanSplitSourceEdgeCarrierCounters {
    pub(crate) fn new(
        recovered_carrier_count: usize,
        distinct_source_edge_count: usize,
        point_carrier_references_inspected: usize,
        interval_carrier_references_inspected: usize,
        group_carrier_references_inspected: usize,
        duplicate_carrier_references_collapsed: usize,
        topology_bound_carrier_count: usize,
    ) -> Self {
        Self {
            recovered_carrier_count,
            distinct_source_edge_count,
            point_carrier_references_inspected,
            interval_carrier_references_inspected,
            group_carrier_references_inspected,
            duplicate_carrier_references_collapsed,
            topology_bound_carrier_count,
        }
    }

    pub fn recovered_carrier_count(self) -> usize {
        self.recovered_carrier_count
    }

    pub fn distinct_source_edge_count(self) -> usize {
        self.distinct_source_edge_count
    }

    pub fn point_carrier_references_inspected(self) -> usize {
        self.point_carrier_references_inspected
    }

    pub fn interval_carrier_references_inspected(self) -> usize {
        self.interval_carrier_references_inspected
    }

    pub fn group_carrier_references_inspected(self) -> usize {
        self.group_carrier_references_inspected
    }

    pub fn duplicate_carrier_references_collapsed(self) -> usize {
        self.duplicate_carrier_references_collapsed
    }

    pub fn topology_bound_carrier_count(self) -> usize {
        self.topology_bound_carrier_count
    }
}
