use super::super::{
    AspectContract, AspectKey, AspectMask, AuthoritativePatchConstructionDenial,
    AuthoritativeRecordAspectPatch, ContractValidatedAspectArtifact, FieldKey, MutationMask,
};
use crate::values::AspectValue;
use forge_proof::TransitionOutcome;

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
        FieldLevelPatchBuilder {
            contract,
            mask,
            field_sets: Vec::new(),
            field_clears: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct WholeAspectPatchBuilder {
    sets: Vec<ContractValidatedAspectArtifact>,
    clears: Vec<AspectKey>,
}

impl WholeAspectPatchBuilder {
    pub fn set(mut self, artifact: ContractValidatedAspectArtifact) -> Self {
        self.sets.push(artifact);
        self
    }

    pub fn clear(mut self, key: AspectKey) -> Self {
        self.clears.push(key);
        self
    }

    pub fn finish(
        self,
    ) -> TransitionOutcome<AuthoritativeRecordAspectPatch, AuthoritativePatchConstructionDenial>
    {
        AuthoritativeRecordAspectPatch::whole_aspect(self.sets, self.clears)
    }
}

pub struct FieldLevelPatchBuilder<'a> {
    contract: &'a AspectContract,
    mask: &'a AspectMask<MutationMask>,
    field_sets: Vec<(FieldKey, AspectValue)>,
    field_clears: Vec<FieldKey>,
}

impl<'a> FieldLevelPatchBuilder<'a> {
    pub fn set_field(mut self, key: FieldKey, value: AspectValue) -> Self {
        self.field_sets.push((key, value));
        self
    }

    pub fn clear_field(mut self, key: FieldKey) -> Self {
        self.field_clears.push(key);
        self
    }

    pub fn finish(
        self,
    ) -> TransitionOutcome<AuthoritativeRecordAspectPatch, AuthoritativePatchConstructionDenial>
    {
        AuthoritativeRecordAspectPatch::field_level(
            self.contract,
            self.mask,
            self.field_sets,
            self.field_clears,
        )
    }
}
