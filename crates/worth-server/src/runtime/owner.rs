use super::WorthServerRuntimeAssembly;

#[derive(Debug)]
pub(crate) struct WorthServerRuntime {
    assembly: WorthServerRuntimeAssembly,
}

impl WorthServerRuntime {
    pub(crate) fn from_assembly(assembly: WorthServerRuntimeAssembly) -> Self {
        Self { assembly }
    }

    pub(crate) fn assembly(&self) -> &WorthServerRuntimeAssembly {
        &self.assembly
    }
}
