use worth_store_physical_format::store_namespace::StableStoreIdentity;
use worth_store_physical_format::WalSegmentIdentity;

use super::{PhysicalArtifactScope, PhysicalArtifactScopeIdentity};
use crate::localization::PhysicalByteRange;

impl PhysicalArtifactScope {
    pub const fn wal_frame(
        store: StableStoreIdentity,
        identity: WalSegmentIdentity,
        range: PhysicalByteRange,
    ) -> Self {
        Self::new(
            store,
            PhysicalArtifactScopeIdentity::WalFrame(identity),
            range,
        )
    }

    pub const fn wal_segment_identity(self) -> Option<WalSegmentIdentity> {
        match self.identity {
            PhysicalArtifactScopeIdentity::WalFrame(identity) => Some(identity),
            _ => None,
        }
    }
}
