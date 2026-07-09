use worth_store_physical_format::{PhysicalGeneration, PhysicalPageHeader, SlotGenerationCell};
use worth_store_security::{
    StorePhysicalSecurityMetadataCarrier, StorePhysicalSecurityMetadataEnvelope,
    StoreSegmentPageSecurityMetadataEnvelope,
};

use crate::{
    CurrentPhysicalRoot, PhysicalByteGuardScope, PhysicalReadProtectedFootprintBasis,
    StablePhysicalReadHandle,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StableReadSecurityScopeCarrierBasis {
    page_header_generation: PhysicalGeneration,
    manifest_page_slot: SlotGenerationCell,
    guard_scope: PhysicalByteGuardScope,
}

impl StableReadSecurityScopeCarrierBasis {
    pub fn new(
        guard_scope: PhysicalByteGuardScope,
        page_header: &StorePhysicalSecurityMetadataEnvelope<PhysicalPageHeader>,
        manifest_entry: &StoreSegmentPageSecurityMetadataEnvelope,
    ) -> Self {
        Self {
            page_header_generation: page_header.header().generation(),
            manifest_page_slot: manifest_entry.artifact().page_slot(),
            guard_scope,
        }
    }

    pub const fn page_header_generation(self) -> PhysicalGeneration {
        self.page_header_generation
    }

    pub const fn manifest_page_slot(self) -> SlotGenerationCell {
        self.manifest_page_slot
    }

    pub const fn guard_scope(self) -> PhysicalByteGuardScope {
        self.guard_scope
    }

    pub fn matches_guard_scope(self, guard_scope: PhysicalByteGuardScope) -> bool {
        self.guard_scope == guard_scope
            && self.page_header_generation.get() == guard_scope.reference().generation().get()
            && self.manifest_page_slot.generation().get()
                == guard_scope.reference().generation().get()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StableReadSecurityScopePropagationInput {
    protected_root: CurrentPhysicalRoot,
    footprint_basis: PhysicalReadProtectedFootprintBasis,
    carrier_basis: StableReadSecurityScopeCarrierBasis,
    page_metadata: StorePhysicalSecurityMetadataCarrier,
    manifest_metadata: StorePhysicalSecurityMetadataCarrier,
}

impl StableReadSecurityScopePropagationInput {
    pub fn new(
        handle: &StablePhysicalReadHandle,
        guard_scope: PhysicalByteGuardScope,
        page_header: &StorePhysicalSecurityMetadataEnvelope<PhysicalPageHeader>,
        manifest_entry: &StoreSegmentPageSecurityMetadataEnvelope,
    ) -> Self {
        Self {
            protected_root: handle.plan().root(),
            footprint_basis: handle.plan().footprint().declared_footprint_basis(),
            carrier_basis: StableReadSecurityScopeCarrierBasis::new(
                guard_scope,
                page_header,
                manifest_entry,
            ),
            page_metadata: page_header.security_metadata(),
            manifest_metadata: manifest_entry.security_metadata(),
        }
    }

    pub const fn protected_root(self) -> CurrentPhysicalRoot {
        self.protected_root
    }

    pub const fn footprint_basis(self) -> PhysicalReadProtectedFootprintBasis {
        self.footprint_basis
    }

    pub const fn guard_scope(self) -> PhysicalByteGuardScope {
        self.carrier_basis.guard_scope()
    }

    pub const fn carrier_basis(self) -> StableReadSecurityScopeCarrierBasis {
        self.carrier_basis
    }

    pub const fn page_metadata(self) -> StorePhysicalSecurityMetadataCarrier {
        self.page_metadata
    }

    pub const fn manifest_metadata(self) -> StorePhysicalSecurityMetadataCarrier {
        self.manifest_metadata
    }
}
