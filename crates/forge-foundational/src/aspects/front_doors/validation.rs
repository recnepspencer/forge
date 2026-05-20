use super::super::{
    validate_aspect_value, AspectContract, ContractValidatedAspectArtifact,
    ContractValidationDenial, ContractValidationInput,
};
use forge_proof::TransitionOutcome;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AspectValidationFrontDoor;

impl AspectValidationFrontDoor {
    pub fn against(self, contract: &AspectContract) -> AspectValidationInputStep<'_> {
        AspectValidationInputStep { contract }
    }
}

pub struct AspectValidationInputStep<'a> {
    contract: &'a AspectContract,
}

impl<'a> AspectValidationInputStep<'a> {
    pub fn value(
        self,
        input: impl Into<ContractValidationInput>,
    ) -> TransitionOutcome<ContractValidatedAspectArtifact, ContractValidationDenial> {
        validate_aspect_value(self.contract, input.into())
    }
}
