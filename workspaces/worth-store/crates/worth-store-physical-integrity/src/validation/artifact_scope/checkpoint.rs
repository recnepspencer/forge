use worth_store_physical_format::store_namespace::StableStoreIdentity;
use worth_store_physical_format::PhysicalCheckpointIdentity;

use super::{PhysicalArtifactScope, PhysicalArtifactScopeIdentity};
use crate::localization::PhysicalByteRange;

/// Expected identity available before a checkpoint stream header is inspected.
///
/// The sequence may be omitted only for the header because it exists inside
/// that record's checksummed framing. All later record scopes require the
/// canonical checkpoint identity exposed by an admitted header.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckpointStreamHeaderScopeIdentity {
    StagedFromChecksummedStream(StableStoreIdentity),
    Known(PhysicalCheckpointIdentity),
}

impl CheckpointStreamHeaderScopeIdentity {
    pub const fn staged(store: StableStoreIdentity) -> Self {
        Self::StagedFromChecksummedStream(store)
    }

    pub const fn known(identity: PhysicalCheckpointIdentity) -> Self {
        Self::Known(identity)
    }

    pub const fn store_identity(self) -> StableStoreIdentity {
        match self {
            Self::StagedFromChecksummedStream(store) => store,
            Self::Known(identity) => identity.store_identity(),
        }
    }

    pub const fn checkpoint_identity(self) -> Option<PhysicalCheckpointIdentity> {
        match self {
            Self::StagedFromChecksummedStream(_) => None,
            Self::Known(identity) => Some(identity),
        }
    }
}

impl PhysicalArtifactScope {
    pub const fn checkpoint_stream_header(
        identity: CheckpointStreamHeaderScopeIdentity,
        range: PhysicalByteRange,
    ) -> Self {
        Self::new(
            identity.store_identity(),
            PhysicalArtifactScopeIdentity::CheckpointStreamHeader(identity),
            range,
        )
    }

    pub const fn checkpoint_dirty_basis(
        identity: PhysicalCheckpointIdentity,
        range: PhysicalByteRange,
    ) -> Self {
        Self::checkpoint_record(
            identity,
            PhysicalArtifactScopeIdentity::CheckpointDirtyBasis(identity),
            range,
        )
    }

    pub const fn checkpoint_binding_compaction(
        identity: PhysicalCheckpointIdentity,
        range: PhysicalByteRange,
    ) -> Self {
        Self::checkpoint_record(
            identity,
            PhysicalArtifactScopeIdentity::CheckpointBindingCompaction(identity),
            range,
        )
    }

    pub const fn checkpoint_binding(
        identity: PhysicalCheckpointIdentity,
        range: PhysicalByteRange,
    ) -> Self {
        Self::checkpoint_record(
            identity,
            PhysicalArtifactScopeIdentity::CheckpointBinding(identity),
            range,
        )
    }

    pub const fn checkpoint_footer(
        identity: PhysicalCheckpointIdentity,
        range: PhysicalByteRange,
    ) -> Self {
        Self::checkpoint_record(
            identity,
            PhysicalArtifactScopeIdentity::CheckpointFooter(identity),
            range,
        )
    }

    const fn checkpoint_record(
        identity: PhysicalCheckpointIdentity,
        scope_identity: PhysicalArtifactScopeIdentity,
        range: PhysicalByteRange,
    ) -> Self {
        Self::new(identity.store_identity(), scope_identity, range)
    }

    pub const fn checkpoint_stream_header_identity(
        self,
    ) -> Option<CheckpointStreamHeaderScopeIdentity> {
        match self.identity {
            PhysicalArtifactScopeIdentity::CheckpointStreamHeader(identity) => Some(identity),
            _ => None,
        }
    }

    pub const fn checkpoint_identity(self) -> Option<PhysicalCheckpointIdentity> {
        match self.identity {
            PhysicalArtifactScopeIdentity::CheckpointStreamHeader(identity) => {
                identity.checkpoint_identity()
            }
            PhysicalArtifactScopeIdentity::CheckpointDirtyBasis(identity)
            | PhysicalArtifactScopeIdentity::CheckpointBindingCompaction(identity)
            | PhysicalArtifactScopeIdentity::CheckpointBinding(identity)
            | PhysicalArtifactScopeIdentity::CheckpointFooter(identity) => Some(identity),
            _ => None,
        }
    }
}
