mod field_level_builder;
mod whole_aspect_builder;

pub use field_level_builder::FieldLevelPatchBuilder;
pub use whole_aspect_builder::WholeAspectPatchBuilder;

use super::super::{AspectContract, AspectMask, MutationMask};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AspectPatchFrontDoor;

impl AspectPatchFrontDoor {
    pub fn whole_aspect(self) -> WholeAspectPatchBuilder {
        WholeAspectPatchBuilder::default()
    }

    pub fn field_level<'a>(
        self,
        contract: &'a AspectContract,
        mask: &'a AspectMask<MutationMask>,
    ) -> FieldLevelPatchBuilder<'a> {
        FieldLevelPatchBuilder::new(contract, mask)
    }
}
