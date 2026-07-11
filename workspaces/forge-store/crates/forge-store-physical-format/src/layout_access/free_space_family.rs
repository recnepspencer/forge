use super::counters::PhysicalLayoutAccessCounterSnapshot;
use super::grammar::{AdmittedFreeSpaceLayoutRule, PhysicalLayoutAccessFamily};
use super::root_discovery_family::canonical_root_manifest;
use crate::{
    FreeSpaceReuseCell, PhysicalForegroundBoundednessOutcome, PhysicalFreeSpaceSearchPolicy,
    PlatformPhysicalFacade, PlatformPhysicalFacadeDenial, PlatformPhysicalFacadeDenialKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FreeSpaceLayoutFamilyHome;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FreeSpaceLayoutFamilyAdmission;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FreeSpaceLayoutReport {
    entries: Vec<FreeSpaceReuseCell>,
    counters: PhysicalLayoutAccessCounterSnapshot,
    bounded: PhysicalForegroundBoundednessOutcome,
}

impl FreeSpaceLayoutFamilyHome {
    pub const fn s8() -> Self {
        Self
    }

    pub fn admit(
        &self,
        _rule: &AdmittedFreeSpaceLayoutRule,
    ) -> Result<FreeSpaceLayoutFamilyAdmission, PlatformPhysicalFacadeDenial> {
        Ok(FreeSpaceLayoutFamilyAdmission)
    }
}

#[derive(Debug)]
pub struct AdmittedFreeSpaceLayoutFamily<'a> {
    facade: &'a mut PlatformPhysicalFacade,
    _admission: FreeSpaceLayoutFamilyAdmission,
}

impl<'a> AdmittedFreeSpaceLayoutFamily<'a> {
    pub(crate) fn new(
        facade: &'a mut PlatformPhysicalFacade,
        admission: FreeSpaceLayoutFamilyAdmission,
    ) -> Self {
        Self {
            facade,
            _admission: admission,
        }
    }

    pub fn bounded_candidates(
        &mut self,
        policy: PhysicalFreeSpaceSearchPolicy,
    ) -> Result<FreeSpaceLayoutReport, PlatformPhysicalFacadeDenial> {
        let access = canonical_root_manifest(self.facade)?;
        let root = access.root();
        let pressure = policy.evaluate(
            root.allocation_classes().len() as u32,
            root.free_space().len() as u32,
        );
        if !pressure.is_admitted() {
            return Err(PlatformPhysicalFacadeDenial::new(
                PlatformPhysicalFacadeDenialKind::FullStoreMaterializationRejected,
            ));
        }
        let limit = policy.foreground_candidate_bound() as usize;
        let entries: Vec<_> = root
            .free_space()
            .iter()
            .take(limit)
            .map(|entry| entry.reuse_cell())
            .collect();
        let visited = root
            .free_space()
            .len()
            .min(limit)
            .try_into()
            .unwrap_or(u16::MAX);
        let _ = self.facade.mark_read();
        Ok(FreeSpaceLayoutReport {
            entries,
            counters: access.manifest_traversal_counters(u32::from(visited)),
            bounded: pressure.outcome(),
        })
    }
}

impl FreeSpaceLayoutReport {
    pub fn family(&self) -> PhysicalLayoutAccessFamily {
        PhysicalLayoutAccessFamily::FreeSpace
    }

    pub fn entries(&self) -> &[FreeSpaceReuseCell] {
        &self.entries
    }

    pub const fn counters(&self) -> PhysicalLayoutAccessCounterSnapshot {
        self.counters
    }

    pub const fn bounded(&self) -> PhysicalForegroundBoundednessOutcome {
        self.bounded
    }
}
