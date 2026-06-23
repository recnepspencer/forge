use super::{
    ForgeQueryRuntimeError, ForgeQueryRuntimeFacadeFamily, ForgeQuerySharedReadContext,
    ForgeQuerySharedReadCounters, ForgeQueryWorkspace,
};

impl ForgeQueryWorkspace {
    pub fn shared_read_context(
        &self,
    ) -> Result<ForgeQuerySharedReadContext, ForgeQueryRuntimeError> {
        self.runtime
            .admit_facade_family(ForgeQueryRuntimeFacadeFamily::SharedRead)?;
        self.runtime.mint_shared_read_context()
    }

    pub fn shared_read_counters(&self) -> ForgeQuerySharedReadCounters {
        self.runtime.shared_read_counters()
    }

    pub fn record_shared_read_hot_path_lock_for_certification(&self) {
        self.runtime
            .record_shared_read_hot_path_lock_for_certification();
    }
}
