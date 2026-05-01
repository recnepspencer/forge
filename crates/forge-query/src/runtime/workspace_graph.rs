use super::{
    ForgeQueryBatchWriteReceipt, ForgeQueryGraphCompositionBuilder, ForgeQueryRuntimeError,
    ForgeQueryWorkspace,
};

impl ForgeQueryWorkspace {
    pub fn compose_graph(
        &mut self,
        declaration: impl FnOnce(
            &mut ForgeQueryGraphCompositionBuilder,
        ) -> Result<(), ForgeQueryRuntimeError>,
    ) -> Result<ForgeQueryBatchWriteReceipt, ForgeQueryRuntimeError> {
        let mut builder = ForgeQueryGraphCompositionBuilder::new();
        declaration(&mut builder)?;
        self.runtime.write_batch(builder.finish()?)
    }
}
