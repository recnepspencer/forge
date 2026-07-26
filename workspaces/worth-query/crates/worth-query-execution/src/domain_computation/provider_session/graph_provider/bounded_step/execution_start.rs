use super::{
    WorthQueryGraphProviderMemoryArena, WorthQueryGraphProviderRetainedMemory,
    WorthQueryGraphProviderStepDenial,
};

/// Provider-facing construction port bound to one admitted managed execution.
///
/// A provider can retain start-time state only by moving memory returned by
/// this port into its execution object.
pub struct WorthQueryGraphProviderExecutionStart {
    memory: WorthQueryGraphProviderMemoryArena,
    denial: Option<WorthQueryGraphProviderStepDenial>,
}

impl WorthQueryGraphProviderExecutionStart {
    pub(crate) fn new(memory: WorthQueryGraphProviderMemoryArena) -> Self {
        Self {
            memory,
            denial: None,
        }
    }

    pub fn retain_bytes(
        &mut self,
        byte_count: usize,
    ) -> Result<WorthQueryGraphProviderRetainedMemory, WorthQueryGraphProviderStepDenial> {
        if let Some(denial) = &self.denial {
            return Err(denial.clone());
        }
        match self.memory.retain_bytes(byte_count) {
            Ok(memory) => Ok(memory),
            Err(denial) => {
                self.denial = Some(denial.clone());
                Err(denial)
            }
        }
    }

    pub(crate) fn finish(self) -> Result<(), WorthQueryGraphProviderStepDenial> {
        self.denial.map_or(Ok(()), Err)
    }
}
