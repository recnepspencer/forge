use crate::manifest::universe::manifest_shape_counters;
use crate::{
    FreeSpaceReuseAddress, FreeSpaceReuseCell, ManifestDiscoveryCounterSnapshot,
    ManifestDiscoveryDenial, ManifestDiscoveryDenialKind, PhysicalReferenceAdmissionWitness,
    PhysicalReferenceAuthority, PhysicalReferenceKind, PhysicalReferenceValidationWitness,
    PhysicalRootManifest, PhysicalSegmentId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ManifestDiscoveryAuthority {
    references: PhysicalReferenceAuthority,
}

impl ManifestDiscoveryAuthority {
    pub const fn s1() -> Self {
        Self {
            references: PhysicalReferenceAuthority::s1(),
        }
    }

    pub fn reopen_from_root<'a>(
        self,
        root: &'a PhysicalRootManifest,
        admission: PhysicalReferenceAdmissionWitness,
    ) -> Result<ManifestDiscoveryReport<'a>, ManifestDiscoveryDenial> {
        let root_counters = ManifestDiscoveryCounterSnapshot::for_reopen();
        self.references
            .validate_root_publication(admission, root.root_publication())
            .map_err(|denial| {
                ManifestDiscoveryDenial::reference_validation_denied(denial, root_counters)
            })?;
        let counters = manifest_shape_counters(
            root_counters,
            root.segments().len(),
            root.page_slots().len(),
            root.extents().len(),
            root.allocation_classes().len(),
            root.free_space().len(),
        );
        Ok(ManifestDiscoveryReport { root, counters })
    }

    pub fn locate_page_slot(
        self,
        report: ManifestDiscoveryReport<'_>,
        admission: PhysicalReferenceAdmissionWitness,
    ) -> Result<PhysicalReferenceValidationWitness, ManifestDiscoveryDenial> {
        let counters = report.counters.with_manifest_index_probe();
        let reference = admission.reference();
        if !segment_manifest_contains(report.root, reference.segment_id()) {
            return Err(ManifestDiscoveryDenial::new(
                ManifestDiscoveryDenialKind::MissingSegmentManifestMembership,
                Some(reference),
                counters,
            ));
        }
        let Some(entry) = report.root.page_slots().iter().find(|entry| {
            let slot = entry.page_slot();
            reference.kind() == PhysicalReferenceKind::PageSlot
                && reference.segment_id() == Some(slot.segment_id())
                && reference.page_id() == Some(slot.page_id())
                && reference.slot() == Some(slot.slot())
        }) else {
            return Err(ManifestDiscoveryDenial::new(
                ManifestDiscoveryDenialKind::MissingPageSlotManifestMembership,
                Some(reference),
                counters,
            ));
        };
        self.references
            .validate_page_slot(admission, entry.page_slot())
            .map_err(|denial| {
                ManifestDiscoveryDenial::reference_validation_denied(denial, counters)
            })
    }

    pub fn locate_extent(
        self,
        report: ManifestDiscoveryReport<'_>,
        admission: PhysicalReferenceAdmissionWitness,
    ) -> Result<PhysicalReferenceValidationWitness, ManifestDiscoveryDenial> {
        let counters = report.counters.with_manifest_index_probe();
        let reference = admission.reference();
        if !segment_manifest_contains(report.root, reference.segment_id()) {
            return Err(ManifestDiscoveryDenial::new(
                ManifestDiscoveryDenialKind::MissingSegmentManifestMembership,
                Some(reference),
                counters,
            ));
        }
        let Some(entry) = report.root.extents().iter().find(|entry| {
            let extent = entry.extent();
            reference.kind() == PhysicalReferenceKind::ExtentBacked
                && reference.segment_id() == Some(extent.segment_id())
                && reference.extent_id() == Some(extent.extent_id())
        }) else {
            return Err(ManifestDiscoveryDenial::new(
                ManifestDiscoveryDenialKind::MissingExtentManifestMembership,
                Some(reference),
                counters,
            ));
        };
        self.references
            .validate_extent(admission, entry.extent())
            .map_err(|denial| {
                ManifestDiscoveryDenial::reference_validation_denied(denial, counters)
            })
    }

    pub fn validate_free_space_reuse(
        self,
        report: ManifestDiscoveryReport<'_>,
        admission: PhysicalReferenceAdmissionWitness,
    ) -> Result<PhysicalReferenceValidationWitness, ManifestDiscoveryDenial> {
        let counters = report.counters.with_manifest_index_probe();
        let reference = admission.reference();
        if !segment_manifest_contains(report.root, reference.segment_id()) {
            return Err(ManifestDiscoveryDenial::new(
                ManifestDiscoveryDenialKind::MissingSegmentManifestMembership,
                Some(reference),
                counters,
            ));
        }
        let Some(entry) = report
            .root
            .free_space()
            .iter()
            .find(|entry| free_space_reference_matches_cell(reference, entry.reuse_cell()))
        else {
            return Err(ManifestDiscoveryDenial::new(
                ManifestDiscoveryDenialKind::MissingFreeSpaceManifestMembership,
                Some(reference),
                counters,
            ));
        };
        self.references
            .validate_free_space_reuse(admission, entry.reuse_cell())
            .map_err(|denial| {
                ManifestDiscoveryDenial::reference_validation_denied(denial, counters)
            })
    }

    pub fn reject_backend_residue(
        self,
        report: ManifestDiscoveryReport<'_>,
        admission: PhysicalReferenceAdmissionWitness,
    ) -> ManifestDiscoveryDenial {
        ManifestDiscoveryDenial::new(
            ManifestDiscoveryDenialKind::BackendResidueDiscoverySource,
            Some(admission.reference()),
            report.counters.with_backend_residue_rejection(),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ManifestDiscoveryReport<'a> {
    root: &'a PhysicalRootManifest,
    counters: ManifestDiscoveryCounterSnapshot,
}

impl<'a> ManifestDiscoveryReport<'a> {
    pub(crate) const fn new(
        root: &'a PhysicalRootManifest,
        counters: ManifestDiscoveryCounterSnapshot,
    ) -> Self {
        Self { root, counters }
    }

    pub const fn root(self) -> &'a PhysicalRootManifest {
        self.root
    }

    pub const fn counters(self) -> ManifestDiscoveryCounterSnapshot {
        self.counters
    }
}

fn segment_manifest_contains(
    root: &PhysicalRootManifest,
    segment_id: Option<PhysicalSegmentId>,
) -> bool {
    let Some(segment_id) = segment_id else {
        return false;
    };
    root.segments()
        .iter()
        .any(|entry| entry.segment().segment_id() == segment_id)
}

fn free_space_reference_matches_cell(
    reference: crate::PhysicalReference,
    cell: FreeSpaceReuseCell,
) -> bool {
    reference.kind() == PhysicalReferenceKind::FreeSpaceReuse
        && reference.allocation_class() == Some(cell.allocation_class())
        && free_space_placement_matches(reference, cell.address())
}

fn free_space_placement_matches(
    reference: crate::PhysicalReference,
    address: FreeSpaceReuseAddress,
) -> bool {
    match address {
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
