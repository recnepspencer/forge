use super::proofs::{
    ChunkShapeVersion, EquivalenceContractVersion, MaxAdmittedAspectSlicesPerRead,
    MaxAdmittedBlockDecodeBreadth, MaxAdmittedControlReplayBreadthForParity,
    MaxDeterministicChunkWidth,
};

pub const LAYOUT_FAMILY_VERSION: u32 = 1;
pub const STRUCTURAL_BLOCK_FAMILY_VERSION: u32 = 1;
pub const CHUNK_SHAPE_VERSION: ChunkShapeVersion = ChunkShapeVersion::new(1);
pub const EQUIVALENCE_CONTRACT_VERSION: EquivalenceContractVersion =
    EquivalenceContractVersion::new(1);

pub const FIRST_SHIP_MAX_ADMITTED_ASPECT_SLICES_PER_READ: MaxAdmittedAspectSlicesPerRead =
    MaxAdmittedAspectSlicesPerRead::new(32);
pub const FIRST_SHIP_MAX_ADMITTED_BLOCK_DECODE_BREADTH: MaxAdmittedBlockDecodeBreadth =
    MaxAdmittedBlockDecodeBreadth::new(32);
pub const FIRST_SHIP_MAX_ADMITTED_CONTROL_REPLAY_BREADTH_FOR_PARITY:
    MaxAdmittedControlReplayBreadthForParity =
    MaxAdmittedControlReplayBreadthForParity::new(32);
pub const FIRST_SHIP_MAX_DETERMINISTIC_CHUNK_WIDTH: MaxDeterministicChunkWidth =
    MaxDeterministicChunkWidth::new(16);
