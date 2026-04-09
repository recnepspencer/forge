use super::*;

impl RuntimeBridge {
    pub fn source_registry(&self) -> &AdmittedSourceRegistry {
        &self.source_registry
    }

    pub fn structural_registry(&self) -> &AdmittedStructuralRegistry {
        &self.structural_registry
    }

    pub fn source_adapter(&self) -> Option<&Arc<dyn BridgeSourceAdapter>> {
        self.source_adapter.as_ref()
    }
}
