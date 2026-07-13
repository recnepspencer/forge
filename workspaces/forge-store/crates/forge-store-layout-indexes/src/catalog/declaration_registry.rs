#[cfg(test)]
use crate::{
    blob_basis::BlobIdentityKeyBasis,
    catalog::{
        classify_family, declare_authority_role, declare_derived_accuracy_class,
        require_exact_accuracy_claim, require_production_authority, require_scope_partition,
        require_strategy_lifecycle, ArtifactAuthorityRoleWitness, ArtifactDerivedAccuracyWitness,
        ArtifactFamilyAuthorityWitness, ArtifactFamilyClassification,
        ArtifactFamilyLifecycleAdmission, ArtifactScopePartitionWitness,
    },
    keyspace::{
        admit_blob_identity_key, admit_physical_reference_key, admit_root_manifest_key,
        compare_concrete_physical_keys, declare_composite_key_ordering, declare_hash_collision_law,
        declare_physical_key_domain, declare_tenant_scoped_key_domain, hash_digest_for_key,
        prefix_bytes_for_key, prefix_successor_bytes, range_end_bytes_for_key,
        range_start_bytes_for_key, require_exact_hash_identity_claim, require_prefix_law,
        require_range_bound_law, verify_hash_identity, CompositeKeyOrderingLaw, HashCollisionLaw,
        PrefixLawWitness, RangeBoundLawWitness, TenantScopedKeyDomain,
    },
};
use crate::{
    catalog::{
        ArtifactFamilyDenial, ArtifactFamilyInventory, ExistingArtifactFamilySurface,
        PhysicalArtifactFamilyDeclaration,
    },
    keyspace::{
        admit_page_address_key, admit_wal_record_key, canonical_bytes_for_key,
        declare_comparator_law, require_canonical_key_encoding, CanonicalKeyBytes,
        CanonicalKeyEncoding, ComparatorLaw, ConcretePhysicalKeyWitness, PhysicalKeyDomainWitness,
    },
};
use forge_store_contracts::{DurableArtifactFamilyId, WalRecordFamily};
use forge_store_physical_format::{PhysicalPageId, PhysicalSegmentId};
#[cfg(test)]
use forge_store_physical_format::{PhysicalReferenceAdmissionWitness, PhysicalRootReference};
use forge_store_security::StoreCurrentSecurityScopeWitnessSet;
use forge_store_wal::StoreWalRecordIdentity;
#[cfg(test)]
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutDeclarationsFacade;

impl LayoutDeclarationsFacade {
    pub fn admit_physical_key_domain(
        &self,
        family: crate::AdmittedPhysicalArtifactFamily,
        security: &StoreCurrentSecurityScopeWitnessSet,
    ) -> Result<crate::keyspace::AdmittedPhysicalKeyDomain, ArtifactFamilyDenial> {
        crate::keyspace::AdmittedPhysicalKeyDomain::admit(family, security)
    }

    pub fn admit_physical_artifact_family(
        &self,
        declaration: &'static PhysicalArtifactFamilyDeclaration,
        security: &StoreCurrentSecurityScopeWitnessSet,
    ) -> Result<crate::artifact_family::AdmittedPhysicalArtifactFamily, ArtifactFamilyDenial> {
        crate::artifact_family::AdmittedPhysicalArtifactFamily::admit(declaration, security)
    }

    pub const fn artifact_families(&self) -> ArtifactFamilyInventory {
        ArtifactFamilyInventory::current()
    }

    pub fn declaration(
        &self,
        family_id: DurableArtifactFamilyId,
    ) -> Result<&'static PhysicalArtifactFamilyDeclaration, ArtifactFamilyDenial> {
        self.artifact_families().declaration(family_id)
    }

    pub fn admit_existing_family(
        &self,
        family: &impl ExistingArtifactFamilySurface,
    ) -> Result<&'static PhysicalArtifactFamilyDeclaration, ArtifactFamilyDenial> {
        self.artifact_families().admit_existing_family(family)
    }

    #[cfg(test)]
    pub(crate) fn require_production_authority(
        &self,
        classification: ArtifactFamilyClassification,
    ) -> Result<ArtifactFamilyAuthorityWitness, ArtifactFamilyDenial> {
        require_production_authority(classification)
    }

    #[cfg(test)]
    pub(crate) fn require_strategy_lifecycle(
        &self,
        authority: ArtifactFamilyAuthorityWitness,
    ) -> Result<ArtifactFamilyLifecycleAdmission, ArtifactFamilyDenial> {
        require_strategy_lifecycle(authority)
    }

    #[cfg(test)]
    pub(crate) fn classify_family(
        &self,
        declaration: &'static PhysicalArtifactFamilyDeclaration,
    ) -> ArtifactFamilyClassification {
        classify_family(declaration)
    }

    #[cfg(test)]
    pub(crate) fn declare_authority_role(
        &self,
        classification: ArtifactFamilyClassification,
    ) -> ArtifactAuthorityRoleWitness {
        declare_authority_role(classification)
    }

    #[cfg(test)]
    pub(crate) fn declare_derived_accuracy_class(
        &self,
        role: ArtifactAuthorityRoleWitness,
    ) -> ArtifactDerivedAccuracyWitness {
        declare_derived_accuracy_class(role)
    }

    #[cfg(test)]
    pub(crate) fn require_exact_accuracy_claim(
        &self,
        accuracy: ArtifactDerivedAccuracyWitness,
    ) -> Result<ArtifactDerivedAccuracyWitness, ArtifactFamilyDenial> {
        require_exact_accuracy_claim(accuracy)
    }

    #[cfg(test)]
    pub(crate) fn require_scope_partition(
        &self,
        accuracy: ArtifactDerivedAccuracyWitness,
        security_scope: &StoreCurrentSecurityScopeWitnessSet,
    ) -> Result<ArtifactScopePartitionWitness, ArtifactFamilyDenial> {
        require_scope_partition(accuracy, security_scope)
    }

    #[cfg(test)]
    pub(crate) fn declare_physical_key_domain(
        &self,
        scope: ArtifactScopePartitionWitness,
    ) -> crate::keyspace::KeyDomainAdmissionOutcome {
        crate::keyspace::issue_key_domain_admission(declare_physical_key_domain(scope))
    }

    pub(crate) fn require_canonical_key_encoding(
        &self,
        domain: PhysicalKeyDomainWitness,
    ) -> CanonicalKeyEncoding {
        require_canonical_key_encoding(domain)
    }

    pub(crate) fn declare_comparator_law(&self, encoding: CanonicalKeyEncoding) -> ComparatorLaw {
        declare_comparator_law(encoding)
    }

    #[cfg(test)]
    pub(crate) fn require_range_bound_law(
        &self,
        comparator: ComparatorLaw,
    ) -> Result<RangeBoundLawWitness, ArtifactFamilyDenial> {
        require_range_bound_law(comparator)
    }

    #[cfg(test)]
    pub(crate) fn require_prefix_law(
        &self,
        encoding: CanonicalKeyEncoding,
    ) -> Result<PrefixLawWitness, ArtifactFamilyDenial> {
        require_prefix_law(encoding)
    }

    #[cfg(test)]
    pub(crate) fn declare_hash_collision_law(
        &self,
        domain: PhysicalKeyDomainWitness,
    ) -> HashCollisionLaw {
        declare_hash_collision_law(domain)
    }

    #[cfg(test)]
    pub(crate) fn require_exact_hash_identity_claim(
        &self,
        law: HashCollisionLaw,
    ) -> Result<HashCollisionLaw, ArtifactFamilyDenial> {
        require_exact_hash_identity_claim(law)
    }

    #[cfg(test)]
    pub(crate) fn declare_composite_key_ordering(
        &self,
        domain: PhysicalKeyDomainWitness,
    ) -> CompositeKeyOrderingLaw {
        declare_composite_key_ordering(domain)
    }

    #[cfg(test)]
    pub(crate) fn declare_tenant_scoped_key_domain(
        &self,
        domain: PhysicalKeyDomainWitness,
    ) -> TenantScopedKeyDomain {
        declare_tenant_scoped_key_domain(domain)
    }

    #[cfg(test)]
    pub(crate) fn admit_root_manifest_key(
        &self,
        domain: PhysicalKeyDomainWitness,
        root_reference: PhysicalRootReference,
    ) -> Result<ConcretePhysicalKeyWitness, ArtifactFamilyDenial> {
        admit_root_manifest_key(domain, root_reference)
    }

    pub(crate) fn admit_page_address_key(
        &self,
        domain: PhysicalKeyDomainWitness,
        segment_id: PhysicalSegmentId,
        page_id: PhysicalPageId,
    ) -> Result<ConcretePhysicalKeyWitness, ArtifactFamilyDenial> {
        admit_page_address_key(domain, segment_id, page_id)
    }

    pub(crate) fn admit_page_key(
        &self,
        domain: crate::AdmittedPhysicalKeyDomain,
        segment_id: PhysicalSegmentId,
        page_id: PhysicalPageId,
    ) -> Result<crate::keyspace::AdmittedConcretePhysicalKey, ArtifactFamilyDenial> {
        crate::keyspace::admit_page_key(domain, segment_id, page_id)
    }

    #[cfg(test)]
    pub(crate) fn admit_physical_reference_key(
        &self,
        domain: PhysicalKeyDomainWitness,
        reference: PhysicalReferenceAdmissionWitness,
    ) -> Result<ConcretePhysicalKeyWitness, ArtifactFamilyDenial> {
        admit_physical_reference_key(domain, reference)
    }

    pub(crate) fn admit_wal_record_key(
        &self,
        domain: PhysicalKeyDomainWitness,
        family: WalRecordFamily,
        sequence: StoreWalRecordIdentity,
    ) -> Result<ConcretePhysicalKeyWitness, ArtifactFamilyDenial> {
        admit_wal_record_key(domain, family, sequence)
    }

    pub(crate) fn admit_wal_key(
        &self,
        domain: crate::AdmittedPhysicalKeyDomain,
        family: WalRecordFamily,
        sequence: StoreWalRecordIdentity,
    ) -> Result<crate::keyspace::AdmittedConcretePhysicalKey, ArtifactFamilyDenial> {
        crate::keyspace::admit_wal_key(domain, family, sequence)
    }

    #[cfg(test)]
    pub(crate) fn admit_blob_identity_key(
        &self,
        domain: PhysicalKeyDomainWitness,
        identity: BlobIdentityKeyBasis,
    ) -> Result<ConcretePhysicalKeyWitness, ArtifactFamilyDenial> {
        admit_blob_identity_key(domain, identity)
    }

    pub(crate) fn canonical_key_bytes(
        &self,
        comparator: ComparatorLaw,
        key: ConcretePhysicalKeyWitness,
    ) -> Result<CanonicalKeyBytes, ArtifactFamilyDenial> {
        canonical_bytes_for_key(comparator, key)
    }

    #[cfg(test)]
    pub(crate) fn compare_concrete_keys(
        &self,
        comparator: ComparatorLaw,
        left: ConcretePhysicalKeyWitness,
        right: ConcretePhysicalKeyWitness,
    ) -> Result<Ordering, ArtifactFamilyDenial> {
        compare_concrete_physical_keys(comparator, left, right)
    }

    #[cfg(test)]
    pub(crate) fn range_start_bytes(
        &self,
        law: RangeBoundLawWitness,
        key: ConcretePhysicalKeyWitness,
    ) -> Result<CanonicalKeyBytes, ArtifactFamilyDenial> {
        range_start_bytes_for_key(law, key)
    }

    #[cfg(test)]
    pub(crate) fn range_end_bytes(
        &self,
        law: RangeBoundLawWitness,
        key: ConcretePhysicalKeyWitness,
    ) -> Result<CanonicalKeyBytes, ArtifactFamilyDenial> {
        range_end_bytes_for_key(law, key)
    }

    #[cfg(test)]
    pub(crate) fn prefix_bytes(
        &self,
        law: PrefixLawWitness,
        key: ConcretePhysicalKeyWitness,
    ) -> Result<CanonicalKeyBytes, ArtifactFamilyDenial> {
        prefix_bytes_for_key(law, key)
    }

    #[cfg(test)]
    pub(crate) fn prefix_successor_bytes(&self, prefix: &CanonicalKeyBytes) -> CanonicalKeyBytes {
        prefix_successor_bytes(prefix)
    }

    #[cfg(test)]
    pub(crate) fn hash_digest_for_key(
        &self,
        law: HashCollisionLaw,
        key: ConcretePhysicalKeyWitness,
    ) -> Result<u64, ArtifactFamilyDenial> {
        hash_digest_for_key(law, key)
    }

    #[cfg(test)]
    pub(crate) fn verify_hash_identity(
        &self,
        law: HashCollisionLaw,
        left: ConcretePhysicalKeyWitness,
        right: ConcretePhysicalKeyWitness,
    ) -> Result<(), ArtifactFamilyDenial> {
        verify_hash_identity(law, left, right)
    }

    #[cfg(test)]
    pub(crate) fn seed_family(&self) -> &'static PhysicalArtifactFamilyDeclaration {
        self.declaration(DurableArtifactFamilyId::PhysicalRootManifest)
            .expect("seed family must stay declared")
    }
}

pub const fn layout_declarations() -> LayoutDeclarationsFacade {
    LayoutDeclarationsFacade
}
