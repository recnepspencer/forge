use super::UiNativeReadinessGrant;

impl UiNativeReadinessGrant {
    pub(in crate::native::event_loop) const fn issued(
        generation: u64,
        surface_basis_generation: u64,
        scale_factor_milli: u32,
        client_physical_size: [u32; 2],
    ) -> Self {
        Self {
            generation,
            surface_basis_generation,
            scale_factor_milli,
            client_physical_size,
        }
    }

    pub const fn generation(&self) -> u64 {
        self.generation
    }

    pub const fn surface_basis_generation(&self) -> u64 {
        self.surface_basis_generation
    }

    pub const fn scale_factor_milli(&self) -> u32 {
        self.scale_factor_milli
    }

    pub const fn client_physical_size(&self) -> [u32; 2] {
        self.client_physical_size
    }
}
