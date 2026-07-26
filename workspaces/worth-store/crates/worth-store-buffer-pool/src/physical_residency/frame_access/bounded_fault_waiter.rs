use std::sync::Arc;

use super::{PhysicalFrameLoadTerminal, PhysicalFrameLoadingIdentity};
use crate::physical_residency::pool::PoolInner;
use crate::physical_residency::{PhysicalBoundedFrameKey, PhysicalFrameLease};

/// Move-owned attachment to a bounded loading identity, with no source method.
#[derive(Debug)]
pub struct PhysicalBoundedFrameFaultWaiter {
    pub(crate) owner: Arc<PoolInner>,
    pub(crate) key: PhysicalBoundedFrameKey,
    pub(crate) identity: PhysicalFrameLoadingIdentity,
    pub(crate) armed: bool,
}

impl PhysicalBoundedFrameFaultWaiter {
    pub const fn loading_identity(&self) -> PhysicalFrameLoadingIdentity {
        self.identity
    }

    pub fn wait(mut self) -> Result<PhysicalFrameLease, PhysicalFrameLoadTerminal> {
        let outcome = self.owner.join_bounded_loading(self.key, self.identity);
        self.armed = false;
        outcome
    }
}

impl Drop for PhysicalBoundedFrameFaultWaiter {
    fn drop(&mut self) {
        if self.armed {
            self.owner
                .release_bounded_loading_waiter(self.key, self.identity);
        }
    }
}
