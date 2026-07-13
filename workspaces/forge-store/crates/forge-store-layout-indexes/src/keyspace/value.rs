use super::declaration::{PhysicalKeyDomain, PhysicalKeyDomainWitness};
use super::AdmittedPhysicalKeyDomain;
use crate::blob_basis::BlobIdentityKeyBasis;
use crate::catalog::ArtifactFamilyDenial;
use forge_store_contracts::WalRecordFamily;
use forge_store_physical_format::{
    PhysicalExtentId, PhysicalPageId, PhysicalReference, PhysicalReferenceAdmissionWitness,
    PhysicalRootReference, PhysicalSegmentId,
};
use forge_store_security::{StoreKeyScope, StoreTenantScope};
use forge_store_wal::StoreWalRecordIdentity;

#[derive(Debug, Clone, PartialEq, Eq)]
enum ConcretePhysicalKey {
    RootManifest {
        root_reference: PhysicalRootReference,
    },
    PageAddress {
        segment_id: PhysicalSegmentId,
        page_id: PhysicalPageId,
    },
    SegmentAddress {
        segment_id: PhysicalSegmentId,
    },
    ExtentAddress {
        segment_id: PhysicalSegmentId,
        extent_id: PhysicalExtentId,
    },
    PhysicalReference {
        admission: PhysicalReferenceAdmissionWitness,
    },
    WalRecord {
        family: WalRecordFamily,
        sequence: StoreWalRecordIdentity,
    },
    BlobIdentity {
        identity: BlobIdentityKeyBasis,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConcretePhysicalKeyWitness {
    domain: PhysicalKeyDomainWitness,
    key: ConcretePhysicalKey,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedConcretePhysicalKey {
    domain: AdmittedPhysicalKeyDomain,
    key: ConcretePhysicalKeyWitness,
}

impl AdmittedConcretePhysicalKey {
    fn new(domain: AdmittedPhysicalKeyDomain, key: ConcretePhysicalKeyWitness) -> Self {
        Self { domain, key }
    }

    pub const fn domain(&self) -> AdmittedPhysicalKeyDomain {
        self.domain
    }

    pub(crate) fn into_raw(self) -> ConcretePhysicalKeyWitness {
        self.key
    }
}

impl ConcretePhysicalKeyWitness {
    const fn new(domain: PhysicalKeyDomainWitness, key: ConcretePhysicalKey) -> Self {
        Self { domain, key }
    }

    pub const fn domain(&self) -> PhysicalKeyDomainWitness {
        self.domain
    }

    pub const fn tenant_scope(&self) -> StoreTenantScope {
        self.domain.scope().admitted_tenant_scope()
    }

    pub const fn key_scope(&self) -> StoreKeyScope {
        self.domain.scope().admitted_key_scope()
    }

    pub const fn root_reference(&self) -> Option<PhysicalRootReference> {
        match self.key {
            ConcretePhysicalKey::RootManifest { root_reference } => Some(root_reference),
            _ => None,
        }
    }

    pub const fn page_address(&self) -> Option<(PhysicalSegmentId, PhysicalPageId)> {
        match self.key {
            ConcretePhysicalKey::PageAddress {
                segment_id,
                page_id,
            } => Some((segment_id, page_id)),
            _ => None,
        }
    }

    pub const fn segment_id(&self) -> Option<PhysicalSegmentId> {
        match self.key {
            ConcretePhysicalKey::SegmentAddress { segment_id }
            | ConcretePhysicalKey::PageAddress { segment_id, .. }
            | ConcretePhysicalKey::ExtentAddress { segment_id, .. } => Some(segment_id),
            _ => None,
        }
    }

    pub const fn extent_address(&self) -> Option<(PhysicalSegmentId, PhysicalExtentId)> {
        match self.key {
            ConcretePhysicalKey::ExtentAddress {
                segment_id,
                extent_id,
            } => Some((segment_id, extent_id)),
            _ => None,
        }
    }

    pub const fn physical_reference(&self) -> Option<PhysicalReference> {
        match self.key {
            ConcretePhysicalKey::PhysicalReference { admission } => Some(admission.reference()),
            _ => None,
        }
    }

    pub const fn wal_record(&self) -> Option<(WalRecordFamily, StoreWalRecordIdentity)> {
        match self.key {
            ConcretePhysicalKey::WalRecord { family, sequence } => Some((family, sequence)),
            _ => None,
        }
    }

    pub fn blob_identity(&self) -> Option<&BlobIdentityKeyBasis> {
        match &self.key {
            ConcretePhysicalKey::BlobIdentity { identity } => Some(identity),
            _ => None,
        }
    }
}

pub(crate) fn admit_root_manifest_key(
    domain: PhysicalKeyDomainWitness,
    root_reference: PhysicalRootReference,
) -> Result<ConcretePhysicalKeyWitness, ArtifactFamilyDenial> {
    match domain.domain() {
        PhysicalKeyDomain::RootManifestKey => Ok(ConcretePhysicalKeyWitness::new(
            domain,
            ConcretePhysicalKey::RootManifest { root_reference },
        )),
        _ => Err(ArtifactFamilyDenial::ConcreteKeyKindDoesNotMatchPhysicalKeyDomain),
    }
}

pub(crate) fn admit_page_address_key(
    domain: PhysicalKeyDomainWitness,
    segment_id: PhysicalSegmentId,
    page_id: PhysicalPageId,
) -> Result<ConcretePhysicalKeyWitness, ArtifactFamilyDenial> {
    match domain.domain() {
        PhysicalKeyDomain::PageAddressKey => Ok(ConcretePhysicalKeyWitness::new(
            domain,
            ConcretePhysicalKey::PageAddress {
                segment_id,
                page_id,
            },
        )),
        _ => Err(ArtifactFamilyDenial::ConcreteKeyKindDoesNotMatchPhysicalKeyDomain),
    }
}

pub(crate) fn admit_segment_address_key(
    domain: PhysicalKeyDomainWitness,
    segment_id: PhysicalSegmentId,
) -> Result<ConcretePhysicalKeyWitness, ArtifactFamilyDenial> {
    match domain.domain() {
        PhysicalKeyDomain::SegmentAddressKey => Ok(ConcretePhysicalKeyWitness::new(
            domain,
            ConcretePhysicalKey::SegmentAddress { segment_id },
        )),
        _ => Err(ArtifactFamilyDenial::ConcreteKeyKindDoesNotMatchPhysicalKeyDomain),
    }
}

pub(crate) fn admit_extent_address_key(
    domain: PhysicalKeyDomainWitness,
    segment_id: PhysicalSegmentId,
    extent_id: PhysicalExtentId,
) -> Result<ConcretePhysicalKeyWitness, ArtifactFamilyDenial> {
    match domain.domain() {
        PhysicalKeyDomain::ExtentAddressKey => Ok(ConcretePhysicalKeyWitness::new(
            domain,
            ConcretePhysicalKey::ExtentAddress {
                segment_id,
                extent_id,
            },
        )),
        _ => Err(ArtifactFamilyDenial::ConcreteKeyKindDoesNotMatchPhysicalKeyDomain),
    }
}

pub(crate) fn admit_physical_reference_key(
    domain: PhysicalKeyDomainWitness,
    admission: PhysicalReferenceAdmissionWitness,
) -> Result<ConcretePhysicalKeyWitness, ArtifactFamilyDenial> {
    match domain.domain() {
        PhysicalKeyDomain::PhysicalReferenceKey => Ok(ConcretePhysicalKeyWitness::new(
            domain,
            ConcretePhysicalKey::PhysicalReference { admission },
        )),
        _ => Err(ArtifactFamilyDenial::ConcreteKeyKindDoesNotMatchPhysicalKeyDomain),
    }
}

pub(crate) fn admit_wal_record_key(
    domain: PhysicalKeyDomainWitness,
    family: WalRecordFamily,
    sequence: StoreWalRecordIdentity,
) -> Result<ConcretePhysicalKeyWitness, ArtifactFamilyDenial> {
    match domain.domain() {
        PhysicalKeyDomain::WalRecordKey => Ok(ConcretePhysicalKeyWitness::new(
            domain,
            ConcretePhysicalKey::WalRecord { family, sequence },
        )),
        _ => Err(ArtifactFamilyDenial::ConcreteKeyKindDoesNotMatchPhysicalKeyDomain),
    }
}

pub(crate) fn admit_blob_identity_key(
    domain: PhysicalKeyDomainWitness,
    identity: BlobIdentityKeyBasis,
) -> Result<ConcretePhysicalKeyWitness, ArtifactFamilyDenial> {
    match domain.domain() {
        PhysicalKeyDomain::BlobIdentityKey => Ok(ConcretePhysicalKeyWitness::new(
            domain,
            ConcretePhysicalKey::BlobIdentity { identity },
        )),
        _ => Err(ArtifactFamilyDenial::ConcreteKeyKindDoesNotMatchPhysicalKeyDomain),
    }
}

pub(crate) fn admit_page_key(
    domain: AdmittedPhysicalKeyDomain,
    segment_id: PhysicalSegmentId,
    page_id: PhysicalPageId,
) -> Result<AdmittedConcretePhysicalKey, ArtifactFamilyDenial> {
    admit_page_address_key(domain.witness(), segment_id, page_id)
        .map(|key| AdmittedConcretePhysicalKey::new(domain, key))
}

#[cfg(test)]
pub(crate) fn admit_root_key(
    domain: AdmittedPhysicalKeyDomain,
    root_reference: PhysicalRootReference,
) -> Result<AdmittedConcretePhysicalKey, ArtifactFamilyDenial> {
    admit_root_manifest_key(domain.witness(), root_reference)
        .map(|key| AdmittedConcretePhysicalKey::new(domain, key))
}

#[cfg(test)]
pub(crate) fn admit_segment_key(
    domain: AdmittedPhysicalKeyDomain,
    segment_id: PhysicalSegmentId,
) -> Result<AdmittedConcretePhysicalKey, ArtifactFamilyDenial> {
    admit_segment_address_key(domain.witness(), segment_id)
        .map(|key| AdmittedConcretePhysicalKey::new(domain, key))
}

pub(crate) fn admit_wal_key(
    domain: AdmittedPhysicalKeyDomain,
    family: WalRecordFamily,
    sequence: StoreWalRecordIdentity,
) -> Result<AdmittedConcretePhysicalKey, ArtifactFamilyDenial> {
    admit_wal_record_key(domain.witness(), family, sequence)
        .map(|key| AdmittedConcretePhysicalKey::new(domain, key))
}

pub(crate) fn admit_blob_key(
    domain: AdmittedPhysicalKeyDomain,
    identity: BlobIdentityKeyBasis,
) -> Result<AdmittedConcretePhysicalKey, ArtifactFamilyDenial> {
    admit_blob_identity_key(domain.witness(), identity)
        .map(|key| AdmittedConcretePhysicalKey::new(domain, key))
}
