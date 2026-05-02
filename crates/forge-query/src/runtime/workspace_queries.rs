use serde_json::Value;

use super::{
    ForgeQueryDerivedViewHandle, ForgeQueryInspection, ForgeQueryInspectionTarget,
    ForgeQueryInstalledProgram, ForgeQueryPatchBatch, ForgeQueryProgram, ForgeQueryRuntimeError,
    ForgeQueryWorkspace,
};

impl ForgeQueryWorkspace {
    pub fn read<T>(
        &self,
        view: &super::ForgeQueryLiveView<T>,
    ) -> Vec<crate::memory_workspace::ForgeQueryEntity> {
        self.runtime.read_live(view)
    }

    pub fn observe<T>(&mut self, view: &super::ForgeQueryLiveView<T>) -> ForgeQueryPatchBatch {
        self.runtime.drain_patches(view)
    }

    pub fn materialize<T>(&self, view: &ForgeQueryDerivedViewHandle<T>) -> Vec<Value> {
        self.runtime.read_derived(view)
    }

    pub fn observe_computed(&mut self, view_name: &str) -> ForgeQueryPatchBatch {
        self.runtime.drain_derived_patches(view_name)
    }

    pub fn install_program(
        &mut self,
        program: ForgeQueryProgram,
    ) -> Result<ForgeQueryInstalledProgram, ForgeQueryRuntimeError> {
        self.runtime.install_program(program)
    }

    pub fn inspect<'a, T>(
        &'a self,
        target: T,
    ) -> Result<ForgeQueryInspection, ForgeQueryRuntimeError>
    where
        T: Into<ForgeQueryInspectionTarget<'a>>,
    {
        self.runtime.inspect(target)
    }
}
