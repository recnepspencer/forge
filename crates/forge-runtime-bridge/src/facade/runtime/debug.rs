use super::*;

impl std::fmt::Debug for RuntimeBridge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RuntimeBridge")
            .field("policy", &self.policy)
            .field("diagnostics", &self.diagnostics)
            .field(
                "mapping_registration_count",
                &self.mapping_registry.registrations().len(),
            )
            .field(
                "aspect_registration_count",
                &self.aspect_registry.registrations().len(),
            )
            .finish_non_exhaustive()
    }
}

