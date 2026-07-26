use super::{
    WorthQueryCooperativeGraphProviderExecution, WorthQueryGraphProviderMemoryArena,
    WorthQueryGraphProviderRetainedMemory, WorthQueryGraphProviderStepDenial,
    WorthQueryGraphProviderStepDenialKind,
};

/// Provider-facing construction port bound to one admitted managed execution.
///
/// A provider can retain start-time state only by moving memory returned by
/// this port into its execution object.
pub struct WorthQueryGraphProviderExecutionStart {
    memory: WorthQueryGraphProviderMemoryArena,
    denial: Option<WorthQueryGraphProviderStepDenial>,
    execution_admission_issued: bool,
}

impl WorthQueryGraphProviderExecutionStart {
    pub(crate) fn new(memory: WorthQueryGraphProviderMemoryArena) -> Self {
        Self {
            memory,
            denial: None,
            execution_admission_issued: false,
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

    pub fn admit_cooperative_execution<E>(
        &mut self,
        execution: E,
    ) -> Result<
        WorthQueryCooperativeGraphProviderExecution<E>,
        WorthQueryGraphProviderStepDenial,
    > {
        if let Some(denial) = &self.denial {
            return Err(denial.clone());
        }
        if self.execution_admission_issued {
            let denial = WorthQueryGraphProviderStepDenial::new(
                WorthQueryGraphProviderStepDenialKind::MultipleExecutionAdmissions,
                "provider start can admit exactly one cooperative execution",
            );
            self.denial = Some(denial.clone());
            return Err(denial);
        }
        self.execution_admission_issued = true;
        Ok(WorthQueryCooperativeGraphProviderExecution::new(
            self.memory.snapshot().arena_identity(),
            execution,
        ))
    }

    pub(crate) fn validate_returned_execution<E>(
        &mut self,
        admitted: WorthQueryCooperativeGraphProviderExecution<E>,
    ) -> Result<E, crate::domain_computation::WorthQueryGraphProviderFailure> {
        if admitted.arena_identity() != self.memory.snapshot().arena_identity() {
            let denial = WorthQueryGraphProviderStepDenial::new(
                WorthQueryGraphProviderStepDenialKind::ForeignExecutionAdmission,
                "cooperative provider execution belongs to another managed start",
            );
            self.denial = Some(denial.clone());
            return Err(crate::domain_computation::WorthQueryGraphProviderFailure::new(
                denial.detail(),
            ));
        }
        Ok(admitted.into_execution())
    }

    pub(crate) fn finish(self) -> Result<(), WorthQueryGraphProviderStepDenial> {
        self.denial.map_or(Ok(()), Err)
    }
}
