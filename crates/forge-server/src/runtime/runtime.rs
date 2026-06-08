use super::ForgeServerRuntimeAssembly;

#[derive(Debug)]
pub(crate) struct ForgeServerRuntime {
    assembly: ForgeServerRuntimeAssembly,
}

impl ForgeServerRuntime {
    pub(crate) fn from_assembly(assembly: ForgeServerRuntimeAssembly) -> Self {
        Self { assembly }
    }

    pub(crate) fn assembly(&self) -> &ForgeServerRuntimeAssembly {
        &self.assembly
    }
}
