use super::root_discovery::canonical_root_manifest;
use crate::access::counters::PhysicalLayoutAccessCounterSnapshot;
use crate::access::grammar::PhysicalLayoutAccessFamily;
use crate::{
    ManifestDiscoveryAuthority, PhysicalReference, PhysicalReferenceAdmissionWitness,
    PhysicalReferenceAuthority, PhysicalReferenceKind, PhysicalStoreRuntime,
    PhysicalStoreRuntimeDenial, PhysicalStoreRuntimeDenialKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ManifestMembershipLayoutReport {
    reference: PhysicalReference,
    counters: PhysicalLayoutAccessCounterSnapshot,
}

#[derive(Debug)]
pub struct ManifestAccess<'a> {
    facade: &'a mut PhysicalStoreRuntime,
}

impl<'a> ManifestAccess<'a> {
    pub(crate) fn new(facade: &'a mut PhysicalStoreRuntime) -> Self {
        Self { facade }
    }

    pub fn validate_membership(
        &mut self,
        reference: PhysicalReference,
    ) -> Result<ManifestMembershipLayoutReport, PhysicalStoreRuntimeDenial> {
        let access = canonical_root_manifest(self.facade)?;
        let root = access.root();
        let references = PhysicalReferenceAuthority::for_canonical_physical_format();
        let discovery = ManifestDiscoveryAuthority::for_canonical_physical_format();
        let admission = PhysicalReferenceAdmissionWitness::new(reference);
        let discovery_report = access.manifest_report();
        let counters = match reference.kind() {
            PhysicalReferenceKind::RootPublication => access.manifest_traversal_counters(0),
            _ => access.manifest_lookup_counters(),
        };
        match reference.kind() {
            PhysicalReferenceKind::PageSlot => {
                discovery
                    .locate_page_slot(discovery_report, admission)
                    .map_err(|denial| {
                        PhysicalStoreRuntimeDenial::new(
                            PhysicalStoreRuntimeDenialKind::ManifestDiscoveryDenied,
                        )
                        .with_manifest_denial(denial)
                    })?;
            }
            PhysicalReferenceKind::ExtentBacked => {
                discovery
                    .locate_extent(discovery_report, admission)
                    .map_err(|denial| {
                        PhysicalStoreRuntimeDenial::new(
                            PhysicalStoreRuntimeDenialKind::ManifestDiscoveryDenied,
                        )
                        .with_manifest_denial(denial)
                    })?;
            }
            PhysicalReferenceKind::FreeSpaceReuse => {
                discovery
                    .validate_free_space_reuse(discovery_report, admission)
                    .map_err(|denial| {
                        PhysicalStoreRuntimeDenial::new(
                            PhysicalStoreRuntimeDenialKind::ManifestDiscoveryDenied,
                        )
                        .with_manifest_denial(denial)
                    })?;
            }
            PhysicalReferenceKind::RootPublication => {
                references
                    .validate_root_publication(admission, root.root_publication())
                    .map_err(|denial| {
                        PhysicalStoreRuntimeDenial::new(
                            PhysicalStoreRuntimeDenialKind::ManifestDiscoveryDenied,
                        )
                        .with_reference_denial(denial)
                    })?;
            }
        }
        let _ = self.facade.mark_locate();
        Ok(ManifestMembershipLayoutReport {
            reference,
            counters,
        })
    }
}

impl ManifestMembershipLayoutReport {
    pub const fn family(self) -> PhysicalLayoutAccessFamily {
        PhysicalLayoutAccessFamily::ManifestIndex
    }

    pub const fn reference(self) -> PhysicalReference {
        self.reference
    }

    pub const fn counters(self) -> PhysicalLayoutAccessCounterSnapshot {
        self.counters
    }
}
