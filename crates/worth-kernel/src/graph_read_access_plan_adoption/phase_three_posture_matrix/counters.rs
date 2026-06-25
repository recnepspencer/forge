use super::cap_ledger::WorthGraphReadAccessPostureCapReport;
use super::posture_resolution::WorthGraphReadRequirementPostureMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessPostureMatrixCounters {
    resolved_posture_count: usize,
    requirement_identity_count: usize,
    cap_family_count: usize,
    uncapped_posture_family_count: usize,
    cap_exceeded_family_count: usize,
    graph_traversal_attempt_count: usize,
    dense_frontier_allocation_attempt_count: usize,
    streaming_page_creation_attempt_count: usize,
    index_construction_attempt_count: usize,
    access_plan_consumption_attempt_count: usize,
    graph_read_execution_count: usize,
    graph_read_receipt_count: usize,
}

impl WorthGraphReadAccessPostureMatrixCounters {
    pub(crate) fn from_products(
        posture_map: &WorthGraphReadRequirementPostureMap,
        cap_report: &WorthGraphReadAccessPostureCapReport,
    ) -> Self {
        Self {
            resolved_posture_count: posture_map.resolved_postures().len(),
            requirement_identity_count: posture_map.requirement_identity_count(),
            cap_family_count: cap_report.ledger().rows().len(),
            uncapped_posture_family_count: cap_report.uncapped_posture_family_count(),
            cap_exceeded_family_count: cap_report.cap_exceeded_family_count(),
            graph_traversal_attempt_count: 0,
            dense_frontier_allocation_attempt_count: 0,
            streaming_page_creation_attempt_count: 0,
            index_construction_attempt_count: 0,
            access_plan_consumption_attempt_count: 0,
            graph_read_execution_count: 0,
            graph_read_receipt_count: 0,
        }
    }

    pub const fn resolved_posture_count(&self) -> usize {
        self.resolved_posture_count
    }

    pub const fn requirement_identity_count(&self) -> usize {
        self.requirement_identity_count
    }

    pub const fn cap_family_count(&self) -> usize {
        self.cap_family_count
    }

    pub const fn uncapped_posture_family_count(&self) -> usize {
        self.uncapped_posture_family_count
    }

    pub const fn cap_exceeded_family_count(&self) -> usize {
        self.cap_exceeded_family_count
    }

    pub const fn graph_traversal_attempt_count(&self) -> usize {
        self.graph_traversal_attempt_count
    }

    pub const fn dense_frontier_allocation_attempt_count(&self) -> usize {
        self.dense_frontier_allocation_attempt_count
    }

    pub const fn streaming_page_creation_attempt_count(&self) -> usize {
        self.streaming_page_creation_attempt_count
    }

    pub const fn index_construction_attempt_count(&self) -> usize {
        self.index_construction_attempt_count
    }

    pub const fn access_plan_consumption_attempt_count(&self) -> usize {
        self.access_plan_consumption_attempt_count
    }

    pub const fn graph_read_execution_count(&self) -> usize {
        self.graph_read_execution_count
    }

    pub const fn graph_read_receipt_count(&self) -> usize {
        self.graph_read_receipt_count
    }
}
