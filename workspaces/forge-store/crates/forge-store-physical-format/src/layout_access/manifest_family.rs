use super::counters::PhysicalLayoutAccessCounterSnapshot;
use super::grammar::{AdmittedManifestIndexLayoutRule, PhysicalLayoutAccessFamily};
use super::root_discovery_family::canonical_root_manifest;
use crate::{
    ManifestDiscoveryAuthority, PhysicalReference, PhysicalReferenceAdmissionWitness,
    PhysicalReferenceAuthority, PhysicalReferenceKind, PlatformPhysicalFacade,
    PlatformPhysicalFacadeDenial, PlatformPhysicalFacadeDenialKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ManifestLayoutFamilyHome;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ManifestLayoutFamilyAdmission;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ManifestMembershipLayoutReport {
    reference: PhysicalReference,
    counters: PhysicalLayoutAccessCounterSnapshot,
}

impl ManifestLayoutFamilyHome {
    pub const fn s8() -> Self {
        Self
    }

    pub fn admit(
        &self,
        _rule: &AdmittedManifestIndexLayoutRule,
    ) -> Result<ManifestLayoutFamilyAdmission, PlatformPhysicalFacadeDenial> {
        Ok(ManifestLayoutFamilyAdmission)
    }
}

#[derive(Debug)]
pub struct AdmittedManifestLayoutFamily<'a> {
    facade: &'a mut PlatformPhysicalFacade,
    _admission: ManifestLayoutFamilyAdmission,
}

impl<'a> AdmittedManifestLayoutFamily<'a> {
    pub(crate) fn new(
        facade: &'a mut PlatformPhysicalFacade,
        admission: ManifestLayoutFamilyAdmission,
    ) -> Self {
        Self {
            facade,
            _admission: admission,
        }
    }

    pub fn validate_membership(
        &mut self,
        reference: PhysicalReference,
    ) -> Result<ManifestMembershipLayoutReport, PlatformPhysicalFacadeDenial> {
        let access = canonical_root_manifest(self.facade)?;
        let root = access.root();
        let references = PhysicalReferenceAuthority::s1();
        let discovery = ManifestDiscoveryAuthority::s1();
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
                        PlatformPhysicalFacadeDenial::new(
                            PlatformPhysicalFacadeDenialKind::ManifestDiscoveryDenied,
                        )
                        .with_manifest_denial(denial)
                    })?;
            }
            PhysicalReferenceKind::ExtentBacked => {
                discovery
                    .locate_extent(discovery_report, admission)
                    .map_err(|denial| {
                        PlatformPhysicalFacadeDenial::new(
                            PlatformPhysicalFacadeDenialKind::ManifestDiscoveryDenied,
                        )
                        .with_manifest_denial(denial)
                    })?;
            }
            PhysicalReferenceKind::FreeSpaceReuse => {
                discovery
                    .validate_free_space_reuse(discovery_report, admission)
                    .map_err(|denial| {
                        PlatformPhysicalFacadeDenial::new(
                            PlatformPhysicalFacadeDenialKind::ManifestDiscoveryDenied,
                        )
                        .with_manifest_denial(denial)
                    })?;
            }
            PhysicalReferenceKind::RootPublication => {
                references
                    .validate_root_publication(admission, root.root_publication())
                    .map_err(|denial| {
                        PlatformPhysicalFacadeDenial::new(
                            PlatformPhysicalFacadeDenialKind::ManifestDiscoveryDenied,
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
