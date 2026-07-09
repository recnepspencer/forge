use super::{
    lower_json_aspect_value, lower_json_record_aspect_state, JsonCompatibilityAspectInput,
    JsonCompatibilityLoweringOutcome,
};
use crate::aspects::{AspectContract, ContractValidatedAspectArtifact};
use crate::locators::BoundarySourceLocator;
use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CompatibilityFrontDoor;

impl CompatibilityFrontDoor {
    pub const fn json(self) -> JsonCompatibilityFrontDoor {
        JsonCompatibilityFrontDoor
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct JsonCompatibilityFrontDoor;

impl JsonCompatibilityFrontDoor {
    pub fn input(
        self,
        contract: AspectContract,
        source: BoundarySourceLocator,
        value: Value,
    ) -> JsonCompatibilityAspectInput {
        JsonCompatibilityAspectInput::new(contract, source, value)
    }

    pub fn lower_value(
        self,
        contract: &AspectContract,
        source: BoundarySourceLocator,
        value: &Value,
    ) -> JsonCompatibilityLoweringOutcome<ContractValidatedAspectArtifact> {
        lower_json_aspect_value(contract, source, value)
    }

    pub fn lower_state(
        self,
        inputs: impl IntoIterator<Item = JsonCompatibilityAspectInput>,
    ) -> JsonCompatibilityLoweringOutcome<crate::aspects::AuthoritativeRecordAspectStateArtifact>
    {
        lower_json_record_aspect_state(inputs)
    }
}

pub fn compatibility() -> CompatibilityFrontDoor {
    CompatibilityFrontDoor
}
