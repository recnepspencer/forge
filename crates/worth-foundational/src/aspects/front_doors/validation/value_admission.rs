use super::AspectValidationInputStep;
use crate::{
    validate_aspect_value, ContractValidatedAspectArtifact, ContractValidationDenial,
    ContractValidationInput,
};
use worth_proof::TransitionOutcome;

impl<'a> AspectValidationInputStep<'a> {
    pub fn value(
        self,
        input: impl Into<ContractValidationInput>,
    ) -> TransitionOutcome<ContractValidatedAspectArtifact, ContractValidationDenial> {
        validate_aspect_value(self.contract, input.into())
    }
}
