use worth_store_tiering::S7ColdPlacementState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapsuleChunkAvailabilityPosture {
    HotReady,
    ColdReady,
    ColdFetchRequired,
    ColdUnavailable,
    ColdScopeDenied,
    ColdRebindRequired,
}

pub const fn classify_capsule_chunk_availability(
    cold_state: Option<S7ColdPlacementState>,
) -> CapsuleChunkAvailabilityPosture {
    match cold_state {
        None | Some(S7ColdPlacementState::HotAvailable) => {
            CapsuleChunkAvailabilityPosture::HotReady
        }
        Some(S7ColdPlacementState::ColdAvailable) => CapsuleChunkAvailabilityPosture::ColdReady,
        Some(S7ColdPlacementState::ColdFetchRequired)
        | Some(S7ColdPlacementState::ColdFetchInProgress) => {
            CapsuleChunkAvailabilityPosture::ColdFetchRequired
        }
        Some(S7ColdPlacementState::ColdUnavailable) | Some(S7ColdPlacementState::ColdStale) => {
            CapsuleChunkAvailabilityPosture::ColdUnavailable
        }
        Some(S7ColdPlacementState::ColdScopeDenied) => {
            CapsuleChunkAvailabilityPosture::ColdScopeDenied
        }
        Some(S7ColdPlacementState::ColdRebindRequired) => {
            CapsuleChunkAvailabilityPosture::ColdRebindRequired
        }
    }
}
