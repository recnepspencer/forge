use crate::runtime::launch::runtime_instance::WorthUiRuntime;

use super::UiAllocationFrameQueueDisposition;

impl WorthUiRuntime {
    pub(crate) fn shutdown_allocation_frame_dispatcher(
        &mut self,
    ) -> UiAllocationFrameQueueDisposition {
        self.allocation_frame_scheduler.shutdown()
    }
}
