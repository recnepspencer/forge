use serde::{Deserialize, Serialize};

use super::super::performance::ResourceBoundaryPerformanceEnvelope;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceBranchRestoreReport {
    restored_in_flight_width: u32,
    retained_summary_width: u32,
    broad_rebuild_denial_count: u32,
    performance: ResourceBoundaryPerformanceEnvelope,
}

impl ResourceBranchRestoreReport {
    pub(crate) fn new(
        restored_in_flight_width: u32,
        retained_summary_width: u32,
        broad_rebuild_denial_count: u32,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            restored_in_flight_width,
            retained_summary_width,
            broad_rebuild_denial_count,
            performance,
        }
    }

    pub fn restored_in_flight_width(self) -> u32 {
        self.restored_in_flight_width
    }

    pub fn retained_summary_width(self) -> u32 {
        self.retained_summary_width
    }

    pub fn broad_rebuild_denial_count(self) -> u32 {
        self.broad_rebuild_denial_count
    }

    pub fn performance(self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }
}
