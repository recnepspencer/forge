use std::collections::BTreeMap;

#[derive(Debug, Clone, Default)]
pub(crate) struct RecordSlotDirectory {
    logical_by_physical: Vec<u64>,
    physical_by_logical: BTreeMap<u64, usize>,
}

impl RecordSlotDirectory {
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            logical_by_physical: Vec::with_capacity(capacity),
            physical_by_logical: BTreeMap::new(),
        }
    }

    pub(crate) fn restore(slots: Vec<u64>) -> Result<Self, &'static str> {
        let mut directory = Self::with_capacity(slots.len());
        for slot in slots {
            directory.insert(slot)?;
        }
        Ok(directory)
    }

    pub(crate) fn physical_index(&self, logical_slot: usize) -> Option<usize> {
        self.physical_by_logical
            .get(&(logical_slot as u64))
            .copied()
    }

    pub(crate) fn insert(&mut self, logical_slot: u64) -> Result<usize, &'static str> {
        if self.physical_by_logical.contains_key(&logical_slot) {
            return Err("record slot directory contains a duplicate logical slot");
        }
        let physical = self.logical_by_physical.len();
        self.logical_by_physical.push(logical_slot);
        self.physical_by_logical.insert(logical_slot, physical);
        Ok(physical)
    }

    pub(crate) fn occupied_slots(&self) -> Vec<usize> {
        self.physical_by_logical
            .keys()
            .map(|slot| *slot as usize)
            .collect()
    }

    pub(crate) fn slots(&self) -> &[u64] {
        &self.logical_by_physical
    }

    pub(crate) fn len(&self) -> usize {
        self.logical_by_physical.len()
    }

    pub(crate) fn allocation_bytes(&self) -> u64 {
        (self.logical_by_physical.capacity() as u64)
            .saturating_mul(std::mem::size_of::<u64>() as u64)
            .saturating_add(
                (self.physical_by_logical.len() as u64).saturating_mul(
                    (std::mem::size_of::<u64>() + std::mem::size_of::<usize>()) as u64,
                ),
            )
    }
}
