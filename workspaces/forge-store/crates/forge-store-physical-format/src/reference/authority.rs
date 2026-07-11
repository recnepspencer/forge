use crate::{
    ExtentGenerationCell, FreeSpaceReuseAddress, FreeSpaceReuseCell, PhysicalReference,
    PhysicalReferenceAdmissionWitness, PhysicalReferenceDenialKind, PhysicalReferenceKind,
    PhysicalReferenceValidationCounterSnapshot, PhysicalReferenceValidationDenial,
    PhysicalReferenceValidationWitness, RootPublicationCell, RootPublicationValidationWitness,
    SlotGenerationCell, StalePhysicalReference,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalReferenceAuthority {
    scope: PhysicalReferenceAuthorityScope,
}

impl PhysicalReferenceAuthority {
    pub const fn for_canonical_physical_format() -> Self {
        Self {
            scope: PhysicalReferenceAuthorityScope::StorageFoundationS1,
        }
    }

    pub const fn scope(self) -> PhysicalReferenceAuthorityScope {
        self.scope
    }

    pub const fn admit_page_slot(
        self,
        cell: SlotGenerationCell,
    ) -> PhysicalReferenceAdmissionWitness {
        PhysicalReferenceAdmissionWitness::new(PhysicalReference::from_slot_cell(cell))
    }

    pub const fn admit_extent(
        self,
        cell: ExtentGenerationCell,
    ) -> PhysicalReferenceAdmissionWitness {
        PhysicalReferenceAdmissionWitness::new(PhysicalReference::from_extent_cell(cell))
    }

    pub const fn admit_free_space_reuse(
        self,
        cell: FreeSpaceReuseCell,
    ) -> PhysicalReferenceAdmissionWitness {
        PhysicalReferenceAdmissionWitness::new(PhysicalReference::from_free_space_cell(cell))
    }

    pub const fn admit_root_publication(
        self,
        cell: RootPublicationCell,
    ) -> PhysicalReferenceAdmissionWitness {
        PhysicalReferenceAdmissionWitness::new(PhysicalReference::from_root_publication_cell(cell))
    }

    pub fn validate_page_slot(
        self,
        admission: PhysicalReferenceAdmissionWitness,
        current: SlotGenerationCell,
    ) -> Result<PhysicalReferenceValidationWitness, PhysicalReferenceValidationDenial> {
        let reference = admission.reference();
        let placement_counters =
            PhysicalReferenceValidationCounterSnapshot::for_page_slot_attempt();
        if !matches!(reference.kind(), PhysicalReferenceKind::PageSlot) {
            return Err(PhysicalReferenceValidationDenial::wrong_kind(
                reference,
                placement_counters.with_wrong_kind_rejection(),
            ));
        }
        if reference.segment_id() != Some(current.segment_id())
            || reference.page_id() != Some(current.page_id())
            || reference.slot() != Some(current.slot())
        {
            return Err(PhysicalReferenceValidationDenial::placement_mismatch(
                reference,
                placement_counters.with_placement_mismatch_rejection(),
            ));
        }
        let generation_counters = placement_counters.with_generation_check();
        if reference.generation() != current.generation() {
            return Err(PhysicalReferenceValidationDenial::stale(
                StalePhysicalReference::new(
                    PhysicalReferenceDenialKind::StaleSlotGeneration,
                    reference,
                    generation_counters.with_stale_generation_rejection(),
                ),
            ));
        }
        Ok(PhysicalReferenceValidationWitness::new(
            reference,
            generation_counters,
        ))
    }

    pub fn validate_extent(
        self,
        admission: PhysicalReferenceAdmissionWitness,
        current: ExtentGenerationCell,
    ) -> Result<PhysicalReferenceValidationWitness, PhysicalReferenceValidationDenial> {
        let reference = admission.reference();
        let placement_counters = PhysicalReferenceValidationCounterSnapshot::for_extent_attempt();
        if !matches!(reference.kind(), PhysicalReferenceKind::ExtentBacked) {
            return Err(PhysicalReferenceValidationDenial::wrong_kind(
                reference,
                placement_counters.with_wrong_kind_rejection(),
            ));
        }
        if reference.segment_id() != Some(current.segment_id())
            || reference.extent_id() != Some(current.extent_id())
        {
            return Err(PhysicalReferenceValidationDenial::placement_mismatch(
                reference,
                placement_counters.with_placement_mismatch_rejection(),
            ));
        }
        let generation_counters = placement_counters.with_generation_check();
        if reference.generation() != current.generation() {
            return Err(PhysicalReferenceValidationDenial::stale(
                StalePhysicalReference::new(
                    PhysicalReferenceDenialKind::StaleExtentGeneration,
                    reference,
                    generation_counters.with_stale_generation_rejection(),
                ),
            ));
        }
        Ok(PhysicalReferenceValidationWitness::new(
            reference,
            generation_counters,
        ))
    }

    pub fn validate_free_space_reuse(
        self,
        admission: PhysicalReferenceAdmissionWitness,
        current: FreeSpaceReuseCell,
    ) -> Result<PhysicalReferenceValidationWitness, PhysicalReferenceValidationDenial> {
        let reference = admission.reference();
        let placement_counters = free_space_attempt_counters(current);
        if !matches!(reference.kind(), PhysicalReferenceKind::FreeSpaceReuse) {
            return Err(PhysicalReferenceValidationDenial::wrong_kind(
                reference,
                placement_counters.with_wrong_kind_rejection(),
            ));
        }
        if !free_space_placement_matches(reference, current) {
            return Err(PhysicalReferenceValidationDenial::placement_mismatch(
                reference,
                placement_counters.with_placement_mismatch_rejection(),
            ));
        }
        let generation_counters = placement_counters.with_generation_check();
        if reference.generation() != current.generation() {
            return Err(PhysicalReferenceValidationDenial::stale(
                StalePhysicalReference::new(
                    PhysicalReferenceDenialKind::StaleFreeSpaceReuseGeneration,
                    reference,
                    generation_counters.with_stale_generation_rejection(),
                ),
            ));
        }
        Ok(PhysicalReferenceValidationWitness::new(
            reference,
            generation_counters,
        ))
    }

    pub fn validate_root_publication(
        self,
        admission: PhysicalReferenceAdmissionWitness,
        current: RootPublicationCell,
    ) -> Result<RootPublicationValidationWitness, PhysicalReferenceValidationDenial> {
        let reference = admission.reference();
        let placement_counters =
            PhysicalReferenceValidationCounterSnapshot::for_root_publication_attempt();
        if !matches!(reference.kind(), PhysicalReferenceKind::RootPublication) {
            return Err(PhysicalReferenceValidationDenial::wrong_kind(
                reference,
                placement_counters.with_wrong_kind_rejection(),
            ));
        }
        if reference.root_reference() != Some(current.root_reference()) {
            return Err(PhysicalReferenceValidationDenial::placement_mismatch(
                reference,
                placement_counters.with_placement_mismatch_rejection(),
            ));
        }
        let generation_counters = placement_counters.with_generation_check();
        if reference.generation() != current.generation() {
            return Err(PhysicalReferenceValidationDenial::stale(
                StalePhysicalReference::new(
                    PhysicalReferenceDenialKind::StaleRootPublicationGeneration,
                    reference,
                    generation_counters.with_stale_generation_rejection(),
                ),
            ));
        }
        Ok(RootPublicationValidationWitness::new(
            PhysicalReferenceValidationWitness::new(reference, generation_counters),
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalReferenceAuthorityScope {
    StorageFoundationS1,
}

fn free_space_placement_matches(reference: PhysicalReference, current: FreeSpaceReuseCell) -> bool {
    if reference.allocation_class() != Some(current.allocation_class()) {
        return false;
    }
    match current.address() {
        FreeSpaceReuseAddress::PageSlot {
            segment_id,
            page_id,
            slot,
        } => {
            reference.segment_id() == Some(segment_id)
                && reference.page_id() == Some(page_id)
                && reference.slot() == Some(slot)
        }
        FreeSpaceReuseAddress::Extent {
            segment_id,
            extent_id,
        } => reference.segment_id() == Some(segment_id) && reference.extent_id() == Some(extent_id),
    }
}

const fn free_space_attempt_counters(
    current: FreeSpaceReuseCell,
) -> PhysicalReferenceValidationCounterSnapshot {
    match current.address() {
        FreeSpaceReuseAddress::PageSlot { .. } => {
            PhysicalReferenceValidationCounterSnapshot::for_free_space_slot_attempt()
        }
        FreeSpaceReuseAddress::Extent { .. } => {
            PhysicalReferenceValidationCounterSnapshot::for_free_space_extent_attempt()
        }
    }
}
