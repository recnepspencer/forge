use super::identity::{decode_identity, encode_identity};
use super::record::{read_u64, CheckpointStreamDecodeDenial};
use super::PhysicalCheckpointIdentity;

pub(super) const HEADER_PAYLOAD_BYTES: usize = 72;
pub const CHECKPOINT_STREAM_HEADER_RECORD_BYTES: usize = 92;
const CONCURRENT_MUTATION_POSTURE: u8 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CheckpointWalSourceRange {
    admitted_begin_lsn: u64,
    covered_end_lsn_exclusive: u64,
}

impl CheckpointWalSourceRange {
    pub const fn new(admitted_begin_lsn: u64, covered_end_lsn_exclusive: u64) -> Option<Self> {
        if admitted_begin_lsn >= covered_end_lsn_exclusive {
            return None;
        }
        Some(Self {
            admitted_begin_lsn,
            covered_end_lsn_exclusive,
        })
    }

    pub const fn admitted_begin_lsn(self) -> u64 {
        self.admitted_begin_lsn
    }

    pub const fn covered_end_lsn_exclusive(self) -> u64 {
        self.covered_end_lsn_exclusive
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CheckpointRootBasis {
    generation: u64,
    tree_identity: u64,
}

impl CheckpointRootBasis {
    pub const fn new(generation: u64, tree_identity: u64) -> Self {
        Self {
            generation,
            tree_identity,
        }
    }

    pub const fn generation(self) -> u64 {
        self.generation
    }

    pub const fn tree_identity(self) -> u64 {
        self.tree_identity
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalCheckpointSource {
    identity: PhysicalCheckpointIdentity,
    wal: CheckpointWalSourceRange,
    root: CheckpointRootBasis,
    dirty_generation_frontier: u64,
}

impl PhysicalCheckpointSource {
    pub const fn concurrent(
        identity: PhysicalCheckpointIdentity,
        wal: CheckpointWalSourceRange,
        root: CheckpointRootBasis,
        dirty_generation_frontier: u64,
    ) -> Self {
        Self {
            identity,
            wal,
            root,
            dirty_generation_frontier,
        }
    }

    pub const fn identity(self) -> PhysicalCheckpointIdentity {
        self.identity
    }

    pub const fn wal(self) -> CheckpointWalSourceRange {
        self.wal
    }

    pub const fn root(self) -> CheckpointRootBasis {
        self.root
    }

    pub const fn dirty_generation_frontier(self) -> u64 {
        self.dirty_generation_frontier
    }
}

pub(super) fn encode_header(source: PhysicalCheckpointSource) -> [u8; HEADER_PAYLOAD_BYTES] {
    let mut payload = [0; HEADER_PAYLOAD_BYTES];
    encode_identity(&mut payload[..24], source.identity());
    payload[24..32].copy_from_slice(&source.wal().admitted_begin_lsn().to_le_bytes());
    payload[32..40].copy_from_slice(&source.wal().covered_end_lsn_exclusive().to_le_bytes());
    payload[40..48].copy_from_slice(&source.root().generation().to_le_bytes());
    payload[48..56].copy_from_slice(&source.root().tree_identity().to_le_bytes());
    payload[56..64].copy_from_slice(&source.dirty_generation_frontier().to_le_bytes());
    payload[64] = CONCURRENT_MUTATION_POSTURE;
    payload
}

pub(super) fn decode_header(
    payload: &[u8],
) -> Result<PhysicalCheckpointSource, CheckpointStreamDecodeDenial> {
    if payload[65..72] != [0; 7] {
        return Err(CheckpointStreamDecodeDenial::ReservedFieldNonZero);
    }
    let identity = decode_identity(&payload[..24])?;
    let wal = CheckpointWalSourceRange::new(read_u64(payload, 24), read_u64(payload, 32))
        .ok_or(CheckpointStreamDecodeDenial::InvalidWalRange)?;
    let root = CheckpointRootBasis::new(read_u64(payload, 40), read_u64(payload, 48));
    if payload[64] != CONCURRENT_MUTATION_POSTURE {
        return Err(CheckpointStreamDecodeDenial::InvalidCapturePosture(
            payload[64],
        ));
    }
    Ok(PhysicalCheckpointSource::concurrent(
        identity,
        wal,
        root,
        read_u64(payload, 56),
    ))
}
