use super::{WorthQueryRuntimeBatchAuthority, WorthQueryRuntimeSupportProfile};

impl WorthQueryRuntimeSupportProfile {
    pub fn batch_authority(&self) -> WorthQueryRuntimeBatchAuthority {
        self.batch_authority
    }

    pub fn with_direct_atomic_batch_authority(mut self) -> Self {
        self.batch_authority = WorthQueryRuntimeBatchAuthority::BackendAtomicDirect;
        self
    }
}
