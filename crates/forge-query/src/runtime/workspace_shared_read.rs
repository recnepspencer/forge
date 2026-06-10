use super::{
    ForgeQueryRuntimeError, ForgeQueryRuntimeFacadeFamily, ForgeQuerySharedReadContext,
    ForgeQueryWorkspace,
};

impl ForgeQueryWorkspace {
    pub fn shared_read_context(
        &self,
    ) -> Result<ForgeQuerySharedReadContext, ForgeQueryRuntimeError> {
        self.runtime
            .admit_facade_family(ForgeQueryRuntimeFacadeFamily::SharedRead)?;
        self.runtime.mint_shared_read_context()
    }
}
