use crate::blob_basis::BlobIdentityKeyBasis;
use crate::catalog::ArtifactFamilyDenial;
use crate::catalog::ArtifactScopePartitionWitness;
use crate::keyspace::{
    admit_blob_identity_key, admit_extent_address_key, admit_page_address_key,
    admit_physical_reference_key, admit_root_manifest_key, admit_segment_address_key,
    admit_wal_record_key, canonical_bytes_for_key, compare_concrete_physical_keys,
    declare_comparator_law, declare_composite_key_ordering, declare_hash_collision_law,
    declare_physical_key_domain, declare_tenant_scoped_key_domain, hash_digest_for_key,
    prefix_bytes_for_key, prefix_successor_bytes, range_end_bytes_for_key,
    range_start_bytes_for_key, require_canonical_key_encoding, require_exact_hash_identity_claim,
    require_prefix_law, require_range_bound_law, verify_hash_identity, CanonicalKeyBytes,
    CanonicalKeyEncoding, ComparatorLaw, CompositeKeyOrderingLaw, ConcretePhysicalKeyWitness,
    HashCollisionLaw, PhysicalKeyDomainWitness, PrefixLawWitness, RangeBoundLawWitness,
    TenantScopedKeyDomain,
};
use forge_store_contracts::WalRecordFamily;
use forge_store_physical_format::{
    PhysicalExtentId, PhysicalPageId, PhysicalReferenceAdmissionWitness, PhysicalRootReference,
    PhysicalSegmentId,
};
use forge_store_wal::StoreWalRecordIdentity;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyDomainLawFacade;

impl KeyDomainLawFacade {
    pub fn declare_physical_key_domain(
        &self,
        scope: ArtifactScopePartitionWitness,
    ) -> crate::keyspace::KeyDomainAdmissionOutcome {
        crate::keyspace::issue_key_domain_admission(declare_physical_key_domain(scope))
    }

    pub fn require_canonical_key_encoding(
        &self,
        domain: PhysicalKeyDomainWitness,
    ) -> CanonicalKeyEncoding {
        require_canonical_key_encoding(domain)
    }

    pub fn declare_comparator_law(&self, encoding: CanonicalKeyEncoding) -> ComparatorLaw {
        declare_comparator_law(encoding)
    }

    pub fn require_range_bound_law(
        &self,
        comparator: ComparatorLaw,
    ) -> Result<RangeBoundLawWitness, ArtifactFamilyDenial> {
        require_range_bound_law(comparator)
    }

    pub fn require_prefix_law(
        &self,
        encoding: CanonicalKeyEncoding,
    ) -> Result<PrefixLawWitness, ArtifactFamilyDenial> {
        require_prefix_law(encoding)
    }

    pub fn declare_hash_collision_law(&self, domain: PhysicalKeyDomainWitness) -> HashCollisionLaw {
        declare_hash_collision_law(domain)
    }

    pub fn require_exact_hash_identity_claim(
        &self,
        law: HashCollisionLaw,
    ) -> Result<HashCollisionLaw, ArtifactFamilyDenial> {
        require_exact_hash_identity_claim(law)
    }

    pub fn declare_composite_key_ordering(
        &self,
        domain: PhysicalKeyDomainWitness,
    ) -> CompositeKeyOrderingLaw {
        declare_composite_key_ordering(domain)
    }

    pub fn declare_tenant_scoped_key_domain(
        &self,
        domain: PhysicalKeyDomainWitness,
    ) -> TenantScopedKeyDomain {
        declare_tenant_scoped_key_domain(domain)
    }

    pub fn admit_root_manifest_key(
        &self,
        domain: PhysicalKeyDomainWitness,
        root_reference: PhysicalRootReference,
    ) -> Result<ConcretePhysicalKeyWitness, ArtifactFamilyDenial> {
        admit_root_manifest_key(domain, root_reference)
    }

    pub fn admit_page_address_key(
        &self,
        domain: PhysicalKeyDomainWitness,
        segment_id: PhysicalSegmentId,
        page_id: PhysicalPageId,
    ) -> Result<ConcretePhysicalKeyWitness, ArtifactFamilyDenial> {
        admit_page_address_key(domain, segment_id, page_id)
    }

    pub fn admit_segment_address_key(
        &self,
        domain: PhysicalKeyDomainWitness,
        segment_id: PhysicalSegmentId,
    ) -> Result<ConcretePhysicalKeyWitness, ArtifactFamilyDenial> {
        admit_segment_address_key(domain, segment_id)
    }

    pub fn admit_extent_address_key(
        &self,
        domain: PhysicalKeyDomainWitness,
        segment_id: PhysicalSegmentId,
        extent_id: PhysicalExtentId,
    ) -> Result<ConcretePhysicalKeyWitness, ArtifactFamilyDenial> {
        admit_extent_address_key(domain, segment_id, extent_id)
    }

    pub fn admit_physical_reference_key(
        &self,
        domain: PhysicalKeyDomainWitness,
        reference: PhysicalReferenceAdmissionWitness,
    ) -> Result<ConcretePhysicalKeyWitness, ArtifactFamilyDenial> {
        admit_physical_reference_key(domain, reference)
    }

    pub fn admit_wal_record_key(
        &self,
        domain: PhysicalKeyDomainWitness,
        family: WalRecordFamily,
        sequence: StoreWalRecordIdentity,
    ) -> Result<ConcretePhysicalKeyWitness, ArtifactFamilyDenial> {
        admit_wal_record_key(domain, family, sequence)
    }

    pub fn admit_blob_identity_key(
        &self,
        domain: PhysicalKeyDomainWitness,
        identity: BlobIdentityKeyBasis,
    ) -> Result<ConcretePhysicalKeyWitness, ArtifactFamilyDenial> {
        admit_blob_identity_key(domain, identity)
    }

    pub fn canonical_key_bytes(
        &self,
        comparator: ComparatorLaw,
        key: ConcretePhysicalKeyWitness,
    ) -> Result<CanonicalKeyBytes, ArtifactFamilyDenial> {
        canonical_bytes_for_key(comparator, key)
    }

    pub fn compare_concrete_keys(
        &self,
        comparator: ComparatorLaw,
        left: ConcretePhysicalKeyWitness,
        right: ConcretePhysicalKeyWitness,
    ) -> Result<Ordering, ArtifactFamilyDenial> {
        compare_concrete_physical_keys(comparator, left, right)
    }

    pub fn range_start_bytes(
        &self,
        law: RangeBoundLawWitness,
        key: ConcretePhysicalKeyWitness,
    ) -> Result<CanonicalKeyBytes, ArtifactFamilyDenial> {
        range_start_bytes_for_key(law, key)
    }

    pub fn range_end_bytes(
        &self,
        law: RangeBoundLawWitness,
        key: ConcretePhysicalKeyWitness,
    ) -> Result<CanonicalKeyBytes, ArtifactFamilyDenial> {
        range_end_bytes_for_key(law, key)
    }

    pub fn prefix_bytes(
        &self,
        law: PrefixLawWitness,
        key: ConcretePhysicalKeyWitness,
    ) -> Result<CanonicalKeyBytes, ArtifactFamilyDenial> {
        prefix_bytes_for_key(law, key)
    }

    pub fn prefix_successor_bytes(&self, prefix: &CanonicalKeyBytes) -> CanonicalKeyBytes {
        prefix_successor_bytes(prefix)
    }

    pub fn hash_digest_for_key(
        &self,
        law: HashCollisionLaw,
        key: ConcretePhysicalKeyWitness,
    ) -> Result<u64, ArtifactFamilyDenial> {
        hash_digest_for_key(law, key)
    }

    pub fn verify_hash_identity(
        &self,
        law: HashCollisionLaw,
        left: ConcretePhysicalKeyWitness,
        right: ConcretePhysicalKeyWitness,
    ) -> Result<(), ArtifactFamilyDenial> {
        verify_hash_identity(law, left, right)
    }
}

pub const fn key_domain_law() -> KeyDomainLawFacade {
    KeyDomainLawFacade
}
