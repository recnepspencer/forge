use super::counters::PhysicalLayoutAccessCounterSnapshot;
use super::grammar::PhysicalLayoutAccessFamily;
use super::root_discovery_family::canonical_root_manifest;
use crate::{
    PhysicalFragmentationPressureReport, PhysicalFreeSpaceSearchPolicy, PlatformPhysicalFacade,
    PlatformPhysicalFacadeDenial,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FragmentationLayoutFamilyHome;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FragmentationLayoutFamilyAdmission;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FragmentationLayoutReport {
    pressure: PhysicalFragmentationPressureReport,
    counters: PhysicalLayoutAccessCounterSnapshot,
}

impl FragmentationLayoutFamilyHome {
    pub const fn physical() -> Self {
        Self
    }

    pub fn admit(
        &self,
    ) -> Result<FragmentationLayoutFamilyAdmission, PlatformPhysicalFacadeDenial> {
        Ok(FragmentationLayoutFamilyAdmission)
    }
}

#[derive(Debug)]
pub struct AdmittedFragmentationLayoutFamily<'a> {
    facade: &'a mut PlatformPhysicalFacade,
    _admission: FragmentationLayoutFamilyAdmission,
}

impl<'a> AdmittedFragmentationLayoutFamily<'a> {
    pub(crate) fn new(
        facade: &'a mut PlatformPhysicalFacade,
        admission: FragmentationLayoutFamilyAdmission,
    ) -> Self {
        Self {
            facade,
            _admission: admission,
        }
    }

    pub fn pressure(
        &mut self,
        policy: PhysicalFreeSpaceSearchPolicy,
    ) -> Result<FragmentationLayoutReport, PlatformPhysicalFacadeDenial> {
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
