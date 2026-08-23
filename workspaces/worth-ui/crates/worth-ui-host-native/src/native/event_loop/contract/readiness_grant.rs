use super::UiNativeReadinessGrant;

impl UiNativeReadinessGrant {
    pub(in crate::native::event_loop) const fn issued(
        generation: u64,
        scale_factor_milli: u32,
        client_physical_size: [u32; 2],
    ) -> Self {
        Self {
            generation,
            scale_factor_milli,
            client_physical_size,
        }
    }

    pub const fn generation(&self) -> u64 {
        self.generation
    }

    pub const fn scale_factor_milli(&self) -> u32 {
        self.scale_factor_milli
    }

    pub const fn client_physical_size(&self) -> [u32; 2] {
        self.client_physical_size
    }
}
