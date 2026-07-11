use super::counters::PhysicalLayoutAccessCounterSnapshot;
use super::grammar::{AdmittedAllocationLayoutRule, PhysicalLayoutAccessFamily};
use super::root_discovery_family::canonical_root_manifest;
use crate::{AllocationClassKind, PlatformPhysicalFacade, PlatformPhysicalFacadeDenial};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AllocationLayoutFamilyHome;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AllocationLayoutFamilyAdmission;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllocationLayoutReport {
    classes: Vec<AllocationClassKind>,
    counters: PhysicalLayoutAccessCounterSnapshot,
}

impl AllocationLayoutFamilyHome {
    pub const fn s8() -> Self {
        Self
    }

    pub fn admit(
        &self,
        _rule: &AdmittedAllocationLayoutRule,
    ) -> Result<AllocationLayoutFamilyAdmission, PlatformPhysicalFacadeDenial> {
        Ok(AllocationLayoutFamilyAdmission)
    }
}

#[derive(Debug)]
pub struct AdmittedAllocationLayoutFamily<'a> {
    facade: &'a mut PlatformPhysicalFacade,
    _admission: AllocationLayoutFamilyAdmission,
}

impl<'a> AdmittedAllocationLayoutFamily<'a> {
    pub(crate) fn new(
        facade: &'a mut PlatformPhysicalFacade,
        admission: AllocationLayoutFamilyAdmission,
    ) -> Self {
        Self {
            facade,
            _admission: admission,
        }
    }

    pub fn allocation_classes(
        &mut self,
    ) -> Result<AllocationLayoutReport, PlatformPhysicalFacadeDenial> {
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
