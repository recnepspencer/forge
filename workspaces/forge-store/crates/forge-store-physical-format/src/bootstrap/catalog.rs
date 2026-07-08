use crate::{
    CurrentRootManifestAdmission, ManifestDiscoveryAuthority, ManifestDiscoveryDenial,
    OfflineManifestCodec, OfflineVerifierCounterSnapshot, OfflineVerifierDenial,
    PhysicalChunkChecksumAuthority, PhysicalChunkChecksumDenial, PhysicalChunkPayloadIntegrityWitness,
    PhysicalFormatVersion, PhysicalRootManifest,
};

use super::{
    identity::PhysicalBootstrapCatalogIdentity, sections::PhysicalBootstrapCatalogOpenWitness,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalBootstrapCatalogWitness {
    identity: PhysicalBootstrapCatalogIdentity,
    checksum: PhysicalChunkPayloadIntegrityWitness,
    root_entry_count: u32,
    segment_count: u32,
    page_slot_count: u32,
    extent_count: u32,
    allocation_class_count: u32,
    free_space_count: u32,
}

impl PhysicalBootstrapCatalogWitness {
    pub fn current_root(&self) -> CurrentRootManifestAdmission {
        CurrentRootManifestAdmission::from_root_owner(self.identity.root_owner())
    }

    pub const fn identity(&self) -> &PhysicalBootstrapCatalogIdentity {
        &self.identity
    }

    pub fn root_reference(&self) -> crate::PhysicalRootReference {
        self.identity.root_reference()
    }

    pub fn physical_format_version(&self) -> PhysicalFormatVersion {
        self.identity.physical_format_version()
    }

    pub const fn checksum(&self) -> &PhysicalChunkPayloadIntegrityWitness {
        &self.checksum
    }

    pub fn root_entry_count(&self) -> u32 {
        self.root_entry_count
    }

    pub fn segment_count(&self) -> u32 {
        self.segment_count
    }

    pub fn page_slot_count(&self) -> u32 {
        self.page_slot_count
    }

    pub fn extent_count(&self) -> u32 {
        self.extent_count
    }

    pub fn allocation_class_count(&self) -> u32 {
        self.allocation_class_count
    }

    pub fn free_space_count(&self) -> u32 {
        self.free_space_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhysicalBootstrapCatalogDenial {
    ManifestDecodeDenied(OfflineVerifierDenial),
    ManifestDiscoveryDenied(ManifestDiscoveryDenial),
    BootstrapChecksumDenied(PhysicalChunkChecksumDenial),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalBootstrapCatalogAuthority {
    manifests: ManifestDiscoveryAuthority,
    checksums: PhysicalChunkChecksumAuthority,
}

impl PhysicalBootstrapCatalogAuthority {
    pub const fn s8_minimal() -> Self {
        Self {
            manifests: ManifestDiscoveryAuthority::s1(),
            checksums: PhysicalChunkChecksumAuthority::s7_canonical(),
        }
    }

    pub fn discover_catalog(
        self,
        open_witness: &PhysicalBootstrapCatalogOpenWitness,
    ) -> Result<PhysicalBootstrapCatalogWitness, PhysicalBootstrapCatalogDenial> {
        let mut decode_denial = None;
        let mut discovery_denial = None;
        for root_manifest in open_witness.root_manifest_candidates() {
            let decoded = match OfflineManifestCodec::decode(
                open_witness.byte_order(),
                root_manifest,
                open_witness.segment_manifest(),
                open_witness.extent_manifest(),
                open_witness.free_space_map(),
                OfflineVerifierCounterSnapshot::empty(),
            ) {
                Ok(decoded) => decoded,
                Err(denial) => {
                    decode_denial = Some(denial);
                    continue;
                }
            };
            let root = build_root_manifest(&decoded);
            let admission = crate::PhysicalReferenceAuthority::s1()
                .admit_root_publication(root.root_publication());
            let report = match self.manifests.reopen_from_root(&root, admission) {
                Ok(report) => report,
                Err(denial) => {
                    discovery_denial = Some(denial);
                    continue;
                }
            };
            let checksum = self
                .checksums
                .admit_bootstrap_payload(&bootstrap_payload_bytes(open_witness, root_manifest))
                .map_err(PhysicalBootstrapCatalogDenial::BootstrapChecksumDenied)?;

            return Ok(PhysicalBootstrapCatalogWitness {
                identity: PhysicalBootstrapCatalogIdentity::new(
                    admission.owner(),
                    root.root_publication().root_reference(),
                    open_witness.physical_format_version(),
                    checksum.checksum().checksum().clone(),
                ),
                checksum,
                root_entry_count: report.counters().root_manifest_entry_count(),
                segment_count: report.counters().segment_manifest_entry_count(),
                page_slot_count: root.page_slots().len() as u32,
                extent_count: report.counters().extent_manifest_entry_count(),
                allocation_class_count: report.counters().allocation_class_entry_count(),
                free_space_count: report.counters().free_space_map_entry_count(),
            });
        }

        if let Some(denial) = discovery_denial {
            return Err(PhysicalBootstrapCatalogDenial::ManifestDiscoveryDenied(
                denial,
            ));
        }
        Err(PhysicalBootstrapCatalogDenial::ManifestDecodeDenied(
            decode_denial.expect("bootstrap open witness always carries at least one root candidate"),
        ))
    }
}

pub const fn physical_bootstrap_catalog() -> PhysicalBootstrapCatalogAuthority {
    PhysicalBootstrapCatalogAuthority::s8_minimal()
}

fn bootstrap_payload_bytes(
    open_witness: &PhysicalBootstrapCatalogOpenWitness,
    root_manifest: &[u8],
) -> Vec<u8> {
    let mut bytes = Vec::new();
    append_section(&mut bytes, root_manifest);
    append_section(&mut bytes, open_witness.segment_manifest());
    append_section(&mut bytes, open_witness.extent_manifest());
    append_section(&mut bytes, open_witness.free_space_map());
    bytes
}

fn append_section(bytes: &mut Vec<u8>, section: &[u8]) {
    bytes.extend_from_slice(&(section.len() as u32).to_le_bytes());
    bytes.extend_from_slice(section);
}

fn build_root_manifest(
    decoded: &crate::offline_verifier::DecodedOfflineManifestSections,
) -> PhysicalRootManifest {
    PhysicalRootManifest::new(
        decoded.root,
        decoded.segments.clone(),
        decoded.page_slots.clone(),
        decoded.extents.clone(),
        decoded.allocation_classes.clone(),
        decoded.free_space.clone(),
    )
}
