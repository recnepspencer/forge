use crate::access::counters::PhysicalLayoutAccessCounterSnapshot;
use crate::access::grammar::PhysicalLayoutAccessFamily;
use crate::access::manifest::root_discovery::canonical_root_manifest;
use crate::{
    FreeSpaceReuseCell, InMemoryPhysicalFormatModel, InMemoryPhysicalFormatModelDenial,
    InMemoryPhysicalFormatModelDenialKind, PhysicalForegroundBoundednessOutcome,
    PhysicalFreeSpaceSearchPolicy,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FreeSpaceLayoutReport {
    entries: Vec<FreeSpaceReuseCell>,
    counters: PhysicalLayoutAccessCounterSnapshot,
    bounded: PhysicalForegroundBoundednessOutcome,
}

#[derive(Debug)]
pub struct FreeSpaceAccess<'a> {
    facade: &'a mut InMemoryPhysicalFormatModel,
}

impl<'a> FreeSpaceAccess<'a> {
    pub(crate) fn new(facade: &'a mut InMemoryPhysicalFormatModel) -> Self {
        Self { facade }
    }

    pub fn bounded_candidates(
        &mut self,
        policy: PhysicalFreeSpaceSearchPolicy,
    ) -> Result<FreeSpaceLayoutReport, InMemoryPhysicalFormatModelDenial> {
        let access = canonical_root_manifest(self.facade)?;
        let root = access.root();
        let pressure = policy.evaluate(
            root.allocation_classes().len() as u32,
            root.free_space().len() as u32,
        );
        if !pressure.is_admitted() {
            return Err(InMemoryPhysicalFormatModelDenial::new(
                InMemoryPhysicalFormatModelDenialKind::FullStoreMaterializationRejected,
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
