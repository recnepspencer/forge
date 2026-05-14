use std::collections::BTreeMap;

use super::AuthoritativeStateAdmissionDenial;
use crate::aspects::keys::AspectKey;
use crate::aspects::validation::ContractValidatedAspectValue;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalAspectStateMap {
    entries: BTreeMap<AspectKey, ContractValidatedAspectValue>,
}

impl CanonicalAspectStateMap {
    pub(crate) fn from_validated_entries(
        entries: impl IntoIterator<Item = ContractValidatedAspectValue>,
    ) -> Result<Self, AuthoritativeStateAdmissionDenial> {
        let mut canonical_entries = BTreeMap::new();
        for entry in entries {
            let key = entry.key().clone();
            if canonical_entries.insert(key.clone(), entry).is_some() {
                return Err(AuthoritativeStateAdmissionDenial::DuplicateAspectKey(key));
            }
        }

        Ok(Self {
            entries: canonical_entries,
        })
    }

    pub(crate) fn from_canonical_entries(
        entries: BTreeMap<AspectKey, ContractValidatedAspectValue>,
    ) -> Self {
        Self { entries }
    }

    pub fn entries(&self) -> impl Iterator<Item = (&AspectKey, &ContractValidatedAspectValue)> {
        self.entries.iter()
    }

    pub fn get(&self, key: &AspectKey) -> Option<&ContractValidatedAspectValue> {
        self.entries.get(key)
    }

    pub(crate) fn cloned_entries(&self) -> BTreeMap<AspectKey, ContractValidatedAspectValue> {
        self.entries.clone()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
