use super::declaration::{PhysicalKeyDomain, PhysicalKeyDomainWitness};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompositeKeyField {
    VersionByte,
    TenantScope,
    KeyScope,
    RootReference,
    SegmentId,
    PageId,
    ExtentId,
    ReferenceKind,
    RecordSlot,
    Generation,
    WalFamily,
    WalSequence,
    Digest,
    BackupSequence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompositeKeyOrderingLaw {
    domain: PhysicalKeyDomainWitness,
    fields: &'static [CompositeKeyField],
}

impl CompositeKeyOrderingLaw {
    pub(crate) const fn new(
        domain: PhysicalKeyDomainWitness,
        fields: &'static [CompositeKeyField],
    ) -> Self {
        Self { domain, fields }
    }

    pub const fn domain(self) -> PhysicalKeyDomainWitness {
        self.domain
    }

    pub const fn fields(self) -> &'static [CompositeKeyField] {
        self.fields
    }
}

pub(crate) const fn declare_composite_key_ordering(
    domain: PhysicalKeyDomainWitness,
) -> CompositeKeyOrderingLaw {
    let fields: &'static [CompositeKeyField] = match domain.domain() {
        PhysicalKeyDomain::RootManifestKey => &[
            CompositeKeyField::VersionByte,
            CompositeKeyField::TenantScope,
            CompositeKeyField::KeyScope,
            CompositeKeyField::RootReference,
        ],
        PhysicalKeyDomain::PageAddressKey => &[
            CompositeKeyField::VersionByte,
            CompositeKeyField::TenantScope,
            CompositeKeyField::KeyScope,
            CompositeKeyField::SegmentId,
            CompositeKeyField::PageId,
        ],
        PhysicalKeyDomain::SegmentAddressKey => &[
            CompositeKeyField::VersionByte,
            CompositeKeyField::TenantScope,
            CompositeKeyField::KeyScope,
            CompositeKeyField::SegmentId,
        ],
        PhysicalKeyDomain::ExtentAddressKey => &[
            CompositeKeyField::VersionByte,
            CompositeKeyField::TenantScope,
            CompositeKeyField::KeyScope,
            CompositeKeyField::SegmentId,
            CompositeKeyField::ExtentId,
        ],
        PhysicalKeyDomain::PhysicalReferenceKey => &[
            CompositeKeyField::VersionByte,
            CompositeKeyField::TenantScope,
            CompositeKeyField::KeyScope,
            CompositeKeyField::ReferenceKind,
            CompositeKeyField::SegmentId,
            CompositeKeyField::PageId,
            CompositeKeyField::ExtentId,
            CompositeKeyField::RecordSlot,
            CompositeKeyField::Generation,
        ],
        PhysicalKeyDomain::WalRecordKey => &[
            CompositeKeyField::VersionByte,
            CompositeKeyField::TenantScope,
            CompositeKeyField::KeyScope,
            CompositeKeyField::WalFamily,
            CompositeKeyField::WalSequence,
        ],
        PhysicalKeyDomain::BlobIdentityKey => &[
            CompositeKeyField::VersionByte,
            CompositeKeyField::TenantScope,
            CompositeKeyField::KeyScope,
            CompositeKeyField::Digest,
            CompositeKeyField::Generation,
        ],
    };

    CompositeKeyOrderingLaw::new(domain, fields)
}
