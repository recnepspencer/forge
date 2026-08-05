//! Installation-owned compilation of capability delegation meaning.

use worth_query_declaration::facade::{
    application_capability::{
        application_capability_delegation_activation_program_targets,
        ErasedApplicationCapabilityContract,
    },
    application_schema::ApplicationOperationProgramTarget,
};

pub(super) struct CompiledApplicationCapabilityDelegation {
    activation_program: Option<Vec<ApplicationOperationProgramTarget>>,
}

impl CompiledApplicationCapabilityDelegation {
    pub(super) fn compile(contract: &ErasedApplicationCapabilityContract) -> Self {
        Self {
            activation_program: application_capability_delegation_activation_program_targets(
                contract,
            ),
        }
    }

    pub(super) fn activation_program(&self) -> Option<&[ApplicationOperationProgramTarget]> {
        self.activation_program.as_deref()
    }
}
