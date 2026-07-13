use forge_store_security::StoreCurrentSecurityScopeWitnessSet;

use super::{
    declare_comparator_law, declare_composite_key_ordering, declare_hash_collision_law,
    declare_physical_key_domain, declare_tenant_scoped_key_domain, require_canonical_key_encoding,
    require_prefix_law, require_range_bound_law, CanonicalKeyEncoding, ComparatorLaw,
    CompositeKeyOrderingLaw, HashCollisionLaw, PhysicalKeyDomainWitness, PrefixLawWitness,
    RangeBoundLawWitness, TenantScopedKeyDomain,
};
use crate::{
    catalog::{
        declare_authority_role, declare_derived_accuracy_class, require_scope_partition,
        ArtifactFamilyDenial,
    },
    AdmittedPhysicalArtifactFamily,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedPhysicalKeyDomain {
    family: AdmittedPhysicalArtifactFamily,
    domain: PhysicalKeyDomainWitness,
    encoding: CanonicalKeyEncoding,
    comparator: ComparatorLaw,
    prefix: Option<PrefixLawWitness>,
    range: Option<RangeBoundLawWitness>,
    hash_collision: HashCollisionLaw,
    composite_ordering: CompositeKeyOrderingLaw,
    tenant_partition: TenantScopedKeyDomain,
}

impl AdmittedPhysicalKeyDomain {
    pub(crate) fn admit(
        family: AdmittedPhysicalArtifactFamily,
        security: &StoreCurrentSecurityScopeWitnessSet,
    ) -> Result<Self, ArtifactFamilyDenial> {
        if family.security_identity() != security.key_scope().identity() {
            return Err(ArtifactFamilyDenial::SecurityAuthorityMismatch);
        }
        if family.authority_identity() != security.authority_identity() {
            return Err(ArtifactFamilyDenial::SecurityAuthorityMismatch);
        }
        let role = declare_authority_role(family.classification());
        let accuracy = declare_derived_accuracy_class(role);
        let scope = require_scope_partition(accuracy, security)?;
        let domain = declare_physical_key_domain(scope)?;
        let encoding = require_canonical_key_encoding(domain);
        let comparator = declare_comparator_law(encoding);
        Ok(Self {
            family,
            domain,
            encoding,
            comparator,
            prefix: require_prefix_law(encoding).ok(),
            range: require_range_bound_law(comparator).ok(),
            hash_collision: declare_hash_collision_law(domain),
            composite_ordering: declare_composite_key_ordering(domain),
            tenant_partition: declare_tenant_scoped_key_domain(domain),
        })
    }

    pub const fn family(self) -> AdmittedPhysicalArtifactFamily {
        self.family
    }

    pub const fn witness(self) -> PhysicalKeyDomainWitness {
        self.domain
    }

    pub const fn domain(self) -> super::PhysicalKeyDomain {
        self.domain.domain()
    }

    pub const fn encoding(self) -> CanonicalKeyEncoding {
        self.encoding
    }

    pub const fn comparator(self) -> ComparatorLaw {
        self.comparator
    }

    pub const fn prefix(self) -> Option<PrefixLawWitness> {
        self.prefix
    }

    pub const fn range(self) -> Option<RangeBoundLawWitness> {
        self.range
    }

    pub const fn hash_collision(self) -> HashCollisionLaw {
        self.hash_collision
    }

    pub const fn composite_ordering(self) -> CompositeKeyOrderingLaw {
        self.composite_ordering
    }

    pub const fn tenant_partition(self) -> TenantScopedKeyDomain {
        self.tenant_partition
    }
}
