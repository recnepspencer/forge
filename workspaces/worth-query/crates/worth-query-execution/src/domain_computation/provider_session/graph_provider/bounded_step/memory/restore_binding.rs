use super::{WorthQueryGraphProviderMemoryArena, WorthQueryGraphProviderRetainedMemory};
use crate::domain_computation::provider_session::graph_provider::bounded_step::WorthQueryCooperativeGraphProviderExecution;
use crate::domain_computation::{
    WorthQueryGraphProviderStepDenial, WorthQueryGraphProviderStepDenialKind,
};

/// Step-external port that can provisionally rebind retained provider memory
/// during the existing same-runtime readmission transaction.
pub struct WorthQueryGraphProviderRestoreMemory {
    arena: WorthQueryGraphProviderMemoryArena,
    execution_admission_issued: bool,
    denial: Option<WorthQueryGraphProviderStepDenial>,
}

impl WorthQueryGraphProviderRestoreMemory {
    pub(crate) fn new(arena: WorthQueryGraphProviderMemoryArena) -> Self {
        Self {
            arena,
            execution_admission_issued: false,
            denial: None,
        }
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

    pub fn admit_cooperative_execution<E>(
        &mut self,
        execution: E,
    ) -> Result<WorthQueryCooperativeGraphProviderExecution<E>, WorthQueryGraphProviderStepDenial>
    {
        if let Some(denial) = &self.denial {
            return Err(denial.clone());
        }
        if self.execution_admission_issued {
            let denial = WorthQueryGraphProviderStepDenial::new(
                WorthQueryGraphProviderStepDenialKind::MultipleExecutionAdmissions,
                "provider restore can admit exactly one cooperative execution",
            );
            self.denial = Some(denial.clone());
            return Err(denial);
        }
        self.execution_admission_issued = true;
        Ok(WorthQueryCooperativeGraphProviderExecution::new(
            self.arena.snapshot().arena_identity(),
            execution,
        ))
    }

    pub(crate) fn validate_returned_execution<E>(
        &mut self,
        admitted: WorthQueryCooperativeGraphProviderExecution<E>,
    ) -> Result<E, crate::domain_computation::WorthQueryGraphProviderFailure> {
        if let Some(denial) = &self.denial {
            return Err(
                crate::domain_computation::WorthQueryGraphProviderFailure::new(denial.detail()),
            );
        }
        if admitted.arena_identity() != self.arena.snapshot().arena_identity() {
            self.denial = Some(WorthQueryGraphProviderStepDenial::new(
                WorthQueryGraphProviderStepDenialKind::ForeignExecutionAdmission,
                "cooperative provider execution belongs to another managed restore",
            ));
            return Err(
                crate::domain_computation::WorthQueryGraphProviderFailure::new(
                    self.denial
                        .as_ref()
                        .expect("foreign restore admission latches a denial")
                        .detail(),
                ),
            );
        }
        Ok(admitted.into_execution())
    }
}
