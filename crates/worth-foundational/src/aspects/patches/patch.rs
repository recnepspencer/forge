use std::collections::{BTreeMap, BTreeSet};

use crate::aspects::contracts::AspectContract;
use crate::aspects::keys::AspectKey;
use crate::aspects::masks::{AspectMask, MutationMask};
use crate::aspects::structs::FieldKey;
use crate::aspects::validation::ContractValidatedAspectValue;
use crate::values::AspectValue;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthoritativeRecordAspectPatch {
    pub(crate) whole_aspect_sets: BTreeMap<AspectKey, ContractValidatedAspectValue>,
    pub(crate) whole_aspect_clears: BTreeMap<AspectKey, AspectContract>,
    pub(crate) field_patches: BTreeMap<AspectKey, FieldLevelAspectPatch>,
}

impl AuthoritativeRecordAspectPatch {
    pub fn whole_aspect_sets(
        &self,
    ) -> impl Iterator<Item = (&AspectKey, &ContractValidatedAspectValue)> {
        self.whole_aspect_sets.iter()
    }

    pub fn whole_aspect_clears(&self) -> impl Iterator<Item = &AspectKey> {
        self.whole_aspect_clears.keys()
    }

    pub fn whole_aspect_clear_contracts(
        &self,
    ) -> impl Iterator<Item = (&AspectKey, &AspectContract)> {
        self.whole_aspect_clears.iter()
    }

    pub fn field_patches(&self) -> impl Iterator<Item = (&AspectKey, &FieldLevelAspectPatch)> {
        self.field_patches.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.whole_aspect_sets.is_empty()
            && self.whole_aspect_clears.is_empty()
            && self.field_patches.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldLevelAspectPatch {
    pub(crate) contract: AspectContract,
    pub(crate) mask: AspectMask<MutationMask>,
    pub(crate) field_sets: BTreeMap<FieldKey, AspectValue>,
    pub(crate) field_clears: BTreeSet<FieldKey>,
}

impl FieldLevelAspectPatch {
    pub fn key(&self) -> &AspectKey {
        self.contract.key()
    }

    pub fn contract(&self) -> &AspectContract {
        &self.contract
    }

    pub fn mask(&self) -> &AspectMask<MutationMask> {
        &self.mask
    }

    pub fn field_sets(&self) -> impl Iterator<Item = (&FieldKey, &AspectValue)> {
        self.field_sets.iter()
    }

    pub fn field_clears(&self) -> impl Iterator<Item = &FieldKey> {
        self.field_clears.iter()
    }
}
