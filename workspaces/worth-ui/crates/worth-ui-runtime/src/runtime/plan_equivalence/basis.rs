use crate::runtime::WorthUiRuntimeHandleAllocationReceipt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiExecutionPlanEquivalenceBasis {
    handle_receipt: WorthUiRuntimeHandleAllocationReceipt,
    plan_node_count: usize,
    child_range_count: usize,
    lane_partition_count: usize,
    lookup_entry_count: usize,
    egui_boundary_count: usize,
    render_resource_ref_count: usize,
    executable_shape_fingerprint: u64,
}

pub(crate) struct WorthUiExecutionPlanEquivalenceBasisInput {
    pub handle_receipt: WorthUiRuntimeHandleAllocationReceipt,
    pub plan_node_count: usize,
    pub child_range_count: usize,
    pub lane_partition_count: usize,
    pub lookup_entry_count: usize,
    pub egui_boundary_count: usize,
    pub render_resource_ref_count: usize,
    pub executable_shape_fingerprint: u64,
}

impl WorthUiExecutionPlanEquivalenceBasis {
    pub(crate) fn new(input: WorthUiExecutionPlanEquivalenceBasisInput) -> Self {
        let WorthUiExecutionPlanEquivalenceBasisInput {
            handle_receipt,
            plan_node_count,
            child_range_count,
            lane_partition_count,
            lookup_entry_count,
            egui_boundary_count,
            render_resource_ref_count,
            executable_shape_fingerprint,
        } = input;
        Self {
            handle_receipt,
            plan_node_count,
            child_range_count,
            lane_partition_count,
            lookup_entry_count,
            egui_boundary_count,
            render_resource_ref_count,
            executable_shape_fingerprint,
        }
    }

    pub fn handle_receipt(self) -> WorthUiRuntimeHandleAllocationReceipt {
        self.handle_receipt
    }

    pub fn plan_node_count(self) -> usize {
        self.plan_node_count
    }

    pub fn child_range_count(self) -> usize {
        self.child_range_count
    }

    pub fn lane_partition_count(self) -> usize {
        self.lane_partition_count
    }

    pub fn lookup_entry_count(self) -> usize {
        self.lookup_entry_count
    }

    pub fn egui_boundary_count(self) -> usize {
        self.egui_boundary_count
    }

    pub fn render_resource_ref_count(self) -> usize {
        self.render_resource_ref_count
    }

    pub fn executable_shape_fingerprint(self) -> u64 {
        self.executable_shape_fingerprint
    }
}
