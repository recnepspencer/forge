use crate::protocol_bindings::{
    CompactionVisibilityOwnerCase, CompactionVisibilityOwnerCaseFamily,
};

use crate::runner::ProtocolCounterexample;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompactionVisibilityCounterexampleLocalization {
    counterexample: ProtocolCounterexample,
    owner_case: CompactionVisibilityOwnerCase,
    abstraction_function: CompactionVisibilityAbstractionFunction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompactionVisibilityAbstractionFunction {
    LsmMembershipObservation,
    LsmExecutionObservation,
    LsmMaintenanceObservation,
    PhysicalCompactionObservation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompactionVisibilityCounterexampleLocalizationDenial {
    owner_family: CompactionVisibilityOwnerCaseFamily,
    abstraction_family: CompactionVisibilityOwnerCaseFamily,
}

impl CompactionVisibilityCounterexampleLocalization {
    pub fn localize(
        counterexample: ProtocolCounterexample,
        owner_case: CompactionVisibilityOwnerCase,
        abstraction_function: CompactionVisibilityAbstractionFunction,
    ) -> Result<Self, CompactionVisibilityCounterexampleLocalizationDenial> {
        let owner_family = owner_case.family();
        let abstraction_family = abstraction_function.owner_family();
        if owner_family != abstraction_family {
            return Err(CompactionVisibilityCounterexampleLocalizationDenial {
                owner_family,
                abstraction_family,
            });
        }
        Ok(Self {
            counterexample,
            owner_case,
            abstraction_function,
        })
    }

    pub const fn counterexample(&self) -> &ProtocolCounterexample {
        &self.counterexample
    }

    pub const fn owner_case(&self) -> CompactionVisibilityOwnerCase {
        self.owner_case
    }

    pub const fn abstraction_function(&self) -> CompactionVisibilityAbstractionFunction {
        self.abstraction_function
    }
}

impl CompactionVisibilityAbstractionFunction {
    pub const fn owner_family(self) -> CompactionVisibilityOwnerCaseFamily {
        match self {
            Self::LsmMembershipObservation => CompactionVisibilityOwnerCaseFamily::LsmMembership,
            Self::LsmExecutionObservation => CompactionVisibilityOwnerCaseFamily::LsmExecution,
            Self::LsmMaintenanceObservation => CompactionVisibilityOwnerCaseFamily::LsmMaintenance,
            Self::PhysicalCompactionObservation => {
                CompactionVisibilityOwnerCaseFamily::PhysicalCompaction
            }
        }
    }
}

impl CompactionVisibilityCounterexampleLocalizationDenial {
    pub const fn owner_family(self) -> CompactionVisibilityOwnerCaseFamily {
        self.owner_family
    }

    pub const fn abstraction_family(self) -> CompactionVisibilityOwnerCaseFamily {
        self.abstraction_family
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol_bindings::ProtocolFamily;
    use worth_store_physical_isolation::CompactionOwnerCaseId;

    #[test]
    fn localization_rejects_an_abstraction_function_from_another_owner_family() {
        let counterexample = ProtocolCounterexample::diagnostic(
            ProtocolFamily::CompactionVisibility,
            vec!["rewrite-visible-before-publication".to_owned()],
        );
        let owner_case = CompactionVisibilityOwnerCase::PhysicalCompaction(
            CompactionOwnerCaseId::PublishRewrite,
        );

        let denial = CompactionVisibilityCounterexampleLocalization::localize(
            counterexample,
            owner_case,
            CompactionVisibilityAbstractionFunction::LsmExecutionObservation,
        )
        .expect_err("localization must retain the concrete mapping family");

        assert_eq!(
            denial.owner_family(),
            CompactionVisibilityOwnerCaseFamily::PhysicalCompaction
        );
        assert_eq!(
            denial.abstraction_family(),
            CompactionVisibilityOwnerCaseFamily::LsmExecution
        );
    }
}
