use super::model_storage::InMemoryPhysicalFormatModelStorage;
use crate::InMemoryPhysicalFormatModelCounterSnapshot;
use crate::{
    InMemoryPhysicalFormatModelDenial, InMemoryPhysicalFormatModelDenialKind,
    PhysicalHeaderAuthority, PhysicalPageRecordAuthority, PhysicalReference,
    PhysicalReferenceAuthority, PhysicalStoreIdentity,
};
use worth_store_contracts::RoadmapScope;

/// Detached heap model of physical-format algorithms.
///
/// This value performs no media I/O, owns no store root, and cannot be promoted
/// into `worth_store::physical_runtime` authority. Cloneability is model-data
/// cloneability, not physical runtime authority duplication.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InMemoryPhysicalFormatModel {
    pub(in crate::in_memory_physical_format_model) scope: RoadmapScope,
    pub(in crate::in_memory_physical_format_model) headers: PhysicalHeaderAuthority,
    pub(in crate::in_memory_physical_format_model) page_records: PhysicalPageRecordAuthority,
    pub(in crate::in_memory_physical_format_model) extent_records:
        crate::PhysicalExtentRecordAuthority,
    pub(in crate::in_memory_physical_format_model) references: PhysicalReferenceAuthority,
    pub(in crate::in_memory_physical_format_model) storage: InMemoryPhysicalFormatModelStorage,
    pub(in crate::in_memory_physical_format_model) counters:
        InMemoryPhysicalFormatModelCounterSnapshot,
    pub(in crate::in_memory_physical_format_model) next_root_generation: u64,
    pub(in crate::in_memory_physical_format_model) store_identity: PhysicalStoreIdentity,
}

impl InMemoryPhysicalFormatModel {
    pub const fn counters(&self) -> InMemoryPhysicalFormatModelCounterSnapshot {
        self.counters
    }

    pub(crate) const fn storage_ref(&self) -> &InMemoryPhysicalFormatModelStorage {
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
        storage: InMemoryPhysicalFormatModelStorage,
        counters: InMemoryPhysicalFormatModelCounterSnapshot,
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
    ) -> Result<(), InMemoryPhysicalFormatModelDenial> {
        if self.storage.has_admitted_reference(reference) {
            Ok(())
        } else {
            Err(InMemoryPhysicalFormatModelDenial::new(
                InMemoryPhysicalFormatModelDenialKind::MissingPhysicalRecord,
            ))
        }
    }

    pub(crate) fn mark_locate(&mut self) -> InMemoryPhysicalFormatModelCounterSnapshot {
        self.counters = self.counters.with_locate();
        self.counters
    }

    pub(crate) fn mark_read(&mut self) -> InMemoryPhysicalFormatModelCounterSnapshot {
        self.counters = self.counters.with_read();
        self.counters
    }
}
