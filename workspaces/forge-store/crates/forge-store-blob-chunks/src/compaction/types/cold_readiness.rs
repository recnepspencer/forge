use forge_store_tiering::{cold_posture_permits_compaction, ColdPlacementState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobCompactionColdReadiness {
    Available(ColdPlacementState),
    Unavailable(ColdPlacementState),
}

impl BlobCompactionColdReadiness {
    pub const fn from_state(state: ColdPlacementState) -> Self {
        if cold_posture_permits_compaction(state) {
            Self::Available(state)
        } else {
            Self::Unavailable(state)
        }
    }

    pub const fn permits_compaction(self) -> bool {
        matches!(self, Self::Available(_))
    }

    pub const fn state(self) -> ColdPlacementState {
        match self {
            Self::Available(state) | Self::Unavailable(state) => state,
        }
    }
}

#[allow(dead_code)]
fn _cold_is_part_of_the_boundary(_: BlobCompactionColdReadiness) {}
