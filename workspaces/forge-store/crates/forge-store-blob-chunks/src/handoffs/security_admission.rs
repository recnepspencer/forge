use forge_store_security::{
    StoreAdmittedSecurityScope, StoreSecurityScopeAdmissionReceipt, StoreSecurityScopeIdentity,
};

use crate::{
    BlobChunkSecurityMetadataWitness, BlobChunkSecurityScope, BlobChunkSecurityScopeDenial,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobChunkSecurityPermission {
    metadata: BlobChunkSecurityMetadataWitness,
}

impl BlobChunkSecurityPermission {
    pub const fn identity(self) -> StoreSecurityScopeIdentity {
        self.metadata.identity()
    }

    pub const fn metadata(self) -> BlobChunkSecurityMetadataWitness {
        self.metadata
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct AdmittedBlobChunkSecurity {
    permission: BlobChunkSecurityPermission,
    scope: BlobChunkSecurityScope,
}

impl AdmittedBlobChunkSecurity {
    pub fn from_admitted_security_scope(
        security_scope: StoreAdmittedSecurityScope,
    ) -> Result<Self, BlobChunkSecurityScopeDenial> {
        BlobChunkSecurityScope::from_admitted_security_scope(security_scope)
            .map(Self::from_blob_security_scope)
    }

    pub(crate) fn from_blob_security_scope(scope: BlobChunkSecurityScope) -> Self {
        Self {
            permission: BlobChunkSecurityPermission {
                metadata: scope.metadata(),
            },
            scope,
        }
    }

    pub const fn permission(&self) -> BlobChunkSecurityPermission {
        self.permission
    }

    pub const fn receipt(&self) -> StoreSecurityScopeAdmissionReceipt {
        self.scope.receipt()
    }

    pub(crate) fn into_scope(self) -> BlobChunkSecurityScope {
        self.scope
    }
}
