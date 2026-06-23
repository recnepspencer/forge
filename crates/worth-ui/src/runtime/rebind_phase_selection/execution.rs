use crate::runtime::{
    WorthUiHeaderFramePlan, WorthUiHeaderFrameRebindRequest, WorthUiPageHostPlan,
    WorthUiPageHostRequest, WorthUiRuntimeHost,
};

use super::{
    WorthUiRebindPhaseExecutionDenial, WorthUiRebindPhaseExecutionReceipt,
    WorthUiRebindPhaseSelectionBatch,
};

impl WorthUiRuntimeHost {
    pub fn execute_rebind_phase_selection(
        &mut self,
        current_header_plan: &WorthUiHeaderFramePlan,
        header_request: WorthUiHeaderFrameRebindRequest,
        page_host_request: WorthUiPageHostRequest,
        batch: WorthUiRebindPhaseSelectionBatch,
    ) -> Result<
        (
            WorthUiHeaderFramePlan,
            WorthUiPageHostPlan,
            WorthUiRebindPhaseExecutionReceipt,
        ),
        WorthUiRebindPhaseExecutionDenial,
    > {
        let runtime_instance = batch.runtime_instance();
        let change_evidence_digest = batch.change_evidence_digest();
        let counters = batch.counters();
        let rows = batch.rows().to_vec();
        let replay_digest = batch.replay_digest();
        let (header_phase_plan, page_host_phase_plan) = batch.into_plans();
        let (next_header_plan, header_rebind) = self
            .rebind_header_frame_from_phase_plan(
                current_header_plan,
                header_request,
                header_phase_plan,
            )
            .map_err(WorthUiRebindPhaseExecutionDenial::HeaderFrame)?;
        let (next_page_host_plan, page_host_rebind) = self
            .rebind_page_host_from_phase_plan(page_host_request, page_host_phase_plan)
            .map_err(WorthUiRebindPhaseExecutionDenial::PageHost)?;
        Ok((
            next_header_plan,
            next_page_host_plan,
            WorthUiRebindPhaseExecutionReceipt::new(
                runtime_instance,
                change_evidence_digest,
                counters,
                rows,
                replay_digest,
                header_rebind,
                page_host_rebind,
            ),
        ))
    }
}
