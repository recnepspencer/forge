use super::{storage::PlatformPhysicalFacadeStorage, PlatformPhysicalFacadeCounterSnapshot};
use crate::{
    PhysicalHeaderAuthority, PhysicalPageRecordAuthority, PhysicalReference,
    PhysicalReferenceAuthority, PlatformPhysicalFacadeDenial, PlatformPhysicalFacadeDenialKind,
};
use forge_store_contracts::RoadmapScope;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformPhysicalFacade {
    pub(super) scope: RoadmapScope,
    pub(super) headers: PhysicalHeaderAuthority,
    pub(super) page_records: PhysicalPageRecordAuthority,
    pub(super) extent_records: crate::PhysicalExtentRecordAuthority,
    pub(super) references: PhysicalReferenceAuthority,
    pub(super) storage: PlatformPhysicalFacadeStorage,
    pub(super) counters: PlatformPhysicalFacadeCounterSnapshot,
    pub(super) next_root_generation: u64,
}

impl PlatformPhysicalFacade {
    pub const fn counters(&self) -> PlatformPhysicalFacadeCounterSnapshot {
        self.counters
    }

    pub(crate) const fn storage_ref(&self) -> &PlatformPhysicalFacadeStorage {
        &self.storage
    }

    pub(crate) const fn headers_ref(&self) -> &PhysicalHeaderAuthority {
        &self.headers
    }

    pub(crate) const fn page_records_ref(&self) -> &PhysicalPageRecordAuthority {
        &self.page_records
    }

    pub(crate) const fn extent_records_ref(&self) -> &crate::PhysicalExtentRecordAuthority {
        &self.extent_records
    }

    pub(crate) fn new(
        scope: RoadmapScope,
        headers: PhysicalHeaderAuthority,
        storage: PlatformPhysicalFacadeStorage,
        counters: PlatformPhysicalFacadeCounterSnapshot,
    ) -> Self {
        Self {
            scope,
            page_records: PhysicalPageRecordAuthority::for_canonical_physical_format(headers.clone()),
            extent_records: crate::PhysicalExtentRecordAuthority::for_canonical_physical_format(headers.clone()),
            references: PhysicalReferenceAuthority::for_canonical_physical_format(),
            headers,
            storage,
            counters,
            next_root_generation: 1,
        }
    }

    pub(crate) fn ensure_admitted_reference(
        &self,
        reference: PhysicalReference,
    ) -> Result<(), PlatformPhysicalFacadeDenial> {
        if self.storage.has_admitted_reference(reference) {
            Ok(())
        } else {
            Err(PlatformPhysicalFacadeDenial::new(
                PlatformPhysicalFacadeDenialKind::MissingPhysicalRecord,
            ))
        }
    }

    pub(crate) fn mark_locate(&mut self) -> PlatformPhysicalFacadeCounterSnapshot {
        self.counters = self.counters.with_locate();
        self.counters
    }

    pub(crate) fn mark_read(&mut self) -> PlatformPhysicalFacadeCounterSnapshot {
        self.counters = self.counters.with_read();
        self.counters
    }
}
