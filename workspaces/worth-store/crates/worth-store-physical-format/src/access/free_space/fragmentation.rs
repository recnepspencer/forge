use crate::access::counters::PhysicalLayoutAccessCounterSnapshot;
use crate::access::grammar::PhysicalLayoutAccessFamily;
use crate::access::manifest::root_discovery::canonical_root_manifest;
use crate::{
    InMemoryPhysicalFormatModel, InMemoryPhysicalFormatModelDenial,
    PhysicalFragmentationPressureReport, PhysicalFreeSpaceSearchPolicy,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FragmentationLayoutReport {
    pressure: PhysicalFragmentationPressureReport,
    counters: PhysicalLayoutAccessCounterSnapshot,
}

#[derive(Debug)]
pub struct FragmentationAccess<'a> {
    facade: &'a mut InMemoryPhysicalFormatModel,
}

impl<'a> FragmentationAccess<'a> {
    pub(crate) fn new(facade: &'a mut InMemoryPhysicalFormatModel) -> Self {
        Self { facade }
    }

    pub fn pressure(
        &mut self,
        policy: PhysicalFreeSpaceSearchPolicy,
    ) -> Result<FragmentationLayoutReport, InMemoryPhysicalFormatModelDenial> {
        let access = canonical_root_manifest(self.facade)?;
        let allocation_count = access.root().allocation_classes().len() as u32;
        let free_space_count = access.root().free_space().len() as u32;
        let _ = self.facade.mark_read();
        Ok(FragmentationLayoutReport {
            pressure: policy
                .evaluate(allocation_count, free_space_count)
                .pressure(),
            counters: access.manifest_traversal_counters(0),
        })
    }
}

impl FragmentationLayoutReport {
    pub fn family(&self) -> PhysicalLayoutAccessFamily {
        PhysicalLayoutAccessFamily::Fragmentation
    }

    pub const fn pressure(&self) -> PhysicalFragmentationPressureReport {
        self.pressure
    }

    pub const fn counters(&self) -> PhysicalLayoutAccessCounterSnapshot {
        self.counters
    }
}
