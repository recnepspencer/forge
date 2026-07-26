use std::sync::Arc;

use super::{PhysicalFrameLoadTerminal, PhysicalFrameLoadingIdentity};
use crate::physical_residency::pool::PoolInner;
use crate::physical_residency::{PhysicalFrameKey, PhysicalFrameLease};

/// Move-owned attachment to an already admitted loading identity.
///
/// This type has no source-loading method.
#[derive(Debug)]
pub struct PhysicalFrameFaultWaiter {
    pub(crate) owner: Arc<PoolInner>,
    pub(crate) key: PhysicalFrameKey,
    pub(crate) identity: PhysicalFrameLoadingIdentity,
    pub(crate) armed: bool,
}

impl PhysicalFrameFaultWaiter {
    pub const fn loading_identity(&self) -> PhysicalFrameLoadingIdentity {
        self.identity
    }

    pub fn wait(mut self) -> Result<PhysicalFrameLease, PhysicalFrameLoadTerminal> {
        let outcome = self.owner.join_loading(self.key, self.identity);
        self.armed = false;
        outcome
    }
}

impl Drop for PhysicalFrameFaultWaiter {
    fn drop(&mut self) {
        if self.armed {
            self.owner.release_loading_waiter(self.key, self.identity);
        }
    }
}
