use super::counters::PhysicalLayoutAccessCounterSnapshot;
use super::grammar::PhysicalLayoutAccessFamily;
use super::manifest::root_discovery::canonical_root_manifest;
use crate::{AllocationClassKind, PhysicalStoreRuntime, PhysicalStoreRuntimeDenial};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllocationLayoutReport {
    classes: Vec<AllocationClassKind>,
    counters: PhysicalLayoutAccessCounterSnapshot,
}

#[derive(Debug)]
pub struct AllocationAccess<'a> {
    facade: &'a mut PhysicalStoreRuntime,
}

impl<'a> AllocationAccess<'a> {
    pub(crate) fn new(facade: &'a mut PhysicalStoreRuntime) -> Self {
        Self { facade }
    }

    pub fn allocation_classes(
        &mut self,
    ) -> Result<AllocationLayoutReport, PhysicalStoreRuntimeDenial> {
        let access = canonical_root_manifest(self.facade)?;
        let classes: Vec<_> = access
            .root()
            .allocation_classes()
            .iter()
            .map(|entry| entry.allocation_class())
            .collect();
        let _ = self.facade.mark_read();
        Ok(AllocationLayoutReport {
            classes,
            counters: access.manifest_traversal_counters(
                access
                    .root()
                    .allocation_classes()
                    .len()
                    .try_into()
                    .unwrap_or(u32::MAX),
            ),
        })
    }
}

impl AllocationLayoutReport {
    pub fn family(&self) -> PhysicalLayoutAccessFamily {
        PhysicalLayoutAccessFamily::Allocation
    }

    pub fn classes(&self) -> &[AllocationClassKind] {
        &self.classes
    }

    pub const fn counters(&self) -> PhysicalLayoutAccessCounterSnapshot {
        self.counters
    }
}
