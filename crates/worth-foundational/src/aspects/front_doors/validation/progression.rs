use crate::AspectContract;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AspectValidationFrontDoor;

impl AspectValidationFrontDoor {
    pub fn against(self, contract: &AspectContract) -> AspectValidationInputStep<'_> {
        AspectValidationInputStep { contract }
    }
}

pub struct AspectValidationInputStep<'a> {
    pub(super) contract: &'a AspectContract,
}
