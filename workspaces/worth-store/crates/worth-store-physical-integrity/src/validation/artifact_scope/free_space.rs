use worth_store_physical_format::store_namespace::StableStoreIdentity;
use worth_store_physical_format::{
    FreeSpaceHeaderScopeIdentity, FreeSpaceMembershipBlockScopeIdentity,
    PhysicalRecordFormatDeclaration,
};

use super::{PhysicalArtifactScope, PhysicalArtifactScopeIdentity};
use crate::localization::PhysicalByteRange;

impl PhysicalArtifactScope {
    pub const fn free_space_header(
        store: StableStoreIdentity,
        record_format: PhysicalRecordFormatDeclaration,
        identity: FreeSpaceHeaderScopeIdentity,
        range: PhysicalByteRange,
    ) -> Self {
        Self::new(
            store,
            PhysicalArtifactScopeIdentity::FreeSpaceHeader {
                record_format,
                identity,
            },
            range,
        )
    }

    pub const fn free_space_membership_block(
        store: StableStoreIdentity,
        record_format: PhysicalRecordFormatDeclaration,
        identity: FreeSpaceMembershipBlockScopeIdentity,
        range: PhysicalByteRange,
    ) -> Self {
        Self::new(
            store,
            PhysicalArtifactScopeIdentity::FreeSpaceMembershipBlock {
                record_format,
                identity,
            },
            range,
        )
    }

    pub const fn free_space_header_identity(self) -> Option<FreeSpaceHeaderScopeIdentity> {
        match self.identity {
            PhysicalArtifactScopeIdentity::FreeSpaceHeader { identity, .. } => Some(identity),
            _ => None,
        }
    }

    pub const fn free_space_membership_block_identity(
        self,
    ) -> Option<FreeSpaceMembershipBlockScopeIdentity> {
        match self.identity {
            PhysicalArtifactScopeIdentity::FreeSpaceMembershipBlock { identity, .. } => {
                Some(identity)
            }
            _ => None,
        }
    }
}
