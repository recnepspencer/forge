use forge_store_recovery_physics::RecoveryCompletion;

#[derive(Debug, Clone, Copy)]
pub struct PhysicalIsolationEntryRequest<'a> {
    recovery_completion: &'a RecoveryCompletion,
}

impl<'a> PhysicalIsolationEntryRequest<'a> {
    pub const fn from_recovery_completion(recovery_completion: &'a RecoveryCompletion) -> Self {
        Self {
            recovery_completion,
        }
    }

    pub const fn recovery_completion(&self) -> &'a RecoveryCompletion {
        self.recovery_completion
    }
}
