use worth_ui::facade::{WorthUiAdmittedRuntimeChangeEvidence, WorthUiRebindPhaseExecutionReceipt};

use super::ValidationRuntimeWorkbench;
use crate::app_capabilities::validation_header_frame_rebind_request;

impl ValidationRuntimeWorkbench {
    pub(crate) fn runtime_change_rebind_receipts(
        &mut self,
        admitted_change: &WorthUiAdmittedRuntimeChangeEvidence,
    ) -> Option<WorthUiRebindPhaseExecutionReceipt> {
        let batch = self
            .runtime
            .plan_rebind_phase_selection(
                admitted_change,
                self.runtime
                    .admit_projection_plan(self.header_frame_plan.clone())
                    .expect("active header frame plan should remain admissible"),
                self.runtime
                    .admit_projection_plan(self.page_host_plan.clone())
                    .expect("active page-host plan should remain admissible"),
            )
            .expect("runtime-owned phase selection should classify active validation lanes");
        self.runtime
            .execute_rebind_phase_selection(
                &self.header_frame_plan,
                validation_header_frame_rebind_request(),
                self.validation_page_host_request(),
                batch,
            )
            .map(|(next_header_plan, next_page_host_plan, receipt)| {
                self.header_frame_plan = next_header_plan;
                self.page_host_plan = next_page_host_plan;
                receipt
            })
            .ok()
    }
}
