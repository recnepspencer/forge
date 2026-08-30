use worth_store_physical_format::integrity_declarations::{
    families::root::{
        CURRENT_SELECTOR_INTEGRITY_DECLARATION, PREVIOUS_SELECTOR_INTEGRITY_DECLARATION,
        ROOT_MANIFEST_INTEGRITY_DECLARATION,
    },
    PhysicalIntegrityArtifactFamily, PhysicalIntegrityFormatDeclaration,
};
use worth_store_physical_format::store_namespace::StableStoreIdentity;
use worth_store_physical_format::{durable_artifact_checksum, PhysicalRecordFormatDeclaration};

use crate::localization::PhysicalByteRange;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RootArtifactScope {
    CurrentSelector,
    PreviousSelector,
    Manifest { generation: u64 },
}

/// Exact descriptive scope against which bytes are validated.
///
/// Selector scopes are deliberately role-addressed. Their selector identity
/// and addressed root generation exist only inside the checksummed envelope
/// and therefore enter the sealed validated result, not this expected scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalArtifactScope {
    store: StableStoreIdentity,
    artifact: RootArtifactScope,
    record_format: PhysicalRecordFormatDeclaration,
    range: PhysicalByteRange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalArtifactScopeDenial {
    ZeroRootGeneration,
}

impl PhysicalArtifactScope {
    pub const fn current_root_selector(
        store: StableStoreIdentity,
        record_format: PhysicalRecordFormatDeclaration,
        range: PhysicalByteRange,
    ) -> Self {
        Self {
            store,
            artifact: RootArtifactScope::CurrentSelector,
            record_format,
            range,
        }
    }

    pub const fn previous_root_selector(
        store: StableStoreIdentity,
        record_format: PhysicalRecordFormatDeclaration,
        range: PhysicalByteRange,
    ) -> Self {
        Self {
            store,
            artifact: RootArtifactScope::PreviousSelector,
            record_format,
            range,
        }
    }

    pub const fn root_manifest(
        store: StableStoreIdentity,
        record_format: PhysicalRecordFormatDeclaration,
        generation: u64,
        range: PhysicalByteRange,
    ) -> Result<Self, PhysicalArtifactScopeDenial> {
        if generation == 0 {
            return Err(PhysicalArtifactScopeDenial::ZeroRootGeneration);
        }
        Ok(Self {
            store,
            artifact: RootArtifactScope::Manifest { generation },
            record_format,
            range,
        })
    }

    pub const fn store_identity(self) -> StableStoreIdentity {
        self.store
    }

    pub const fn artifact_family(self) -> PhysicalIntegrityArtifactFamily {
        match self.artifact {
            RootArtifactScope::CurrentSelector => {
                PhysicalIntegrityArtifactFamily::CurrentRootSelector
            }
            RootArtifactScope::PreviousSelector => {
                PhysicalIntegrityArtifactFamily::PreviousRootSelector
            }
            RootArtifactScope::Manifest { .. } => PhysicalIntegrityArtifactFamily::RootManifest,
        }
    }

    pub const fn record_format(self) -> PhysicalRecordFormatDeclaration {
        self.record_format
    }

    pub const fn declaration(self) -> PhysicalIntegrityFormatDeclaration {
        match self.artifact {
            RootArtifactScope::CurrentSelector => CURRENT_SELECTOR_INTEGRITY_DECLARATION,
            RootArtifactScope::PreviousSelector => PREVIOUS_SELECTOR_INTEGRITY_DECLARATION,
            RootArtifactScope::Manifest { .. } => ROOT_MANIFEST_INTEGRITY_DECLARATION,
        }
    }

    pub const fn byte_range(self) -> PhysicalByteRange {
        self.range
    }

    pub const fn root_generation(self) -> Option<u64> {
        match self.artifact {
            RootArtifactScope::Manifest { generation } => Some(generation),
            RootArtifactScope::CurrentSelector | RootArtifactScope::PreviousSelector => None,
        }
    }

    pub(crate) const fn is_current_selector(self) -> bool {
        matches!(self.artifact, RootArtifactScope::CurrentSelector)
    }

    pub(crate) const fn is_previous_selector(self) -> bool {
        matches!(self.artifact, RootArtifactScope::PreviousSelector)
    }

    pub(crate) const fn is_root_manifest(self) -> bool {
        matches!(self.artifact, RootArtifactScope::Manifest { .. })
    }

    pub(crate) fn exact_scope_digest(self) -> u32 {
        let mut bytes = [0_u8; 51];
        bytes[..16].copy_from_slice(&self.store.bytes());
        bytes[16] = match self.artifact {
            RootArtifactScope::CurrentSelector => 1,
            RootArtifactScope::PreviousSelector => 2,
            RootArtifactScope::Manifest { .. } => 3,
        };
        bytes[17..25].copy_from_slice(&self.root_generation().unwrap_or(0).to_le_bytes());
        bytes[25..33].copy_from_slice(&self.range.offset().to_le_bytes());
        bytes[33..41].copy_from_slice(&self.range.length().to_le_bytes());
        bytes[41..].copy_from_slice(&self.record_format.canonical_identity_bytes());
        durable_artifact_checksum(&bytes)
    }
}

#[cfg(test)]
mod tests {
    use worth_store_physical_format::integrity_declarations::PhysicalIntegrityArtifactFamily;
    use worth_store_physical_format::store_namespace::{
        ProposedStoreIdentity, StoreNamespaceIdentityRecord, StoreNamespaceVersion,
    };

    use super::{PhysicalArtifactScope, PhysicalArtifactScopeDenial};
    use crate::localization::PhysicalByteRange;

    fn store() -> worth_store_physical_format::store_namespace::StableStoreIdentity {
        StoreNamespaceIdentityRecord::new(
            StoreNamespaceVersion::CURRENT,
            ProposedStoreIdentity::from_nonzero_bytes([7; 16]).unwrap(),
        )
        .published_identity()
    }

    fn format() -> worth_store_physical_format::PhysicalRecordFormatDeclaration {
        worth_store_physical_format::PhysicalRecordFormatDeclaration::builder()
            .admit()
            .unwrap()
    }

    #[test]
    fn selector_scope_does_not_invent_payload_only_root_generation() {
        let scope = PhysicalArtifactScope::current_root_selector(
            store(),
            format(),
            PhysicalByteRange::new(0, 107).unwrap(),
        );
        assert_eq!(
            scope.artifact_family(),
            PhysicalIntegrityArtifactFamily::CurrentRootSelector
        );
        assert_eq!(scope.root_generation(), None);
    }

    #[test]
    fn root_manifest_scope_requires_exact_nonzero_generation() {
        let range = PhysicalByteRange::new(0, 368).unwrap();
        assert_eq!(
            PhysicalArtifactScope::root_manifest(store(), format(), 0, range),
            Err(PhysicalArtifactScopeDenial::ZeroRootGeneration)
        );
        assert_eq!(
            PhysicalArtifactScope::root_manifest(store(), format(), 9, range)
                .unwrap()
                .root_generation(),
            Some(9)
        );
    }

    #[test]
    fn exact_scope_digest_changes_with_format_generation_or_range() {
        use worth_store_physical_format::PhysicalPageSizeClass;

        let range = PhysicalByteRange::new(4096, 368).unwrap();
        let baseline = PhysicalArtifactScope::root_manifest(store(), format(), 9, range).unwrap();
        let other_format = worth_store_physical_format::PhysicalRecordFormatDeclaration::builder()
            .page_size(PhysicalPageSizeClass::KiB32)
            .admit()
            .unwrap();

        assert_ne!(
            baseline.exact_scope_digest(),
            PhysicalArtifactScope::root_manifest(store(), other_format, 9, range)
                .unwrap()
                .exact_scope_digest()
        );
        assert_ne!(
            baseline.exact_scope_digest(),
            PhysicalArtifactScope::root_manifest(store(), format(), 10, range)
                .unwrap()
                .exact_scope_digest()
        );
        assert_ne!(
            baseline.exact_scope_digest(),
            PhysicalArtifactScope::root_manifest(
                store(),
                format(),
                9,
                PhysicalByteRange::new(8192, 368).unwrap(),
            )
            .unwrap()
            .exact_scope_digest()
        );
    }
}
