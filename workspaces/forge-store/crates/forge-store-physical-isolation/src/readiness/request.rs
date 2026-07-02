use forge_store_recovery_physics::S5PhysicalIsolationRecoveryReadiness;

#[derive(Debug, Clone, Copy)]
pub struct PhysicalIsolationEntryRequest<'a> {
    recovery_readiness: &'a S5PhysicalIsolationRecoveryReadiness,
}

impl<'a> PhysicalIsolationEntryRequest<'a> {
    pub const fn from_s4_recovery_readiness(
        recovery_readiness: &'a S5PhysicalIsolationRecoveryReadiness,
    ) -> Self {
        Self { recovery_readiness }
    }

    pub const fn recovery_readiness(&self) -> &'a S5PhysicalIsolationRecoveryReadiness {
        self.recovery_readiness
    }
}
