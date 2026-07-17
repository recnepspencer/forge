use crate::{
    OfflineVerifierCounterSnapshot, OfflineVerifierDenial, OfflineVerifierDenialKind,
    PersistedPhysicalLayout, PhysicalByteOrder, PhysicalFormatVersion, PhysicalHeaderAuthority,
};

use super::catalog::PhysicalBootstrapCatalogDenial;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalBootstrapCatalogOpenWitness {
    byte_order: PhysicalByteOrder,
    physical_format_version: PhysicalFormatVersion,
    root_manifest_candidates: Vec<Vec<u8>>,
    segment_manifest: Vec<u8>,
    extent_manifest: Vec<u8>,
    free_space_map: Vec<u8>,
}

impl PhysicalBootstrapCatalogOpenWitness {
    pub(crate) fn admit_persisted_layout(
        headers: &PhysicalHeaderAuthority,
        layout: &PersistedPhysicalLayout,
    ) -> Result<Self, PhysicalBootstrapCatalogDenial> {
        if layout.root_manifest_candidates().is_empty() {
            return Err(PhysicalBootstrapCatalogDenial::ManifestDecodeDenied(
                Box::new(OfflineVerifierDenial::new(
                    OfflineVerifierDenialKind::MissingRootManifest,
                    OfflineVerifierCounterSnapshot::empty(),
                )),
            ));
        }

        Ok(Self {
            byte_order: headers.byte_order(),
            physical_format_version: headers.physical_format_version(),
            root_manifest_candidates: layout.root_manifest_candidates().to_vec(),
            segment_manifest: layout.segment_manifest().to_vec(),
            extent_manifest: layout.extent_manifest().to_vec(),
            free_space_map: layout.free_space_map().to_vec(),
        })
    }

    pub(crate) const fn byte_order(&self) -> PhysicalByteOrder {
        self.byte_order
    }

    pub(crate) fn physical_format_version(&self) -> PhysicalFormatVersion {
        self.physical_format_version
    }

    pub(crate) fn root_manifest_candidates(&self) -> &[Vec<u8>] {
        &self.root_manifest_candidates
    }

    pub(crate) fn segment_manifest(&self) -> &[u8] {
        &self.segment_manifest
    }

    pub(crate) fn extent_manifest(&self) -> &[u8] {
        &self.extent_manifest
    }

    pub(crate) fn free_space_map(&self) -> &[u8] {
        &self.free_space_map
    }
}
