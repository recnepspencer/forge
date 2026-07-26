use super::{WorthQueryGraphProviderMemoryArena, WorthQueryGraphProviderRetainedMemory};
use crate::domain_computation::{
    WorthQueryGraphProviderStepDenial, WorthQueryGraphProviderStepDenialKind,
};

/// Step-external port that can provisionally rebind retained provider memory
/// during the existing same-runtime readmission transaction.
pub struct WorthQueryGraphProviderRestoreMemory {
    arena: WorthQueryGraphProviderMemoryArena,
}

impl WorthQueryGraphProviderRestoreMemory {
    pub(crate) fn new(arena: WorthQueryGraphProviderMemoryArena) -> Self {
        Self { arena }
    }

    pub fn rebind(
        &mut self,
        retained: &WorthQueryGraphProviderRetainedMemory,
    ) -> Result<WorthQueryGraphProviderRetainedMemory, WorthQueryGraphProviderStepDenial> {
        let expected = self.arena.snapshot().arena_identity();
        if retained.arena_identity() != expected {
            return Err(WorthQueryGraphProviderStepDenial::new(
                WorthQueryGraphProviderStepDenialKind::ForeignRetainedMemory,
                "provider restore memory belongs to a different managed execution",
            ));
        }
        Ok(retained.provisional_restore_alias())
    }
}
