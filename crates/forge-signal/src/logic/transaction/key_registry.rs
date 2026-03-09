use std::collections::BTreeMap;
use std::num::NonZeroU32;

use crate::data::output::{ComputationFamily, ComputationKey, StructuralMemoKey};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) struct RuntimeStringId(NonZeroU32);

impl RuntimeStringId {
    pub(super) fn from_index(index: usize) -> Self {
        Self(NonZeroU32::new((index + 1) as u32).expect("runtime string ids are one-based"))
    }

    pub(super) fn index(self) -> usize {
        (self.0.get() - 1) as usize
    }
}

#[derive(Debug, Clone, Default)]
pub(super) struct RuntimeKeyRegistry {
    pub(super) families: Vec<ComputationFamily>,
    pub(super) family_lookup: BTreeMap<ComputationFamily, RuntimeStringId>,
    pub(super) keys: Vec<ComputationKey>,
    pub(super) key_lookup: BTreeMap<ComputationKey, RuntimeStringId>,
    pub(super) memo_keys: Vec<StructuralMemoKey>,
    pub(super) memo_key_lookup: BTreeMap<StructuralMemoKey, RuntimeStringId>,
}

impl RuntimeKeyRegistry {
    pub(super) fn intern_family(&mut self, family: &ComputationFamily) -> RuntimeStringId {
        if let Some(id) = self.family_lookup.get(family).copied() {
            return id;
        }
        let owned = family.clone();
        let id = RuntimeStringId::from_index(self.families.len());
        self.families.push(owned.clone());
        self.family_lookup.insert(owned, id);
        id
    }

    pub(super) fn intern_key(&mut self, key: &ComputationKey) -> RuntimeStringId {
        if let Some(id) = self.key_lookup.get(key).copied() {
            return id;
        }
        let owned = key.clone();
        let id = RuntimeStringId::from_index(self.keys.len());
        self.keys.push(owned.clone());
        self.key_lookup.insert(owned, id);
        id
    }

    pub(super) fn intern_memo_key(&mut self, memo_key: &StructuralMemoKey) -> RuntimeStringId {
        if let Some(id) = self.memo_key_lookup.get(memo_key).copied() {
            return id;
        }
        let owned = memo_key.clone();
        let id = RuntimeStringId::from_index(self.memo_keys.len());
        self.memo_keys.push(owned.clone());
        self.memo_key_lookup.insert(owned, id);
        id
    }

    pub(super) fn family(&self, id: RuntimeStringId) -> &ComputationFamily {
        &self.families[id.index()]
    }

    pub(super) fn memo_key(&self, id: RuntimeStringId) -> &StructuralMemoKey {
        &self.memo_keys[id.index()]
    }
}
