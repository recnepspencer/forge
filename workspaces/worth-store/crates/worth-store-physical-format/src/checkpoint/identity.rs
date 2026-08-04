use std::num::NonZeroU64;

use crate::store_namespace::{ProposedStoreIdentity, StableStoreIdentity};

use super::record::{read_u64, CheckpointStreamDecodeDenial};

/// Path-independent identity of one checkpoint attempt and artifact.
///
/// This is format meaning only. It grants no publication or retention authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalCheckpointIdentity {
    store: StableStoreIdentity,
    sequence: NonZeroU64,
}

pub(super) fn encode_identity(target: &mut [u8], identity: PhysicalCheckpointIdentity) {
    target[..16].copy_from_slice(&identity.store_identity().bytes());
    target[16..24].copy_from_slice(&identity.sequence().get().to_le_bytes());
}

pub(super) fn decode_identity(
    bytes: &[u8],
) -> Result<PhysicalCheckpointIdentity, CheckpointStreamDecodeDenial> {
    let proposed = ProposedStoreIdentity::from_nonzero_bytes(bytes[..16].try_into().unwrap())
        .ok_or(CheckpointStreamDecodeDenial::InvalidIdentity)?;
    let sequence = NonZeroU64::new(read_u64(bytes, 16))
        .ok_or(CheckpointStreamDecodeDenial::InvalidIdentity)?;
    Ok(PhysicalCheckpointIdentity::new(
        StableStoreIdentity::from_published_record(proposed),
        sequence,
    ))
}

impl PhysicalCheckpointIdentity {
    pub const fn new(store: StableStoreIdentity, sequence: NonZeroU64) -> Self {
        Self { store, sequence }
    }

    pub const fn store_identity(self) -> StableStoreIdentity {
        self.store
    }

    pub const fn sequence(self) -> NonZeroU64 {
        self.sequence
    }
}
