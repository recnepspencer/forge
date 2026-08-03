use crate::physical_runtime::instance::PhysicalExecutionCall;

use super::{
    super::{AdmittedDirtyFrame, PhysicalWritebackFailureCause},
    FrameWritebackPort,
};

impl FrameWritebackPort {
    pub(super) fn require_current_dirty(
        &self,
        dirty: &AdmittedDirtyFrame,
    ) -> Result<PhysicalExecutionCall, PhysicalWritebackFailureCause> {
        let call = self
            .execution
            .admit_call()
            .map_err(PhysicalWritebackFailureCause::PreEffect)?;
        if dirty.pool_incarnation() != self.frame_ports.incarnation() {
            return Err(PhysicalWritebackFailureCause::StaleOrForeignDirtyFrame);
        }
        Ok(call)
    }
}
