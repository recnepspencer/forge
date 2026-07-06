#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobChunkDedupePolicy {
    NoDedupe,
    SameBlobGenerationOnly,
    SameTenantSameKeyScope,
    SameTenantDifferentKeyScopeWithExplicitPolicy,
    CrossTenantDenied,
    CrossTenantExplicitlyAdmittedLater,
}

impl BlobChunkDedupePolicy {
    pub const fn same_tenant_same_key_scope() -> Self {
        Self::SameTenantSameKeyScope
    }

    pub const fn allows_same_scope_sharing(self) -> bool {
        matches!(
            self,
            Self::SameBlobGenerationOnly
                | Self::SameTenantSameKeyScope
                | Self::SameTenantDifferentKeyScopeWithExplicitPolicy
        )
    }

    pub const fn admits_cross_tenant_now(self) -> bool {
        false
    }
}
