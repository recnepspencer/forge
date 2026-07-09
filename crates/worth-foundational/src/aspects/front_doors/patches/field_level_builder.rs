use worth_proof::TransitionOutcome;

use super::super::super::{
    AspectContract, AspectMask, AuthoritativePatchConstructionDenial,
    AuthoritativeRecordAspectPatch, FieldKey, MutationMask,
};
use crate::values::AspectValue;

pub struct FieldLevelPatchBuilder<'a> {
    contract: &'a AspectContract,
    mask: &'a AspectMask<MutationMask>,
    field_sets: Vec<(FieldKey, AspectValue)>,
    field_clears: Vec<FieldKey>,
}

impl<'a> FieldLevelPatchBuilder<'a> {
    pub fn new(contract: &'a AspectContract, mask: &'a AspectMask<MutationMask>) -> Self {
        Self {
            contract,
            mask,
            field_sets: Vec::new(),
            field_clears: Vec::new(),
        }
    }

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
