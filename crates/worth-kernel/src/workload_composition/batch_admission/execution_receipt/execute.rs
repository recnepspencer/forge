use super::BatchAdmissionExecutionReceipt;
use crate::workload_composition::SelectedBatchAdmissionPlan;

pub fn execute_selected_batch_admission_plan(
    selected_plan: &SelectedBatchAdmissionPlan,
) -> BatchAdmissionExecutionReceipt {
    BatchAdmissionExecutionReceipt::from_selected_plan(selected_plan)
}
