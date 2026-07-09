mod contract;
mod masks;
mod patches;
mod state;
mod validation;
mod vocabulary;

pub use contract::{
    AspectContractFrontDoor, AspectContractIdentityStep, AspectContractRevisionStep,
    AspectContractShapeStep,
};
pub use masks::{
    AspectMaskContractFrontDoor, DiagnosticMaskFrontDoor, MutationMaskFrontDoor,
    ProjectionMaskFrontDoor, StructFieldBuilder, StructFieldsFrontDoor,
};
pub use patches::{AspectPatchFrontDoor, FieldLevelPatchBuilder, WholeAspectPatchBuilder};
pub use state::AuthoritativeStateFrontDoor;
pub use validation::{AspectValidationFrontDoor, AspectValidationInputStep};
pub use vocabulary::{
    AspectFrontDoorConstructionDenial, AspectVocabularyFrontDoor, StructValueBuilder,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AspectsFrontDoor;

impl AspectsFrontDoor {
    pub const fn contract(self) -> AspectContractFrontDoor {
        AspectContractFrontDoor
    }

    pub const fn struct_fields(self) -> StructFieldsFrontDoor {
        StructFieldsFrontDoor
    }

    pub const fn mask_contract(self) -> AspectMaskContractFrontDoor {
        AspectMaskContractFrontDoor
    }

    pub const fn projection_mask(self) -> ProjectionMaskFrontDoor {
        ProjectionMaskFrontDoor
    }

    pub const fn mutation_mask(self) -> MutationMaskFrontDoor {
        MutationMaskFrontDoor
    }

    pub const fn diagnostic_mask(self) -> DiagnosticMaskFrontDoor {
        DiagnosticMaskFrontDoor
    }

    pub const fn validate(self) -> AspectValidationFrontDoor {
        AspectValidationFrontDoor
    }

    pub const fn authoritative_state(self) -> AuthoritativeStateFrontDoor {
        AuthoritativeStateFrontDoor
    }

    pub const fn patch(self) -> AspectPatchFrontDoor {
        AspectPatchFrontDoor
    }

    pub const fn vocabulary(self) -> AspectVocabularyFrontDoor {
        AspectVocabularyFrontDoor
    }
}

pub fn aspects() -> AspectsFrontDoor {
    AspectsFrontDoor
}
