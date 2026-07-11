use super::declaration::PhysicalKeyDomainWitness;
use crate::catalog::{ArtifactKeyScopePartition, ArtifactTenantScopePartition};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TenantScopedKeyDomain {
    domain: PhysicalKeyDomainWitness,
    tenant_partition: ArtifactTenantScopePartition,
    key_partition: ArtifactKeyScopePartition,
}

impl TenantScopedKeyDomain {
    pub(crate) const fn new(
        domain: PhysicalKeyDomainWitness,
        tenant_partition: ArtifactTenantScopePartition,
        key_partition: ArtifactKeyScopePartition,
    ) -> Self {
        Self {
            domain,
            tenant_partition,
            key_partition,
        }
    }

    pub const fn domain(self) -> PhysicalKeyDomainWitness {
        self.domain
    }

    pub const fn tenant_partition(self) -> ArtifactTenantScopePartition {
        self.tenant_partition
    }

    pub const fn key_partition(self) -> ArtifactKeyScopePartition {
        self.key_partition
    }
}

pub(crate) const fn declare_tenant_scoped_key_domain(
    domain: PhysicalKeyDomainWitness,
) -> TenantScopedKeyDomain {
    TenantScopedKeyDomain::new(
        domain,
        domain.scope().tenant_partition(),
        domain.scope().key_partition(),
    )
}
