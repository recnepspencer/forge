use super::runtime_storage::PhysicalStoreRuntimeStorage;
use crate::PhysicalStoreRuntimeCounterSnapshot;
use crate::{
    PhysicalHeaderAuthority, PhysicalPageRecordAuthority, PhysicalReference,
    PhysicalReferenceAuthority, PhysicalStoreIdentity, PhysicalStoreRuntimeDenial,
    PhysicalStoreRuntimeDenialKind,
};
use worth_store_contracts::RoadmapScope;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalStoreRuntime {
    pub(in crate::physical_store_runtime) scope: RoadmapScope,
    pub(in crate::physical_store_runtime) headers: PhysicalHeaderAuthority,
    pub(in crate::physical_store_runtime) page_records: PhysicalPageRecordAuthority,
    pub(in crate::physical_store_runtime) extent_records: crate::PhysicalExtentRecordAuthority,
    pub(in crate::physical_store_runtime) references: PhysicalReferenceAuthority,
    pub(in crate::physical_store_runtime) storage: PhysicalStoreRuntimeStorage,
    pub(in crate::physical_store_runtime) counters: PhysicalStoreRuntimeCounterSnapshot,
    pub(in crate::physical_store_runtime) next_root_generation: u64,
    pub(in crate::physical_store_runtime) store_identity: PhysicalStoreIdentity,
}

impl PhysicalStoreRuntime {
    pub const fn counters(&self) -> PhysicalStoreRuntimeCounterSnapshot {
        self.counters
    }

    pub(crate) const fn storage_ref(&self) -> &PhysicalStoreRuntimeStorage {
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
        storage: PhysicalStoreRuntimeStorage,
        counters: PhysicalStoreRuntimeCounterSnapshot,
        store_identity: PhysicalStoreIdentity,
    ) -> Self {
        Self {
            scope,
            page_records: PhysicalPageRecordAuthority::for_canonical_physical_format(
                headers.clone(),
            ),
            extent_records: crate::PhysicalExtentRecordAuthority::for_canonical_physical_format(
                headers.clone(),
            ),
            references: PhysicalReferenceAuthority::for_canonical_physical_format(),
            headers,
            storage,
            counters,
            next_root_generation: 1,
            store_identity,
        }
    }

    pub const fn store_identity(&self) -> &PhysicalStoreIdentity {
        &self.store_identity
    }

    pub(crate) fn ensure_admitted_reference(
        &self,
        reference: PhysicalReference,
    ) -> Result<(), PhysicalStoreRuntimeDenial> {
        if self.storage.has_admitted_reference(reference) {
            Ok(())
        } else {
            Err(PhysicalStoreRuntimeDenial::new(
                PhysicalStoreRuntimeDenialKind::MissingPhysicalRecord,
            ))
        }
    }

    pub(crate) fn mark_locate(&mut self) -> PhysicalStoreRuntimeCounterSnapshot {
        self.counters = self.counters.with_locate();
        self.counters
    }

    pub(crate) fn mark_read(&mut self) -> PhysicalStoreRuntimeCounterSnapshot {
        self.counters = self.counters.with_read();
        self.counters
    }
}
