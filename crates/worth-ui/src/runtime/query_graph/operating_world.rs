use forge_query::facade::runtime::ForgeQueryGraphObligationOperatingWorldDescriptor;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryGraphOperatingWorld {
    descriptor: ForgeQueryGraphObligationOperatingWorldDescriptor,
}

impl WorthUiQueryGraphOperatingWorld {
    pub fn runtime_preview() -> Self {
        Self {
            descriptor: ForgeQueryGraphObligationOperatingWorldDescriptor::preview(),
        }
    }

    pub fn descriptor(&self) -> &ForgeQueryGraphObligationOperatingWorldDescriptor {
        &self.descriptor
    }

    pub fn descriptor_digest(&self) -> &str {
        self.descriptor.descriptor_digest()
    }
}
