use worth_store_physical_format::store_namespace::StableStoreIdentity;
use worth_store_physical_format::{
    PhysicalRecordFormatDeclaration, RootRoutingBlockScopeIdentity,
    SegmentMembershipBlockScopeIdentity,
};

use super::{PhysicalArtifactScope, PhysicalArtifactScopeIdentity};
use crate::localization::PhysicalByteRange;

impl PhysicalArtifactScope {
    pub const fn root_routing_block(
        store: StableStoreIdentity,
        record_format: PhysicalRecordFormatDeclaration,
        identity: RootRoutingBlockScopeIdentity,
        range: PhysicalByteRange,
    ) -> Self {
        Self::new(
            store,
            PhysicalArtifactScopeIdentity::RootRoutingBlock {
                record_format,
                identity,
            },
            range,
        )
    }

    pub const fn segment_membership_block(
        store: StableStoreIdentity,
        record_format: PhysicalRecordFormatDeclaration,
        identity: SegmentMembershipBlockScopeIdentity,
        range: PhysicalByteRange,
    ) -> Self {
        Self::new(
            store,
            PhysicalArtifactScopeIdentity::SegmentMembershipBlock {
                record_format,
                identity,
            },
            range,
        )
    }

    pub const fn root_routing_block_identity(self) -> Option<RootRoutingBlockScopeIdentity> {
        match self.identity {
            PhysicalArtifactScopeIdentity::RootRoutingBlock { identity, .. } => Some(identity),
            _ => None,
        }
    }

    pub const fn segment_membership_block_identity(
        self,
    ) -> Option<SegmentMembershipBlockScopeIdentity> {
        match self.identity {
            PhysicalArtifactScopeIdentity::SegmentMembershipBlock { identity, .. } => {
                Some(identity)
            }
            _ => None,
        }
    }
}
