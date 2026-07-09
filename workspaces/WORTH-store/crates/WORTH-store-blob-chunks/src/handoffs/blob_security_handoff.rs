use worth_store_security::{
    S51AdmittedSecurityScopeReadiness, StoreSecurityScopeAdmissionReceipt,
    StoreSecurityScopeIdentity,
};

use crate::{
    BlobChunkSecurityMetadataWitness, BlobChunkSecurityScope, BlobChunkSecurityScopeDenial,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S7BlobChunkSecurityPermission {
    metadata: BlobChunkSecurityMetadataWitness,
}

impl S7BlobChunkSecurityPermission {
    pub const fn identity(self) -> StoreSecurityScopeIdentity {
        self.metadata.identity()
    }

    pub const fn metadata(self) -> BlobChunkSecurityMetadataWitness {
        self.metadata
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct S7BlobChunkSecurityHandoff {
    permission: S7BlobChunkSecurityPermission,
    scope: BlobChunkSecurityScope,
}

impl S7BlobChunkSecurityHandoff {
    pub fn from_s5_1_readiness(
        readiness: S51AdmittedSecurityScopeReadiness,
    ) -> Result<Self, BlobChunkSecurityScopeDenial> {
        BlobChunkSecurityScope::from_s5_1_readiness(readiness).map(Self::from_blob_security_scope)
    }

    pub(crate) fn from_blob_security_scope(scope: BlobChunkSecurityScope) -> Self {
        Self {
            permission: S7BlobChunkSecurityPermission {
                metadata: scope.metadata(),
            },
            scope,
        }
    }

    pub const fn permission(&self) -> S7BlobChunkSecurityPermission {
        self.permission
    }

    pub const fn receipt(&self) -> StoreSecurityScopeAdmissionReceipt {
        self.scope.receipt()
    }

    pub(crate) fn into_scope(self) -> BlobChunkSecurityScope {
        self.scope
    }
}
