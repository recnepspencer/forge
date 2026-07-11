#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StorageBoundaryInterposerDriver {
    backend_profile: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StorageBoundaryEvent {
    seam: &'static str,
    backend_profile: &'static str,
    fault_ordinal: u16,
}

impl StorageBoundaryInterposerDriver {
    pub const fn production_like(backend_profile: &'static str) -> Self {
        Self { backend_profile }
    }

    pub const fn backend_profile(&self) -> &'static str {
        self.backend_profile
    }

    pub const fn lower_boundary_event(
        &self,
        seam: &'static str,
        fault_ordinal: u16,
    ) -> StorageBoundaryEvent {
        StorageBoundaryEvent {
            seam,
            backend_profile: self.backend_profile,
            fault_ordinal,
        }
    }
}

impl StorageBoundaryEvent {
    pub const fn seam(&self) -> &'static str {
        self.seam
    }

    pub const fn backend_profile(&self) -> &'static str {
        self.backend_profile
    }

    pub const fn fault_ordinal(&self) -> u16 {
        self.fault_ordinal
    }
}
